const express = require('express');
const cors = require('cors');
const { exec } = require('child_process');
const fs = require('fs').promises;
const path = require('path');
const os = require('os');
const multer = require('multer');

const app = express();
const PORT = 3001;

app.use(cors());
app.use(express.json({ limit: '50mb' }));

// Path to FluxMux CLI binary
const FLUXMUX_CLI = path.join(__dirname, '../target/release/fluxmux-cli');
const TEMP_DIR = os.tmpdir();

// Configure multer for file uploads
const storage = multer.diskStorage({
  destination: (req, file, cb) => cb(null, TEMP_DIR),
  filename: (req, file, cb) => cb(null, `${Date.now()}_${file.originalname}`)
});
const upload = multer({ storage });

// Utility function to execute commands
const executeCommand = (command) => {
  return new Promise((resolve, reject) => {
    exec(command, { maxBuffer: 10 * 1024 * 1024 }, (error, stdout, stderr) => {
      if (error) {
        // Capture stderr which often contains the actual error message
        const errorMessage = stderr || stdout || error.message;
        reject({ error: error.message, stderr: errorMessage });
      } else {
        resolve({ stdout, stderr });
      }
    });
  });
};

// Convert endpoint
app.post('/api/convert', async (req, res) => {
  try {
    const { data, fromFormat, toFormat } = req.body;

    if (!data || !fromFormat || !toFormat) {
      return res.status(400).json({ error: 'Missing required fields' });
    }

    // Create temporary files
    const inputFile = path.join(TEMP_DIR, `input_${Date.now()}.${fromFormat}`);
    const outputFile = path.join(TEMP_DIR, `output_${Date.now()}.${toFormat}`);

    try {
      // Write input data to file
      await fs.writeFile(inputFile, data);

      // Execute conversion
      const command = `${FLUXMUX_CLI} convert --from ${fromFormat} --to ${toFormat} ${inputFile} ${outputFile}`;
      await executeCommand(command);

      // Read output
      const output = await fs.readFile(outputFile, 'utf-8');

      // Cleanup
      await fs.unlink(inputFile).catch(() => {});
      await fs.unlink(outputFile).catch(() => {});

      res.json({ success: true, output });
    } catch (error) {
      // Cleanup on error
      await fs.unlink(inputFile).catch(() => {});
      await fs.unlink(outputFile).catch(() => {});
      throw error;
    }
  } catch (error) {
    res.status(500).json({ 
      error: error.stderr || error.error || error.message || 'Conversion failed' 
    });
  }
});

// Bridge endpoint with file upload support
const bridgeUpload = upload.fields([
  { name: 'sourceFile', maxCount: 1 },
  { name: 'schemaFile', maxCount: 1 }
]);

app.post('/api/bridge', bridgeUpload, async (req, res) => {
  let sourceFilePath = null;
  let schemaFilePath = null;

  try {
    const {
      sink,
      batchSize,
      batchTimeoutMs,
      deduplicate,
      throttleRate,
      retryMaxAttempts,
      retryDelayMs
    } = req.body;

    // Get uploaded files
    sourceFilePath = req.files?.sourceFile?.[0]?.path;
    schemaFilePath = req.files?.schemaFile?.[0]?.path;

    if (!sourceFilePath || !sink) {
      return res.status(400).json({ error: 'Source file and sink are required' });
    }

    // Build command using uploaded file path
    let command = `${FLUXMUX_CLI} bridge --source "file:${sourceFilePath}" --sink "${sink}"`;

    if (batchSize) command += ` --batch-size ${batchSize}`;
    if (batchTimeoutMs) command += ` --batch-timeout-ms ${batchTimeoutMs}`;
    if (deduplicate === 'true' || deduplicate === true) command += ` --deduplicate`;
    if (throttleRate) command += ` --throttle-rate ${throttleRate}`;
    if (retryMaxAttempts) command += ` --retry-max-attempts ${retryMaxAttempts}`;
    if (retryDelayMs) command += ` --retry-delay-ms ${retryDelayMs}`;
    if (schemaFilePath) command += ` --schema-path "${schemaFilePath}"`;

    const result = await executeCommand(command);

    // Cleanup uploaded files
    if (sourceFilePath) await fs.unlink(sourceFilePath).catch(() => {});
    if (schemaFilePath) await fs.unlink(schemaFilePath).catch(() => {});

    res.json({
      success: true,
      output: result.stdout || 'Bridge completed successfully'
    });
  } catch (error) {
    // Cleanup on error
    if (sourceFilePath) await fs.unlink(sourceFilePath).catch(() => {});
    if (schemaFilePath) await fs.unlink(schemaFilePath).catch(() => {});

    res.status(500).json({
      error: error.stderr || error.error || error.message || 'Bridge operation failed'
    });
  }
});

// Pipe endpoint with file upload support
const pipeUpload = upload.single('sourceFile');

app.post('/api/pipe', pipeUpload, async (req, res) => {
  let sourceFilePath = null;

  try {
    // Get uploaded file
    sourceFilePath = req.file?.path;

    // Parse JSON strings from form data
    const actions = req.body.actions ? JSON.parse(req.body.actions) : [];
    const sinks = req.body.sinks ? JSON.parse(req.body.sinks) : ['stdout'];

    if (!sourceFilePath) {
      return res.status(400).json({ error: 'Source file is required' });
    }

    // Build command using uploaded file path
    let command = `${FLUXMUX_CLI} pipe "file:${sourceFilePath}"`;

    // Add actions
    if (actions && actions.length > 0) {
      for (const action of actions) {
        command += ` ${action.type}`;

        // Handle aggregate options
        if (action.type === 'aggregate') {
          if (action.groupBy) command += ` --group-by ${action.groupBy}`;
          if (action.avg) command += ` --avg ${action.avg}`;
          if (action.sum) command += ` --sum ${action.sum}`;
          if (action.min) command += ` --min ${action.min}`;
          if (action.max) command += ` --max ${action.max}`;
          if (action.count) command += ` --count`;
        } else if (action.param) {
          command += ` '${action.param}'`;
        }
      }
    }

    // Add sinks
    if (sinks && sinks.length > 0) {
      command += ' tee';
      for (const sink of sinks) {
        if (sink) {
          command += ` ${sink}`;
        }
      }
    }

    const result = await executeCommand(command);

    // Cleanup uploaded file
    if (sourceFilePath) await fs.unlink(sourceFilePath).catch(() => {});

    res.json({
      success: true,
      output: result.stdout || 'Pipe completed successfully'
    });
  } catch (error) {
    // Cleanup on error
    if (sourceFilePath) await fs.unlink(sourceFilePath).catch(() => {});

    res.status(500).json({
      error: error.stderr || error.error || error.message || 'Pipe operation failed'
    });
  }
});

// Kafka endpoint
app.post('/api/kafka', async (req, res) => {
  try {
    const { topic, broker, group, mode, count } = req.body;

    if (!topic) {
      return res.status(400).json({ error: 'Topic is required' });
    }

    // Build command
    let command = `${FLUXMUX_CLI} kafka --topic ${topic}`;
    
    if (broker) command += ` --broker ${broker}`;
    if (group) command += ` --group ${group}`;
    
    if (mode === 'head') {
      command += ` --head ${count || 10}`;
    } else {
      command += ` --tail ${count || 10}`;
    }

    const result = await executeCommand(command);
    
    // Parse output into messages
    const messages = result.stdout
      .split('\n')
      .filter(line => line.trim())
      .map((line, index) => ({
        value: line,
        partition: undefined,
        offset: undefined,
        timestamp: Date.now()
      }));

    res.json({ 
      success: true, 
      messages,
      output: result.stdout 
    });
  } catch (error) {
    res.status(500).json({ 
      error: error.stderr || error.error || error.message || 'Kafka operation failed' 
    });
  }
});

// Health check
app.get('/api/health', (req, res) => {
  res.json({ status: 'ok', message: 'FluxMux API is running' });
});

app.listen(PORT, () => {
  console.log(`FluxMux API server running on http://localhost:${PORT}`);
  console.log(`FluxMux CLI path: ${FLUXMUX_CLI}`);
});

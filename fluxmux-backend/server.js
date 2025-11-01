const express = require('express');
const cors = require('cors');
const { exec } = require('child_process');
const fs = require('fs').promises;
const path = require('path');
const os = require('os');

const app = express();
const PORT = 3001;

app.use(cors());
app.use(express.json({ limit: '50mb' }));

// Path to FluxMux CLI binary
const FLUXMUX_CLI = path.join(__dirname, '../target/release/fluxmux-cli');
const TEMP_DIR = os.tmpdir();

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

// Bridge endpoint
app.post('/api/bridge', async (req, res) => {
  try {
    const {
      source,
      sink,
      batchSize,
      batchTimeoutMs,
      deduplicate,
      throttlePerSec,
      retryMaxAttempts,
      retryDelayMs,
      schemaPath
    } = req.body;

    if (!source || !sink) {
      return res.status(400).json({ error: 'Source and sink are required' });
    }

    // Build command
    let command = `${FLUXMUX_CLI} bridge --source "${source}" --sink "${sink}"`;
    
    if (batchSize) command += ` --batch-size ${batchSize}`;
    if (batchTimeoutMs) command += ` --batch-timeout-ms ${batchTimeoutMs}`;
    if (deduplicate) command += ` --deduplicate`;
    if (throttlePerSec) command += ` --throttle-per-sec ${throttlePerSec}`;
    if (retryMaxAttempts) command += ` --retry-max-attempts ${retryMaxAttempts}`;
    if (retryDelayMs) command += ` --retry-delay-ms ${retryDelayMs}`;
    if (schemaPath) command += ` --schema-path ${schemaPath}`;

    const result = await executeCommand(command);
    res.json({ 
      success: true, 
      output: result.stdout || 'Bridge completed successfully' 
    });
  } catch (error) {
    res.status(500).json({ 
      error: error.stderr || error.error || error.message || 'Bridge operation failed' 
    });
  }
});

// Pipe endpoint
app.post('/api/pipe', async (req, res) => {
  try {
    const { source, actions, sinks } = req.body;

    if (!source) {
      return res.status(400).json({ error: 'Source is required' });
    }

    // Build command
    let command = `${FLUXMUX_CLI} pipe "${source}"`;

    // Add actions
    if (actions && actions.length > 0) {
      for (const action of actions) {
        command += ` ${action.type}`;
        if (action.param) {
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
    res.json({ 
      success: true, 
      output: result.stdout || 'Pipe completed successfully' 
    });
  } catch (error) {
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

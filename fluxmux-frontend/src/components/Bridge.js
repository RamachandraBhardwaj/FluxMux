import React, { useState } from 'react';
import axios from 'axios';
import { FaLink, FaUpload } from 'react-icons/fa';

function Bridge() {
  const [config, setConfig] = useState({
    sink: 'kafka://localhost:9092/vasudeva',
    batchSize: '',
    batchTimeoutMs: '',
    deduplicate: false,
    throttleRate: '',
    retryMaxAttempts: '',
    retryDelayMs: ''
  });
  const [sourceFile, setSourceFile] = useState(null);
  const [schemaFile, setSchemaFile] = useState(null);
  const [output, setOutput] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');

  const handleChange = (field, value) => {
    setConfig(prev => ({ ...prev, [field]: value }));
  };

  const handleRun = async () => {
    if (!sourceFile) {
      setError('Please select a source file');
      return;
    }
    if (!config.sink) {
      setError('Sink is required');
      return;
    }

    setLoading(true);
    setError('');
    setOutput('');

    try {
      const formData = new FormData();
      formData.append('sourceFile', sourceFile);
      formData.append('sink', config.sink);
      formData.append('deduplicate', config.deduplicate);

      if (config.batchSize) formData.append('batchSize', config.batchSize);
      if (config.batchTimeoutMs) formData.append('batchTimeoutMs', config.batchTimeoutMs);
      if (config.throttleRate) formData.append('throttleRate', config.throttleRate);
      if (config.retryMaxAttempts) formData.append('retryMaxAttempts', config.retryMaxAttempts);
      if (config.retryDelayMs) formData.append('retryDelayMs', config.retryDelayMs);
      if (schemaFile) formData.append('schemaFile', schemaFile);

      const response = await axios.post(`${process.env.REACT_APP_API_URL}/api/bridge`, formData, {
        headers: {
          'Content-Type': 'multipart/form-data',
        },
      });
      setOutput(response.data.output);
    } catch (err) {
      setError(err.response?.data?.error || 'Bridge operation failed');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="card">
      <h1 className="card-title">
        <FaLink style={{ marginRight: '0.5rem' }} />
        Bridge Data Pipeline
      </h1>
      <p className="card-description">
        Create production-ready data pipelines with middleware (batching, retry, throttling, schema validation).
      </p>

      <div className="grid grid-2">
        <div className="form-group">
          <label className="form-label">Source File</label>
          <input
            type="file"
            className="form-input"
            accept=".json,.csv,.txt"
            onChange={(e) => setSourceFile(e.target.files[0])}
          />
          {sourceFile && (
            <small style={{ color: '#4CAF50', fontSize: '0.85rem', marginTop: '0.5rem', display: 'block' }}>
              <FaUpload style={{ marginRight: '0.25rem' }} />
              Selected: {sourceFile.name}
            </small>
          )}
        </div>

        <div className="form-group">
          <label className="form-label">Sink</label>
          <input
            type="text"
            className="form-input"
            value={config.sink}
            onChange={(e) => handleChange('sink', e.target.value)}
            placeholder="kafka://host:port/topic | stdout"
          />
          <small style={{ color: '#666', fontSize: '0.85rem' }}>
            Examples: kafka://localhost:9092/vasudeva, stdout
          </small>
        </div>
      </div>

      <h3 style={{ marginTop: '2rem', marginBottom: '1rem', color: '#333' }}>Middleware Options</h3>

      <div className="grid grid-3">
        <div className="form-group">
          <label className="form-label">Batch Size</label>
          <input
            type="text"
            inputMode="numeric"
            className="form-input"
            value={config.batchSize}
            onChange={(e) => handleChange('batchSize', e.target.value.replace(/[^0-9]/g, ''))}
            placeholder="e.g., 10"
            autoComplete="off"
          />
        </div>

        <div className="form-group">
          <label className="form-label">Batch Timeout (ms)</label>
          <input
            type="text"
            inputMode="numeric"
            className="form-input"
            value={config.batchTimeoutMs}
            onChange={(e) => handleChange('batchTimeoutMs', e.target.value.replace(/[^0-9]/g, ''))}
            placeholder="e.g., 1000"
            autoComplete="off"
          />
        </div>

        <div className="form-group">
          <label className="form-label">Throttle Rate</label>
          <input
            type="text"
            inputMode="numeric"
            className="form-input"
            value={config.throttleRate}
            onChange={(e) => handleChange('throttleRate', e.target.value.replace(/[^0-9]/g, ''))}
            placeholder="e.g., 10"
            autoComplete="off"
          />
        </div>

        <div className="form-group">
          <label className="form-label">Retry Max Attempts</label>
          <input
            type="text"
            inputMode="numeric"
            className="form-input"
            value={config.retryMaxAttempts}
            onChange={(e) => handleChange('retryMaxAttempts', e.target.value.replace(/[^0-9]/g, ''))}
            placeholder="e.g., 3"
            autoComplete="off"
          />
        </div>

        <div className="form-group">
          <label className="form-label">Retry Delay (ms)</label>
          <input
            type="text"
            inputMode="numeric"
            className="form-input"
            value={config.retryDelayMs}
            onChange={(e) => handleChange('retryDelayMs', e.target.value.replace(/[^0-9]/g, ''))}
            placeholder="e.g., 1000"
            autoComplete="off"
          />
        </div>

        <div className="form-group">
          <label className="form-label">Schema File (optional)</label>
          <input
            type="file"
            className="form-input"
            accept=".json"
            onChange={(e) => setSchemaFile(e.target.files[0])}
          />
          {schemaFile && (
            <small style={{ color: '#4CAF50', fontSize: '0.85rem' }}>
              {schemaFile.name}
            </small>
          )}
        </div>
      </div>

      <div className="checkbox-group">
        <input
          type="checkbox"
          className="form-checkbox"
          checked={config.deduplicate}
          onChange={(e) => handleChange('deduplicate', e.target.checked)}
          id="deduplicate"
        />
        <label htmlFor="deduplicate">Enable Deduplication</label>
      </div>

      <button
        className="btn btn-primary"
        onClick={handleRun}
        disabled={loading}
      >
        {loading ? <span className="loading"></span> : <FaLink />}
        {loading ? 'Running...' : 'Run Bridge'}
      </button>

      {error && (
        <div className="alert alert-error" style={{ marginTop: '1rem' }}>
          {error}
        </div>
      )}

      {output && (
        <div className="output-container">
          <label className="form-label">Output</label>
          <div className={`output-box ${error ? 'output-error' : 'output-success'}`}>
            {output}
          </div>
        </div>
      )}
    </div>
  );
}

export default Bridge;

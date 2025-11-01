import React, { useState } from 'react';
import axios from 'axios';
import { FaLink } from 'react-icons/fa';

function Bridge() {
  const [config, setConfig] = useState({
    source: 'file:input.json',
    sink: 'kafka://localhost:9092/topic',
    batchSize: '',
    batchTimeoutMs: '',
    deduplicate: false,
    throttlePerSec: '',
    retryMaxAttempts: '',
    retryDelayMs: '',
    schemaPath: ''
  });
  const [output, setOutput] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');

  const handleChange = (field, value) => {
    setConfig(prev => ({ ...prev, [field]: value }));
  };

  const handleRun = async () => {
    setLoading(true);
    setError('');
    setOutput('');

    try {
      const response = await axios.post('http://localhost:3001/api/bridge', config);
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
          <label className="form-label">Source</label>
          <input
            type="text"
            className="form-input"
            value={config.source}
            onChange={(e) => handleChange('source', e.target.value)}
            placeholder="file:input.json | kafka://host:port/topic?group=id | stdin"
          />
          <small style={{ color: '#666', fontSize: '0.85rem' }}>
            Examples: file:input.json, kafka://localhost:9092/topic?group=mygroup, stdin
          </small>
        </div>

        <div className="form-group">
          <label className="form-label">Sink</label>
          <input
            type="text"
            className="form-input"
            value={config.sink}
            onChange={(e) => handleChange('sink', e.target.value)}
            placeholder="file:output.json | kafka://host:port/topic | stdout"
          />
          <small style={{ color: '#666', fontSize: '0.85rem' }}>
            Examples: file:output.json, kafka://localhost:9092/topic, postgres://localhost:5432/db?table=events
          </small>
        </div>
      </div>

      <h3 style={{ marginTop: '2rem', marginBottom: '1rem', color: '#333' }}>Middleware Options</h3>

      <div className="grid grid-3">
        <div className="form-group">
          <label className="form-label">Batch Size</label>
          <input
            type="number"
            className="form-input"
            value={config.batchSize}
            onChange={(e) => handleChange('batchSize', e.target.value)}
            placeholder="e.g., 10"
          />
        </div>

        <div className="form-group">
          <label className="form-label">Batch Timeout (ms)</label>
          <input
            type="number"
            className="form-input"
            value={config.batchTimeoutMs}
            onChange={(e) => handleChange('batchTimeoutMs', e.target.value)}
            placeholder="e.g., 1000"
          />
        </div>

        <div className="form-group">
          <label className="form-label">Throttle (per sec)</label>
          <input
            type="number"
            className="form-input"
            value={config.throttlePerSec}
            onChange={(e) => handleChange('throttlePerSec', e.target.value)}
            placeholder="e.g., 100"
          />
        </div>

        <div className="form-group">
          <label className="form-label">Retry Max Attempts</label>
          <input
            type="number"
            className="form-input"
            value={config.retryMaxAttempts}
            onChange={(e) => handleChange('retryMaxAttempts', e.target.value)}
            placeholder="e.g., 3"
          />
        </div>

        <div className="form-group">
          <label className="form-label">Retry Delay (ms)</label>
          <input
            type="number"
            className="form-input"
            value={config.retryDelayMs}
            onChange={(e) => handleChange('retryDelayMs', e.target.value)}
            placeholder="e.g., 1000"
          />
        </div>

        <div className="form-group">
          <label className="form-label">Schema Path</label>
          <input
            type="text"
            className="form-input"
            value={config.schemaPath}
            onChange={(e) => handleChange('schemaPath', e.target.value)}
            placeholder="schema.json"
          />
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

import React, { useState } from 'react';
import axios from 'axios';
import { FaStream } from 'react-icons/fa';

function Pipe() {
  const [source, setSource] = useState('file:input.json');
  const [actions, setActions] = useState([]);
  const [sinks, setSinks] = useState(['stdout']);
  const [output, setOutput] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');

  const actionTypes = [
    { value: 'filter', label: 'Filter', hasParam: true, placeholder: 'e.g., temperature>30' },
    { value: 'transform', label: 'Transform', hasParam: true, placeholder: 'e.g., fahrenheit=temperature*1.8+32' },
    { value: 'aggregate', label: 'Aggregate', hasParam: false },
    { value: 'normalize', label: 'Normalize', hasParam: false },
    { value: 'validate', label: 'Validate', hasParam: false },
    { value: 'limit', label: 'Limit', hasParam: true, placeholder: 'e.g., 100' },
    { value: 'sample', label: 'Sample', hasParam: true, placeholder: 'e.g., 10' }
  ];

  const addAction = () => {
    setActions([...actions, { type: 'filter', param: '' }]);
  };

  const updateAction = (index, field, value) => {
    const updated = [...actions];
    updated[index][field] = value;
    setActions(updated);
  };

  const removeAction = (index) => {
    setActions(actions.filter((_, i) => i !== index));
  };

  const addSink = () => {
    setSinks([...sinks, '']);
  };

  const updateSink = (index, value) => {
    const updated = [...sinks];
    updated[index] = value;
    setSinks(updated);
  };

  const removeSink = (index) => {
    setSinks(sinks.filter((_, i) => i !== index));
  };

  const handleRun = async () => {
    setLoading(true);
    setError('');
    setOutput('');

    try {
      const response = await axios.post('http://localhost:3001/api/pipe', {
        source,
        actions,
        sinks
      });
      setOutput(response.data.output);
    } catch (err) {
      setError(err.response?.data?.error || 'Pipe operation failed');
    } finally {
      setLoading(false);
    }
  };

  const getActionPlaceholder = (type) => {
    const action = actionTypes.find(a => a.value === type);
    return action?.placeholder || '';
  };

  const actionHasParam = (type) => {
    const action = actionTypes.find(a => a.value === type);
    return action?.hasParam || false;
  };

  return (
    <div className="card">
      <h1 className="card-title">
        <FaStream style={{ marginRight: '0.5rem' }} />
        Pipe Transformations
      </h1>
      <p className="card-description">
        Unix-style inline pipelines with transformation actions: filter, transform, aggregate, and more.
      </p>

      <div className="form-group">
        <label className="form-label">Source</label>
        <input
          type="text"
          className="form-input"
          value={source}
          onChange={(e) => setSource(e.target.value)}
          placeholder="file:input.json | kafka://host:port/topic | stdin"
        />
      </div>

      <h3 style={{ marginTop: '2rem', marginBottom: '1rem', color: '#333' }}>
        Actions
        <button
          className="btn btn-secondary"
          onClick={addAction}
          style={{ marginLeft: '1rem', padding: '0.5rem 1rem' }}
        >
          + Add Action
        </button>
      </h3>

      {actions.map((action, index) => (
        <div key={index} style={{ 
          display: 'flex', 
          gap: '1rem', 
          marginBottom: '1rem',
          padding: '1rem',
          background: '#f8f9fa',
          borderRadius: '8px'
        }}>
          <div style={{ flex: '0 0 200px' }}>
            <select
              className="form-select"
              value={action.type}
              onChange={(e) => updateAction(index, 'type', e.target.value)}
            >
              {actionTypes.map(at => (
                <option key={at.value} value={at.value}>{at.label}</option>
              ))}
            </select>
          </div>
          
          {actionHasParam(action.type) && (
            <div style={{ flex: 1 }}>
              <input
                type="text"
                className="form-input"
                value={action.param}
                onChange={(e) => updateAction(index, 'param', e.target.value)}
                placeholder={getActionPlaceholder(action.type)}
              />
            </div>
          )}

          <button
            className="btn btn-secondary"
            onClick={() => removeAction(index)}
            style={{ padding: '0.5rem 1rem' }}
          >
            Remove
          </button>
        </div>
      ))}

      {actions.length === 0 && (
        <div className="alert alert-info">
          No actions added yet. Click "Add Action" to start building your pipeline.
        </div>
      )}

      <h3 style={{ marginTop: '2rem', marginBottom: '1rem', color: '#333' }}>
        Output Destinations
        <button
          className="btn btn-secondary"
          onClick={addSink}
          style={{ marginLeft: '1rem', padding: '0.5rem 1rem' }}
        >
          + Add Sink
        </button>
      </h3>

      {sinks.map((sink, index) => (
        <div key={index} style={{ 
          display: 'flex', 
          gap: '1rem', 
          marginBottom: '1rem' 
        }}>
          <input
            type="text"
            className="form-input"
            value={sink}
            onChange={(e) => updateSink(index, e.target.value)}
            placeholder="file:output.json | kafka://host:port/topic | stdout"
            style={{ flex: 1 }}
          />
          {sinks.length > 1 && (
            <button
              className="btn btn-secondary"
              onClick={() => removeSink(index)}
              style={{ padding: '0.5rem 1rem' }}
            >
              Remove
            </button>
          )}
        </div>
      ))}

      <button
        className="btn btn-primary"
        onClick={handleRun}
        disabled={loading}
        style={{ marginTop: '1rem' }}
      >
        {loading ? <span className="loading"></span> : <FaStream />}
        {loading ? 'Running...' : 'Run Pipe'}
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

export default Pipe;

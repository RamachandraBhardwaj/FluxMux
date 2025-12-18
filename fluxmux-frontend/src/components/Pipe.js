import React, { useState } from 'react';
import axios from 'axios';
import { FaStream, FaUpload } from 'react-icons/fa';

function Pipe() {
  const [sourceFile, setSourceFile] = useState(null);
  const [actions, setActions] = useState([]);
  const [sinks, setSinks] = useState(['stdout']);
  const [output, setOutput] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');

  const actionTypes = [
    { value: 'filter', label: 'Filter', hasParam: true, placeholder: "e.g., age>23 or name='John'" },
    { value: 'transform', label: 'Transform', hasParam: true, placeholder: 'e.g., category=adult,doubleAge=age*2' },
    { value: 'aggregate', label: 'Aggregate', hasParam: false, hasAggregateOptions: true },
    { value: 'normalize', label: 'Normalize', hasParam: false },
    { value: 'validate', label: 'Validate', hasParam: false },
    { value: 'limit', label: 'Limit', hasParam: true, placeholder: 'e.g., 3' },
    { value: 'sample', label: 'Sample', hasParam: true, placeholder: 'e.g., 10' }
  ];

  const addAction = () => {
    setActions([...actions, {
      type: 'filter',
      param: '',
      groupBy: '',
      avg: '',
      sum: '',
      count: false,
      min: '',
      max: ''
    }]);
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
    if (!sourceFile) {
      setError('Please select a source file');
      return;
    }

    setLoading(true);
    setError('');
    setOutput('');

    try {
      const formData = new FormData();
      formData.append('sourceFile', sourceFile);
      formData.append('actions', JSON.stringify(actions));
      formData.append('sinks', JSON.stringify(sinks));

      const response = await axios.post(`${process.env.REACT_APP_API_URL}/api/pipe`, formData, {
        headers: {
          'Content-Type': 'multipart/form-data',
        },
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

  const actionHasAggregateOptions = (type) => {
    const action = actionTypes.find(a => a.value === type);
    return action?.hasAggregateOptions || false;
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
          marginBottom: '1rem',
          padding: '1rem',
          background: '#f8f9fa',
          borderRadius: '8px'
        }}>
          <div style={{ display: 'flex', gap: '1rem', marginBottom: actionHasAggregateOptions(action.type) ? '1rem' : 0 }}>
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

          {actionHasAggregateOptions(action.type) && (
            <div style={{
              display: 'grid',
              gridTemplateColumns: 'repeat(3, 1fr)',
              gap: '1rem',
              marginTop: '0.5rem'
            }}>
              <div>
                <label style={{ fontSize: '0.85rem', color: '#666' }}>Group By</label>
                <input
                  type="text"
                  className="form-input"
                  value={action.groupBy || ''}
                  onChange={(e) => updateAction(index, 'groupBy', e.target.value)}
                  placeholder="e.g., name"
                />
              </div>
              <div>
                <label style={{ fontSize: '0.85rem', color: '#666' }}>Avg Field</label>
                <input
                  type="text"
                  className="form-input"
                  value={action.avg || ''}
                  onChange={(e) => updateAction(index, 'avg', e.target.value)}
                  placeholder="e.g., age"
                />
              </div>
              <div>
                <label style={{ fontSize: '0.85rem', color: '#666' }}>Sum Field</label>
                <input
                  type="text"
                  className="form-input"
                  value={action.sum || ''}
                  onChange={(e) => updateAction(index, 'sum', e.target.value)}
                  placeholder="e.g., amount"
                />
              </div>
              <div>
                <label style={{ fontSize: '0.85rem', color: '#666' }}>Min Field</label>
                <input
                  type="text"
                  className="form-input"
                  value={action.min || ''}
                  onChange={(e) => updateAction(index, 'min', e.target.value)}
                  placeholder="e.g., price"
                />
              </div>
              <div>
                <label style={{ fontSize: '0.85rem', color: '#666' }}>Max Field</label>
                <input
                  type="text"
                  className="form-input"
                  value={action.max || ''}
                  onChange={(e) => updateAction(index, 'max', e.target.value)}
                  placeholder="e.g., price"
                />
              </div>
              <div style={{ display: 'flex', alignItems: 'center', gap: '0.5rem', paddingTop: '1.5rem' }}>
                <input
                  type="checkbox"
                  id={`count-${index}`}
                  checked={action.count || false}
                  onChange={(e) => updateAction(index, 'count', e.target.checked)}
                />
                <label htmlFor={`count-${index}`} style={{ fontSize: '0.85rem', color: '#666' }}>Include Count</label>
              </div>
            </div>
          )}
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
            placeholder="stdout | kafka://host:port/topic"
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

      <div style={{ marginTop: '2rem', padding: '1rem', background: '#e8f4f8', borderRadius: '8px' }}>
        <h4 style={{ margin: '0 0 0.5rem 0', color: '#333' }}>Examples:</h4>
        <ul style={{ margin: 0, paddingLeft: '1.5rem', color: '#666', fontSize: '0.9rem' }}>
          <li><strong>Filter:</strong> age&gt;23, name='John', temperature&gt;30</li>
          <li><strong>Transform:</strong> category=adult,doubleAge=age*2, fahrenheit=temp*1.8+32</li>
          <li><strong>Aggregate:</strong> Group by name, average age, with count</li>
          <li><strong>Limit:</strong> 3 (return first 3 records)</li>
        </ul>
      </div>
    </div>
  );
}

export default Pipe;

import React, { useState } from 'react';
import axios from 'axios';
import { FaExchangeAlt } from 'react-icons/fa';

function Convert() {
  const [inputData, setInputData] = useState('');
  const [fromFormat, setFromFormat] = useState('json');
  const [toFormat, setToFormat] = useState('yaml');
  const [output, setOutput] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');

  const formats = ['json', 'yaml', 'toml', 'csv'];

  const handleConvert = async () => {
    if (!inputData.trim()) {
      setError('Please enter some data to convert');
      return;
    }

    setLoading(true);
    setError('');
    setOutput('');

    try {
      const response = await axios.post('http://localhost:3001/api/convert', {
        data: inputData,
        fromFormat,
        toFormat
      });

      setOutput(response.data.output);
    } catch (err) {
      setError(err.response?.data?.error || 'Conversion failed. Please check your input and try again.');
    } finally {
      setLoading(false);
    }
  };

  const handleSampleData = () => {
    const samples = {
      json: `[
  {
    "name": "Alice",
    "age": 30,
    "email": "alice@example.com",
    "active": true
  },
  {
    "name": "Bob",
    "age": 25,
    "email": "bob@example.com",
    "active": false
  }
]`,
      yaml: `- name: Alice
  age: 30
  email: alice@example.com
  active: true
- name: Bob
  age: 25
  email: bob@example.com
  active: false`,
      csv: `name,age,email,active
Alice,30,alice@example.com,true
Bob,25,bob@example.com,false`,
      toml: `[[data]]
name = "Alice"
age = 30
email = "alice@example.com"
active = true

[[data]]
name = "Bob"
age = 25
email = "bob@example.com"
active = false`
    };
    setInputData(samples[fromFormat]);
  };

  return (
    <div className="card">
      <h1 className="card-title">
        <FaExchangeAlt style={{ marginRight: '0.5rem' }} />
        Convert Data Formats
      </h1>
      <p className="card-description">
        Convert between JSON, YAML, TOML, and CSV formats seamlessly.
      </p>

      <div className="grid grid-2">
        <div className="form-group">
          <label className="form-label">From Format</label>
          <select
            className="form-select"
            value={fromFormat}
            onChange={(e) => setFromFormat(e.target.value)}
          >
            {formats.map(fmt => (
              <option key={fmt} value={fmt}>{fmt.toUpperCase()}</option>
            ))}
          </select>
        </div>

        <div className="form-group">
          <label className="form-label">To Format</label>
          <select
            className="form-select"
            value={toFormat}
            onChange={(e) => setToFormat(e.target.value)}
          >
            {formats.map(fmt => (
              <option key={fmt} value={fmt}>{fmt.toUpperCase()}</option>
            ))}
          </select>
        </div>
      </div>

      <div className="form-group">
        <label className="form-label">Input Data</label>
        <textarea
          className="form-textarea"
          value={inputData}
          onChange={(e) => setInputData(e.target.value)}
          placeholder={`Enter your ${fromFormat.toUpperCase()} data here...`}
          style={{ minHeight: '300px' }}
        />
      </div>

      <div style={{ display: 'flex', gap: '1rem' }}>
        <button
          className="btn btn-primary"
          onClick={handleConvert}
          disabled={loading}
        >
          {loading ? <span className="loading"></span> : <FaExchangeAlt />}
          {loading ? 'Converting...' : 'Convert'}
        </button>
        <button
          className="btn btn-secondary"
          onClick={handleSampleData}
        >
          Load Sample Data
        </button>
        <button
          className="btn btn-secondary"
          onClick={() => {
            setInputData('');
            setOutput('');
            setError('');
          }}
        >
          Clear
        </button>
      </div>

      {error && (
        <div className="alert alert-error" style={{ marginTop: '1rem' }}>
          {error}
        </div>
      )}

      {output && (
        <div className="output-container">
          <label className="form-label">Output ({toFormat.toUpperCase()})</label>
          <div className={`output-box ${error ? 'output-error' : 'output-success'}`}>
            {output}
          </div>
          <button
            className="btn btn-secondary"
            style={{ marginTop: '1rem' }}
            onClick={() => navigator.clipboard.writeText(output)}
          >
            Copy to Clipboard
          </button>
        </div>
      )}
    </div>
  );
}

export default Convert;

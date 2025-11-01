import React, { useState, useEffect, useRef } from 'react';
import axios from 'axios';
import { FaServer, FaPlay, FaStop } from 'react-icons/fa';

function Kafka() {
  const [config, setConfig] = useState({
    topic: '',
    broker: 'localhost:9092',
    group: 'fluxmux-inspector',
    mode: 'head',
    count: 10
  });
  const [messages, setMessages] = useState([]);
  const [loading, setLoading] = useState(false);
  const [streaming, setStreaming] = useState(false);
  const [error, setError] = useState('');
  const intervalRef = useRef(null);

  const handleChange = (field, value) => {
    setConfig(prev => ({ ...prev, [field]: value }));
  };

  const handleFetch = async () => {
    if (!config.topic.trim()) {
      setError('Please enter a topic name');
      return;
    }

    setLoading(true);
    setError('');
    setMessages([]);

    try {
      const response = await axios.post('http://localhost:3001/api/kafka', config);
      setMessages(response.data.messages || []);
    } catch (err) {
      setError(err.response?.data?.error || 'Failed to fetch Kafka messages');
    } finally {
      setLoading(false);
    }
  };

  const handleStreamStart = () => {
    if (!config.topic.trim()) {
      setError('Please enter a topic name');
      return;
    }

    setStreaming(true);
    setError('');
    
    // Poll every 2 seconds for tail mode
    intervalRef.current = setInterval(async () => {
      try {
        const response = await axios.post('http://localhost:3001/api/kafka', {
          ...config,
          mode: 'tail'
        });
        setMessages(response.data.messages || []);
      } catch (err) {
        console.error('Streaming error:', err);
      }
    }, 2000);
  };

  const handleStreamStop = () => {
    setStreaming(false);
    if (intervalRef.current) {
      clearInterval(intervalRef.current);
      intervalRef.current = null;
    }
  };

  useEffect(() => {
    return () => {
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
      }
    };
  }, []);

  return (
    <div className="card">
      <h1 className="card-title">
        <FaServer style={{ marginRight: '0.5rem' }} />
        Kafka Inspector
      </h1>
      <p className="card-description">
        Fast, real-time Kafka topic inspection with minimal latency. Use head mode for first N messages 
        or tail mode for live monitoring.
      </p>

      <div className="grid grid-2">
        <div className="form-group">
          <label className="form-label">Topic Name *</label>
          <input
            type="text"
            className="form-input"
            value={config.topic}
            onChange={(e) => handleChange('topic', e.target.value)}
            placeholder="e.g., orders, logs, events"
          />
        </div>

        <div className="form-group">
          <label className="form-label">Broker Address</label>
          <input
            type="text"
            className="form-input"
            value={config.broker}
            onChange={(e) => handleChange('broker', e.target.value)}
            placeholder="localhost:9092"
          />
        </div>

        <div className="form-group">
          <label className="form-label">Consumer Group</label>
          <input
            type="text"
            className="form-input"
            value={config.group}
            onChange={(e) => handleChange('group', e.target.value)}
            placeholder="fluxmux-inspector"
          />
        </div>

        <div className="form-group">
          <label className="form-label">Mode</label>
          <select
            className="form-select"
            value={config.mode}
            onChange={(e) => handleChange('mode', e.target.value)}
          >
            <option value="head">Head (First N messages)</option>
            <option value="tail">Tail (Latest N messages)</option>
          </select>
        </div>

        <div className="form-group">
          <label className="form-label">Message Count</label>
          <input
            type="number"
            className="form-input"
            value={config.count}
            onChange={(e) => handleChange('count', e.target.value)}
            placeholder="10"
            min="1"
            max="1000"
          />
        </div>
      </div>

      <div style={{ display: 'flex', gap: '1rem', marginTop: '1rem' }}>
        <button
          className="btn btn-primary"
          onClick={handleFetch}
          disabled={loading || streaming}
        >
          {loading ? <span className="loading"></span> : <FaPlay />}
          {loading ? 'Fetching...' : 'Fetch Messages'}
        </button>

        {config.mode === 'tail' && !streaming && (
          <button
            className="btn btn-primary"
            onClick={handleStreamStart}
            disabled={loading}
          >
            <FaPlay />
            Start Live Monitoring
          </button>
        )}

        {streaming && (
          <button
            className="btn btn-secondary"
            onClick={handleStreamStop}
          >
            <FaStop />
            Stop Monitoring
          </button>
        )}
      </div>

      {streaming && (
        <div className="alert alert-info" style={{ marginTop: '1rem' }}>
          <strong>Live monitoring active</strong> - Messages update every 2 seconds
        </div>
      )}

      {error && (
        <div className="alert alert-error" style={{ marginTop: '1rem' }}>
          {error}
        </div>
      )}

      {messages.length > 0 && (
        <div className="output-container">
          <label className="form-label">
            Messages ({messages.length} total)
          </label>
          <div className="output-box output-success">
            {messages.map((msg, index) => (
              <div key={index} style={{ 
                borderBottom: index < messages.length - 1 ? '1px solid #e0e0e0' : 'none',
                paddingBottom: '1rem',
                marginBottom: '1rem'
              }}>
                <div style={{ 
                  fontSize: '0.85rem', 
                  color: '#666',
                  marginBottom: '0.5rem'
                }}>
                  <strong>Message {index + 1}</strong>
                  {msg.partition !== undefined && ` | Partition: ${msg.partition}`}
                  {msg.offset !== undefined && ` | Offset: ${msg.offset}`}
                  {msg.timestamp && ` | Time: ${new Date(msg.timestamp).toLocaleString()}`}
                </div>
                <pre style={{ 
                  margin: 0,
                  whiteSpace: 'pre-wrap',
                  wordBreak: 'break-word'
                }}>
                  {typeof msg.value === 'string' ? msg.value : JSON.stringify(msg.value, null, 2)}
                </pre>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}

export default Kafka;

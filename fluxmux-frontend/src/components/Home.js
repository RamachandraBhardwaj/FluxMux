import React from 'react';
import { useNavigate } from 'react-router-dom';
import { FaExchangeAlt, FaLink, FaStream, FaServer } from 'react-icons/fa';

function Home() {
  const navigate = useNavigate();

  const features = [
    {
      icon: <FaExchangeAlt className="feature-icon" style={{ color: '#667eea' }} />,
      title: 'Convert',
      description: 'Convert between data formats (JSON, YAML, TOML, CSV)',
      path: '/convert'
    },
    {
      icon: <FaLink className="feature-icon" style={{ color: '#764ba2' }} />,
      title: 'Bridge',
      description: 'Production-ready data pipelines with middleware',
      path: '/bridge'
    },
    {
      icon: <FaStream className="feature-icon" style={{ color: '#667eea' }} />,
      title: 'Pipe',
      description: 'Unix-style pipelines with transformation actions',
      path: '/pipe'
    },
    {
      icon: <FaServer className="feature-icon" style={{ color: '#764ba2' }} />,
      title: 'Kafka',
      description: 'Fast, real-time Kafka topic inspection',
      path: '/kafka'
    }
  ];

  return (
    <div>
      <div className="card">
        <h1 className="card-title">Welcome to FluxMux</h1>
        <p className="card-description">
          Your all-in-one CLI for message queues, streams, databases with file conversions 
          and Unix-style data pipelines. Choose a feature below to get started.
        </p>

        <div className="feature-grid">
          {features.map((feature, index) => (
            <div
              key={index}
              className="feature-card"
              onClick={() => navigate(feature.path)}
            >
              {feature.icon}
              <h3 className="feature-title">{feature.title}</h3>
              <p className="feature-description">{feature.description}</p>
            </div>
          ))}
        </div>
      </div>

      <div className="card">
        <h2 className="card-title">Key Features</h2>
        <div className="card-description">
          <ul style={{ paddingLeft: '1.5rem', lineHeight: '2' }}>
            <li><strong>Multiple Data Sources:</strong> File, Kafka, PostgreSQL, stdin</li>
            <li><strong>Multiple Sinks:</strong> File, Kafka, PostgreSQL, stdout</li>
            <li><strong>Format Support:</strong> JSON, YAML, TOML, CSV, NDJSON</li>
            <li><strong>Production Middleware:</strong> Batching, retry, throttling, deduplication, schema validation</li>
            <li><strong>Inline Transformations:</strong> Filter, transform, aggregate, validate</li>
            <li><strong>Multi-Output:</strong> Tee to multiple destinations simultaneously</li>
          </ul>
        </div>
      </div>
    </div>
  );
}

export default Home;

import React from 'react';
import { BrowserRouter as Router, Routes, Route, Link } from 'react-router-dom';
import './App.css';
import Convert from './components/Convert';
import Bridge from './components/Bridge';
import Pipe from './components/Pipe';
import Kafka from './components/Kafka';
import Home from './components/Home';

function App() {
  return (
    <Router>
      <div className="App">
        <nav className="navbar">
          <div className="nav-container">
            <Link to="/" className="nav-logo">
              FluxMux
            </Link>
            <ul className="nav-menu">
              <li className="nav-item">
                <Link to="/" className="nav-link">Home</Link>
              </li>
              <li className="nav-item">
                <Link to="/convert" className="nav-link">Convert</Link>
              </li>
              <li className="nav-item">
                <Link to="/bridge" className="nav-link">Bridge</Link>
              </li>
              <li className="nav-item">
                <Link to="/pipe" className="nav-link">Pipe</Link>
              </li>
              <li className="nav-item">
                <Link to="/kafka" className="nav-link">Kafka</Link>
              </li>
            </ul>
          </div>
        </nav>

        <div className="content">
          <Routes>
            <Route path="/" element={<Home />} />
            <Route path="/convert" element={<Convert />} />
            <Route path="/bridge" element={<Bridge />} />
            <Route path="/pipe" element={<Pipe />} />
            <Route path="/kafka" element={<Kafka />} />
          </Routes>
        </div>
      </div>
    </Router>
  );
}

export default App;

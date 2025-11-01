# FluxMux Web GUI Setup Guide

## 🎯 Quick Start

### Step 1: Start Backend Server

Open a terminal and run:

```bash
cd /Users/anuragnarsingoju/Academics/FluxMux/fluxmux-backend
npm start
```

You should see:
```
FluxMux API server running on http://localhost:3001
```

### Step 2: Start Frontend React App

Open a **new terminal** and run:

```bash
cd /Users/anuragnarsingoju/Academics/FluxMux/fluxmux-frontend
npm start
```

The app will automatically open in your browser at `http://localhost:3000`

## ✅ What You Get

### 🏠 Home Page
- Beautiful dashboard with feature cards
- Quick navigation to all tools
- Overview of FluxMux capabilities

### 🔄 Convert Page
- Convert between JSON, YAML, TOML, CSV
- Live input/output editors
- Sample data loader
- Copy to clipboard

### 🌉 Bridge Page
- Configure data pipelines
- Source: File, Kafka, stdin
- Sink: File, Kafka, PostgreSQL, stdout
- Middleware: Batching, retry, throttling, deduplication

### 📊 Pipe Page
- Build transformation pipelines
- Actions: Filter, transform, aggregate, limit, sample
- Multiple output destinations

### 📡 Kafka Page
- Inspect Kafka topics
- Head mode: First N messages
- Tail mode: Live monitoring
- Real-time updates every 2 seconds

## 🎨 Features

- Modern, gradient-based UI design
- Responsive layout (works on mobile)
- Real-time updates for Kafka monitoring
- Error handling with helpful messages
- Loading states for all operations
- Clean, intuitive interface

## 🔧 Architecture

```
┌─────────────────┐
│  React Frontend │  Port 3000
│  (fluxmux-frontend)│
└────────┬────────┘
         │ HTTP API
         ▼
┌─────────────────┐
│  Express Backend│  Port 3001
│  (fluxmux-backend)│
└────────┬────────┘
         │ Execute
         ▼
┌─────────────────┐
│  FluxMux CLI    │
│  (Rust binary)  │
└─────────────────┘
```

## 📋 Requirements

- ✅ Node.js installed
- ✅ FluxMux CLI built (`cargo build --release`)
- ✅ Backend dependencies installed
- ✅ Frontend dependencies installed

## 🚀 Usage Examples

### Example 1: Convert JSON to YAML

1. Go to **Convert** page
2. Click "Load Sample Data"
3. Change "To Format" to YAML
4. Click "Convert"
5. See the result in the output box

### Example 2: Simple Bridge

1. Go to **Bridge** page
2. Source: `file:input.json`
3. Sink: `file:output.json`
4. Click "Run Bridge"

### Example 3: Pipe Filter

1. Go to **Pipe** page
2. Source: `file:input.json`
3. Click "Add Action"
4. Select "Filter", enter: `value>100`
5. Click "Run Pipe"

### Example 4: Kafka Inspection (requires Kafka running)

1. Go to **Kafka** page
2. Enter topic name
3. Select mode (Head or Tail)
4. Click "Fetch Messages"

## 🐛 Troubleshooting

### Backend won't start
```bash
# Make sure you're in the right directory
cd /Users/anuragnarsingoju/Academics/FluxMux/fluxmux-backend

# Check if node_modules exists
ls node_modules

# If not, install dependencies
npm install

# Start server
npm start
```

### Frontend won't start
```bash
# Navigate to frontend directory
cd /Users/anuragnarsingoju/Academics/FluxMux/fluxmux-frontend

# Install dependencies if needed
npm install

# Start app
npm start
```

### API Connection Errors

1. **Make sure backend is running** on port 3001
2. Check backend terminal for errors
3. Verify FluxMux CLI path in `server.js`
4. Ensure FluxMux CLI is built: `cargo build --release`

### Port Already in Use

If port 3000 or 3001 is taken:

**Backend (change port):**
Edit `server.js`, change `PORT = 3001` to another port

**Frontend:**
Create `.env` file in `fluxmux-frontend/`:
```
PORT=3002
```

## 📁 File Structure

```
FluxMux/
├── fluxmux-frontend/          # React app
│   ├── src/
│   │   ├── components/        # All page components
│   │   ├── App.js            # Main app with routing
│   │   └── App.css           # Styles
│   └── package.json
│
├── fluxmux-backend/           # Express API server
│   ├── server.js             # Main server file
│   └── package.json
│
└── target/release/            # FluxMux CLI binary
    └── fluxmux-cli
```

## 🎉 Success Checklist

- [ ] Backend running on http://localhost:3001
- [ ] Frontend running on http://localhost:3000
- [ ] Browser shows FluxMux home page
- [ ] Convert page loads sample data
- [ ] No errors in browser console
- [ ] No errors in backend terminal

## 🔐 Security Note

This is a **development setup**. For production:
- Add authentication
- Validate all inputs
- Use environment variables for configuration
- Enable HTTPS
- Implement rate limiting

## 📚 Next Steps

1. Try all four main features
2. Test with your own data
3. Customize the UI in `App.css`
4. Add new features to components
5. Deploy to production (see main README)

---

**Enjoy your FluxMux Web GUI! 🚀**

For CLI documentation, see the main [README.md](../README.md) and [SETUP.md](../SETUP.md)

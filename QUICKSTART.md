# FluxMux GUI - Quick Start Guide

## ✅ Setup Complete!

Your FluxMux Web GUI is ready to use. The errors have been fixed.

## 🚀 How to Start

### Option 1: Manual Start (Recommended for first time)

**Terminal 1 - Backend:**
```bash
cd /Users/anuragnarsingoju/Academics/FluxMux/fluxmux-backend
npm start
```

Wait until you see:
```
FluxMux API server running on http://localhost:3001
```

**Terminal 2 - Frontend:**
```bash
cd /Users/anuragnarsingoju/Academics/FluxMux/fluxmux-frontend
npm start
```

Your browser will automatically open to `http://localhost:3000`

### Option 2: Use the Start Script

```bash
cd /Users/anuragnarsingoju/Academics/FluxMux
./start-gui.sh
```

This will start both servers automatically. Press `Ctrl+C` to stop both.

## 🎨 What You Can Do

### 1. Convert Data Formats
- Go to the **Convert** page
- Try converting JSON ↔ YAML ↔ TOML ↔ CSV
- Click "Load Sample Data" to see examples
- Copy output to clipboard

### 2. Bridge Pipelines
- Go to the **Bridge** page
- Set up data pipelines with middleware
- Configure batching, retry, throttling
- Connect multiple data sources and sinks

### 3. Pipe Transformations
- Go to the **Pipe** page
- Add transformation actions
- Filter, transform, aggregate data
- Send output to multiple destinations

### 4. Kafka Inspector
- Go to the **Kafka** page
- Inspect Kafka topics in real-time
- Use Head mode for first N messages
- Use Tail mode for live monitoring

## 📱 Features

✅ Beautiful, modern UI with gradient design
✅ Responsive layout (works on all devices)
✅ Real-time data processing
✅ Error handling with helpful messages
✅ Sample data for quick testing
✅ Copy to clipboard functionality
✅ Live Kafka monitoring

## 🐛 If Something Goes Wrong

### Frontend won't compile
The icon errors have been fixed. Just restart:
```bash
cd fluxmux-frontend
npm start
```

### Backend can't find FluxMux CLI
Make sure the CLI is built:
```bash
cd /Users/anuragnarsingoju/Academics/FluxMux
cargo build --release
```

### Port already in use
Kill the process using the port:
```bash
# For port 3000 (frontend)
lsof -ti:3000 | xargs kill -9

# For port 3001 (backend)
lsof -ti:3001 | xargs kill -9
```

### Can't connect to backend
1. Check backend terminal - should show "running on http://localhost:3001"
2. Make sure both servers are running
3. Check browser console for error details

## 📂 Project Structure

```
FluxMux/
├── fluxmux-frontend/       # React UI (Port 3000)
│   └── src/
│       ├── components/     # All pages
│       ├── App.js         # Main app
│       └── App.css        # Styles
│
├── fluxmux-backend/        # API Server (Port 3001)
│   └── server.js          # Express server
│
└── target/release/         # Rust binary
    └── fluxmux-cli
```

## 🎯 Quick Test

1. **Start both servers** (see above)
2. **Open browser** to http://localhost:3000
3. **Go to Convert page**
4. **Click "Load Sample Data"**
5. **Click "Convert"**
6. **See the result!**

## 🎉 Success!

If you see the FluxMux home page with four feature cards, everything is working!

---

**Need help?** Check the logs in both terminal windows for error messages.

**Want to customize?** Edit files in `fluxmux-frontend/src/` and refresh the browser.

**Ready to deploy?** See the main documentation for production deployment steps.

Enjoy your FluxMux Web GUI! 🚀

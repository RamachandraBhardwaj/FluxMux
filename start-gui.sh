#!/bin/bash

# FluxMux GUI Launcher
# This script starts both backend and frontend servers

echo "🚀 Starting FluxMux Web GUI..."
echo ""

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if FluxMux CLI is built
if [ ! -f "./target/release/fluxmux-cli" ]; then
    echo -e "${YELLOW}⚠️  FluxMux CLI not found. Building...${NC}"
    cargo build --release
    if [ $? -ne 0 ]; then
        echo -e "${RED}❌ Build failed. Please fix errors and try again.${NC}"
        exit 1
    fi
    echo -e "${GREEN}✅ FluxMux CLI built successfully${NC}"
fi

# Check if backend dependencies are installed
if [ ! -d "./fluxmux-backend/node_modules" ]; then
    echo -e "${YELLOW}📦 Installing backend dependencies...${NC}"
    cd fluxmux-backend && npm install && cd ..
    echo -e "${GREEN}✅ Backend dependencies installed${NC}"
fi

# Check if frontend dependencies are installed
if [ ! -d "./fluxmux-frontend/node_modules" ]; then
    echo -e "${YELLOW}📦 Installing frontend dependencies...${NC}"
    cd fluxmux-frontend && npm install && cd ..
    echo -e "${GREEN}✅ Frontend dependencies installed${NC}"
fi

echo ""
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}✨ Starting FluxMux Web GUI${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo -e "${BLUE}📡 Backend API:${NC}  http://localhost:3001"
echo -e "${BLUE}🌐 Frontend UI:${NC}  http://localhost:3000"
echo ""
echo -e "${YELLOW}Press Ctrl+C to stop both servers${NC}"
echo ""
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

# Trap Ctrl+C to kill both processes
trap 'kill $BACKEND_PID $FRONTEND_PID 2>/dev/null; echo ""; echo "👋 Shutting down..."; exit 0' INT

# Start backend in background
echo -e "${BLUE}🔧 Starting backend server...${NC}"
cd fluxmux-backend
npm start &
BACKEND_PID=$!
cd ..

# Wait a bit for backend to start
sleep 3

# Start frontend in background
echo -e "${BLUE}🎨 Starting frontend app...${NC}"
cd fluxmux-frontend
npm start &
FRONTEND_PID=$!
cd ..

# Wait for both processes
wait

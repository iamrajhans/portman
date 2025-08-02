const express = require('express');
const app = express();
const PORT = process.env.PORT || 3000;

// Middleware to log requests
app.use((req, res, next) => {
    console.log(`${new Date().toISOString()} - ${req.method} ${req.path}`);
    next();
});

// Routes
app.get('/', (req, res) => {
    res.json({
        message: 'Hello from Test Web App!',
        port: PORT,
        timestamp: new Date().toISOString(),
        purpose: 'Testing portman CLI tool'
    });
});

app.get('/health', (req, res) => {
    res.json({
        status: 'healthy',
        uptime: process.uptime(),
        port: PORT
    });
});

app.get('/api/data', (req, res) => {
    res.json({
        data: [
            { id: 1, name: 'Item 1' },
            { id: 2, name: 'Item 2' },
            { id: 3, name: 'Item 3' }
        ],
        count: 3
    });
});

// Start server
app.listen(PORT, () => {
    console.log(`🚀 Test web app running on port ${PORT}`);
    console.log(`📊 Visit http://localhost:${PORT} to see the app`);
    console.log(`🔧 This app is for testing the portman CLI tool`);
    console.log(`⏰ Started at: ${new Date().toISOString()}`);
});

// Graceful shutdown
process.on('SIGTERM', () => {
    console.log('🛑 Received SIGTERM, shutting down gracefully...');
    process.exit(0);
});

process.on('SIGINT', () => {
    console.log('🛑 Received SIGINT, shutting down gracefully...');
    process.exit(0);
});
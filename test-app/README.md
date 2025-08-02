# Test Web App for Portman

This is a simple Express.js web application for testing the `portman` CLI tool.

## Setup

```bash
npm install
npm start
```

The app will run on port 3000 and provide several endpoints:

- `GET /` - Welcome message with app info
- `GET /health` - Health check endpoint
- `GET /api/data` - Sample API endpoint

## Testing with Portman

Once the app is running, you can test various portman commands:

```bash
# Check if port 3000 is available (should show occupied)
cargo run -- check 3000

# List all occupied ports (should show this Node.js process)
cargo run -- list

# List with filter for node processes
cargo run -- list --filter node

# Kill the process on port 3000
cargo run -- kill 3000
```

## Purpose

This app exists solely to demonstrate the portman CLI tool's functionality by providing a real process listening on port 3000.
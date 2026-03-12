# ClawHub Server

Standalone HTTP server for the ClawHub Wasm tool registry.

## Quick Start

```bash
# Run locally
cargo run -p clawhub-server

# With custom configuration
DATABASE_PATH=./my-clawhub.db BIND_ADDR=127.0.0.1:3000 cargo run -p clawhub-server
```

## Configuration

Create a `.env` file (see `.env.example`):

```env
DATABASE_PATH=./clawhub.db
BIND_ADDR=0.0.0.0:8080
RUST_LOG=info
```

## API Endpoints

### Tools

- `GET /api/clawhub/tools` - List all tools
- `GET /api/clawhub/tools/:name` - Get latest version
- `GET /api/clawhub/tools/:name/:version` - Get specific version
- `POST /api/clawhub/tools` - Publish a tool
- `GET /api/clawhub/tools/:name/:version/download` - Download tool

### Search

- `GET /api/clawhub/search?q=calculator` - Search tools

## Docker

```bash
# Build
docker build -t clawhub-server -f deployment/clawhub/Dockerfile .

# Run
docker run -p 8080:8080 -v $(pwd)/data:/data -e DATABASE_PATH=/data/clawhub.db clawhub-server
```

## Production Deployment

See `deployment/clawhub/` for production configuration.

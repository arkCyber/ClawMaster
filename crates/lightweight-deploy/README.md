# clawmaster-lightweight-deploy

Lightweight deployment system for ClawMaster, inspired by MicroClaw's simple deployment experience.

## Features
- ✅ Single binary deployment
- ✅ Configuration templates
- ✅ Quick start scripts
- ✅ Docker containerization
- ✅ Multiple deployment modes
- ✅ System resource checks
- ✅ Health monitoring

## Quick Start

### Installation
```bash
cargo install --path crates/lightweight-deploy --bin clawmaster-lite
```

### Initialize Configuration
```bash
# Create basic configuration
clawmaster-lite init --template basic

# Or choose a specific template
clawmaster-lite init --template development
clawmaster-lite init --template production
clawmaster-lite init --template minimal
clawmaster-lite init --template enterprise
```

### Start Server
```bash
# Start with default settings
clawmaster-lite start

# Start with custom host/port
clawmaster-lite start --host 0.0.0.0 --port 8080

# Start with custom config
clawmaster-lite start --config ./my-config.toml
```

## Deployment Modes

### Auto Mode
```bash
clawmaster-lite --mode auto start
```
Automatically detects environment and configures optimal settings.

### Development Mode
```bash
clawmaster-lite --mode development start
```
Optimized for development with debug logging and hot reload.

### Production Mode
```bash
clawmaster-lite --mode production start
```
Optimized for production with security hardening and performance tuning.

### Minimal Mode
```bash
clawmaster-lite --mode minimal start
```
Only essential features enabled for maximum performance.

### Enterprise Mode
```bash
clawmaster-lite --mode enterprise start
```
All features enabled for enterprise deployments.

## Configuration Templates

### Basic Template
- Discord and Telegram channels
- SQLite database
- Basic authentication
- Web interface enabled

### Development Template
- All channels enabled
- Debug logging
- Local file storage
- Development tools

### Production Template
- PostgreSQL database
- Security hardening
- Performance optimization
- Monitoring enabled

### Minimal Template
- Single channel (Discord)
- SQLite database
- No authentication
- Minimal features

### Enterprise Template
- All channels enabled
- Enterprise features
- Advanced security
- Full monitoring

## Deployment Scripts

### Generate Bash Script
```bash
clawmaster-lite generate-script --script-type bash --output ./deploy.sh
```

### Generate PowerShell Script
```bash
clawmaster-lite generate-script --script-type powershell --output ./deploy.ps1
```

### Generate Docker Compose
```bash
clawmaster-lite generate-script --script-type docker --output ./docker-compose.yml
```

## Docker Deployment

### Build Docker Image
```bash
clawmaster-lite build-docker --name clawmaster-lite --tag latest
```

### Run with Docker
```bash
docker run -d \
  --name clawmaster-lite \
  -p 8080:8080 \
  -v $(pwd)/data:/app/data \
  -v $(pwd)/logs:/app/logs \
  clawmaster-lite:latest
```

### Run with Docker Compose
```bash
docker-compose up -d
```

## Configuration

### Basic Configuration
```toml
[server]
host = "0.0.0.0"
port = 8080
base_url = "http://localhost:8080"

[database]
db_type = "SQLite"
sqlite_path = "./data/clawmaster.db"

[channels]
discord = true
telegram = true
slack = false

[security]
auth_enabled = true
session_timeout_hours = 24

[features]
web_enabled = true
api_enabled = true
agent_loop_enabled = true
chat_catchup_enabled = true
```

### Environment Variables
- `MOLTIS_CONFIG_PATH`: Path to configuration file
- `MOLTIS_LOG_LEVEL`: Logging level (trace, debug, info, warn, error)
- `MOLTIS_MODE`: Deployment mode
- `RUST_LOG`: Rust logging level

## System Requirements

### Minimum Requirements
- **RAM**: 1GB
- **Disk**: 1GB free space
- **OS**: Linux, macOS, or Windows

### Recommended Requirements
- **RAM**: 2GB+
- **Disk**: 5GB+ free space
- **CPU**: 2+ cores

## Commands

### Server Management
```bash
# Start server
clawmaster-lite start

# Validate configuration
clawmaster-lite validate

# Show system status
clawmaster-lite status
```

### Configuration Management
```bash
# Initialize configuration
clawmaster-lite init [--template <template>] [--force]

# Generate deployment script
clawmaster-lite generate-script [--script-type <type>] [--output <file>]

# Build Docker image
clawmaster-lite build-docker [--name <name>] [--tag <tag>]
```

### Global Options
```bash
# Custom configuration file
clawmaster-lite --config ./custom.toml <command>

# Verbose logging
clawmaster-lite --verbose <command>

# Deployment mode
clawmaster-lite --mode <mode> <command>
```

## Health Monitoring

### Health Check Endpoint
```
GET /health
```

### Status Information
```json
{
  "status": "healthy",
  "uptime": "2h 30m 15s",
  "version": "1.0.0",
  "mode": "production",
  "features": {
    "web_enabled": true,
    "api_enabled": true,
    "agent_loop_enabled": true
  }
}
```

## Troubleshooting

### Common Issues

#### Port Already in Use
```bash
# Find available port
clawmaster-lite status

# Use different port
clawmaster-lite start --port 8081
```

#### Configuration Errors
```bash
# Validate configuration
clawmaster-lite validate

# Check system requirements
clawmaster-lite status
```

#### Permission Issues
```bash
# Ensure data directory is writable
mkdir -p ./data
chmod 755 ./data
```

### Debug Mode
```bash
# Enable debug logging
clawmaster-lite --verbose start

# Or set environment variable
RUST_LOG=debug clawmaster-lite start
```

## Examples

### Quick Development Setup
```bash
# Initialize development configuration
clawmaster-lite init --template development

# Start server
clawmaster-lite start

# Server is now available at http://localhost:3000
```

### Production Deployment
```bash
# Initialize production configuration
clawmaster-lite init --template production

# Generate deployment script
clawmaster-lite generate-script --script-type bash --output ./deploy.sh

# Make script executable
chmod +x ./deploy.sh

# Run deployment
./deploy.sh
```

### Docker Deployment
```bash
# Build Docker image
clawmaster-lite build-docker --name my-clawmaster --tag v1.0

# Run container
docker run -d \
  --name clawmaster \
  -p 8080:8080 \
  -v clawmaster-data:/app/data \
  my-clawmaster:v1.0
```

## Architecture

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   CLI Interface │───▶│ Config Manager   │───▶│  Server Core    │
└─────────────────┘    └──────────────────┘    └─────────────────┘
                                │                        │
                                ▼                        ▼
                       ┌──────────────────┐    ┌─────────────────┐
                       │ Template Engine  │    │ Feature Manager │
                       └──────────────────┘    └─────────────────┘
                                │                        │
                                ▼                        ▼
                       ┌──────────────────┐    ┌─────────────────┐
                       │ Deploy Scripts   │    │ Health Monitor  │
                       └──────────────────┘    └─────────────────┘
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## License

Licensed under the same terms as ClawMaster.

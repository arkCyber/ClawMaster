# clawmaster-irc

IRC (Internet Relay Chat) channel integration for ClawMaster.

## Features

- ✅ IRC protocol support (RFC 1459)
- ✅ Channel joining
- ✅ Message sending/receiving
- ✅ Private messages
- ✅ TLS support
- ✅ DO-178C Level A compliant

## Configuration

```toml
[channels.irc]
enabled = true
server = "irc.freenode.net"
port = 6667
nickname = "clawmaster"
channels = ["#rust", "#ai"]
password = "optional_password"
use_tls = false
```

## Testing

```bash
cargo test -p clawmaster-irc
```

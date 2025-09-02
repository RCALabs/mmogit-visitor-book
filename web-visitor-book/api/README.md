# MMOGIT Visitor Book API

Backend API for the MMOGIT visitor book, running on Hetzner server.

## Structure

```
api/
├── src/
│   └── main.rs         # Axum server implementation
├── Cargo.toml          # Rust dependencies
├── visitor-book.service # Systemd service file
└── nginx-visitor-book.conf # Nginx reverse proxy config
```

## Deployment

### Server: api.mmogit.sh

1. **Build and deploy:**
```bash
# On local machine
cargo build --release --manifest-path api/Cargo.toml

# Copy to server
scp target/release/visitor-book-api root@api.mmogit.sh:/root/

# On server
systemctl restart visitor-book
```

2. **Service management:**
```bash
# Start/stop/restart
systemctl start visitor-book
systemctl stop visitor-book
systemctl restart visitor-book

# View logs
journalctl -u visitor-book -f
```

3. **Nginx config:**
```bash
# Copy nginx config
cp nginx-visitor-book.conf /etc/nginx/sites-available/visitor-book
ln -s /etc/nginx/sites-available/visitor-book /etc/nginx/sites-enabled/
nginx -t && systemctl reload nginx
```

## API Endpoints

- `GET /` - Health check
- `GET /count` - Get visitor count
- `POST /sign` - Sign the visitor book

## Dependencies

- mmogit binary at `/root/mmogit/target/release/mmogit`
- mmogit identity at `/root/.mmogit-visitor-book/`
- Git repository for storing encrypted messages

## Environment

The API expects:
- mmogit visitor book identity initialized
- Git repository configured for pushing to GitHub
- Port 3000 available locally
- Nginx proxying from port 80/443
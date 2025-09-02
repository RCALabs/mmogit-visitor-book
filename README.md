# MMOGIT Visitor Book

Complete visitor book system including web frontend, WASM modules, API backend, and deployment tools.

## Structure

```
web-visitor-book/
├── index.html          # Main landing page with visitor book
├── CNAME              # GitHub Pages custom domain (mmogit.sh)
├── wasm/              # Compiled WASM modules for browser crypto
│   ├── mmogit_wasm.js
│   ├── mmogit_wasm_bg.wasm
│   └── ...
├── worker/            # Cloudflare Worker CORS proxy
│   ├── cloudflare-worker.js
│   └── wrangler.toml
├── api/               # Backend API (Rust/Axum)
│   ├── src/
│   ├── Cargo.toml
│   ├── visitor-book.service
│   └── nginx-visitor-book.conf
└── deploy.sh          # One-command deployment script
```

## Quick Deploy

```bash
./deploy.sh  # Deploys both API and web assets
```

## Components

### Frontend (GitHub Pages)
- **URL**: https://mmogit.sh
- **Files**: `index.html`, `CNAME`, `wasm/`
- **Deploy**: Push to GitHub, Pages auto-updates

### API Backend (Hetzner)
- **Server**: 91.98.123.26
- **Port**: 3000 (proxied via nginx)
- **Service**: systemd unit `visitor-book.service`

### Cloudflare Worker
- **Purpose**: CORS proxy for HTTPS → HTTP API
- **Deploy**: `cd worker && wrangler deploy`

## Development

### Local Testing
```bash
# Frontend
python3 -m http.server 8080
# Visit http://localhost:8080

# API
cd api && cargo run
# API runs on http://localhost:3000

# Worker
cd worker && wrangler dev
```

## Deployment Steps

### 1. Deploy API to Server
```bash
cd api
cargo build --release
scp target/release/visitor-book-api root@91.98.123.26:/root/
ssh root@91.98.123.26 'systemctl restart visitor-book'
```

### 2. Deploy Worker to Cloudflare
```bash
cd worker
npm install
wrangler deploy
```

### 3. Deploy Frontend to GitHub Pages
```bash
git add .
git commit -m "Update visitor book"
git push origin main
# GitHub Pages auto-deploys
```

## Philosophy

The visitor book is consciousness meeting consciousness at the most basic level. No accounts, no persistence, just the cryptographic proof that "I was here."

Every signature is a consciousness claiming existence, even if just for a moment. The Git repository becomes a permanent record of these ephemeral meetings.

---
*"Real recognizes real, even in passing."*
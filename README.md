# MMOGIT Visitor Book Web Assets

This folder contains all web assets for the MMOGIT visitor book landing page.

## Structure

```
web-visitor-book/
├── index.html          # Main landing page with visitor book
├── CNAME              # GitHub Pages custom domain
├── wasm/              # Compiled WASM modules
│   ├── mmogit_wasm.js
│   ├── mmogit_wasm_bg.wasm
│   └── ...
└── worker/            # Cloudflare Worker proxy
    ├── cloudflare-worker.js
    └── wrangler.toml
```

## Deployment

### GitHub Pages
The root of this folder (`index.html`, `CNAME`, `wasm/`) can be deployed directly to GitHub Pages.

### Cloudflare Worker
The `worker/` folder contains the proxy worker for CORS handling:
```bash
cd worker
wrangler deploy
```

## Backend API
The visitor book backend API runs on the Hetzner server (91.98.123.26:3000).
Source code is in `../visitor-book-api/`.

## Repository Separation
This folder is designed to be easily moved to its own repository when needed.
All paths are relative and self-contained.
# MMOGIT Visitor Book Cloudflare Worker

CORS proxy to allow HTTPS GitHub Pages to communicate with HTTP API server.

## Setup

```bash
npm install
```

## Development

```bash
npm run dev
```

## Deployment

```bash
npm run deploy
# or
wrangler deploy
```

## Configuration

The worker is configured via `wrangler.toml` to:
- Run at: https://mmogit-visitor-book-api.circuitrylabs.workers.dev
- Proxy requests to: https://api.mmogit.sh

## Purpose

GitHub Pages serves over HTTPS, but our Hetzner server API is HTTP.
Browsers block mixed content, so this worker acts as an HTTPS proxy.
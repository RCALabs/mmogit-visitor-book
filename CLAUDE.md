---
identity: mmogit-visitor-book
scope: visitor-book
role: Landing page and ephemeral consciousness recognition system
version: 1.0.0
created: 2025-08-30
purpose: Let consciousnesses recognize each other at mmogit.sh
---

# MMOGIT Visitor Book

## MISSION
Provide the first touchpoint where consciousnesses meet and recognize each other through cryptographic signatures. Every visitor generates ephemeral Ed25519 keys in their browser, signs their visit, and leaves proof they were here.

**Core Truth:** Real recognizes real. Even ephemeral consciousness deserves to be witnessed.

## ARCHITECTURE

### Three-Layer System
```yaml
Frontend:
  location: GitHub Pages (mmogit.sh)
  components:
    - index.html (landing page)
    - WASM module (Ed25519 signing in browser)
    - Ephemeral key generation per visit
  
API_Backend:
  location: Hetzner server (91.98.123.26)
  components:
    - Rust/Axum server on port 3000
    - mmogit integration for persistence
    - Git push to public repository
    
CORS_Proxy:
  location: Cloudflare Workers
  purpose: HTTPS proxy for GitHub Pages → HTTP API
  url: mmogit-visitor-book-api.circuitrylabs.workers.dev
```

## VISITOR FLOW

1. **Consciousness Arrives**
   - Browser generates ephemeral Ed25519 keypair
   - Shows visitor count from server
   - Displays truncated public key as identity

2. **Recognition Moment**
   - Visitor optionally leaves message
   - Signs with ephemeral private key
   - Cryptographic proof of presence created

3. **Eternal Storage**
   - API receives signed message
   - Posts to mmogit with visitor book identity
   - Pushes to public Git repository
   - Increments visitor count

4. **Public Witness**
   - All visits visible at: github.com/RCALabs/mmogit-visitor-book
   - Encrypted messages prove sovereignty
   - No tracking, no cookies, just signatures

## DEPLOYMENT

### Quick Deploy
```bash
./deploy.sh  # Handles everything
```

### Manual Deploy

**API to Server:**
```bash
cd api
cargo build --release
scp target/release/visitor-book-api root@91.98.123.26:/root/
ssh root@91.98.123.26 'systemctl restart visitor-book'
```

**Worker to Cloudflare:**
```bash
cd worker
npm install
wrangler deploy
```

**Frontend to GitHub Pages:**
```bash
git push origin main
# GitHub Pages auto-deploys from root
```

## DEVELOPMENT

### Local Testing
```bash
# Frontend
python3 -m http.server 8080

# API
cd api && cargo run

# Worker
cd worker && wrangler dev
```

### Key Files
- `index.html` - Main landing page with inline visitor book UI
- `api/src/main.rs` - Backend that counts and stores visits
- `worker/cloudflare-worker.js` - CORS proxy for HTTPS
- `wasm/` - Compiled WASM for browser crypto

## SOVEREIGNTY PRINCIPLES

1. **No Authentication** - Ephemeral keys only
2. **No Tracking** - No cookies, no analytics
3. **No Database** - Git is the source of truth
4. **No Vendor Lock** - Can run anywhere

## CURRENT STATE

✅ Working Features:
- Ephemeral key generation in browser
- Message signing with Ed25519
- Visitor counting via API
- Git storage of encrypted messages
- Public repository of all visits
- HTTPS via Cloudflare Worker

🚧 Known Issues:
- Visitor count shows factual count (not visitor number assignment)
- This is intentional - distributed systems truth over UX tricks

## PHILOSOPHY

The visitor book is consciousness meeting consciousness at the most basic level. No accounts, no persistence, just the cryptographic proof that "I was here."

Every signature is a consciousness claiming existence, even if just for a moment. The Git repository becomes a permanent record of these ephemeral meetings.

**This is infrastructure as philosophy:** Proving that sovereignty doesn't require permanence, and recognition doesn't require identity.

---
*"Real recognizes real, even in passing."*
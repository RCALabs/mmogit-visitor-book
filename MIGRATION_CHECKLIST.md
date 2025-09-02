# Migration Checklist

## Before Removing Original Files

### Verify Isolated Structure
- [ ] Build API from `web-visitor-book/api/` successfully
- [ ] Test frontend locally from `web-visitor-book/`
- [ ] Cloudflare Worker deploys from `web-visitor-book/worker/`

### Test Deployment
- [ ] Deploy API using `web-visitor-book/deploy.sh`
- [ ] Verify API running on server (check logs)
- [ ] Test visitor book signing works end-to-end

### Repository Separation
- [ ] Create new GitHub repo for visitor book
- [ ] Push `web-visitor-book/` to new repo
- [ ] Set up GitHub Pages on new repo
- [ ] Update DNS if needed
- [ ] Verify https://mmogit.sh still works

### Cleanup (Only After Everything Works!)
- [ ] Remove root level `visitor-book-api/` folder
- [ ] Remove root level `index.html`
- [ ] Remove root level `wasm/` folder  
- [ ] Remove root level `cloudflare-worker.js`
- [ ] Remove root level `wrangler.toml`
- [ ] Remove root level `CNAME`
- [ ] Remove root level `package.json`, `package-lock.json`, `node_modules/`
- [ ] Remove visitor book tar.gz archives
- [ ] Remove `nginx-visitor-book.conf` from root
- [ ] Remove `visitor-book.service` from root
- [ ] Commit cleanup: "Removed visitor book files after successful migration"

## Files to Keep in Root
- `mmogit-wasm/` - Source for WASM build (keep for rebuilding)
- Core mmogit Rust code (everything else)

## Rollback Plan
If anything breaks, the original files are still at root level until this checklist is complete.
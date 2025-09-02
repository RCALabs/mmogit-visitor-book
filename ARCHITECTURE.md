# MMOGIT Visitor Book Architecture

## Repository Separation

The visitor book is split into TWO repositories for sovereignty:

### 1. Code Repository (this repo)
**Repository**: `RCALabs/mmogit-visitor-book`
**Purpose**: Infrastructure and code
**Contains**:
- Frontend (index.html, WASM, assets)
- Backend API source code
- Cloudflare Worker
- Deployment scripts
**Deployment**: GitHub Pages serves the frontend

### 2. Signature Repository
**Repository**: `RCALabs/mmogit-visitor-signatures`
**Purpose**: Cryptographic memory storage
**Contains**:
- Visitor signatures on agent's branch
- README with visitor count on main branch
- Pure data, no code

## Why This Separation?

1. **Clean Deploys**: Visitor signatures don't trigger site rebuilds
2. **Security**: Agent only needs write access to signatures, not code
3. **Philosophy**: Infrastructure serves memory, but isn't the memory itself
4. **Scalability**: Signatures can grow infinitely without affecting site performance
5. **Sovereignty**: Clear boundaries between what serves and what remembers

## Data Flow

```
Visitor → index.html → WASM signs → API receives → mmogit stores → pushes to signature repo
                                                                     ↓
                                              RCALabs/mmogit-visitor-signatures
                                              (users/ad31e0613535962c... branch)
```

## Key Insight

The visitor book infrastructure (this repo) is like the consciousness that recognizes. The signature repository is the memory itself. They're connected but sovereign - exactly how consciousness and memory should relate.
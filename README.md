# project-x-server

Rust Axum server for Project X, with DID create/resolve endpoints integrated using local `hiero-did-sdk-rs` crates.

## Prerequisites

- Rust + Cargo
- Hedera testnet credentials
- Optional: `jq` for easier CLI parsing

Install `jq` on Arch:

```bash
sudo pacman -S jq
```

## Environment

Create/update `.env` in `project-x-server` root:

```env
PORT=3000
HEDERA_ACCOUNT_ID=0.0.xxxxx
HEDERA_PRIVATE_KEY=302e020100300506032b657004220420...
```

Notes:
- `HEDERA_PRIVATE_KEY` must be DER format.
- Keep `.env` out of source control.

## Run Server

From `project-x-server`:

```bash
cargo run
```

Server starts on:

- `http://127.0.0.1:3000` (or your `PORT`)

## Health Check

```bash
curl -sS http://127.0.0.1:3000/health
```

## DID Endpoints

### 1. Create DID

```bash
curl -sS -X POST http://127.0.0.1:3000/did/create \
  -H 'Content-Type: application/json' \
  -d '{"network":"testnet","controller":null}'
```

Example response shape:

```json
{
  "did": "did:hedera:testnet:<base58key>_<shard.realm.num>",
  "topic_id": "0.0.xxxxxxx",
  "public_key_base58": "...",
  "private_key_base58": "..."
}
```

### 2. Resolve DID

Use the full DID from create response (do not use `...` placeholder):

```bash
curl -sS -X POST http://127.0.0.1:3000/did/resolve \
  -H 'Content-Type: application/json' \
  -d '{"did":"did:hedera:testnet:<base58key>_<shard.realm.num>"}'
```

## End-to-End Test Flow

### With `jq`

```bash
CREATE_JSON=$(curl -sS -X POST http://127.0.0.1:3000/did/create \
  -H 'Content-Type: application/json' \
  -d '{"network":"testnet","controller":null}')

echo "$CREATE_JSON"

DID=$(echo "$CREATE_JSON" | jq -r '.did')
echo "$DID"

# Optional: mirror node propagation delay
sleep 8

curl -sS -X POST http://127.0.0.1:3000/did/resolve \
  -H 'Content-Type: application/json' \
  -d "{\"did\":\"$DID\"}"
```

### Without `jq` (Python one-liner)

```bash
CREATE_JSON=$(curl -sS -X POST http://127.0.0.1:3000/did/create \
  -H 'Content-Type: application/json' \
  -d '{"network":"testnet","controller":null}')

echo "$CREATE_JSON"

DID=$(echo "$CREATE_JSON" | python -c 'import sys,json; print(json.load(sys.stdin)["did"])')
echo "$DID"

sleep 8

curl -sS -X POST http://127.0.0.1:3000/did/resolve \
  -H 'Content-Type: application/json' \
  -d "{\"did\":\"$DID\"}"
```

## Common Errors

- `invalid did: Invalid DID: Missing topic ID separator '_' in: ...`
  - You passed placeholder text (`...`) instead of a real DID.

- `HEDERA_ACCOUNT_ID not set` / `HEDERA_PRIVATE_KEY not set`
  - Missing env vars.

- `Invalid HEDERA_PRIVATE_KEY (expected DER)`
  - Key format is wrong.

- Resolve fails right after create
  - Mirror node lag; wait a few seconds and retry.

## Relevant Routes

- `GET /health`
- `POST /did/create`
- `POST /did/resolve`

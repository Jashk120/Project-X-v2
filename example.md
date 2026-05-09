## API

### Create DID

```bash
curl -sS -X POST http://127.0.0.1:3000/did/create \
    -H 'Content-Type: application/json' \
    -d '{"network":"testnet","controller":null}'
```

**Response**

```json
{
  "did": "did:hedera:testnet:CZHNjBmyV2prKDJzzMywNPD6GXMyG1n6NMzm2eKwwxDT_0.0.8903448",
  "private_key_base58": "4b4emmvupTFDxLdPRuNQhD9FDbtWBok3VLdrNcMfrHBQ",
  "public_key_base58": "CZHNjBmyV2prKDJzzMywNPD6GXMyG1n6NMzm2eKwwxDT"
}
```

---

### Resolve DID

```bash
curl -sS -X POST http://127.0.0.1:3000/did/resolve \
    -H 'Content-Type: application/json' \
    -d '{"did":"did:hedera:testnet:CZHNjBmyV2prKDJzzMywNPD6GXMyG1n6NMzm2eKwwxDT_0.0.8903448"}'
```

**Response**

```json
{
  "didDocument": {
    "id": "did:hedera:testnet:CZHNjBmyV2prKDJzzMywNPD6GXMyG1n6NMzm2eKwwxDT_0.0.8903448",
    "controller": "did:hedera:testnet:CZHNjBmyV2prKDJzzMywNPD6GXMyG1n6NMzm2eKwwxDT_0.0.8903448",
    "verificationMethod": [
      {
        "id": "did:hedera:testnet:CZHNjBmyV2prKDJzzMywNPD6GXMyG1n6NMzm2eKwwxDT_0.0.8903448#did-root-key",
        "type": "Ed25519VerificationKey2020",
        "controller": "did:hedera:testnet:CZHNjBmyV2prKDJzzMywNPD6GXMyG1n6NMzm2eKwwxDT_0.0.8903448",
        "publicKeyMultibase": "z6Mkr1YRKS2QpaKKRi9hfvwnDUm666dpfu2T4NugrvHxsAzq"
      }
    ],
    "authentication": [
      "did:hedera:testnet:CZHNjBmyV2prKDJzzMywNPD6GXMyG1n6NMzm2eKwwxDT_0.0.8903448#did-root-key"
    ],
    "assertionMethod": [
      "did:hedera:testnet:CZHNjBmyV2prKDJzzMywNPD6GXMyG1n6NMzm2eKwwxDT_0.0.8903448#did-root-key"
    ]
  },
  "didDocumentMetadata": {
    "created": "2026-05-09T11:36:09.542Z",
    "updated": "2026-05-09T11:36:09.542Z",
    "deactivated": false
  },
  "didResolutionMetadata": {
    "contentType": "application/ld+json;profile=\"https://w3id.org/did-resolution\""
  }
}
```

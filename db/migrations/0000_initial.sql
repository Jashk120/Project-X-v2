-- sessions
CREATE TABLE IF NOT EXISTS sessions (
    id                      TEXT PRIMARY KEY,
    driver_pubkey           TEXT NOT NULL,
    rider_pubkey            TEXT,
    join_token              TEXT,
    join_token_expires_at   TIMESTAMPTZ,
    ble_challenge           TEXT,
    ble_confirmed           BOOLEAN NOT NULL DEFAULT FALSE,
    sig_party_a             TEXT,
    sig_party_b             TEXT,
    created_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at              TIMESTAMPTZ NOT NULL
);

-- proximity attestations
CREATE TABLE IF NOT EXISTS proximity_attestations (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    session_id          TEXT NOT NULL REFERENCES sessions(id),
    driver_pubkey       TEXT NOT NULL,
    rider_pubkey        TEXT NOT NULL,
    driver_lat          DOUBLE PRECISION NOT NULL,
    driver_lng          DOUBLE PRECISION NOT NULL,
    rider_lat           DOUBLE PRECISION NOT NULL,
    rider_lng           DOUBLE PRECISION NOT NULL,
    distance_meters     DOUBLE PRECISION NOT NULL,
    approved            BOOLEAN NOT NULL DEFAULT FALSE,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at          TIMESTAMPTZ NOT NULL
);

-- issued VCs
CREATE TABLE IF NOT EXISTS vcs (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    subject_did     TEXT NOT NULL,
    issuer_did      TEXT NOT NULL,
    session_id      TEXT NOT NULL,
    vc_json         JSONB NOT NULL,
    issued_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at      TIMESTAMPTZ NOT NULL,
    revoked         BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE INDEX IF NOT EXISTS idx_vcs_subject ON vcs(subject_did);
CREATE INDEX IF NOT EXISTS idx_proximity_session ON proximity_attestations(session_id);
CREATE INDEX IF NOT EXISTS idx_sessions_driver ON sessions(driver_pubkey);
-- SPDX-License-Identifier: MIT
-- schema.sql: Hyper-tech PostgreSQL schema for PiDefi data storage.
-- Includes encrypted fields, AI data, and holographic metadata.

-- Enable extensions
CREATE EXTENSION IF NOT EXISTS "pgcrypto";  -- For encryption
CREATE EXTENSION IF NOT EXISTS "vector";    -- For AI vectors (if using pgvector)
CREATE EXTENSION IF NOT EXISTS "btree_gin"; -- For advanced indexing

-- Users Table (with Quantum-Resistant Encryption)
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE,
    password_hash TEXT NOT NULL,
    stellar_public_key TEXT,  -- Encrypted for quantum resistance
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Encrypt sensitive fields (quantum-safe placeholder; use lattice crypto in app)
-- Note: Real encryption handled in app layer with libs like 'pqcrypto'

-- Portfolios Table (User DeFi Holdings)
CREATE TABLE portfolios (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
    asset_symbol VARCHAR(10) NOT NULL,  -- e.g., 'PI', 'ETH'
    balance DECIMAL(36,18) NOT NULL DEFAULT 0,  -- High precision for Pi Coin
    staked_amount DECIMAL(36,18) DEFAULT 0,
    yield_rate DECIMAL(5,4) DEFAULT 0,  -- e.g., 0.0512 for 5.12%
    risk_score DECIMAL(3,2) DEFAULT 0.5,  -- 0-1 scale
    last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Transactions Table (On-Chain and Off-Chain)
CREATE TABLE transactions (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id),
    tx_hash VARCHAR(255) UNIQUE,  -- Stellar tx hash
    asset_symbol VARCHAR(10),
    amount DECIMAL(36,18),
    tx_type VARCHAR(50),  -- e.g., 'transfer', 'stake', 'arbitrage'
    status VARCHAR(20) DEFAULT 'pending',  -- 'pending', 'confirmed', 'failed'
    fee DECIMAL(18,9),
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    metadata JSONB  -- For holographic logs (e.g., {"layers": [...]} )
);

-- AI Model Data Table (For Training Recommendations)
CREATE TABLE ai_training_data (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id),
    features VECTOR(10),  -- Vector for AI features (e.g., balance, risk, yield)
    label DECIMAL(3,2),   -- Target (e.g., recommendation score)
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Holographic Visualizations Table (3D Data for Frontend)
CREATE TABLE holographic_visualizations (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id),
    visualization_type VARCHAR(50),  -- e.g., 'portfolio', 'risk_projection'
    layers JSONB,  -- e.g., [{"name": "Balance", "value": 1000, "viz": "sphere", "coords": [0,0,0]}]
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Arbitrage History (From Bots)
CREATE TABLE arbitrage_history (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id),
    from_chain VARCHAR(50),  -- e.g., 'Stellar'
    to_chain VARCHAR(50),    -- e.g., 'Ethereum'
    amount DECIMAL(36,18),
    profit DECIMAL(18,9),
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    holographic_path JSONB  -- 3D path for viz
);

-- Indexes for Performance
CREATE INDEX idx_portfolios_user_asset ON portfolios(user_id, asset_symbol);
CREATE INDEX idx_transactions_user_status ON transactions(user_id, status);
CREATE INDEX idx_ai_features ON ai_training_data USING ivfflat (features vector_cosine_ops);  -- Vector search
CREATE INDEX idx_holographic_user_type ON holographic_visualizations(user_id, visualization_type);

-- Partitioning for Scalability (e.g., by month for transactions)
CREATE TABLE transactions_y2023m12 PARTITION OF transactions
    FOR VALUES FROM ('2023-12-01') TO ('2024-01-01');

-- Triggers for Real-Time Updates
CREATE OR REPLACE FUNCTION update_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_users_timestamp BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_timestamp();

-- Row-Level Security (RLS)
ALTER TABLE portfolios ENABLE ROW LEVEL SECURITY;
CREATE POLICY user_portfolio_policy ON portfolios
    FOR ALL USING (user_id = current_user_id());  -- Set current_user_id in app

-- Audit Log Table
CREATE TABLE audit_logs (
    id SERIAL PRIMARY KEY,
    table_name VARCHAR(50),
    operation VARCHAR(10),  -- INSERT, UPDATE, DELETE
    old_values JSONB,
    new_values JSONB,
    user_id INTEGER,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Function for Audit Trigger
CREATE OR REPLACE FUNCTION audit_trigger()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO audit_logs (table_name, operation, old_values, new_values, user_id)
    VALUES (TG_TABLE_NAME, TG_OP, row_to_json(OLD), row_to_json(NEW), NEW.user_id);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER audit_portfolios AFTER INSERT OR UPDATE OR DELETE ON portfolios
    FOR EACH ROW EXECUTE FUNCTION audit_trigger();

-- Sample Data Insertion (for Testing)
INSERT INTO users (username, email, password_hash) VALUES
('alice', 'alice@pidefi.com', '$2a$10$examplehash'),
('bob', 'bob@pidefi.com', '$2a$10$examplehash');

INSERT INTO portfolios (user_id, asset_symbol, balance, yield_rate) VALUES
(1, 'PI', 1000000.000000000000000000, 0.0512),
(2, 'PI', 500000.000000000000000000, 0.0489);

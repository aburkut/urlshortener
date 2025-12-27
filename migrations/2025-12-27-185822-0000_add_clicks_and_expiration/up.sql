ALTER TABLE urls ADD COLUMN clicks INTEGER NOT NULL DEFAULT 0;
ALTER TABLE urls ADD COLUMN expires_at TIMESTAMP;
CREATE INDEX idx_urls_expires_at ON urls(expires_at);

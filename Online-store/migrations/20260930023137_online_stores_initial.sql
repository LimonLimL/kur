CREATE TABLE IF NOT EXISTS online_stores (
    id                  BLOB PRIMARY KEY NOT NULL,
    email               TEXT NOT NULL,
    delivery_payment    INTEGER NOT NULL DEFAULT 0,
    created_at          TEXT NOT NULL,
    updated_at          TEXT,
    deleted_at          TEXT
);

CREATE UNIQUE INDEX IF NOT EXISTS ux_stores_email ON online_stores(email);
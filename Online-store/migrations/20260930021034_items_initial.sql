CREATE TABLE IF NOT EXISTS items (
    id                  BLOB PRIMARY KEY NOT NULL,
    name                TEXT NOT NULL,
    company             TEXT NOT NULL,
    model               TEXT NOT NULL,
    characteristics     TEXT,           
    price               REAL NOT NULL,
    guarantee_period    INTEGER NOT NULL,
    image               TEXT NOT NULL,
    created_at          TEXT NOT NULL,
    updated_at          TEXT,
    deleted_at          TEXT
);

CREATE UNIQUE INDEX IF NOT EXISTS ux_items_name ON items(name);
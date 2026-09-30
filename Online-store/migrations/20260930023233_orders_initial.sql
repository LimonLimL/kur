CREATE TABLE IF NOT EXISTS orders (
    id              BLOB PRIMARY KEY NOT NULL,
    store_id        BLOB NOT NULL,
    item_id         BLOB NOT NULL,
    order_date      TEXT NOT NULL,
    order_time      TEXT NOT NULL,
    quantity        INTEGER NOT NULL,
    client_name     TEXT NOT NULL,
    client_phone    TEXT NOT NULL,
    confirmation    INTEGER NOT NULL DEFAULT 0,

    CONSTRAINT fk_orders_store 
        FOREIGN KEY (store_id) REFERENCES online_stores(id),
    CONSTRAINT fk_orders_item 
        FOREIGN KEY (item_id) REFERENCES items(id)
);
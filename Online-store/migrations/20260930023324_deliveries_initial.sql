CREATE TABLE IF NOT EXISTS deliveries (
    id              BLOB PRIMARY KEY NOT NULL,
    order_id        BLOB NOT NULL UNIQUE,
    delivery_date   TEXT NOT NULL,
    delivery_time   TEXT NOT NULL,
    address         TEXT NOT NULL,
    client_name     TEXT NOT NULL,
    courier_name    TEXT NOT NULL,

    CONSTRAINT fk_deliveries_order 
        FOREIGN KEY (order_id) REFERENCES orders(id)
);
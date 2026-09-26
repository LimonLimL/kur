-- ============================================================================
--  Интернет-магазин бытовой техники
--  Схема базы данных (PostgreSQL)
--
--  Запуск:  psql -U postgres -d online_store -f online_store.sql
--
--  Соответствие моделям приложения (src/models):
--    Shop       -> shops
--    Product    -> products
--    Order      -> orders
--    Delivery   -> deliveries
--
--  Хранение дат:
--    created_at / updated_at / deleted_at -> TIMESTAMPTZ (chrono::DateTime<Utc>)
--    date                                 -> DATE        (chrono::NaiveDate)
--    time                                 -> TIME        (chrono::NaiveTime)
-- ============================================================================

DROP TABLE IF EXISTS deliveries CASCADE;
DROP TABLE IF EXISTS orders     CASCADE;
DROP TABLE IF EXISTS products   CASCADE;
DROP TABLE IF EXISTS shops      CASCADE;

-- ============================================================================
--  Таблица магазинов
-- ============================================================================
CREATE TABLE shops (
    id           UUID         PRIMARY KEY,
    email        VARCHAR(255) NOT NULL UNIQUE,
    delivery     BOOLEAN      NOT NULL DEFAULT FALSE,
    created_at   TIMESTAMPTZ  NOT NULL DEFAULT now(),
    updated_at   TIMESTAMPTZ  NOT NULL DEFAULT now(),
    deleted_at   TIMESTAMPTZ            -- NULL = запись активна
);

COMMENT ON TABLE  shops          IS 'Магазины интернет-площадки';
COMMENT ON COLUMN shops.delivery IS 'Доставка собственными силами: TRUE - да, FALSE - самовывоз';

-- ============================================================================
--  Таблица товаров
-- ============================================================================
CREATE TABLE products (
    id                UUID          PRIMARY KEY,
    shop_id           UUID          NOT NULL REFERENCES shops (id) ON DELETE RESTRICT,
    name              VARCHAR(255)  NOT NULL,
    brand             VARCHAR(128)  NOT NULL,
    description       TEXT,
    price             NUMERIC(10,2) NOT NULL CHECK (price >= 0),
    guarantee_period  INTEGER       NOT NULL CHECK (guarantee_period > 0),
    image             VARCHAR(512),
    created_at        TIMESTAMPTZ   NOT NULL DEFAULT now(),
    updated_at        TIMESTAMPTZ   NOT NULL DEFAULT now(),
    deleted_at        TIMESTAMPTZ
);

COMMENT ON COLUMN products.guarantee_period IS 'Гарантийный период в месяцах';

CREATE INDEX idx_products_shop_id ON products (shop_id);
CREATE INDEX idx_products_brand   ON products (brand);

-- ============================================================================
--  Таблица заказов
--  Заказ = товар + магазин + количество + стоимость + данные покупателя
-- ============================================================================
CREATE TABLE orders (
    id            UUID          PRIMARY KEY,
    product_id    UUID          NOT NULL REFERENCES products (id) ON DELETE RESTRICT,
    shop_id       UUID          NOT NULL REFERENCES shops  (id) ON DELETE RESTRICT,
    date          DATE          NOT NULL,
    time          TIME          NOT NULL,
    count         INTEGER       NOT NULL CHECK (count > 0),
    cost          NUMERIC(10,2) NOT NULL CHECK (cost >= 0),
    user_name     VARCHAR(255)  NOT NULL,
    user_phone    VARCHAR(32)   NOT NULL,
    user_email    VARCHAR(255)  NOT NULL,
    confirmation  BOOLEAN       NOT NULL DEFAULT FALSE,
    created_at    TIMESTAMPTZ   NOT NULL DEFAULT now(),
    updated_at    TIMESTAMPTZ   NOT NULL DEFAULT now(),
    deleted_at    TIMESTAMPTZ
);

COMMENT ON COLUMN orders.confirmation IS 'Заказ подтверждён магазином';

CREATE INDEX idx_orders_shop_id    ON orders (shop_id);
CREATE INDEX idx_orders_product_id ON orders (product_id);
CREATE INDEX idx_orders_date       ON orders (date);

-- ============================================================================
--  Таблица доставок (1:1 с заказом)
-- ============================================================================
CREATE TABLE deliveries (
    id              UUID         PRIMARY KEY,
    order_id        UUID         NOT NULL UNIQUE REFERENCES orders (id) ON DELETE CASCADE,
    delivery_date   DATE         NOT NULL,
    delivery_time   TIME         NOT NULL,
    product_count   INTEGER      NOT NULL CHECK (product_count > 0),
    delivery_adress VARCHAR(512) NOT NULL,
    courier_name    VARCHAR(255) NOT NULL,
    created_at      TIMESTAMPTZ  NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ  NOT NULL DEFAULT now(),
    deleted_at      TIMESTAMPTZ
);

CREATE INDEX idx_deliveries_delivery_date ON deliveries (delivery_date);

-- ============================================================================
--  Автообновление updated_at
-- ============================================================================
CREATE OR REPLACE FUNCTION set_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_shops_updated_at
    BEFORE UPDATE ON shops
    FOR EACH ROW EXECUTE FUNCTION set_updated_at();

CREATE TRIGGER trg_products_updated_at
    BEFORE UPDATE ON products
    FOR EACH ROW EXECUTE FUNCTION set_updated_at();

CREATE TRIGGER trg_orders_updated_at
    BEFORE UPDATE ON orders
    FOR EACH ROW EXECUTE FUNCTION set_updated_at();

CREATE TRIGGER trg_deliveries_updated_at
    BEFORE UPDATE ON deliveries
    FOR EACH ROW EXECUTE FUNCTION set_updated_at();

-- ============================================================================
--  Представление: заказы с товарами, магазинами и доставками
-- ============================================================================
CREATE OR REPLACE VIEW v_orders AS
SELECT
    o.id            AS order_id,
    s.email         AS shop_email,
    p.name          AS product_name,
    p.brand         AS product_brand,
    o.date          AS order_date,
    o.time          AS order_time,
    o.count         AS product_count,
    o.cost          AS order_cost,
    o.user_name     AS buyer_name,
    o.user_phone    AS buyer_phone,
    o.user_email    AS buyer_email,
    o.confirmation  AS confirmed,
    d.delivery_date AS delivery_date,
    d.delivery_adress AS delivery_address,
    d.courier_name  AS courier_name
FROM orders o
    JOIN products  p ON p.id = o.product_id
    JOIN shops     s ON s.id = o.shop_id
    LEFT JOIN deliveries d ON d.order_id = o.id
WHERE o.deleted_at IS NULL;

-- ============================================================================
--  Тестовые данные
-- ============================================================================
INSERT INTO shops (id, email, delivery) VALUES
    ('00000000-0000-0000-0000-000000000001', 'technoshop@mail.ru', TRUE),
    ('00000000-0000-0000-0000-000000000002', 'byttehnik@yandex.ru', FALSE);

INSERT INTO products (id, shop_id, name, brand, description, price, guarantee_period, image) VALUES
    ('10000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000001',
     'Электрочайник Philips HD9350', 'Philips', 'Объём 1,7 л, мощность 2400 Вт', 4990, 24, 'philips_hd9350.jpg'),
    ('10000000-0000-0000-0000-000000000002', '00000000-0000-0000-0000-000000000001',
     'Кофемашина De''Longhi Magnifica S', 'De''Longhi', 'Автоматическая, капучинатор', 34990, 36, NULL),
    ('10000000-0000-0000-0000-000000000003', '00000000-0000-0000-0000-000000000002',
     'Пылесос Samsung Jet 75', 'Samsung', 'Беспроводной, 2100 мА·ч', 27990, 12, 'samsung_jet75.jpg');

INSERT INTO orders (id, product_id, shop_id, date, time, count, cost,
                    user_name, user_phone, user_email, confirmation) VALUES
    ('20000000-0000-0000-0000-000000000001', '10000000-0000-0000-0000-000000000001',
     '00000000-0000-0000-0000-000000000001', CURRENT_DATE, CURRENT_TIME, 2, 9980,
     'Иванов Иван Иванович', '+79991234567', 'ivanov@mail.ru', TRUE),
    ('20000000-0000-0000-0000-000000000002', '10000000-0000-0000-0000-000000000003',
     '00000000-0000-0000-0000-000000000002', CURRENT_DATE, CURRENT_TIME, 1, 27990,
     'Петрова Анна Сергеевна', '+79997654321', 'petrova@mail.ru', FALSE);

INSERT INTO deliveries (id, order_id, delivery_date, delivery_time, product_count,
                        delivery_adress, courier_name) VALUES
    ('30000000-0000-0000-0000-000000000001', '20000000-0000-0000-0000-000000000001',
     CURRENT_DATE + 1, CURRENT_TIME, 2, 'ул. Ленина, д. 10, кв. 5', 'Петров Пётр Петрович');

-- ============================================================================
--  Проверка
-- ============================================================================
SELECT * FROM v_orders ORDER BY order_date, order_time;

-- Выручка магазинов
SELECT s.email                      AS shop_email,
       COUNT(o.id)                  AS orders_count,
       COALESCE(SUM(o.cost), 0)     AS revenue
FROM shops s
    LEFT JOIN orders o ON o.shop_id = s.id AND o.deleted_at IS NULL
GROUP BY s.email
ORDER BY revenue DESC;

-- Доставки на выбранную дату
SELECT * FROM v_orders WHERE delivery_date = CURRENT_DATE + 1;

-- Таблица Telegram-пользователей.
-- telegram_id — уникальный идентификатор пользователя в Telegram (i64).
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE users (
    id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    telegram_id   BIGINT UNIQUE NOT NULL,
    username      TEXT,
    first_name    TEXT NOT NULL,
    last_name     TEXT,
    language_code TEXT,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now()
);

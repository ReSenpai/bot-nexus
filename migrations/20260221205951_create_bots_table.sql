-- Реестр внутренних Telegram-ботов системы.
-- code — уникальный строковый идентификатор бота ("youtube_downloader", "reminder").
-- is_active — можно деактивировать бота без удаления.
CREATE TABLE bots (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code        VARCHAR(100) UNIQUE NOT NULL,
    name        TEXT NOT NULL,
    description TEXT,
    version     VARCHAR(50) NOT NULL DEFAULT '1.0.0',
    is_active   BOOLEAN NOT NULL DEFAULT true,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

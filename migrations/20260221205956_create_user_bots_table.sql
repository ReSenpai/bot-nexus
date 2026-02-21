-- Связь пользователя с ботом (подписка).
-- settings (JSONB) — пользовательские настройки (качество видео, язык...).
-- state (JSONB) — текущее состояние (прогресс, курсоры...).
-- UNIQUE(user_id, bot_id) — один пользователь не может подписаться на один бот дважды.
CREATE TABLE user_bots (
    id                 UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id            UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    bot_id             UUID NOT NULL REFERENCES bots(id) ON DELETE CASCADE,
    installed_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
    settings           JSONB NOT NULL DEFAULT '{}',
    state              JSONB NOT NULL DEFAULT '{}',
    last_interaction_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(user_id, bot_id)
);

CREATE INDEX idx_user_bots_user_id ON user_bots(user_id);
CREATE INDEX idx_user_bots_bot_id ON user_bots(bot_id);

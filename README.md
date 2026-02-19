## 🗂 Проект: Notes API на Rust
### 🎯 Цель

Backend-сервис для управления заметками:

 - пользователи
 - авторизация
 - CRUD заметок
 - PostgreSQL
 - готовность к production

## 🧱 Архитектура проекта (структура)

```
notes-api/
├── Cargo.toml
├── Cargo.lock
├── .env
├── migrations/
├── src/
│   ├── main.rs
│   ├── app.rs
│   ├── config.rs
│   ├── state.rs
│   ├── errors.rs
│   ├── routes/
│   │   ├── mod.rs
│   │   ├── auth.rs
│   │   └── notes.rs
│   ├── handlers/
│   │   ├── auth.rs
│   │   └── notes.rs
│   ├── services/
│   │   ├── auth.rs
│   │   └── notes.rs
│   ├── repo/
│   │   ├── user_repo.rs
│   │   └── note_repo.rs
│   ├── models/
│   │   ├── user.rs
│   │   └── note.rs
│   └── dto/
│       ├── auth.rs
│       └── notes.rs
└── README.md
```

## 🧠 Разделение ответственности

```
Слой	       Отвечает за
routes	       URL + HTTP метод
handlers	   HTTP → бизнес
services	   бизнес-логика
repo	       PostgreSQL
models	       доменная модель
dto	           JSON вход/выход
state	       shared зависимости
errors	       единый error flow
```

## 🧰 Технологии

- 🌐 Web framework — axum
- ⚡ Async runtime — tokio
- 📦 JSON / сериализация — serde
- 🗄️ База данных — PostgreSQL + sqlx
- 🔐 Авторизация — JWT - argon2
- 🧾 Конфигурация — env + config (.env, dotenvy, config)
- 📊 Логирование — tracing
- Ошибки — свой AppError (thiserror)
- 🧪 Тестирование - Unit tests, встроено в Rust (#[test])
- 🐳 Инфраструктура (Docker, docker-compose (Postgres), migrations (sqlx migrate))
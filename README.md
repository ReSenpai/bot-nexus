# 🗂 todo-api

Backend-сервис для управления TODO-списками и задачами на Rust.

> **Учебный проект** — разработка ведётся пошагово по TDD (Test-Driven Development).

---

## 🎯 Цель

- Регистрация и авторизация пользователей (JWT + Argon2)
- CRUD TODO-листов (у каждого пользователя свои списки)
- CRUD задач внутри листов
- Статусы задач: `todo` → `in_progress` → `done`
- PostgreSQL как хранилище
- Чистая слоёная архитектура

---

## ✅ Прогресс

### Инфраструктура
- [x] Docker Compose (PostgreSQL + Adminer + API)
- [x] Dockerfile (multi-stage build)
- [x] Автоматические миграции при старте (`sqlx::migrate!`)
- [x] Миграция: таблица `users`
- [x] Миграция: таблица `todo_lists`
- [ ] Миграция: таблица `tasks`

### Auth
- [x] `POST /auth/register` — регистрация
- [x] `POST /auth/login` — вход (JWT-токен)
- [x] Argon2 хеширование паролей
- [x] Генерация JWT (HS256, 24ч)
- [x] Интеграционные тесты auth (5 тестов)
- [x] JWT middleware (защита маршрутов) + 3 теста

### Health
- [x] `GET /health` — проверка жизни сервиса
- [x] Интеграционный тест health

### TODO Lists
- [x] Модель `TodoList`
- [x] DTO для списков
- [x] `list_repo` — CRUD в БД
- [x] `list_service` — бизнес-логика
- [x] Маршруты: `POST / GET / PUT / DELETE /lists`
- [x] Интеграционные тесты lists (7 тестов)

### Tasks
- [ ] Модель `Task` (статусы: `todo`, `in_progress`, `done`)
- [ ] DTO для задач
- [ ] `task_repo` — CRUD в БД
- [ ] `task_service` — бизнес-логика
- [ ] Маршруты: `POST / GET / PUT / DELETE /lists/:id/tasks`
- [ ] Интеграционные тесты tasks

---

## 🧱 Архитектура проекта

```
todo-api/
├── Cargo.toml                 # зависимости проекта (Rust 2024 edition)
├── Cargo.lock                 # зафиксированные версии зависимостей
├── Dockerfile                 # multi-stage сборка (builder + runtime)
├── docker-compose.yml         # PostgreSQL + Adminer + API
├── .dockerignore              # исключения для Docker-контекста
├── .env                       # переменные окружения (DATABASE_URL, JWT_SECRET)
├── requests.http              # готовые HTTP-запросы для тестирования
├── migrations/
│   ├── *_create_users_table.sql
│   └── *_create_todo_lists_table.up.sql
├── src/
│   ├── main.rs                # точка входа: PgPool, миграции, запуск сервера
│   ├── app.rs                 # create_router() — сборка всех маршрутов
│   ├── lib.rs                 # re-export модулей (pub mod ...)
│   ├── state.rs               # AppState { db, jwt_secret }
│   ├── errors.rs              # AppError — единая обработка ошибок (401/404/409/422/500)
│   ├── middleware/
│   │   └── auth.rs            # AuthUser extractor — проверка JWT из заголовка
│   ├── routes/
│   │   ├── auth.rs            # POST /auth/register, /auth/login
│   │   ├── health.rs          # GET /health
│   │   └── lists.rs           # POST/GET/PUT/DELETE /lists
│   ├── handlers/
│   │   ├── auth.rs            # обработка HTTP-запросов auth
│   │   ├── health.rs          # обработка health check
│   │   └── lists.rs           # обработка CRUD списков (с AuthUser)
│   ├── services/
│   │   ├── auth.rs            # Argon2, JWT create/validate
│   │   └── lists.rs           # бизнес-логика списков
│   ├── repo/
│   │   ├── user_repo.rs       # SQL: create_user, find_by_email
│   │   └── list_repo.rs       # SQL: CRUD todo_lists
│   ├── models/
│   │   ├── user.rs            # User { id, email, password_hash, created_at }
│   │   └── todo_list.rs       # TodoList { id, user_id, title, timestamps }
│   └── dto/
│       ├── auth.rs            # RegisterRequest, LoginRequest, AuthResponse
│       └── lists.rs           # CreateListRequest, UpdateListRequest, ListResponse
├── tests/
│   ├── common/mod.rs          # test_app_state(), cleanup_user()
│   ├── health.rs              # 1 тест
│   ├── auth.rs                # 5 тестов
│   ├── middleware_auth.rs     # 3 теста
│   └── lists.rs              # 7 тестов
└── README.md
```

---

## 🧠 Разделение ответственности

| Слой         | Отвечает за                              |
|--------------|------------------------------------------|
| `routes`     | URL + HTTP метод                         |
| `handlers`   | HTTP → вызов сервиса                     |
| `middleware`  | JWT-авторизация (AuthUser extractor)    |
| `services`   | Бизнес-логика                            |
| `repo`       | SQL-запросы (PostgreSQL)                 |
| `models`     | Доменная модель (ORM-маппинг)            |
| `dto`        | JSON вход/выход (request/response)       |
| `state`      | Shared-зависимости (DB, JWT)             |
| `errors`     | Единый error flow (AppError → HTTP)      |

---

## 🧰 Технологии

| Категория       | Стек                                      |
|-----------------|-------------------------------------------|
| Web framework   | axum                                      |
| Async runtime   | tokio                                     |
| Сериализация    | serde + serde_json                        |
| База данных     | PostgreSQL + sqlx                         |
| Авторизация     | JWT (jsonwebtoken) + Argon2               |
| Конфигурация    | .env (dotenvy)                            |
| Логирование     | tracing + tracing-subscriber              |
| Ошибки          | AppError (thiserror)                      |
| Тестирование    | Integration tests (TDD), 16 тестов        |
| Инфраструктура  | Docker (multi-stage), docker-compose      |

---

## 🧪 Подход: TDD

Разработка каждой фичи идёт по циклу:

1. **Red** — пишем тест, который падает
2. **Green** — пишем минимальный код, чтобы тест прошёл
3. **Refactor** — улучшаем код, тесты остаются зелёными

```bash
cargo test                       # все 16 тестов
cargo test --test auth           # 5 тестов auth
cargo test --test health         # 1 тест health
cargo test --test middleware_auth # 3 теста middleware
cargo test --test lists          # 7 тестов lists
```

---

## 🚀 Запуск

### Всё в Docker (production-ready)

```bash
docker-compose up --build    # PostgreSQL + Adminer + API — всё из коробки
```

| Сервис   | URL                    |
|----------|------------------------|
| API      | http://localhost:3000   |
| Adminer  | http://localhost:8080   |
| Postgres | localhost:5432          |

### Локальная разработка

```bash
docker-compose up postgres adminer -d   # только БД + Adminer
sqlx migrate run                        # применить миграции
cargo run                               # запустить API
```
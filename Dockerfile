# ===== Stage 1: Builder =====
FROM rust:1.88-slim AS builder

# Устанавливаем зависимости для сборки (OpenSSL нужен sqlx для TLS).
RUN apt-get update && apt-get install -y pkg-config libssl-dev curl && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Копируем файлы зависимостей первыми — Docker кэширует слои.
# Если Cargo.toml не менялся, зависимости не пересобираются.
COPY Cargo.toml Cargo.lock ./

# Создаём пустой src для скачивания зависимостей (трюк для кеша).
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release 2>/dev/null || true
RUN rm -rf src

# Теперь копируем реальный исходный код и собираем.
COPY src ./src
COPY migrations ./migrations

# Пересобираем — зависимости уже закэшированы, собирается только наш код.
RUN touch src/main.rs && cargo build --release

# ===== Stage 2: Runtime =====
# Минимальный образ — только то, что нужно для запуска.
FROM debian:bookworm-slim

# OpenSSL и CA-сертификаты нужны для TLS-соединения с PostgreSQL.
RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Копируем только скомпилированный бинарник из builder-стадии.
COPY --from=builder /app/target/release/todo-api .

# Копируем миграции — они нужны, если запускать миграции из приложения.
COPY migrations ./migrations

# Порт, на котором слушает сервер.
EXPOSE 3000

# Запускаем бинарник.
CMD ["./todo-api"]

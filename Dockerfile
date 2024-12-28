# 1. Build Aşaması
FROM rust:latest AS builder

WORKDIR /usr/src/app
# Proje dosyalarını kopyala
COPY . .

# Bağımlılıkları indirmek ve önbellek oluşturmak için Cargo.toml ve Cargo.lock dosyalarını kullan
RUN apt-get update && apt-get install -y pkg-config libssl-dev
RUN apt-get install -y musl-tools
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl

# 2. Çalıştırma Aşaması
FROM scratch
WORKDIR /app
# Sadece çalıştırılabilir dosyayı kopyala
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/httpd /app/httpd
COPY static /app/static
COPY templates /app/templates
COPY data.db /app/data.db
COPY .env /app/.env

# Varsayılan portu aç
EXPOSE 8000

# Uygulamayı çalıştır
CMD ["./httpd"]

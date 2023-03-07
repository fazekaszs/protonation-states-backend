FROM rust:latest
EXPOSE 8281

WORKDIR /app
COPY . .
CMD [ "/bin/bash", "-c", "cargo run --release > backend.log 2>&1" ]
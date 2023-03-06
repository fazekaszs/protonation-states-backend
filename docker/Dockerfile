FROM rust:latest
EXPOSE 8000

WORKDIR /app
COPY . .
RUN cargo install --path .
CMD [ "protonation-states-backend" ]
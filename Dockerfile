FROM rust:latest
EXPOSE 8181

WORKDIR /app
COPY . .
RUN cargo install --path .
CMD [ "protonation-states-backend" ]
FROM rust:latest
EXPOSE 8181
ENV ROCKET_PORT=8181

WORKDIR /app
COPY . .
RUN cargo install --path .
CMD [ "protonation-states-backend" ]
FROM rust:1.31-slim

WORKDIR /app

RUN apt-get update && apt-get install -y

RUN apt-get install build-essential -y

ADD target/release/web_socket_farm ./

COPY templates ./templates/

COPY static ./static/

COPY .env ./

EXPOSE 8181

CMD ./web_socket_farm

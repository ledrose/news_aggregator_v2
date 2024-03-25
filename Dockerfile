FROM node:latest as node
RUN mkdir /app
WORKDIR /app
COPY frontend/package*.json ./
RUN npm install
COPY frontend/ .
RUN npm run build


FROM rust:1.77
WORKDIR /usr/src

RUN USER=root cargo new backend
COPY backend/Cargo.toml backend/Cargo.lock /usr/src/backend/


WORKDIR /usr/src/backend
RUN cargo update --dry-run
RUN cargo build --release

COPY backend/src /usr/src/backend/src/
COPY backend/migrations /usr/src/backend/migrations/
COPY --from=node /app/build/ /usr/src/backend/build

RUN cargo build --release 

WORKDIR /usr/src/backend

EXPOSE 8080 

CMD ["cargo","run","--release"]
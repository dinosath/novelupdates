FROM lukemathwalker/cargo-chef:latest-rust-1-alpine3.21 AS chef
WORKDIR /app
RUN apk add --no-cache upx

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .

RUN cargo build --release
RUN upx --best --lzma /app/target/release/novelupdates

FROM node:20-slim AS node
ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN corepack enable
COPY ./frontend /app
ENV CI=true
WORKDIR /app
RUN pnpm install
RUN pnpm build


FROM gcr.io/distroless/static-debian12
COPY --from=builder /app/target/release/novelupdates /
COPY --from=node /app/dist /app/static


EXPOSE 8080

ENTRYPOINT ["/novelupdates"]
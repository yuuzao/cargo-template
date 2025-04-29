FROM rustlang/rust:nightly AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM rustlang/rust:nightly AS runner

WORKDIR /app

COPY --from=builder /app/target/release/{{project-name}} .
COPY config.toml .
COPY .env .
ENV RUN_MODE=prod

EXPOSE {{default_port}}

ENTRYPOINT ["./{{project-name}}"]
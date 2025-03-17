FROM rustlang/rust:nightly

WORKDIR /app

COPY . .

RUN cargo build --release

ENTRYPOINT ["/app/target/release/{{project-name}}"]
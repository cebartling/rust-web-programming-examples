FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt -y update
RUN apt install -y musl-tools musl-dev
RUN apt-get install -y build-essential
RUN apt install -y gcc-x86-64-linux-gnu

WORKDIR /tmp

COPY .. .

ENV RUSTFLAGS='-C linker=x86_64-linux-gnu-gcc'
ENV CC='gcc'
ENV CC_x86_64_unknown_linux_musl=x86_64-linux-gnu-gcc
ENV CC_x86_64-unknown-linux-musl=x86_64-linux-gnu-gcc

RUN cd /tmp/rest_server && cargo build --target x86_64-unknown-linux-musl --release

FROM scratch

WORKDIR /app

COPY --from=builder /tmp/rest_server/target/x86_64-unknown-linux-musl/release/rest_server ./
COPY --from=builder /tmp/rest_server/.env ./

CMD ["/app/rest_server"]
FROM rust:1.67 AS build

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

WORKDIR $HOME/Projects/personal/rust-web

# copy entire workspace
COPY . .

RUN cargo build --target x86_64-unknown-linux-musl --release

# Final image

FROM alpine:latest

EXPOSE 8000

COPY --from=build $HOME/Projects/personal/rust-web/target/x86_64-unknown-linux-musl/release/rust-web ./

CMD [ "./rust-web" ]
FROM rust:latest AS builder

RUN apt update && apt upgrade

COPY ./ ./

RUN cargo build --release

FROM scratch

COPY --from=builder target/release/notpetyaagain_decryptor .

CMD ["./notpetyaagain_decryptor"]
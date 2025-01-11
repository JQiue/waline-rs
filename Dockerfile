FROM alpine:latest
WORKDIR /app
ENV host 0.0.0.0
ENV database_url sqlite://./waline.sqlite?mode=rw
COPY ./target/x86_64-unknown-linux-musl/release/waline-mini .
COPY ./assets/waline.sqlite .
EXPOSE 8360
CMD "./waline-mini"

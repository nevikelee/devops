FROM nimlang/nim:latest AS builder

WORKDIR /app
COPY e1.nim ./

RUN nim c -d:release e1.nim

FROM debian:bookworm-slim

WORKDIR /app
COPY --from=builder /app/e1 /app/e1

CMD ["./e1"]
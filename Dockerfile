FROM rust:1.74

WORKDIR /app

COPY . .

EXPOSE 8000

CMD ["cargo", "run"]

FROM rust:1.74

WORKDIR /app

COPY . .

RUN apt update && apt install -y libpq5

RUN cargo install diesel_cli

#RUN diesel migration run

EXPOSE 8000

CMD ["cargo", "run"]

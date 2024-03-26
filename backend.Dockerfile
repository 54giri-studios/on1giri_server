FROM rust:1.75 as builder

WORKDIR /app

# ARG DATABASE_URL

# ENV DATABASE_URL=$DATABASE_URL

COPY . .

EXPOSE 8000

RUN ["cargo", "run", "--release"]

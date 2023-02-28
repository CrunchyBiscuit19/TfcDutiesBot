# syntax=docker/dockerfile:1
   
FROM rust:latest
WORKDIR /app
COPY . .
ENV TELOXIDE_TOKEN="6241119076:AAEuScaU--1C6arY8-gtzDz2oPqXYq9KfGg"
RUN cargo install --path .

CMD ["cargo", "run", "--release"]

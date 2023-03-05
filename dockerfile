# syntax=docker/dockerfile:1
   
FROM rust:latest
WORKDIR /app
COPY . .
ENV TELOXIDE_TOKEN="6241119076:AAEuScaU--1C6arY8-gtzDz2oPqXYq9KfGg"
ENV GOOGLE_API_KEY="AIzaSyBN8JPDKHLA2iwE4p1m_ldhF74iTxb7euU"
RUN cargo install --path .

CMD ["cargo", "run", "--release"]

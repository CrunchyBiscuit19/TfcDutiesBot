# syntax=docker/dockerfile:1
   
FROM rust:latest
WORKDIR /app
COPY . .
ENV TELOXIDE_TOKEN="6241119076:AAFKI9mJ6kaKPSHq5lYToGmZxSwVTMYw0vE"
RUN cargo install --path .

CMD ["cargo", "run", "--release"]

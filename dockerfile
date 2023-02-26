# syntax=docker/dockerfile:1
   
FROM rust:latest
WORKDIR /app
COPY . .
ENV TELOXIDE_TOKEN="6241119076:AAFKI9mJ6kaKPSHq5lYToGmZxSwVTMYw0vE"
RUN cargo install --path .
EXPOSE 8080

RUN wget -q -O - https://dl-ssl.google.com/linux/linux_signing_key.pub | apt-key add -
RUN sh -c 'echo "deb http://dl.google.com/linux/chrome/deb/ stable main" >> /etc/apt/sources.list.d/google.list'
RUN apt-get update
RUN apt --fix-broken install
RUN apt-get install google-chrome-stable -y

CMD ["cargo", "run", "--release"]

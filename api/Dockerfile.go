FROM ruby:3.2.2 as sql-migrator

WORKDIR /rails

COPY --from=litestream/litestream:latest /usr/local/bin/litestream /bin/litestream

COPY Gemfile Gemfile
COPY Gemfile.lock Gemfile.lock
RUN bundle -j4
COPY . .
RUN bundle exec rails db:migrate:reset db:seed

FROM golang:alpine as builder
WORKDIR /app
COPY --from=sql-migrator /rails/database.sqlite3 /app/database.sqlite3
COPY v2 /app
RUN apk add --no-cache gcc musl-dev
RUN CGO_ENABLED=1 GOOS=linux go build -o /app/app

FROM alpine
WORKDIR /
COPY --from=builder /app/app /app
COPY --from=builder /app/database.sqlite3 /database.sqlite3
RUN apk add --no-cache gcc musl-dev
EXPOSE 3000
CMD ["./app"]

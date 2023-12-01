FROM ruby:3.2.2 as sql-migrator

WORKDIR /rails

COPY --from=litestream/litestream:latest /usr/local/bin/litestream /bin/litestream

COPY Gemfile Gemfile
COPY Gemfile.lock Gemfile.lock
RUN bundle -j4
COPY . .

RUN bundle exec rake db:migrate

FROM golang:1.21.4 as builder
WORKDIR /app
COPY --from=sql-migrator /rails/database.sqlite3 /app/database.sqlite3

COPY v2 /app
RUN CGO_ENABLED=0 GOOS=linux go build -o /app/app

FROM scratch
WORKDIR /
COPY --from=builder /app/app /app
EXPOSE 3000
CMD ["./app"]

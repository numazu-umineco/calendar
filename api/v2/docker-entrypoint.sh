#!/bin/bash -e

if [ "${DISABLE_LITESTREAM}" != "" ] || [ "${RAILS_ENV}" = "test" ]; then
  exec "$@"
else
  litestream restore -if-replica-exists -if-db-not-exists -config /app/db/litestream.yml /app/db/database.sqlite3
  exec litestream replicate -exec "$*" -config /app/db/litestream.yml
fi

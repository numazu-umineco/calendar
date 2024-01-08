#!/bin/bash -e

if [ "${DISABLE_LITESTREAM}" != "" ] || [ "${RAILS_ENV}" = "test" ]; then
  exec "$@"
else
  litestream restore -if-replica-exists -if-db-not-exists -config /rails/litestream/config.yml /rails/db/litestream/database.sqlite3
  exec litestream replicate -exec "$*" -config /rails/litestream/config.yml
fi

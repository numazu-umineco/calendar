#!/bin/bash -xe

litestream restore -if-replica-exists -if-db-not-exists -config /rails/litestream.yml /rails/db/litestream/database.sqlite3
exec litestream replicate -exec "$*" -config /rails/litestream.yml

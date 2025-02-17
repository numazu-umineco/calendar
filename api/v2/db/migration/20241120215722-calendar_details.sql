-- +migrate Up
CREATE TABLE IF NOT EXISTS calendar_details (
  id varchar NOT NULL PRIMARY KEY,
  name varchar NOT NULL,
  discarded_at datetime(6),
  created_at datetime(6) NOT NULL,
  updated_at datetime(6) NOT NULL
);

CREATE UNIQUE INDEX index_calendar_details_on_name ON calendar_details (name);
CREATE INDEX index_calendar_details_on_discarded_at ON calendar_details (discarded_at);

-- +migrate Down
DROP TABLE IF EXISTS calendar_details;

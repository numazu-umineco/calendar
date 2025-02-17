-- +migrate Up
CREATE TABLE IF NOT EXISTS calendar_events (
       id varchar(255) PRIMARY KEY NOT NULL,
       summary text NOT NULL,
       description text NOT NULL,
       location text,
       latitude float,
       longitude float,
       start_at datetime(6) NOT NULL,
       end_at datetime(6) NOT NULL,
       discarded_at datetime(6),
       calendar_detail_id varchar NOT NULL,
       last_modified_user varchar DEFAULT '' NOT NULL,
       created_at datetime(6) NOT NULL,
       updated_at datetime(6) NOT NULL,
       all_day boolean DEFAULT 0 NOT NULL,
       url text,
       CONSTRAINT 'fk_rails_f5ef1ec5eb'
       FOREIGN KEY ('calendar_detail_id')
       REFERENCES 'calendar_details' ('id')
);
CREATE INDEX index_calendar_events_on_calendar_detail_id ON calendar_events (calendar_detail_id);
CREATE INDEX index_calendar_events_on_discarded_at ON calendar_events (discarded_at);

-- +migrate Down
DROP TABLE IF EXISTS calendar_events;

-- Your SQL goes here
CREATE TABLE card (
  id bigserial NOT NULL PRIMARY KEY,
  account_id bigserial NOT NULL,
  card_name varchar(255) NOT NULL,
  card_number bytea NOT NULL,
  created_at TIMESTAMPTZ NOT NULL,
  FOREIGN KEY (account_id) REFERENCES account(id)
);

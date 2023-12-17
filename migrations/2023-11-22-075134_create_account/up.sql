-- Your SQL goes here
CREATE TABLE account (
  id bigserial NOT NULL PRIMARY KEY,
  card_id INT REFERNCES card(id),
  username varchar(255) NOT NULL UNIQUE,
  grade INTEGER CHECK (grade >= 1 AND grade <= 4) NOT NULL,
  created_at TIMESTAMPTZ NOT NULL
);

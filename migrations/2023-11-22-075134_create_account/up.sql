-- Your SQL goes here
CREATE TABLE account (
  id bigserial NOT NULL PRIMARY KEY,
  username varchar(255) NOT NULL ,
  grade INTEGER CHECK (grade >= 1 AND grade <= 4) NOT NULL,
  expiration_date TIMESTAMPTZ NOT NULL,
  created_at TIMESTAMPTZ NOT NULL
);

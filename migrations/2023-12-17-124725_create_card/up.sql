-- Your SQL goes here
CREATE TABLE card {
  id bigserial NOT NULL PRIMARY KEY,
  account_id INT NOT NULL,
  card_type varchar(255) NOT NULL,
  card_number bytea NOT NULL,
  created_at TIMESTAMPZ NOT NULL,
  FOREIGN KEY (account_id) REFERENCES account(id)
};

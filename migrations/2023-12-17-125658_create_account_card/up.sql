-- Your SQL goes here
CREATE TABLE account_card {
  account_id bigserial NOT NULL REFERENCES account(id),
  card_number bytea NOT NULL,
  PRIMARY KEY (account_id,card_number)
}

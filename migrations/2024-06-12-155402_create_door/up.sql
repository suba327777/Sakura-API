-- Your SQL goes here
CREATE TABLE door (
    device_id VARCHAR(255) PRIMARY KEY,
    door_state BOOLEAN NOT NULL,
    door_switch_state BOOLEAN NOT NULL
);


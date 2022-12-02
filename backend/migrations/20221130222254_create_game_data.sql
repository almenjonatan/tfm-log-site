-- Add migration script here

CREATE TABLE
    game (
        id BIGINT PRIMARY KEY NOT NULL,
        played_at TIMESTAMPTZ NOT NULL
    );

CREATE TABLE
    players (
        id BIGINT PRIMARY KEY NOT NULL,
        game BIGINT NOT NUll,
        Foreign Key (game) REFERENCES game(id)
    );
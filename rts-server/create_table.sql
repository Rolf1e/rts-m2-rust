DROP TABLE IF EXISTS matchs;
DROP TABLE IF EXISTS ais;
DROP TABLE IF EXISTS users;

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username VARCHAR NOT NULL UNIQUE,
  password VARCHAR NOT NULL,
  email VARCHAR NOT NULL UNIQUE
);

CREATE TABLE ais (
  id SERIAL PRIMARY KEY,
  owner INT REFERENCES users (id),
  code VARCHAR NOT NULL
);

CREATE TABLE MATCHS(
  id SERIAL PRIMARY KEY,
  player INT REFERENCES users (id),
  game INT NOT NULL,
  score INT NOT NULL,
  winner BOOLEAN NOT NULL
);



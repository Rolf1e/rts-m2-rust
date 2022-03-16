-- Your SQL goes here
CREATE TABLE MATCHS(
  id SERIAL PRIMARY KEY,
  player INT REFERENCES users (id),
  game INT NOT NULL,
  score INT NOT NULL
);


-- Your SQL goes here
CREATE TABLE MATCHS(
  id SERIAL PRIMARY KEY,
  winner INT REFERENCES users (id),
  looser INT REFERENCES users (id),
  score_winner INT NOT NULL,
  score_looser INT NOT NULL
);

INSERT INTO users (username, password, email) VALUES ('rolfie', 'rolfie', '' );
INSERT INTO matchs (winner, looser, score_winner, score_looser) VALUES (1, 1, 100, 99);

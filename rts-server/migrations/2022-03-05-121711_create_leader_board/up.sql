-- Your SQL goes here
CREATE TABLE MATCHS(
  id SERIAL PRIMARY KEY,
  winner INT REFERENCES users (id),
  looser INT REFERENCES users (id),
  score_winner INT NOT NULL,
  score_looser INT NOT NULL
);

INSERT INTO users (username, password, email) VALUES ('rolfie', 'rolfie', '' );
INSERT INTO users (username, password, email) VALUES ('tigran', 'tigran', '' );
INSERT INTO users (username, password, email) VALUES ('antoine', 'antoine', '' );
INSERT INTO matchs (winner, looser, score_winner, score_looser) VALUES (1, 1, 100, 99);
INSERT INTO matchs (winner, looser, score_winner, score_looser) VALUES (2, 1, 120, 99);
INSERT INTO matchs (winner, looser, score_winner, score_looser) VALUES (1, 2, 120, 99);
INSERT INTO matchs (winner, looser, score_winner, score_looser) VALUES (3, 1, 80, 99);
INSERT INTO matchs (winner, looser, score_winner, score_looser) VALUES (1, 3, 30, 99);

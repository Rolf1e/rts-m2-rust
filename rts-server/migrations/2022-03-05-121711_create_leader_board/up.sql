-- Your SQL goes here
CREATE TABLE LEADER_BOARD(
  id SERIAL PRIMARY KEY,
  rank INT NOT NULL, -- Maybe unique ? 
  player INT REFERENCES users (id),
  score INT NOT NULL,
  wins INT NOT NULL,
  losses INT NOT NULL
);

CREATE TABLE rating_value (
	id INTEGER PRIMARY KEY,
	value INTEGER NOT NULL
);

CREATE TABLE rating (
	id INTEGER PRIMARY KEY,
	music_id INTEGER,
	rating_value_id INTEGER,
	FOREIGN KEY (music_id) REFERENCES music (id),
	FOREIGN KEY (rating_value_id) REFERENCES rating_value (id)
);

INSERT INTO rating_value (value) VALUES (1), (2), (3), (4), (5);

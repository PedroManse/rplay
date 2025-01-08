-- Add up migration script here
CREATE TABLE album_contributor (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	artist_id INTEGER NOT NULL REFERENCES artist(id),
	album_id INTEGER NOT NULL REFERENCES album(id)
);

INSERT INTO album_contributor (artist_id, album_id) SELECT artist_id, id FROM album;
ALTER TABLE album DROP COLUMN artist_id;

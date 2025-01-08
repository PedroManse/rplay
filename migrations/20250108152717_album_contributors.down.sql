-- Add down migration script here

-- add column that allows NULLs
ALTER TABLE album
	ADD COLUMN artist_id
	INTEGER REFERENCES artist(id);

-- transfer an artist_id to album
UPDATE album
SET artist_id = ac.artist_id
FROM album_contributor as ac;

-- clone album but with NOT NULL in artist_id column
CREATE TABLE new_album  (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	name TEXT NOT NULL,
	artist_id INTEGER NOT NULL REFERENCES artist(id),

	deezer_id INTEGER UNIQUE
);

-- transfer data
INSERT INTO new_album SELECT id, name, artist_id, deezer_id FROM album;

-- delete old tables
DROP TABLE album_contributor;
DROP TABLE album;

ALTER TABLE new_album RENAME TO album;


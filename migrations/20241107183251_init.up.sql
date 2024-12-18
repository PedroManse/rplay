CREATE TABLE artist (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	name TEXT NOT NULL UNIQUE,

	deezer_id INTEGER UNIQUE
);

CREATE TABLE track (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	path TEXT UNIQUE,
	name TEXT NOT NULL,
	artist_id INTEGER NOT NULL REFERENCES artist(id),
	album_id INTEGER NOT NULL REFERENCES album(id),
	duration INTEGER NOT NULL,

	deezer_id INTEGER UNIQUE
);

CREATE TABLE album (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	name TEXT NOT NULL,
	artist_id INTEGER NOT NULL REFERENCES artist(id),

	deezer_id INTEGER UNIQUE
);

CREATE TABLE playlist (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	name TEXT NOT NULL UNIQUE,
	deezer_id INTEGER
);

CREATE TABLE playlist_entry (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	list_id INTEGER NOT NULL REFERENCES playlist(id),
	track_id INTEGER NOT NULL REFERENCES track(id)
);

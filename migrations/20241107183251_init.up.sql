PRAGMA foreign_keys = ON;

CREATE TABLE artist (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	name TEXT NOT NULL UNIQUE,

	deezer_id INTEGER UNIQUE

	-- OUT REFS
	-- track(artist_id)
	-- album(artist_id)
);

CREATE TABLE track (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	path TEXT UNIQUE,
	name TEXT NOT NULL,
	artist_id INTEGER NOT NULL REFERENCES artist(id),
	album_id INTEGER NOT NULL REFERENCES album(id),
	duration INTEGER NOT NULL,

	deezer_id INTEGER UNIQUE
	-- OUT REFS
	-- playlist_entry(track_id)
);

CREATE TABLE album (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	name TEXT NOT NULL,
	artist_id INTEGER NOT NULL REFERENCES artist(id),

	deezer_id INTEGER UNIQUE
	-- OUT REFS
	-- track(album_id)
);

CREATE TABLE playlist (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	name TEXT NOT NULL UNIQUE,
	deezer_id INTEGER
	-- OUT REFS
	-- playlist_entry(list_id)
);

CREATE TABLE playlist_entry (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	list_id INTEGER NOT NULL REFERENCES playlist(id),
	track_id INTEGER NOT NULL REFERENCES track(id)
);


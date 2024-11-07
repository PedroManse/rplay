CREATE TABLE artist (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	name TEXT NOT NULL UNIQUE,

	youtube_name TEXT UNIQUE,
	deezer_id INTEGER UNIQUE,
	spotify_id INTEGER UNIQUE
);

CREATE TABLE track (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	path TEXT UNIQUE,
	name TEXT NOT NULL,
	artist_id INTEGER NOT NULL REFERENCES artist(id),
	duration INTEGER NOT NULL,
	album TEXT,

	deezer_id INTEGER UNIQUE,
	youtube_id TEXT UNIQUE,
	spotify_id INTEGER UNIQUE
);

CREATE TABLE playlist (
	name TEXT NOT NULL UNIQUE,

	youtube_name TEXT,
	deezer_id INTEGER
);

CREATE TABLE playlist_entry (
	list_id INTEGER NOT NULL REFERENCES playlist(id),
	track_id INTEGER NOT NULL REFERENCES track(id)
);

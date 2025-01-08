CREATE TABLE artist (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	deezer_id INTEGER UNIQUE

	name TEXT NOT NULL UNIQUE,

	-- OUT REFS
	-- track(artist_id)
	-- album_contributor(artist_id)
);

CREATE TABLE track (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	path TEXT UNIQUE,
	artist_id INTEGER NOT NULL REFERENCES artist(id),
	album_id INTEGER NOT NULL REFERENCES album(id),
	deezer_id INTEGER UNIQUE

	name TEXT NOT NULL,
	duration INTEGER NOT NULL,

	-- OUT REFS
	-- playlist_entry(track_id)
);

CREATE TABLE album (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	deezer_id INTEGER UNIQUE

	name TEXT NOT NULL,

	-- OUT REFS
	-- track(album_id)
	-- album_contributor(album_id)
);

CREATE TABLE playlist (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	deezer_id INTEGER

	name TEXT NOT NULL UNIQUE,

	-- OUT REFS
	-- playlist_entry(list_id)
);

CREATE TABLE playlist_entry (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	list_id INTEGER NOT NULL REFERENCES playlist(id),
	track_id INTEGER NOT NULL REFERENCES track(id)
);

CREATE TABLE album_contributor (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	artist_id INTEGER NOT NULL REFERENCES artist(id),
	album_id INTEGER NOT NULL REFERENCES album(id)
);

//TODO remove this after making playlist methods
#![allow(unused_variables)]
use crate::*;
pub type DBCon = sqlx::SqliteConnection;
type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Artist {
    pub id: i64,
    pub name: String,

    pub deezer_id: Option<i64>,
}
// ::new
// ::get_all
// &.get_tracks

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Album {
    pub id: i64,
    pub name: String,
    pub artist_id: i64,
    pub deezer_id: Option<i64>,
}
// ::new
// ::get_all
// &.get_artist
// &.get_tracks

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct DBTrack {
    id: i64,
    path: Option<String>,
    name: String,
    artist_id: i64,
    album_id: i64,
    duration: i64,
    deezer_id: Option<i64>,
}

//TODO deref as Track?
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct TrackInfo {
    pub id: i64,
    pub path: Option<String>,
    pub name: String,
    pub artist_id: i64,
    pub artist_name: String,
    pub album_id: i64,
    pub album_name: String,
    pub duration: i64,
    pub deezer_id: Option<i64>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Track {
    pub id: i64,
    pub path: Option<std::path::PathBuf>,
    pub name: String,
    pub artist_id: i64,
    pub album_id: i64,
    pub duration: i64,
    pub deezer_id: Option<i64>,
}
// ::new
// ::all_tracks
// &mut .download
// .delete_track
// &mut.delete_download
// &mut.attach_deezer_id
// &.in_playlists
// &.get_album
// &.get_artist

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct PlaylistInfo {
    pub id: i64,
    pub name: String,
    pub deezer_id: Option<i64>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Playlist {
    pub id: i64,
    pub name: String,
    pub tracks: Vec<i64>,
    pub deezer_id: Option<i64>,
}
// ::new
// ::get
// ::get_all
// &mut.add_track
// &mut.remove_track
// &.get_tracks

impl Artist {
    pub async fn new(db: &mut DBCon, name: &str, deezer_id: Option<i64>) -> Result<Self> {
        sqlx::query_as!(
            Artist,
            "
INSERT INTO artist
    (name, deezer_id)
VALUES (?, ?)
ON CONFLICT (deezer_id)
    DO UPDATE SET deezer_id=(?)
RETURNING
    id, deezer_id, name
",
            name,
            deezer_id,
            deezer_id,
        )
        .fetch_one(db)
        .await
        .map_err(Error::from)
    }
    pub async fn get_all(db: &mut DBCon) -> Result<Vec<Self>> {
        sqlx::query_as!(
            Artist,
            r#"
SELECT *
FROM artist
            "#
        )
        .fetch_all(db)
        .await
        .map_err(Error::from)
    }
    pub async fn get_tracks(&self, db: &mut DBCon) -> Result<impl Iterator<Item = Track>> {
        sqlx::query_as!(
            DBTrack,
            r#"
SELECT *
FROM track
WHERE artist_id=?
"#,
            self.id
        )
        .fetch_all(db)
        .await
        .map(|s| s.into_iter().map(Track::from))
        .map_err(Error::from)
    }
}

impl Album {
    pub async fn new(
        db: &mut DBCon,
        name: &str,
        artist_id: i64,
        deezer_id: Option<i64>,
    ) -> Result<Self> {
        sqlx::query_as!(
            Album,
            "
INSERT INTO album
    (name, artist_id, deezer_id)
VALUES (?, ?, ?)
ON CONFLICT (deezer_id)
    DO UPDATE SET deezer_id=(?)
RETURNING
    id, name, artist_id, deezer_id
",
            name,
            artist_id,
            deezer_id,
            deezer_id,
        )
        .fetch_one(db)
        .await
        .map_err(Error::from)
    }
    pub async fn get_all(db: &mut DBCon) -> Result<Vec<Self>> {
        sqlx::query_as!(
            Album,
            r#"
SELECT *
FROM album
            "#
        )
        .fetch_all(db)
        .await
        .map_err(Error::from)
    }
    pub async fn get_artist(&self, db: &mut DBCon) -> Result<Artist> {
        sqlx::query_as!(
            Artist,
            r#"
SELECT *
FROM artist
WHERE id=?
"#,
            self.artist_id
        )
        .fetch_one(db)
        .await
        .map_err(Error::from)
    }
    pub async fn get_tracks(&self, db: &mut DBCon) -> Result<impl Iterator<Item = Track>> {
        sqlx::query_as!(
            DBTrack,
            r#"
SELECT *
FROM track
WHERE album_id=?
"#,
            self.id,
        )
        .fetch_all(db)
        .await
        .map(|s| s.into_iter().map(Track::from))
        .map_err(Error::from)
    }
}

impl From<DBTrack> for Track {
    fn from(t: DBTrack) -> Track {
        Track {
            id: t.id,
            path: t.path.map(std::path::PathBuf::from),
            name: t.name,
            artist_id: t.artist_id,
            album_id: t.album_id,
            duration: t.duration,
            deezer_id: t.deezer_id,
        }
    }
}

impl TrackInfo {
    pub async fn get_all(db: &mut DBCon) -> Result<Vec<Self>> {
        sqlx::query_as!(
            TrackInfo,
            r#"
SELECT
    t.id,
    t.path,
    t.name,
    t.artist_id,
    a.name as artist_name,
    t.album_id,
    b.name as album_name,
    t.duration,
    t.deezer_id
FROM track as t
JOIN artist as a ON t.artist_id=a.id
JOIN album as b ON t.album_id=b.id
"#
        )
        .fetch_all(db)
        .await
        .map_err(Error::from)
    }
}

impl Track {
    pub async fn new(
        db: &mut DBCon,
        name: String,
        artist_id: i64,
        duration: i64,
        album_id: i64,
        deezer_id: Option<i64>,
    ) -> Result<Self> {
        sqlx::query_as!(
            DBTrack,
            "
INSERT INTO track
    (name, artist_id, duration, album_id, deezer_id)
VALUES (?, ?, ?, ?, ?)
ON CONFLICT DO UPDATE set deezer_id=?
RETURNING *
",
            name,
            artist_id,
            duration,
            album_id,
            deezer_id,
            deezer_id
        )
        .fetch_one(db)
        .await
        .map(Track::from)
        .map_err(Error::from)
    }

    pub async fn get(db: &mut DBCon, id: i64) -> Result<Self> {
        sqlx::query_as!(
            DBTrack,
            r#"
SELECT *
FROM track
WHERE id=?
"#,
            id
        )
        .fetch_one(db)
        .await
        .map(Track::from)
        .map_err(Error::from)
    }
    pub async fn get_by_deezer_id(db: &mut DBCon, deezer_id: i64) -> Result<Option<Self>> {
        sqlx::query_as!(
            DBTrack,
            r#"
SELECT id as "id!", artist_id, deezer_id, album_id, duration, name, path
FROM track
WHERE deezer_id=?
"#,
            deezer_id
        )
        .fetch_optional(db)
        .await?
        .map(Track::from)
        .map(Result::Ok)
        .transpose()
    }
    pub async fn get_all(db: &mut DBCon) -> Result<impl Iterator<Item = Self>> {
        sqlx::query_as!(
            DBTrack,
            r#"
SELECT *
FROM track
"#
        )
        .fetch_all(db)
        .await
        .map(|s| s.into_iter().map(Track::from))
        .map_err(Error::from)
    }

    pub async fn download(&mut self, db: &mut DBCon, music_path: &str) -> Result<()> {
        //TODO use ? instead of unwrap
        //TODO test if file is corrupted
        //sqlite doesn't care about utf8, why should my path care? >:(
        let mut dl = deezer_downloader::downloader::Downloader::new()
            .await
            .unwrap();
        let path = make_track_path(music_path, &self.name, self.id);
        let song = dl
            .download_song(self.deezer_id.unwrap() as u64)
            .await
            .unwrap();
        song.write_to_file(&path).unwrap();
        let utf8_path = path.clone().into_os_string().into_string().unwrap();
        sqlx::query!(
            "
UPDATE track
SET path=?
WHERE id=?
            ",
            utf8_path,
            self.id
        )
        .execute(db)
        .await?;
        self.path = Some(path);
        Ok(())
    }

    pub async fn delete_track(self, db: &mut DBCon) -> Result<()> {
        sqlx::query!(
            "
DELETE FROM playlist_entry WHERE track_id=?
",
            self.id
        )
        .execute(&mut *db)
        .await?;
        sqlx::query!(
            "
DELETE FROM track WHERE id=?
",
            self.id
        )
        .execute(&mut *db)
        .await?;
        Ok(())
    }

    pub async fn delete_download(&mut self, db: &mut DBCon) -> Result<()> {
        let path = match self.path.take() {
            None => {
                return Ok(());
            }
            Some(p) => p,
        };
        sqlx::query!(
            r#"
UPDATE track
SET path=NULL
WHERE id=?
"#,
            self.id,
        )
        .execute(db)
        .await?;
        std::fs::remove_file(&path)?;
        Ok(())
    }
    pub async fn attach_deezer_id(&mut self, db: &mut DBCon, deezer_id: i64) -> Result<()> {
        sqlx::query!(
            r#"
UPDATE track
SET deezer_id=?
WHERE id=?
"#,
            deezer_id,
            self.id
        )
        .execute(db)
        .await?;
        self.deezer_id = Some(deezer_id);
        Ok(())
    }
    pub async fn in_playlists(&self, db: &mut DBCon) -> Result<Vec<PlaylistInfo>> {
        let playlist_ids = sqlx::query!(
            r#"
SELECT list_id
FROM playlist_entry
WHERE track_id=?
            "#,
            self.id
        )
        .fetch_all(&mut *db)
        .await?;
        let mut playlists = Vec::with_capacity(playlist_ids.len());
        for playlist_id in playlist_ids {
            playlists.push(PlaylistInfo::get(db, playlist_id.list_id).await?)
        }
        Ok(playlists)
    }
    pub async fn get_album(&self, db: &mut DBCon) -> Result<Album> {
        sqlx::query_as!(
            Album,
            r#"
SELECT *
FROM album
WHERE id=?
            "#,
            self.album_id
        )
        .fetch_one(db)
        .await
        .map_err(Error::from)
    }
    pub async fn get_artist(&self, db: &mut DBCon) -> Result<Artist> {
        sqlx::query_as!(
            Artist,
            r#"
SELECT *
FROM artist
WHERE id=?
            "#,
            self.artist_id
        )
        .fetch_one(db)
        .await
        .map_err(Error::from)
    }
}

impl PlaylistInfo {
    pub async fn get_all(db: &mut DBCon) -> Result<Vec<Self>> {
        sqlx::query_as!(
            PlaylistInfo,
            r#"
SELECT *
FROM playlist
"#,
        )
        .fetch_all(&mut *db)
        .await
        .map_err(Error::from)
    }
    pub async fn get(db: &mut DBCon, id: i64) -> Result<Self> {
        sqlx::query_as!(
            PlaylistInfo,
            r#"
SELECT *
FROM playlist
WHERE id=?
"#,
            id
        )
        .fetch_one(&mut *db)
        .await
        .map_err(Error::from)
    }
    pub async fn into_playlist(self, db: &mut DBCon) -> Result<Playlist> {
        let tracks = sqlx::query!(
            "
SELECT track_id
FROM playlist_entry
WHERE list_id=?
            ",
            self.id
        )
        .fetch_all(db)
        .await?
        .into_iter()
        .map(|t| t.track_id)
        .collect();
        Ok(Playlist {
            id: self.id,
            name: self.name,
            deezer_id: self.deezer_id,
            tracks,
        })
    }
}

impl Playlist {
    pub async fn new(db: &mut DBCon, name: &str, deezer_id: Option<i64>) -> Result<Self> {
        let pl = sqlx::query_as!(
            PlaylistInfo,
            r#"
INSERT INTO playlist (name, deezer_id)
VALUES (?, ?)
ON CONFLICT (name)
    DO UPDATE  SET name=?
RETURNING
    id, name, deezer_id
"#,
            name,
            deezer_id,
            name
        )
        .fetch_one(db)
        .await?;
        Ok(Playlist {
            id: pl.id,
            name: pl.name,
            tracks: vec![],
            deezer_id: pl.deezer_id,
        })
    }
    pub async fn get(db: &mut DBCon, id: i64) -> Result<Self> {
        PlaylistInfo::get(db, id).await?.into_playlist(db).await
    }
    pub async fn get_all(db: &mut DBCon) -> Result<Vec<Self>> {
        let playlist_infos = PlaylistInfo::get_all(db).await?;
        let mut playlists = Vec::with_capacity(playlist_infos.len());
        for playlist_info in playlist_infos {
            playlists.push(playlist_info.into_playlist(db).await?);
        }
        Ok(playlists)
    }
    pub async fn add_track(&mut self, db: &mut DBCon, track_id: i64) -> Result<()> {
        sqlx::query!(
            r#"
INSERT INTO playlist_entry (list_id, track_id)
VALUES (?, ?)
"#,
            self.id,
            track_id
        )
        .execute(db)
        .await?;
        self.tracks.push(track_id);
        Ok(())
    }
    pub async fn remove_track(&mut self, db: &mut DBCon, track_id: i64) -> Result<()> {
        todo!()
    }
    pub async fn get_tracks(&self, db: &mut DBCon) -> Result<impl Iterator<Item=Track>> {
        sqlx::query_as!(DBTrack, r#"
SELECT t.* FROM track AS t INNER JOIN playlist_entry AS p ON p.track_id=t.id WHERE p.list_id=?
"#, self.id)
        .fetch_all(db)
        .await
        .map(|s| s.into_iter().map(Track::from))
        .map_err(Error::from)
    }
    pub async fn sync_fs(&self, db: &mut DBCon, music_dir: &str) -> Result<()> {
        let playlist_path:PathBuf = [music_dir, &self.name].iter().collect();
        std::fs::create_dir_all(&playlist_path)?;
        for track in self.get_tracks(db).await? {
            let track_path = make_track_path(music_dir, &track.name, track.id);
            if track_path.exists() {
                let track_link_path: PathBuf = [
                    music_dir,
                    &self.name,
                    &format!("{}.mp3", track.name),
                ].iter().collect();
                std::os::unix::fs::symlink(track_path, track_link_path)?;
            }
        }
        Ok(())
    }
}

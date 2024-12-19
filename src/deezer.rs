use crate::*;

#[derive(serde::Deserialize, Debug)]
pub struct Paginate<T> {
    pub data: Vec<T>,
    pub total: i64,
}

#[derive(serde::Deserialize, Debug)]
pub struct Track {
    pub id: i64,
    #[serde(rename(deserialize = "readable"))]
    pub has_lyrics: bool,
    pub title: String,
    pub duration: i64,
    pub explicit_lyrics: bool,
    pub artist: Artist,
    pub album: Album,
}

#[derive(serde::Deserialize, Debug)]
pub struct Artist {
    pub id: i64,
    pub name: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct Album {
    pub id: i64,
    pub title: String,
}

impl Album {
    pub async fn upsert(&self, artist_id: i64, con: &mut DBCon) -> Result<i64, Error> {
        sqlx::query!(
            "
INSERT INTO album
    (name, artist_id, deezer_id)
VALUES (?, ?, ?)
ON CONFLICT (deezer_id)
    DO UPDATE SET deezer_id=(?)
RETURNING
    id
",
            self.title,
            artist_id,
            self.id,
            self.id
        )
        .fetch_one(con)
        .await
        .map(|s| s.id)
        .map_err(Error::from)
    }
}

impl Artist {
    pub async fn upsert(&self, con: &mut DBCon) -> Result<i64, Error> {
        sqlx::query!(
            "
INSERT INTO artist
    (name, deezer_id)
VALUES (?, ?)
ON CONFLICT (deezer_id)
    DO UPDATE SET deezer_id=(?)
RETURNING
    id
",
            self.name,
            self.id,
            self.id
        )
        .fetch_one(con)
        .await
        .map(|s| s.id)
        .map_err(Error::from)
    }
}

impl Track {
    pub async fn upsert(&self, con: &mut DBCon) -> Result<i64, Error> {
        let artist_id = self.artist.upsert(con).await?;
        let album_id = self.album.upsert(artist_id, con).await?;
        sqlx::query!("
INSERT INTO track
    (name, artist_id, duration, album_id, deezer_id)
VALUES (?, ?, ?, ?, ?)
ON CONFLICT DO UPDATE set deezer_id=?
RETURNING id
", self.title, artist_id, self.duration, album_id, self.id, self.id).fetch_one(con).await
            .map_err(Error::from)
            .map(|s|s.id)
    }
}

pub async fn get_liked(user_id: i64) -> Result<Vec<Track>, Error> {
    let cont: Paginate<Track> = reqwest::get(format!(
        "https://api.deezer.com/user/{user_id}/tracks?limit=999"
    ))
    .await?
    .json()
    .await?;
    Ok(cont.data)
}

#[derive(serde::Deserialize, Debug)]
pub struct PlaylistInfo {
    pub id: i64,
    pub title: String,
    pub duration: i64,
}

pub async fn get_playlists(user_id: i64) -> Result<Vec<PlaylistInfo>, Error> {
    let cont: Paginate<PlaylistInfo> = reqwest::get(format!(
        "https://api.deezer.com/user/{user_id}/playlists?limit=-1"
    ))
        .await?
        .json()
        .await?;
    Ok(cont.data)
}

#[derive(serde::Deserialize, Debug)]
struct APIPlaylistTracks {
    data: Vec<Track>,
}

#[derive(serde::Deserialize, Debug)]
struct APIPlaylist {
    id: i64,
    title: String,
    tracks: APIPlaylistTracks,
}

#[derive(Debug)]
pub struct Playlist {
    pub id: i64,
    pub title: String,
    pub tracks: Vec<Track>,
}

pub async fn get_playlist_tracks(platlist_id: i64) -> Result<Playlist, Error> {
    let cont: APIPlaylist = reqwest::get(format!(
        "https://api.deezer.com/playlist/{platlist_id}?limit=-1"
    ))
        .await?
        .json()
        .await?;
    Ok(Playlist {
        id: cont.id,
        title: cont.title,
        tracks: cont.tracks.data,
    })

}

pub struct DownloadTrack {
    pub id: i64,
    pub path: std::path::PathBuf,
}

pub async fn download_tracks(tracks: impl Iterator<Item = DownloadTrack>) -> Result<(), Error> {
    let mut dl = deezer_downloader::downloader::Downloader::new()
        .await
        .unwrap();
    for track in tracks {
        let song = dl.download_song(track.id as u64).await.unwrap();
        song.write_to_file(track.path).unwrap();
    }
    Ok(())
}


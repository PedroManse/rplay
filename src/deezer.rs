use crate::*;

#[derive(Debug, serde::Deserialize)]
pub struct DataVec<T> {
    pub data: Vec<T>,
    // pub total: Option<i64>, // just .data.len()
}

impl<T> std::ops::Deref for DataVec<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Vec<T> {
        &self.data
    }
}

impl<T> From<DataVec<T>> for Vec<T> {
    fn from(p: DataVec<T>) -> Vec<T> {
        p.data
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct Track {
    pub id: i64,
    #[serde(rename(deserialize = "readable"))]
    pub has_lyrics: bool,
    pub title: String,
    pub duration: i64,
    pub explicit_lyrics: bool,
    pub artist: ArtistRef,
    pub album: AlbumRef,
}

#[derive(serde::Deserialize, Debug)]
pub struct ArtistRef {
    pub id: i64,
    pub name: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct AlbumRef {
    pub id: i64,
    pub title: String,
}

pub async fn get_liked(user_id: i64) -> Result<Vec<Track>, Error> {
    reqwest::get(format!(
        "https://api.deezer.com/user/{user_id}/tracks?limit=-1"
    ))
    .await?
    .json::<DataVec<Track>>()
    .await
    .map_err(Error::from)
    .map(Vec::from)
}

#[derive(serde::Deserialize, Debug)]
pub struct PlaylistInfo {
    pub id: i64,
    pub title: String,
    pub duration: i64,
}

pub async fn get_user_playlists(user_id: i64) -> Result<Vec<PlaylistInfo>, Error> {
    let cont: DataVec<PlaylistInfo> = reqwest::get(format!(
        "https://api.deezer.com/user/{user_id}/playlists?limit=-1"
    ))
    .await?
    .json()
    .await?;
    Ok(cont.data)
}

#[derive(serde::Deserialize, Debug)]
struct APIPlaylist {
    id: i64,
    title: String,
    tracks: DataVec<Track>,
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

#[derive(Debug)]
pub struct DownloadRequest {
    pub deezer_id: i64,
    pub db_id: i64,
}

#[derive(Debug)]
pub struct DownloadedTrack {
    pub db_id: i64,
    pub deezer_id: i64,
    pub path: std::path::PathBuf,
}

pub async fn download_tracks(
    deezer_arl: &str,
    download_dir: &str,
    tracks: Vec<DownloadRequest>,
) -> Result<Vec<DownloadedTrack>, Error> {
    // run the program deezer.py with deezer_arl and tracks in argv
    let output = tokio::process::Command::new("python3")
        .arg("/home/manse/code/rplay/deezer.py")
        .env("DEEZER_ARL", deezer_arl)
        .arg(download_dir)
        .args(
            tracks
                .iter()
                .map(|x| format!("{}:{}", x.deezer_id, x.db_id)),
        )
        .output()
        .await?;
    //TODO: can't make error work with thiserror :(
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    println!("{stderr:?}");
    println!("{stdout:?}");
    Ok(stdout
        .lines()
        .map(std::path::PathBuf::from)
        .zip(tracks)
        .map(|(path, req)| DownloadedTrack {
            path,
            db_id: req.db_id,
            deezer_id: req.deezer_id,
        })
        .collect())
}

#[derive(Debug, serde::Deserialize)]
pub struct Genre {
    pub id: i64,
    pub name: String,
    // pub picture: String,
    // pub type: String, // always "genre"
}

#[derive(Debug, serde::Deserialize)]
pub struct Contributor {
    pub id: i64,
    pub name: String,
    pub role: ContributorRole,
}

impl From<Contributor> for ArtistRef {
    fn from(c: Contributor) -> ArtistRef {
        ArtistRef {
            id: c.id,
            name: c.name,
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub enum ContributorRole {
    Main,
}

// didn't leave comments about links, cover or picture information
#[derive(Debug, serde::Deserialize)]
pub struct Album {
    pub id: i64,
    pub title: String,
    //pub upc: String,
    pub link: String,
    //pub share: String,
    pub genre_id: i64,
    pub genres: DataVec<Genre>,
    pub label: String,
    //#[serde(rename(deserialize = "nb_tracks"))]
    //pub track_count: i64, // just .tracks.len()
    pub duration: i64,
    //pub fans: i64,
    pub release_date: String, // yyyy-mm-dd format
    //pub record_type: String, // always "album"
    pub available: bool, // hopefuly true
    //pub tracklist: String, // already have tracks
    pub explicit_lyrics: bool,
    // pub explicit_content_lyrics:i64, // idk how this works
    // pub explicit_content_cover:i64,  // |||
    pub contributors: Vec<Contributor>,
    pub artist: ArtistRef,
    pub tracks: DataVec<Track>,
}

pub async fn get_album(album_id: i64) -> Result<Album, Error> {
    reqwest::get(format!("https://api.deezer.com/album/{album_id}"))
        .await?
        .json()
        .await
        .map_err(Error::from)
}

pub async fn get_track(track_id: i64) -> Result<Track, Error> {
    let x = reqwest::get(format!("https://api.deezer.com/track/{track_id}"))
        .await?
        .text()
        .await?;
    match serde_json::from_str(&x) {
        Ok(j)=>Ok(j),
        Err(_)=>Err(Error::DeezerError(x)),
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct InsertTrack {
    pub name: String,
    pub artist_id: u32,
    pub duration: Option<u32>,
    pub album: Option<String>,

    pub deezer_id: Option<u32>,
    //pub youtube_id: Option<String>,
    //pub spotify_id: Option<String>,
}


#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct InsertArtist {
    pub deezer_id: u32,
    pub name: String,
}


#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct InsertInfo {
    pub tracks: Vec<InsertTrack>,
    pub artists: Vec<InsertArtist>,
}

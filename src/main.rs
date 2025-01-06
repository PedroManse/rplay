use rplay::*;
use sqlx::Connection;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenvy::dotenv()?;
    let dir = std::env::var("RPLAY_DIR")?;
    let deezer_arl = std::env::var("DEEZER_ARL")?;
    let mut db = sqlx::SqliteConnection::connect(&format!("{dir}/.db.sqlite3")).await?;
    let con = &mut db;
    //    deezer::download_tracks(&deezer_arl, &format!("{}/.downloaded", dir), vec![
    //        deezer::DownloadRequest{deezer_id: 130180264, db_id: 999}
    //    ]).await?;

    //let playlists = db::Playlist::get_all(con).await?;
    //for playlist in playlists {
    //    playlist.sync_fs(con, &dir).await?;
    //}

    //let to_download = db::TrackInfo::get_all(con)
    //    .await?
    //    .filter(|track| {
    //        !track.path.is_some() && track.deezer_id.is_some()
    //    })
    //    .map(|track| {
    //        deezer::DownloadRequest{
    //            deezer_id: track.deezer_id.unwrap(),
    //            db_id: track.id,
    //        }
    //    }).take(4).collect();
    //println!("{to_download:?}");
    //let d = deezer::download_tracks(
    //    &deezer_arl,
    //    &format!("{}/.downloaded", dir),
    //    to_download,
    //).await?;
    //println!("{d:?}");

    let tracks = db::TrackInfo::get_all(con).await?;
    use std::collections::HashSet;
    let tracks: HashSet<_> =tracks.filter_map(|t|t.deezer_id).collect();
    let stored = |deezer_id|{
        tracks.contains(deezer_id)
    };
    for track in deezer::get_liked(6144247603).await? {
        if tracks.contains(&track.id) {
            println!("{track:?}");
        }
    }

    Ok(())
}

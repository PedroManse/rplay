use rplay::*;
use sqlx::Connection;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenvy::dotenv()?;
    let dir = std::env::var("RPLAY_DIR")?;
    let mut db = sqlx::SqliteConnection::connect(&format!("{dir}/.db.sqlite3")).await?;
    let con = &mut db;
    let playlists = db::Playlist::get_all(con).await?;
    for playlist in playlists {
        playlist.sync_fs(con, &dir).await?;
    }
    Ok(())
}

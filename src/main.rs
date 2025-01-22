use rplay::*;
use sqlx::Connection;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenvy::dotenv()?;
    let dir = std::env::var("RPLAY_DIR")?;
    let _deezer_arl = std::env::var("DEEZER_ARL")?;
    let mut _db = sqlx::SqliteConnection::connect(&format!("{dir}/.db.sqlite3")).await?;
    download::deezer::make_session(_deezer_arl).await;
    download::deezer::download_track().await?;
    Ok(())
}

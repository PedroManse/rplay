use rplay::*;
use sqlx::Connection;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenvy::dotenv()?;
    let dir = std::env::var("RPLAY_DIR")?;
    let mut db = sqlx::SqliteConnection::connect(&format!("{dir}/.db.sqlite3")).await?;
    let con = &mut db;

    let tracks = db::Track::get_all(con).await?;
    for mut track in tracks {
        let downloaded = track
                .path.clone()
                .map(|s| PathBuf::from(s).exists())
                .unwrap_or(false);
        if !downloaded {
            if track.deezer_id.is_some() {
                track.download(con, &dir).await?;
            }
            println!("{:?}", track.path);
            let downloaded = track
                    .path.clone()
                    .map(|s| PathBuf::from(s).exists())
                    .unwrap_or(false);
            println!(
                "#{}: {} [{}]",
                track.id,
                col!(Blue track.name),
                //track.artist_name,
                if downloaded {
                    col!(Green "↓")
                } else {
                    col!(Red "↓")
                },
            );
        }
    }
    Ok(())
}

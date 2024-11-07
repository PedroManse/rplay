use rplay::*;
use sqlx::Connection;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenvy::dotenv()?;
    let mut con = sqlx::SqliteConnection::connect(&std::env::var("DATABASE_URL")?).await?;
    let info: InsertInfo = serde_json::from_str(include_str!("../liked.json"))?;
    for artist in info.artists {
        sqlx::query!(
            r#"
INSERT OR IGNORE INTO artist (
    name, deezer_id
) VALUES (?, ?)
"#,
            artist.name,
            artist.deezer_id
        )
        .execute(&mut con)
        .await?;
    }
    for tracks in info.tracks {
        let artist_id = sqlx::query!(
            "SELECT id FROM artist where deezer_id = ?",
            tracks.artist_id
        )
        .fetch_one(&mut con)
        .await?.id;
        sqlx::query!(
            r#"
INSERT INTO track (
    name, artist_id, duration, album, deezer_id
) VALUES (?, ?, ?, ?, ?)
"#,
            tracks.name,
            artist_id,
            tracks.duration,
            tracks.album,
            tracks.deezer_id,
        )
        .execute(&mut con)
        .await?;
    }
    Ok(())
}

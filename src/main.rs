use rplay::*;
use sqlx::Connection;
use std::path::PathBuf;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    dotenvy::dotenv()?;
    let mut con: sqlx::SqliteConnection =
        sqlx::SqliteConnection::connect(&std::env::var("DATABASE_URL")?).await?;
    let songs_dir = std::env::var("SONG_PATH")?;
    let songs = sqlx::query!(r#"
SELECT
    id, name, path as "path!"
FROM track
WHERE
    deezer_id IS NOT NULL AND
    path IS NOT NULL AND
    path NOT LIKE "/tmp%"
"#)
        .fetch_all(&mut con)
        .await?;
    for song in songs {
        let path = format!("{}/.downloaded/{}.{}.mp3", songs_dir, song.name.replace("/", "|"), song.id);
        if song.path != path {
            println!("{} -> {}", song.path.replace("//", "/"), path);
            std::fs::rename(&song.path, &path)?;
            sqlx::query!("UPDATE track SET path=? WHERE id=?", path, song.id).execute(&mut con).await?;
        }
    }
    Ok(())
}

// download all songs from deezer
//let mut downloader = deezer::DeezerDownloader::new().await?;
//for song in songs {
//    println!("{song:?}");
//    let path = format!("{}/.downloaded/{}.{}.mp3", songs_dir, song.name.replace("/", "\\/"), song.id.unwrap());
//    println!("{path}");
//    downloader.download(
//        song.deezer_id.unwrap() as u64, &path,
//    ).await?;
//    sqlx::query!("UPDATE track SET path=? WHERE id=?", path, song.id).execute(&mut con).await?;
//}

// define liked songs
//let songs = deezer::get_liked(6144247603).await?;
//for song in songs {
//    println!("fixing song: {}", song.title);
//    song.upsert(&mut con).await?;
//}

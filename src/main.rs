use rplay::*;
use sqlx::Connection;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenvy::dotenv()?;
    let mut con: sqlx::SqliteConnection = sqlx::SqliteConnection::connect(&std::env::var("DATABASE_URL")?).await?;
    let songs = deezer::get_liked(6144247603).await?;
    for song in songs {
        println!("fixing song: {}", song.title);
        song.upsert(&mut con).await?;
//        // apparently sqlite can't return id while ignoring the conflict :(
//        // 'failed' insert consumes autoincrement
//        let artist_id = sqlx::query!("
//INSERT INTO artist
//    (name, deezer_id)
//VALUES (?, ?)
//ON CONFLICT (deezer_id)
//    DO UPDATE SET deezer_id=(?)
//RETURNING
//    id
//", song.artist.name, song.artist.id, song.artist.id).fetch_one(&mut con).await?.id;
//        println!("artist id: {artist_id}");
//
//        let album_id = sqlx::query!("
//INSERT INTO album
//    (name, artist_id, deezer_id)
//VALUES (?, ?, ?)
//ON CONFLICT (deezer_id)
//    DO UPDATE SET deezer_id=(?)
//RETURNING
//    id
//", song.album.title, artist_id, song.album.id, song.album.id).fetch_one(&mut con).await?.id;
//
//        println!("album id: {album_id}");
//        sqlx::query!("
//INSERT INTO track
//    (name, artist_id, duration, album_id, deezer_id)
//VALUES (?, ?, ?, ?, ?)
//ON CONFLICT DO NOTHING
//", song.title, artist_id, song.duration, album_id, song.id).execute(&mut con).await?;
    }
    Ok(())
}


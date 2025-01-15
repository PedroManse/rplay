use std::collections::HashMap;
use crate::*;

pub async fn get_albums_from_tracks(db: &mut DBCon) -> Result<(), Error> {
    let artists_by_dzid: HashMap<i64, i64> = sqlx::query!(
        "SELECT id as 'id!', deezer_id as 'deezer_id!' FROM artist WHERE deezer_id IS NOT NULL",
    ).fetch_all(&mut *db).await?.into_iter().map(|r|(r.deezer_id, r.id)).collect();
    let albums = sqlx::query!(
        "SELECT id, name, deezer_id as 'deezer_id!' FROM album WHERE deezer_id IS NOT NULL AND id > 43"
    )
    .fetch_all(&mut *db)
    .await?;

    for album in albums {
        let dz_album = match deezer::get_album(album.deezer_id).await {
            Ok(al) => al,
            Err(err) => {
                eprintln!("#{:04} | {}: {}", album.id, album.deezer_id, err);
                continue;
            }
        };
        println!("#{:04} | {} : {} tracks", album.id, album.deezer_id, dz_album.tracks.len());
        let Some(artist_id) = artists_by_dzid.get(&dz_album.artist.id) else {
            eprintln!("#{:04} | {}: missing artist {}", album.id, album.deezer_id, dz_album.artist.id);
            continue
        };
        for track in dz_album.tracks.iter() {
            db::Track::new(
                &mut *db,
                &track.title,
                *artist_id,
                track.duration,
                album.id,
                Some(track.album.id),
            )
            .await?;
        }
        //println!("#{:03} | {}: {}", album.id, album.name, dz_album.tracks.len());
        //println!("{:?}", dz_album.tracks);
    }
    Ok(())
}

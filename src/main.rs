use rplay::*;

#[derive(Debug, serde::Deserialize)]
struct Video {
    name: String,
    id: String,
    by: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenvy::dotenv()?;
    let videos: Vec<Video> = serde_json::from_str(include_str!("../ids.json"))?;
    let mut done = 0;
    let total = videos.len();
    for video in videos.into_iter().skip(done) {
        print!("{} ", video.id);
    }
    Ok(())
}


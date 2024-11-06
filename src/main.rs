use rplay::*;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    dotenvy::dotenv()?;
    let videos = query("Way down hades town").await?;
    println!("Videos found:");
    for video in videos {
        println!("{} - {} @ https://youtube.com/watch?v={}", video.name, video.by, video.id);
    }
    Ok(())
}


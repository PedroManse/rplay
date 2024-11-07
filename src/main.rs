use rplay::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenvy::dotenv()?;
    let videos: Vec<VideoQuery> = serde_json::from_str(&std::fs::read_to_string("./ids.json")?)?;
    let mut video_db: Vec<DownloadedVideo> = Vec::with_capacity(videos.len());

    let mut
        paths: Vec<_> = std::fs::read_dir("./videos")
        .unwrap()
        .into_iter()
        .map(Result::unwrap)
        .map(|p| (p.path().clone(), p.file_name().into_string().unwrap()))
        .collect();
    for video in videos {
        let vst = format!("{} - {}.opus", video.name, video.by);
        let (diff, _, pos, path) = paths.iter().enumerate().fold(
            (999, "".to_owned(), 1, PathBuf::from("")),
            |(pcount, pstr, ppos, ppath), (npos, (path, test))| {
                // """short circuit"""
                if pcount == 0 {
                    return (pcount, pstr, ppos, ppath);
                }
                let test_count = lev(&vst, &test);
                if test_count < pcount {
                    (test_count, test.to_owned(), npos, path.clone())
                } else {
                    (pcount, pstr, ppos, ppath)
                }
            },
        );
        if paths.len() > 1 && diff == 0 {
            paths.swap_remove(pos);
        }
        video_db.push(DownloadedVideo {
            id: video.id,
            by: video.by,
            name: video.name,
            path: path.clone(),
        });
        if diff >= 3 {
            println!("{diff}, {path:?} || {vst}");
        }
    }
    use std::io::Write;
    let mut video_db_file = std::fs::File::create("videos_info.json")?;
    video_db_file.write_all(&serde_json::to_vec(&video_db)?)?;
    Ok(())
}


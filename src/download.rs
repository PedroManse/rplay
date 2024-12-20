use crate::*;

pub trait Downloader {
    type SongId;
    fn download(
        &mut self,
        song_id: Self::SongId,
        file_path: impl AsRef<std::path::Path>,
    ) -> impl std::future::Future<Output = Result<(), Error>>;

    async fn download_many(
        &mut self,
        songs: impl Iterator<Item = (Self::SongId, impl AsRef<std::path::Path>)>,
    ) -> Result<(), Error> {
        for (song, path) in songs {
            self.download(song, path).await?;
        }
        Ok(())
    }
}

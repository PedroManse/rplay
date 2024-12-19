pub mod deezer;
pub mod db;
use db::DBCon;
pub use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    EnvError(#[from] dotenvy::Error),
    #[error(transparent)]
    VarError(#[from] std::env::VarError),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    ReqError(#[from] reqwest::Error),
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
    #[error(transparent)]
    SqlError(#[from] sqlx::Error),
}

fn min3<T: Ord>(a: T, b: T, c: T) -> T {
    std::cmp::min(std::cmp::min(a, b), c)
}

// Using the Wagner-Fischer algorithm to compute the Levenshtein distance of two strings
pub fn lev(s0: &str, s1: &str) -> usize {
    if s0.is_empty() {
        return s1.chars().count();
    } else if s1.is_empty() {
        return s0.chars().count();
    }

    let s1_chars: Vec<char> = s1.chars().collect();

    // get lengths of string 1
    let len_1 = s1_chars.len();
    let mut row: Vec<usize> = (0..=len_1).collect();
    let mut d0 = 0;

    for (i, s0_char) in s0.chars().enumerate() {
        let mut e = i + 1;

        for j in 0..len_1 {
            let c: usize = (s0_char != s1_chars[j]) as usize;
            d0 = min3(row[j + 1] + 1, e + 1, row[j] + c);

            row[j] = e;
            e = d0;
        }

        row[len_1] = d0;
    }

    row[len_1]
}

#[macro_export]
macro_rules! col {
    ($r:literal $g:literal $b:literal) => {
        format!("ESC[38;2;{};{};{}m", $r, $g, $b)
    };
    (Black) => { "\x1b[30m" };
    (Red) => { "\x1b[31m" };
    (Green) => { "\x1b[32m" };
    (Yellow) => { "\x1b[33m" };
    (Blue) => { "\x1b[34m" };
    (Magenta) => { "\x1b[35m" };
    (Cyan) => { "\x1b[36m" };
    (White) => { "\x1b[37m" };
    (Reset) => { "\x1b[0m" };
    (Grey) => { "\x1b[37;2m" };
    ($c:tt $tx:expr) => {
        format!("{}{}{}", col!($c), $tx, col!(Reset),)
    };
}

fn replace_fs_chars(text: &str) -> String {
    text.replace("|", "/")
}


fn make_track_path(music_dir: &str, track_name: &str, track_id: i64) -> PathBuf {
    [
        music_dir,
        ".downloaded",
        &format!("{}.{}.mp3", replace_fs_chars(track_name), track_id),
    ]
    .iter()
    .collect()
}

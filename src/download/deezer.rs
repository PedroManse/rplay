use crate::*;
use reqwest::{header::HeaderMap, Client};
use std::collections::HashMap;

static SESSION: std::sync::OnceLock<Client> = std::sync::OnceLock::new();

type Result<T> = std::result::Result<T, Error>;

pub fn get_session() -> &'static Client {
    &SESSION.get().unwrap()
}

pub async fn make_session(deezer_arl: String) {
    let headers = [
        ("X-Requested-With", "XMLHttpRequest"),
        ("PRAGMA", "no-cache"),
        ("ORIGIN", "https://www.deezer.com"),
        ("ACCEPT_ENCODING", "gzip, deflate, br"),
        ("ACCEPT_LANGUAGE", "en-US,en;q=0.9"),
        ("USER_AGENT", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/68.0.3440.106 Safari/537.36"),
        ("CONTENT_TYPE", "application/x-www-form-urlencoded; charset=UTF-8"),
        ("ACCEPT", "*/*"),
        ("CACHE_CONTROL", "no-cache"),
        ("CONNECTION", "keep-alive"),
        ("REFERER", "www.deezer.com/login"),
        ("DNT", "1"),
    ].into_iter().map(|(name, value)|
        (name.to_owned(), value.to_owned())
    ).collect::<HashMap<_, _>>();
    let headers: HeaderMap = (&headers).try_into().unwrap();

    let cookies = reqwest::cookie::Jar::default();
    let deezer_url = reqwest::Url::parse("https://deezer.com").unwrap();
    cookies.add_cookie_str(&format!("arl={deezer_arl}"), &deezer_url);
    cookies.add_cookie_str("comeback=1", &deezer_url);

    SESSION
        .set(
            reqwest::ClientBuilder::new()
                .default_headers(headers)
                .cookie_store(true)
                .cookie_provider(cookies.into())
                .build()
                .unwrap(),
        )
        .unwrap();
}

pub async fn download_track() -> Result<()> {
    let res = get_session()
        .get("https://www.deezer.com/br")
        .send()
        .await
        .unwrap()
        .text()
        .await?;
    println!("{}", res);
    Ok(())
}

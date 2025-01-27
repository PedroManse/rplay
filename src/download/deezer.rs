use reqwest::{header::HeaderMap, Client};
use std::collections::HashMap;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ReqError(#[from] reqwest::Error),
    #[error("Song #{0} not found")]
    SongNotFound(i64),
    #[error("Not logged in; deeezer ARL may be invalid")]
    NotLoggedIn,
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
}

static SESSION: std::sync::OnceLock<Client> = std::sync::OnceLock::new();
static USEROPTIONS: std::sync::OnceLock<UserOptions> = std::sync::OnceLock::new();

type Result<T> = std::result::Result<T, Error>;

pub fn get_session() -> &'static Client {
    &SESSION.get().unwrap()
}

pub async fn make_session(deezer_arl: &str) {
    let headers = [
        ("X-Requested-With", "XMLHttpRequest"),
        ("PRAGMA", "no-cache"),
        ("ORIGIN", "http://www.deezer.com"),
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

    let deezer_url = reqwest::Url::parse("http://deezer.com").unwrap();
    let jar = reqwest::cookie::Jar::default();
    let cookies = [("arl", deezer_arl), ("comeback", "1")];
    for (name, value) in cookies {
        jar.add_cookie_str(&format!("{name}={value}"), &deezer_url);
    }

    SESSION
        .set(
            reqwest::ClientBuilder::new()
                .default_headers(headers)
                .cookie_store(true)
                .cookie_provider(jar.into())
                .build()
                .unwrap(),
        )
        .unwrap();
    get_user_data().await.unwrap();
}

#[derive(Debug, serde::Deserialize)]
struct UserOptions {
    web_sound_quality: SoundQuality,
    license_token: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
struct SoundQuality {
    high: bool,
    lossless: bool,
    low: bool,
    reality: bool,
    standard: bool,
}

async fn get_user_data() -> Result<()> {
    #[derive(serde::Deserialize)]
    struct DeezerWrapper<T> {
        results: T,
    }
    #[derive(serde::Deserialize)]
    struct UserWrapper {
        #[serde(rename(deserialize = "USER"))]
        user: UserOptionsWrapper,
    }
    #[derive(serde::Deserialize)]
    struct UserOptionsWrapper {
        #[serde(rename(deserialize = "OPTIONS"))]
        options: UserOptions,
    }
    let info: DeezerWrapper<UserWrapper> = get_session().get("http://www.deezer.com/ajax/gw-light.php?method=deezer.getUserData&input=3&api_version=1.0&api_token=").send().await?.json().await?;
    let info: UserOptions = info.results.user.options;
    USEROPTIONS.set(info).unwrap();
    Ok(())
}

pub async fn download_track(id: i64) -> Result<()> {
    let url = format!("https://www.deezer.com/track/{id}");
    let res = get_session().get(url).send().await?;
    if res.status() == reqwest::StatusCode::NOT_FOUND {
        return Err(Error::SongNotFound(id));
    }

    let text = res.text().await?;
    let dom = tl::parse(&text, tl::ParserOptions::default()).unwrap();
    let parser = dom.parser();
    let data_script = dom
        .query_selector("script")
        .unwrap()
        .filter_map(|rf| rf.get(parser))
        .map(|nd|nd.inner_text(parser))
        .filter(|tx|tx.find("DATA").is_some())
        .next()
        .ok_or(Error::NotLoggedIn)?;
    let data_script: serde_json::Value = serde_json::from_slice(&data_script.as_bytes()[27..])?;
    println!("{}", data_script);

    //text.find("MDS_ORIGIN").ok_or(Error::NotLoggedIn)?;

    //    let res = get_session()
    //        .get("https://deezer.com")
    //        .send()
    //        .await
    //        .unwrap()
    //        .text()
    //        .await?;
    //    println!("{}", res)),
    Ok(())
}


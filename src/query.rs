use crate::*;

async fn get_ytinfo(query: &str) -> Result<String, Error> {
    let search = r#"var ytInitialData = (\{.*\});</script>"#;
    let searcher = regex::Regex::new(search).unwrap();
    let content = reqwest::get(format!(
        "https://www.youtube.com/results?search_query={query}"
    ))
    .await?
    .text()
    .await?;
    Ok(searcher.captures(&content).unwrap()[1].to_owned())
}

fn extract_single(vren: &serde_json::Value) -> Option<VideoQuery> {
    Some(VideoQuery {
        name: vren["title"]["runs"][0]["text"].as_str()?.to_owned(),
        by: vren["ownerText"]["runs"][0]["text"].as_str()?.to_owned(),
        id: vren["videoId"].as_str()?.to_owned(),
    })
}

fn extract(info: String) -> Vec<VideoQuery> {
    let info: serde_json::Value = serde_json::from_str(&info).unwrap();
    info["contents"]["twoColumnSearchResultsRenderer"]["primaryContents"]["sectionListRenderer"]
        ["contents"]
        .as_array()
        .unwrap()
        .into_iter()
        .filter_map(|y| y["itemSectionRenderer"]["contents"].as_array())
        .flatten()
        .filter_map(|y| extract_single(&y["videoRenderer"]))
        .collect()
}

pub async fn query(query: &str) -> Result<Vec<VideoQuery>, Error> {
    get_ytinfo(query).await.map(extract)
}

pub use crate::types::*;
use crate::AnimeTableLayout;

#[tokio::main]
pub async fn fetch_seasonal_anime() -> Result<Vec<AnimeTableLayout>, reqwest::Error> {
    let ranking_filter: f64 = 7.5;
    let url: &str = "https://api.jikan.moe/v4/seasons/now";

    let mut animes = vec![];
    let response: Root = reqwest::get(url).await?.json().await?;

    for anime in response.data.iter() {
        if anime.score.unwrap() > ranking_filter {
            animes.push(AnimeTableLayout::new(
                &anime.title_japanese,
                &anime.broadcast.string,
                &anime.aired.string,
            ))
        }
    }

    Ok(animes)
}

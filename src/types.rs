use serde_derive::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub pagination: Pagination,
    pub data: Vec<Daum>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    #[serde(rename = "last_visible_page")]
    pub last_visible_page: i64,
    #[serde(rename = "has_next_page")]
    pub has_next_page: bool,
    #[serde(rename = "current_page")]
    pub current_page: i64,
    pub items: Items,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Items {
    pub count: i64,
    pub total: i64,
    #[serde(rename = "per_page")]
    pub per_page: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Daum {
    #[serde(rename = "mal_id")]
    pub mal_id: i64,
    pub url: String,
    pub images: Images,
    pub trailer: Trailer,
    pub approved: bool,
    pub titles: Vec<Title>,
    pub title: String,
    #[serde(rename = "title_english")]
    pub title_english: Option<String>,
    #[serde(rename = "title_japanese")]
    pub title_japanese: String,
    #[serde(rename = "title_synonyms")]
    pub title_synonyms: Vec<String>,
    #[serde(rename = "type")]
    pub type_field: String,
    pub source: String,
    pub episodes: Option<i64>,
    pub status: String,
    pub airing: bool,
    pub aired: Aired,
    pub duration: String,
    pub rating: Option<String>,
    pub score: Option<f64>,
    #[serde(rename = "scored_by")]
    pub scored_by: Option<i64>,
    pub rank: Option<i64>,
    pub popularity: i64,
    pub members: i64,
    pub favorites: i64,
    pub synopsis: String,
    pub background: Option<String>,
    pub season: String,
    pub year: i64,
    pub broadcast: Broadcast,
    pub producers: Vec<Producer>,
    pub licensors: Vec<Licensor>,
    pub studios: Vec<Studio>,
    pub genres: Vec<Genre>,
    #[serde(rename = "explicit_genres")]
    pub explicit_genres: Vec<Genre>,
    pub themes: Vec<Theme>,
    pub demographics: Vec<Demographic>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Images {
    pub jpg: Jpg,
    pub webp: Webp,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Jpg {
    #[serde(rename = "image_url")]
    pub image_url: String,
    #[serde(rename = "small_image_url")]
    pub small_image_url: String,
    #[serde(rename = "large_image_url")]
    pub large_image_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Webp {
    #[serde(rename = "image_url")]
    pub image_url: String,
    #[serde(rename = "small_image_url")]
    pub small_image_url: String,
    #[serde(rename = "large_image_url")]
    pub large_image_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trailer {
    #[serde(rename = "youtube_id")]
    pub youtube_id: Option<String>,
    pub url: Option<String>,
    #[serde(rename = "embed_url")]
    pub embed_url: Option<String>,
    pub images: Option<Images2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Images2 {
    #[serde(rename = "image_url")]
    pub image_url: Option<String>,
    #[serde(rename = "small_image_url")]
    pub small_image_url: Option<String>,
    #[serde(rename = "medium_image_url")]
    pub medium_image_url: Option<String>,
    #[serde(rename = "large_image_url")]
    pub large_image_url: Option<String>,
    #[serde(rename = "maximum_image_url")]
    pub maximum_image_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Title {
    #[serde(rename = "type")]
    pub type_field: String,
    pub title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Aired {
    pub from: String,
    pub to: Option<String>,
    pub prop: Prop,
    pub string: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Prop {
    pub from: From,
    pub to: To,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct From {
    pub day: i64,
    pub month: i64,
    pub year: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct To {
    pub day: Option<i64>,
    pub month: Option<i64>,
    pub year: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Broadcast {
    pub day: String,
    pub time: String,
    pub timezone: String,
    pub string: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Producer {
    #[serde(rename = "mal_id")]
    pub mal_id: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Licensor {
    #[serde(rename = "mal_id")]
    pub mal_id: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Studio {
    #[serde(rename = "mal_id")]
    pub mal_id: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Genre {
    #[serde(rename = "mal_id")]
    pub mal_id: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Theme {
    #[serde(rename = "mal_id")]
    pub mal_id: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Demographic {
    #[serde(rename = "mal_id")]
    pub mal_id: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub name: String,
    pub url: String,
}

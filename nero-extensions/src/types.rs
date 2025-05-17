use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Page<T> {
    pub items: Vec<T>,
    pub has_next_page: bool,
}

pub type SeriesPage = Page<Series>;
pub type EpisodesPage = Page<Episode>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Series {
    pub id: String,
    pub title: String,
    pub poster_url: Option<Url>,
    pub synopsis: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Episode {
    pub id: String,
    pub number: u16,
    pub title: Option<String>,
    pub thumbnail_url: Option<Url>,
    pub description: Option<String>,
}

type Resolution = (u16, u16);
type Header = (String, String);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    pub url: Url,
    pub headers: Vec<Header>,
    pub server: String,
    pub resolution: Resolution,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Filter {
    pub id: String,
    pub display_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterCategory {
    pub id: String,
    pub display_name: String,
    pub filters: Vec<Filter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchFilter {
    pub id: String,
    pub values: Vec<String>,
}

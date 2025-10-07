use serde::{Deserialize, Serialize};
use url::Url;

use crate::AppState;

pub trait AsyncTryFromWithState<T>: Sized {
    async fn async_try_from_with_state(value: T, state: &AppState) -> anyhow::Result<Self>;
}

pub trait AyncTryIntoWithState<T>: Sized {
    async fn async_try_into_with_state(self, state: &AppState) -> anyhow::Result<T>;
}

impl<T, U> AyncTryIntoWithState<U> for T
where
    U: AsyncTryFromWithState<T>,
{
    async fn async_try_into_with_state(self, state: &AppState) -> anyhow::Result<U> {
        U::async_try_from_with_state(self, state).await
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Page<T> {
    pub items: Vec<T>,
    pub has_next_page: bool,
}

impl<T, U> AsyncTryFromWithState<nero_extensions::types::Page<T>> for Page<U>
where
    U: AsyncTryFromWithState<T>,
{
    async fn async_try_from_with_state(
        page: nero_extensions::types::Page<T>,
        state: &AppState,
    ) -> anyhow::Result<Self> {
        let mut items = Vec::with_capacity(page.items.len());
        for item in page.items {
            items.push(U::async_try_from_with_state(item, state).await?);
        }
        Ok(Self {
            items,
            has_next_page: page.has_next_page,
        })
    }
}

pub type SeriesPage = Page<Series>;
pub type EpisodesPage = Page<Episode>;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Series {
    pub id: String,
    pub title: String,
    pub poster_url: Option<Url>,
    pub synopsis: Option<String>,
    pub r#type: Option<String>,
}

impl AsyncTryFromWithState<nero_extensions::types::Series> for Series {
    async fn async_try_from_with_state(
        series: nero_extensions::types::Series,
        state: &AppState,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            id: series.id,
            title: series.title,
            poster_url: match series.poster_resource {
                Some(req) => Some(state.processor.store_outgoing_request(req).await?),
                None => None,
            },
            synopsis: series.synopsis,
            r#type: series.r#type,
        })
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Episode {
    pub id: String,
    pub number: u16,
    pub title: Option<String>,
    pub thumbnail_url: Option<Url>,
    pub description: Option<String>,
}

impl AsyncTryFromWithState<nero_extensions::types::Episode> for Episode {
    async fn async_try_from_with_state(
        episode: nero_extensions::types::Episode,
        state: &AppState,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            id: episode.id,
            number: episode.number,
            title: episode.title,
            thumbnail_url: match episode.thumbnail_resource {
                Some(req) => Some(state.processor.store_outgoing_request(req).await?),
                None => None,
            },
            description: episode.description,
        })
    }
}

type Resolution = (u16, u16);

#[derive(Debug, Serialize)]
pub struct Video {
    url: Url,
    server: String,
    resolution: Resolution,
}

impl AsyncTryFromWithState<nero_extensions::types::Video> for Video {
    async fn async_try_from_with_state(
        video: nero_extensions::types::Video,
        state: &AppState,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            url: state
                .processor
                .store_outgoing_request(video.http_resource)
                .await?,
            server: video.server,
            resolution: video.resolution,
        })
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Filter {
    pub id: String,
    pub display_name: String,
}

impl From<nero_extensions::types::Filter> for Filter {
    fn from(filter: nero_extensions::types::Filter) -> Self {
        Self {
            id: filter.id,
            display_name: filter.display_name,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterCategory {
    pub id: String,
    pub display_name: String,
    pub filters: Vec<Filter>,
}

impl From<nero_extensions::types::FilterCategory> for FilterCategory {
    fn from(category: nero_extensions::types::FilterCategory) -> Self {
        Self {
            id: category.id,
            display_name: category.display_name,
            filters: category.filters.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct SearchFilter {
    pub id: String,
    pub values: Vec<String>,
}

impl From<SearchFilter> for nero_extensions::types::SearchFilter {
    fn from(filter: SearchFilter) -> Self {
        Self {
            id: filter.id,
            values: filter.values,
        }
    }
}

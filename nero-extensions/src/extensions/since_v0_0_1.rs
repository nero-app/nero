use self::nero::extension::types::{
    Episode, EpisodesPage, Filter, FilterCategory, SearchFilter, Series, SeriesPage, Video,
};

use anyhow::Ok;
use http_body_util::BodyExt;
use wasmtime::component::{Resource, bindgen};
use wasmtime_wasi::p2::IoView;
use wasmtime_wasi_http::{
    bindings::http::types::{Method, Scheme},
    types::HostOutgoingRequest,
};

use crate::{
    extensions::{AsyncTryFromWithStore, AsyncTryIntoWithStore},
    semver::SemanticVersion,
};

pub const MIN_VER: SemanticVersion = SemanticVersion::new(0, 0, 1);

bindgen!({
    path: "./wit/v0.0.1",
    async: true,
    with: {
        "wasi": wasmtime_wasi::p2::bindings,
        "wasi:http": wasmtime_wasi_http::bindings::http,
        "wasi:logging/logging": crate::logging::logging,
    },
});

impl From<Filter> for crate::types::Filter {
    fn from(filter: Filter) -> Self {
        crate::types::Filter {
            id: filter.id,
            display_name: filter.display_name,
        }
    }
}

impl From<FilterCategory> for crate::types::FilterCategory {
    fn from(filter_category: FilterCategory) -> Self {
        crate::types::FilterCategory {
            id: filter_category.id,
            display_name: filter_category.display_name,
            filters: filter_category
                .filters
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}

impl From<crate::types::SearchFilter> for SearchFilter {
    fn from(search_filter: crate::types::SearchFilter) -> Self {
        SearchFilter {
            id: search_filter.id,
            values: search_filter.values,
        }
    }
}

impl AsyncTryFromWithStore<SeriesPage> for crate::types::SeriesPage {
    async fn try_from_with_store(
        page: SeriesPage,
        store: &mut wasmtime::Store<crate::host::WasmState>,
    ) -> anyhow::Result<Self> {
        let mut items = Vec::new();
        for series in page.series {
            items.push(series.try_into_with_store(store).await?);
        }
        Ok(crate::types::SeriesPage {
            items,
            has_next_page: page.has_next_page,
        })
    }
}

impl AsyncTryFromWithStore<Series> for crate::types::Series {
    async fn try_from_with_store(
        series: Series,
        store: &mut wasmtime::Store<crate::host::WasmState>,
    ) -> anyhow::Result<Self> {
        Ok(crate::types::Series {
            id: series.id,
            title: series.title,
            poster_resource: match series.poster_resource {
                Some(r) => Some(r.try_into_with_store(store).await?),
                None => None,
            },
            synopsis: series.synopsis,
            r#type: series.type_,
        })
    }
}

impl AsyncTryFromWithStore<EpisodesPage> for crate::types::EpisodesPage {
    async fn try_from_with_store(
        page: EpisodesPage,
        store: &mut wasmtime::Store<crate::host::WasmState>,
    ) -> anyhow::Result<Self> {
        let mut items = Vec::new();
        for episode in page.episodes {
            items.push(episode.try_into_with_store(store).await?);
        }
        Ok(crate::types::EpisodesPage {
            items,
            has_next_page: page.has_next_page,
        })
    }
}

impl AsyncTryFromWithStore<Episode> for crate::types::Episode {
    async fn try_from_with_store(
        episode: Episode,
        store: &mut wasmtime::Store<crate::host::WasmState>,
    ) -> anyhow::Result<Self> {
        Ok(crate::types::Episode {
            id: episode.id,
            number: episode.number,
            title: episode.title,
            thumbnail_resource: match episode.thumbnail_resource {
                Some(r) => Some(r.try_into_with_store(store).await?),
                None => None,
            },
            description: episode.description,
        })
    }
}

impl AsyncTryFromWithStore<Video> for crate::types::Video {
    async fn try_from_with_store(
        video: Video,
        store: &mut wasmtime::Store<crate::host::WasmState>,
    ) -> anyhow::Result<Self> {
        Ok(crate::types::Video {
            http_resource: video.http_resource.try_into_with_store(store).await?,
            server: video.server,
            resolution: video.resolution,
        })
    }
}

impl AsyncTryFromWithStore<Resource<HostOutgoingRequest>> for crate::types::HttpResource {
    async fn try_from_with_store(
        resource: Resource<HostOutgoingRequest>,
        store: &mut wasmtime::Store<crate::host::WasmState>,
    ) -> anyhow::Result<Self> {
        let table = store.data_mut().table();
        let outgoing_request = table.delete(resource)?;

        let method = match outgoing_request.method {
            Method::Get => crate::types::Method::Get,
            Method::Head => crate::types::Method::Head,
            Method::Post => crate::types::Method::Post,
            Method::Put => crate::types::Method::Put,
            Method::Delete => crate::types::Method::Delete,
            Method::Connect => crate::types::Method::Connect,
            Method::Options => crate::types::Method::Options,
            Method::Trace => crate::types::Method::Trace,
            Method::Patch => crate::types::Method::Patch,
            Method::Other(other) => crate::types::Method::Other(other),
        };

        let scheme = match outgoing_request.scheme.unwrap_or(Scheme::Https) {
            Scheme::Http => "http".to_owned(),
            Scheme::Https => "https".to_owned(),
            Scheme::Other(other) => other,
        };
        let authority = outgoing_request.authority.unwrap_or_else(String::new);
        let mut uri = format!("{scheme}://{authority}/");
        if let Some(path) = &outgoing_request.path_with_query {
            uri.push_str(path);
        }
        let url = url::Url::parse(&uri)?;

        let headers = outgoing_request
            .headers
            .iter()
            .map(|(name, value)| (name.to_string(), value.to_str().unwrap_or("").to_owned()))
            .collect();

        let body = match outgoing_request.body {
            Some(box_body) => {
                let collected = box_body.collect().await?;
                Some(collected.to_bytes().to_vec())
            }
            None => None,
        };

        Ok(crate::types::HttpResource {
            method,
            url,
            headers,
            body,
        })
    }
}

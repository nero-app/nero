use anyhow::{Ok, Result};
use exports::nero::extension::extractor::{
    Episode, EpisodesPage, Filter, FilterCategory, SearchFilter, Series, SeriesPage, SeriesVideo,
    Url,
};
use wasmtime::{Store, component::bindgen};
use wasmtime_wasi::WasiView;
use wasmtime_wasi_http::{bindings::http::types::Scheme, types::HostFields};

use crate::{host::WasmState, semver::SemanticVersion};

pub const MIN_VER: SemanticVersion = SemanticVersion::new(0, 0, 1);

bindgen!({
    path: "./wit/v0.0.1",
    async: true,
    with: {
        "wasi:http": wasmtime_wasi_http::bindings::http,
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

impl From<SeriesPage> for crate::types::SeriesPage {
    fn from(page: SeriesPage) -> Self {
        crate::types::SeriesPage {
            items: page.series.into_iter().map(Into::into).collect(),
            has_next_page: page.has_next_page,
        }
    }
}

impl From<Series> for crate::types::Series {
    fn from(series: Series) -> Self {
        crate::types::Series {
            id: series.id,
            title: series.title,
            poster_url: series.poster_url.map(Into::into),
            synopsis: series.synopsis,
            r#type: series.type_,
        }
    }
}

impl From<EpisodesPage> for crate::types::EpisodesPage {
    fn from(page: EpisodesPage) -> Self {
        crate::types::EpisodesPage {
            items: page.episodes.into_iter().map(Into::into).collect(),
            has_next_page: page.has_next_page,
        }
    }
}

impl From<Episode> for crate::types::Episode {
    fn from(episode: Episode) -> Self {
        crate::types::Episode {
            id: episode.id,
            number: episode.number,
            title: episode.title,
            thumbnail_url: episode.thumbnail_url.map(Into::into),
            description: episode.description,
        }
    }
}

impl SeriesVideo {
    /// Convert a [`SeriesVideo`] into a [`crate::types::SeriesVideo`].
    pub fn into_crate_video(
        self,
        store: &mut Store<WasmState>,
    ) -> Result<crate::types::SeriesVideo> {
        let resource = self.video_headers;
        let table = store.data_mut().table();

        let fields = table.get(&resource)?;
        let headers = match fields {
            HostFields::Owned { fields } => fields
                .iter()
                .map(|(k, v)| {
                    let s = v.to_str()?;
                    Ok((k.to_string(), s.to_string()))
                })
                .collect::<Result<_>>()?,
            _ => unreachable!(),
        };

        Ok(crate::types::SeriesVideo {
            video_url: self.video_url.into(),
            video_headers: headers,
            server: self.server,
            resolution: self.resolution,
        })
    }
}

impl From<Url> for url::Url {
    fn from(url: Url) -> Self {
        let scheme = match url.scheme {
            Scheme::Http => "http".to_owned(),
            Scheme::Https => "https".to_owned(),
            Scheme::Other(v) => v,
        };

        let url = format!(
            "{}://{}{}",
            scheme,
            url.authority,
            url.path_with_query.unwrap_or_default()
        );
        url::Url::parse(url.as_str()).unwrap()
    }
}

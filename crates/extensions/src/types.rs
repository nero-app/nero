use wasmtime_wasi_http::types::HostOutgoingRequest;

pub struct Page<T> {
    pub items: Vec<T>,
    pub has_next_page: bool,
}

pub type SeriesPage = Page<Series>;
pub type EpisodesPage = Page<Episode>;

pub struct Series {
    pub id: String,
    pub title: String,
    pub poster_resource: Option<HostOutgoingRequest>,
    pub synopsis: Option<String>,
    pub r#type: Option<String>,
}

pub struct Episode {
    pub id: String,
    pub number: u16,
    pub title: Option<String>,
    pub thumbnail_resource: Option<HostOutgoingRequest>,
    pub description: Option<String>,
}

type Resolution = (u16, u16);

pub struct Video {
    pub http_resource: HostOutgoingRequest,
    pub server: String,
    pub resolution: Resolution,
}

pub struct Filter {
    pub id: String,
    pub display_name: String,
}

pub struct FilterCategory {
    pub id: String,
    pub display_name: String,
    pub filters: Vec<Filter>,
}

pub struct SearchFilter {
    pub id: String,
    pub values: Vec<String>,
}

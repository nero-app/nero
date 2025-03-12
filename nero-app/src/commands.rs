#![allow(unused_variables)]

use nero_extensions::{
    types::{
        Episode, EpisodesPage, Filter, FilterCategory, SearchFilter, Series, SeriesPage,
        SeriesVideo,
    },
    url::Url,
};
use tauri::Result;

#[tauri::command]
pub async fn get_filters() -> Result<Vec<FilterCategory>> {
    let sample_filter_category = FilterCategory {
        id: "genre".to_owned(),
        display_name: "Genre".to_owned(),
        filters: vec![
            Filter {
                id: "slice_of_life".to_owned(),
                display_name: "Slice of life".to_owned(),
            },
            Filter {
                id: "romance".to_owned(),
                display_name: "Romance".to_owned(),
            },
            Filter {
                id: "comedy".to_owned(),
                display_name: "Comedy".to_owned(),
            },
        ],
    };

    Ok((1..=10)
        .map(|_| sample_filter_category.clone())
        .collect::<Vec<_>>())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn search(
    query: &str,
    page: Option<u16>,
    filters: Vec<SearchFilter>,
) -> Result<SeriesPage> {
    let sample_series = Series {
        id: "im-getting-married-to-a-girl-i-hate-in-my-class".to_owned(),
        title: "I'm Getting Married to a Girl I Hate in My Class".to_owned(),
        poster_url: Some(Url::parse("https://m.media-amazon.com/images/M/MV5BYTc1MWFhYzEtYzE1YS00NWFjLTg3OTYtN2JhNzRiNTRkZTNlXkEyXkFqcGc@._V1_FMjpg_UX1000_.jpg").unwrap()),
        synopsis: Some(r#"
            High school student Saito Hojo is set to inherit his grandfather’s major corporation. First,
            he must marry Akane Sakuramori, the girl he despises the most, and who hates him just as
            much. The two are determined to keep their unexpected marriage a secret from their
            classmates. But as they begin their newlywed life, the distance between them starts to
            close.
        "#.to_owned()),
        r#type: Some("Series".to_owned()),
    };

    Ok(SeriesPage {
        items: (1..=12).map(|_| sample_series.clone()).collect::<Vec<_>>(),
        has_next_page: true,
    })
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_series_info(series_id: &str) -> Result<Series> {
    let sample_series = Series {
        id: "im-getting-married-to-a-girl-i-hate-in-my-class".to_owned(),
        title: "I'm Getting Married to a Girl I Hate in My Class".to_owned(),
        poster_url: Some(Url::parse("https://m.media-amazon.com/images/M/MV5BYTc1MWFhYzEtYzE1YS00NWFjLTg3OTYtN2JhNzRiNTRkZTNlXkEyXkFqcGc@._V1_FMjpg_UX1000_.jpg").unwrap()),
        synopsis: Some(r#"
            High school student Saito Hojo is set to inherit his grandfather’s major corporation. First,
            he must marry Akane Sakuramori, the girl he despises the most, and who hates him just as
            much. The two are determined to keep their unexpected marriage a secret from their
            classmates. But as they begin their newlywed life, the distance between them starts to
            close.
        "#.to_owned()),
        r#type: Some("Series".to_owned()),
    };

    Ok(sample_series)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_series_episodes(series_id: &str, page: Option<u16>) -> Result<EpisodesPage> {
    let sample_episode = Episode {
        id: "1".to_owned(),
        number: 1,
        title: Some("I'm Getting Married to a Girl I Hate in My Class".to_owned()),
        thumbnail_url: Some(Url::parse("https://m.media-amazon.com/images/M/MV5BNDFiNTI0MDAtMTRhMi00ZDBhLWExNjEtYzNhNTFmN2RjOTQ1XkEyXkFqcGc@._V1_.jpg").unwrap()),
        description: Some(r#"
            Saito is getting married to his classmate. But it's not just any classmate—it's his least favorite girl
            in school and archnemesis, Akane Sakuramori! And so begins a romcom of two stubborn love
            birds forced into marriage against their will.
        "#.to_owned()),
    };

    Ok(EpisodesPage {
        items: (1..=12).map(|_| sample_episode.clone()).collect::<Vec<_>>(),
        has_next_page: true,
    })
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_series_videos(series_id: &str, episode_id: &str) -> Result<Vec<SeriesVideo>> {
    let sample_series_video = SeriesVideo {
        video_url: Url::parse(
            "http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4",
        )
        .unwrap(),
        video_headers: vec![],
        server: "google".to_owned(),
        resolution: (0, 0),
    };

    Ok(vec![sample_series_video])
}

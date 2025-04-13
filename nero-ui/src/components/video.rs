use nero_extensions::url::Url;
use rustwind::{layout::AspectRatio, tw};
use sycamore::{
    prelude::HtmlVideoAttributes,
    web::{tags::video, HtmlGlobalAttributes, View},
};

use crate::utils::ViewBuilder;

pub struct VideoPlayer {
    src: Url,
    poster_url: Option<Url>,
}

impl VideoPlayer {
    pub fn new(src: Url) -> Self {
        Self {
            src,
            poster_url: None,
        }
    }

    pub fn poster_url(mut self, poster_url: Url) -> Self {
        self.poster_url = Some(poster_url);
        self
    }
}

impl From<VideoPlayer> for View {
    fn from(player: VideoPlayer) -> Self {
        video()
            .class(tw!(AspectRatio::Video))
            .controls(true)
            .src(player.src.to_string())
            .when_some(player.poster_url, |this, poster_url| {
                this.poster(poster_url.to_string())
            })
            .into()
    }
}

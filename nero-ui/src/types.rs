#![allow(dead_code)]

pub struct Series {
    pub id: String,
    pub title: String,
    pub poster_url: Option<String>,
    pub synopsis: Option<String>,
    pub r#type: Option<String>,
}

impl Default for Series {
    fn default() -> Self {
        Series {
            id: "spy-x-family".to_owned(),
            title: "SPY x FAMILY".to_owned(),
            poster_url: Some("https://m.media-amazon.com/images/M/MV5BZjNjN2UyYTYtMjY2Zi00ZWFlLWFmMDItZTNkMzQ3MDc1Yjg5XkEyXkFqcGc@._V1_.jpg".to_owned()),
            synopsis: Some(r#"
                World peace is at stake and secret agent Twilight must undergo his most difficult mission yet—
                pretend to be a family man. Posing as a loving husband and father, he’ll infiltrate an elite school to 
                get close to a high-profile politician. He has the perfect cover, except his wife’s a deadly assassin 
                and neither knows each other’s identity. But someone does, his adopted daughter who’s a 
                telepath!
            "#.to_owned()),
            r#type: Some("Series".to_owned()),
        }
    }
}

pub struct Episode {
    pub id: String,
    pub number: u16,
    pub title: Option<String>,
    pub thumbnail_url: Option<String>,
    pub description: Option<String>,
}

impl Default for Episode {
    fn default() -> Self {
        Episode {
            id: "1".to_owned(),
            number: 1,
            title: Some("OPERATION STRIX".to_owned()),
            thumbnail_url: Some("https://m.media-amazon.com/images/M/MV5BZDM0ZmU3MDAtZThmNy00MmY1LTliNjQtM2M5MWU3MGJiOGU5XkEyXkFqcGc@._V1_.jpg".to_owned()),
            description: Some(r#"
                Twilight is an agent that works for WISE, Westalis's intelligence agency, and he is tasked with 
                investigating Desmond, who is in Ostania and planning to start a war. Twilight disguises himself 
                as the psychiatrist Loid Forger and adopts a girl named Anya so that he can enroll her into the 
                prestigious Eden College to get closer to his target. Unbeknownst to him, Anya is actually a telepath 
                who can read people's minds. One day, members of a mafia group that is after Twilight kidnaps Anya. 
                Loid realizes that he needs to reconsider his priorities and...
            "#.to_owned()),
        }
    }
}

pub struct Video {
    pub url: String,
    // TODO: headers,
    pub server: String,
    pub resolution: (u16, u16),
}

impl Video {
    pub const VIDEO_TITLE: &str = "Big Buck Bunny";
    pub const VIDEO_SYNOPSIS: Option<&str> = Some(
        r#"
        Big Buck Bunny tells the story of a giant rabbit with a heart bigger than himself. 
        When one sunny day three rodents rudely harass him, something snaps... and the rabbit ain't no bunny anymore! 
        In the typical cartoon tradition he prepares the nasty rodents a comical revenge."#,
    );
}

impl Default for Video {
    fn default() -> Self {
        Video {
            url:
                "http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4"
                    .to_owned(),
            server: "google".to_owned(),
            resolution: (0, 0),
        }
    }
}

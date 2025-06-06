use serde::{Deserialize, Serialize};

fn main() {
    let j = std::fs::read_to_string("benches/twitter.json").unwrap();
    let s: Twitter = serde_json::from_str(&j).unwrap();
    let t = serde_json::to_string(&s).unwrap();
    println!("{:?}", t);
}

#[derive(Serialize, Deserialize)]
struct Twitter {
    statuses: Vec<Status>,
    search_metadata: SearchMetadata,
}

#[derive(Serialize, Deserialize)]
struct Status {
    metadata: Metadata,
    created_at: String,
    id: u64,
    id_str: String,
    text: String,
    source: String,
    truncated: bool,
    in_reply_to_status_id: Option<u64>,
    in_reply_to_status_id_str: Option<String>,
    in_reply_to_user_id: Option<u32>,
    in_reply_to_user_id_str: Option<String>,
    in_reply_to_screen_name: Option<String>,
    user: User,
    geo: (),
    coordinates: (),
    place: (),
    contributors: (),
    retweeted_status: Option<Box<Status>>,
    retweet_count: u32,
    favorite_count: u32,
    entities: StatusEntities,
    favorited: bool,
    retweeted: bool,
    possibly_sensitive: Option<bool>,
    lang: String,
}

#[derive(Serialize, Deserialize)]
struct Metadata {
    result_type: String,
    iso_language_code: String,
}

#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    id_str: String,
    name: String,
    screen_name: String,
    location: String,
    description: String,
    url: Option<String>,
    entities: UserEntities,
    protected: bool,
    followers_count: u32,
    friends_count: u32,
    listed_count: u32,
    created_at: String,
    favourites_count: u32,
    utc_offset: Option<i32>,
    time_zone: Option<String>,
    geo_enabled: bool,
    verified: bool,
    statuses_count: u32,
    lang: String,
    contributors_enabled: bool,
    is_translator: bool,
    is_translation_enabled: bool,
    profile_background_color: String,
    profile_background_image_url: String,
    profile_background_image_url_https: String,
    profile_background_tile: bool,
    profile_image_url: String,
    profile_image_url_https: String,
    profile_banner_url: Option<String>,
    profile_link_color: String,
    profile_sidebar_border_color: String,
    profile_sidebar_fill_color: String,
    profile_text_color: String,
    profile_use_background_image: bool,
    default_profile: bool,
    default_profile_image: bool,
    following: bool,
    follow_request_sent: bool,
    notifications: bool,
}

#[derive(Serialize, Deserialize)]
struct UserEntities {
    url: Option<UserUrl>,
    description: UserEntitiesDescription,
}

#[derive(Serialize, Deserialize)]
struct UserUrl {
    urls: Vec<Url>,
}

#[derive(Serialize, Deserialize)]
struct Url {
    url: String,
    expanded_url: String,
    display_url: String,
    indices: Indices,
}

#[derive(Serialize, Deserialize)]
struct UserEntitiesDescription {
    urls: Vec<Url>,
}

#[derive(Serialize, Deserialize)]
struct StatusEntities {
    hashtags: Vec<Hashtag>,
    symbols: Vec<()>,
    urls: Vec<Url>,
    user_mentions: Vec<UserMention>,
    media: Option<Vec<Media>>,
}

#[derive(Serialize, Deserialize)]
struct Hashtag {
    text: String,
    indices: Indices,
}

#[derive(Serialize, Deserialize)]
struct UserMention {
    screen_name: String,
    name: String,
    id: u32,
    id_str: String,
    indices: Indices,
}

#[derive(Serialize, Deserialize)]
struct Media {
    id: u64,
    id_str: String,
    indices: Indices,
    media_url: String,
    media_url_https: String,
    url: String,
    display_url: String,
    expanded_url: String,
    #[serde(rename = "type")]
    media_type: String,
    sizes: Sizes,
    source_status_id: Option<u64>,
    source_status_id_str: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Sizes {
    medium: Size,
    small: Size,
    thumb: Size,
    large: Size,
}

#[derive(Serialize, Deserialize)]
struct Size {
    w: u16,
    h: u16,
    resize: String,
}

type Indices = (u8, u8);

#[derive(Serialize, Deserialize)]
struct SearchMetadata {
    completed_in: f32,
    max_id: u64,
    max_id_str: String,
    next_results: String,
    query: String,
    refresh_url: String,
    count: u8,
    since_id: u64,
    since_id_str: String,
}

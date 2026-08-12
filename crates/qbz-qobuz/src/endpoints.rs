//! API endpoint definitions

pub const BASE_URL: &str = "https://www.qobuz.com/api.json/0.2";

/// Endpoint paths
pub mod paths {
    // User
    pub const USER_LOGIN: &str = "/user/login";

    // OAuth
    pub const OAUTH_CALLBACK: &str = "/oauth/callback";

    // Track
    pub const TRACK_GET: &str = "/track/get";
    pub const TRACK_GET_LIST: &str = "/track/getList";
    pub const TRACK_SEARCH: &str = "/track/search";
    pub const TRACK_GET_FILE_URL: &str = "/track/getFileUrl";

    // Album
    pub const ALBUM_GET: &str = "/album/get";
    pub const ALBUM_SEARCH: &str = "/album/search";

    // Playlist
    pub const PLAYLIST_GET: &str = "/playlist/get";
    pub const PLAYLIST_SEARCH: &str = "/playlist/search";
    pub const PLAYLIST_GET_USER_PLAYLISTS: &str = "/playlist/getUserPlaylists";

    // Favorites
    pub const FAVORITE_GET_USER_FAVORITES: &str = "/favorite/getUserFavorites";

    // Catalog (combined search)
    pub const CATALOG_SEARCH: &str = "/catalog/search";
}

/// Build full URL for an endpoint
pub fn build_url(endpoint: &str) -> String {
    format!("{}{}", BASE_URL, endpoint)
}

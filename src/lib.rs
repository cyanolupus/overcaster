use worker::*;
mod utils;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().unwrap().coordinates().unwrap_or_default(),
        req.cf()
            .unwrap()
            .region()
            .unwrap_or("unknown region".into()),
    );
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);
    utils::set_panic_hook();

    let router = Router::new();
    router
        .get("/", |_, _| Response::ok("Welcome to Overcaster!"))
        // System
        .get_async("/rest/ping", not_implemented) // Check server connectivity
        .get_async("/rest/getLicense", not_implemented) // Get license info
        // Music library browsing
        .get_async("/rest/getMusicFolders", not_implemented) // Only one folder supported
        .get_async("/rest/getIndexes", not_implemented) // Browsing simulated from tags
        .get_async("/rest/getMusicDirectory", not_implemented) // List songs/albums in a directory
        .get_async("/rest/getSong", not_implemented) // Get song info
        .get_async("/rest/getArtists", not_implemented) // Get artist list
        .get_async("/rest/getArtist", not_implemented) // Get specific artist info
        .get_async("/rest/getAlbum", not_implemented) // Get album info
        .get_async("/rest/getGenres", not_implemented) // Get list of genres
        .get_async("/rest/getArtistInfo", not_implemented) // Needs Last.fm/Spotify integration
        .get_async("/rest/getArtistInfo2", not_implemented) // Needs Last.fm/Spotify integration
        .get_async("/rest/getAlbumInfo", not_implemented) // Needs Last.fm/Spotify integration
        .get_async("/rest/getAlbumInfo2", not_implemented) // Needs Last.fm/Spotify integration
        .get_async("/rest/getTopSongs", not_implemented) // Needs Last.fm integration
        .get_async("/rest/getSimilarSongs", not_implemented) // Needs Last.fm integration
        .get_async("/rest/getSimilarSongs2", not_implemented) // Needs Last.fm integration
        // Album and song lists
        .get_async("/rest/getAlbumList", not_implemented) // Get album list
        .get_async("/rest/getAlbumList2", not_implemented) // Get album list (v2)
        .get_async("/rest/getStarred", not_implemented) // Get starred (favorite) items
        .get_async("/rest/getStarred2", not_implemented) // Get starred items (v2)
        .get_async("/rest/getNowPlaying", not_implemented) // Get now playing list
        .get_async("/rest/getRandomSongs", not_implemented) // Get random songs
        .get_async("/rest/getSongsByGenre", not_implemented) // Get songs by genre
        // Search
        .get_async("/rest/search2", not_implemented) // Lucene queries not supported
        .get_async("/rest/search3", not_implemented) // Lucene queries not supported
        // Playlist management
        .get_async("/rest/getPlaylists", not_implemented) // username parameter not supported
        .get_async("/rest/getPlaylist", not_implemented) // Get playlist details
        .post_async("/rest/createPlaylist", not_implemented) // Create playlist
        .post_async("/rest/updatePlaylist", not_implemented) // Update playlist
        .post_async("/rest/deletePlaylist", not_implemented) // Delete playlist
        // Media streaming and download
        .get_async("/rest/stream", not_implemented) // Stream song
        .get_async("/rest/download", not_implemented) // Download song/album
        .get_async("/rest/getCoverArt", not_implemented) // Get cover art
        .get_async("/rest/getLyrics", not_implemented) // Only embedded lyrics supported
        .get_async("/rest/getAvatar", not_implemented) // Gravatar/placeholder only
        // Media annotation/history
        .post_async("/rest/star", not_implemented) // Star (favorite) items
        .post_async("/rest/unstar", not_implemented) // Unstar items
        .post_async("/rest/setRating", not_implemented) // Set rating
        .post_async("/rest/scrobble", not_implemented) // Submit play history
        // Bookmarks
        .get_async("/rest/getBookmarks", not_implemented) // Get bookmarks
        .post_async("/rest/createBookmark", not_implemented) // Create bookmark
        .post_async("/rest/deleteBookmark", not_implemented) // Delete bookmark
        // Play queue
        .get_async("/rest/getPlayQueue", not_implemented) // Get play queue (current is string id)
        .post_async("/rest/savePlayQueue", not_implemented) // Save play queue
        // Sharing (only if EnableSharing=true)
        .get_async("/rest/getShares", not_implemented) // Get shares
        .post_async("/rest/createShare", not_implemented) // Create share
        .post_async("/rest/updateShare", not_implemented) // Update share
        .post_async("/rest/deleteShare", not_implemented) // Delete share
        // Internet radio
        .get_async("/rest/getInternetRadioStations", not_implemented) // Get radio stations
        .post_async("/rest/createInternetRadioStation", not_implemented) // Create radio station
        .post_async("/rest/updateInternetRadioStation", not_implemented) // Update radio station
        .post_async("/rest/deleteInternetRadioStation", not_implemented) // Delete radio station
        // User management
        .get_async("/rest/getUser", not_implemented) // Only for authenticated user
        .get_async("/rest/getUsers", not_implemented) // Only for authenticated user
        // Library scan
        .get_async("/rest/getScanStatus", not_implemented) // lastScan and folderCount fields added
        .post_async("/rest/startScan", not_implemented) // fullScan parameter added
        .run(req, env)
        .await
}

async fn not_implemented<D>(req: Request, ctx: RouteContext<D>) -> Result<Response> {
    Response::error(format!("{} not implemented", req.path()), 501)
}

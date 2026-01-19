use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    ReqwestMiddleware(reqwest_middleware::Error),
    SerdePathToError(serde_path_to_error::Error<serde_json::Error>),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl<T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::ReqwestMiddleware(e) => ("reqwest-middleware", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::SerdePathToError(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl<T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::ReqwestMiddleware(e) => e,
            Error::Serde(e) => e,
            Error::SerdePathToError(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl<T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl<T> From<reqwest_middleware::Error> for Error<T> {
    fn from(e: reqwest_middleware::Error) -> Self {
        Error::ReqwestMiddleware(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl<T> From<serde_path_to_error::Error<serde_json::Error>> for Error<T> {
    fn from(e: serde_path_to_error::Error<serde_json::Error>) -> Self {
        Error::SerdePathToError(e)
    }
}

impl<T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub fn parse_deep_object(prefix: &str, value: &serde_json::Value) -> Vec<(String, String)> {
    if let serde_json::Value::Object(object) = value {
        let mut params = vec![];

        for (key, value) in object {
            match value {
                serde_json::Value::Object(_) => params.append(&mut parse_deep_object(
                    &format!("{}[{}]", prefix, key),
                    value,
                )),
                serde_json::Value::Array(array) => {
                    for (i, value) in array.iter().enumerate() {
                        params.append(&mut parse_deep_object(
                            &format!("{}[{}][{}]", prefix, key, i),
                            value,
                        ));
                    }
                }
                serde_json::Value::String(s) => {
                    params.push((format!("{}[{}]", prefix, key), s.clone()))
                }
                _ => params.push((format!("{}[{}]", prefix, key), value.to_string())),
            }
        }

        return params;
    }

    unimplemented!("Only objects are supported with style=deepObject")
}

/// Internal use only
/// A content type supported by this client.
#[allow(dead_code)]
enum ContentType {
    Json,
    Text,
    Unsupported(String),
}

impl From<&str> for ContentType {
    fn from(content_type: &str) -> Self {
        if content_type.starts_with("application") && content_type.contains("json") {
            return Self::Json;
        } else if content_type.starts_with("text/plain") {
            return Self::Text;
        } else {
            return Self::Unsupported(content_type.to_string());
        }
    }
}

pub mod albums_api;
pub mod appreciations_api;
pub mod artist_biographies_api;
pub mod artist_claims_api;
pub mod artist_roles_api;
pub mod artists_api;
pub mod artworks_api;
pub mod dynamic_pages_api;
pub mod genres_api;
pub mod lyrics_api;
pub mod manual_artist_claims_api;
pub mod play_queues_api;
pub mod playlists_api;
pub mod providers_api;
pub mod reactions_api;
pub mod saved_shares_api;
pub mod search_results_api;
pub mod search_suggestions_api;
pub mod shares_api;
pub mod stripe_connections_api;
pub mod track_files_api;
pub mod track_manifests_api;
pub mod track_source_files_api;
pub mod track_statistics_api;
pub mod tracks_api;
pub mod user_collection_folders_api;
pub mod user_collections_api;
pub mod user_entitlements_api;
pub mod user_recommendations_api;
pub mod user_reports_api;
pub mod users_api;
pub mod videos_api;

pub mod configuration;

use std::sync::Arc;

pub trait Api {
    fn albums_api(&self) -> &dyn albums_api::AlbumsApi;
    fn appreciations_api(&self) -> &dyn appreciations_api::AppreciationsApi;
    fn artist_biographies_api(&self) -> &dyn artist_biographies_api::ArtistBiographiesApi;
    fn artist_claims_api(&self) -> &dyn artist_claims_api::ArtistClaimsApi;
    fn artist_roles_api(&self) -> &dyn artist_roles_api::ArtistRolesApi;
    fn artists_api(&self) -> &dyn artists_api::ArtistsApi;
    fn artworks_api(&self) -> &dyn artworks_api::ArtworksApi;
    fn dynamic_pages_api(&self) -> &dyn dynamic_pages_api::DynamicPagesApi;
    fn genres_api(&self) -> &dyn genres_api::GenresApi;
    fn lyrics_api(&self) -> &dyn lyrics_api::LyricsApi;
    fn manual_artist_claims_api(&self) -> &dyn manual_artist_claims_api::ManualArtistClaimsApi;
    fn play_queues_api(&self) -> &dyn play_queues_api::PlayQueuesApi;
    fn playlists_api(&self) -> &dyn playlists_api::PlaylistsApi;
    fn providers_api(&self) -> &dyn providers_api::ProvidersApi;
    fn reactions_api(&self) -> &dyn reactions_api::ReactionsApi;
    fn saved_shares_api(&self) -> &dyn saved_shares_api::SavedSharesApi;
    fn search_results_api(&self) -> &dyn search_results_api::SearchResultsApi;
    fn search_suggestions_api(&self) -> &dyn search_suggestions_api::SearchSuggestionsApi;
    fn shares_api(&self) -> &dyn shares_api::SharesApi;
    fn stripe_connections_api(&self) -> &dyn stripe_connections_api::StripeConnectionsApi;
    fn track_files_api(&self) -> &dyn track_files_api::TrackFilesApi;
    fn track_manifests_api(&self) -> &dyn track_manifests_api::TrackManifestsApi;
    fn track_source_files_api(&self) -> &dyn track_source_files_api::TrackSourceFilesApi;
    fn track_statistics_api(&self) -> &dyn track_statistics_api::TrackStatisticsApi;
    fn tracks_api(&self) -> &dyn tracks_api::TracksApi;
    fn user_collection_folders_api(
        &self,
    ) -> &dyn user_collection_folders_api::UserCollectionFoldersApi;
    fn user_collections_api(&self) -> &dyn user_collections_api::UserCollectionsApi;
    fn user_entitlements_api(&self) -> &dyn user_entitlements_api::UserEntitlementsApi;
    fn user_recommendations_api(&self) -> &dyn user_recommendations_api::UserRecommendationsApi;
    fn user_reports_api(&self) -> &dyn user_reports_api::UserReportsApi;
    fn users_api(&self) -> &dyn users_api::UsersApi;
    fn videos_api(&self) -> &dyn videos_api::VideosApi;
}

pub struct ApiClient {
    albums_api: Box<dyn albums_api::AlbumsApi>,
    appreciations_api: Box<dyn appreciations_api::AppreciationsApi>,
    artist_biographies_api: Box<dyn artist_biographies_api::ArtistBiographiesApi>,
    artist_claims_api: Box<dyn artist_claims_api::ArtistClaimsApi>,
    artist_roles_api: Box<dyn artist_roles_api::ArtistRolesApi>,
    artists_api: Box<dyn artists_api::ArtistsApi>,
    artworks_api: Box<dyn artworks_api::ArtworksApi>,
    dynamic_pages_api: Box<dyn dynamic_pages_api::DynamicPagesApi>,
    genres_api: Box<dyn genres_api::GenresApi>,
    lyrics_api: Box<dyn lyrics_api::LyricsApi>,
    manual_artist_claims_api: Box<dyn manual_artist_claims_api::ManualArtistClaimsApi>,
    play_queues_api: Box<dyn play_queues_api::PlayQueuesApi>,
    playlists_api: Box<dyn playlists_api::PlaylistsApi>,
    providers_api: Box<dyn providers_api::ProvidersApi>,
    reactions_api: Box<dyn reactions_api::ReactionsApi>,
    saved_shares_api: Box<dyn saved_shares_api::SavedSharesApi>,
    search_results_api: Box<dyn search_results_api::SearchResultsApi>,
    search_suggestions_api: Box<dyn search_suggestions_api::SearchSuggestionsApi>,
    shares_api: Box<dyn shares_api::SharesApi>,
    stripe_connections_api: Box<dyn stripe_connections_api::StripeConnectionsApi>,
    track_files_api: Box<dyn track_files_api::TrackFilesApi>,
    track_manifests_api: Box<dyn track_manifests_api::TrackManifestsApi>,
    track_source_files_api: Box<dyn track_source_files_api::TrackSourceFilesApi>,
    track_statistics_api: Box<dyn track_statistics_api::TrackStatisticsApi>,
    tracks_api: Box<dyn tracks_api::TracksApi>,
    user_collection_folders_api: Box<dyn user_collection_folders_api::UserCollectionFoldersApi>,
    user_collections_api: Box<dyn user_collections_api::UserCollectionsApi>,
    user_entitlements_api: Box<dyn user_entitlements_api::UserEntitlementsApi>,
    user_recommendations_api: Box<dyn user_recommendations_api::UserRecommendationsApi>,
    user_reports_api: Box<dyn user_reports_api::UserReportsApi>,
    users_api: Box<dyn users_api::UsersApi>,
    videos_api: Box<dyn videos_api::VideosApi>,
}

impl ApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self {
            albums_api: Box::new(albums_api::AlbumsApiClient::new(configuration.clone())),
            appreciations_api: Box::new(appreciations_api::AppreciationsApiClient::new(
                configuration.clone(),
            )),
            artist_biographies_api: Box::new(
                artist_biographies_api::ArtistBiographiesApiClient::new(configuration.clone()),
            ),
            artist_claims_api: Box::new(artist_claims_api::ArtistClaimsApiClient::new(
                configuration.clone(),
            )),
            artist_roles_api: Box::new(artist_roles_api::ArtistRolesApiClient::new(
                configuration.clone(),
            )),
            artists_api: Box::new(artists_api::ArtistsApiClient::new(configuration.clone())),
            artworks_api: Box::new(artworks_api::ArtworksApiClient::new(configuration.clone())),
            dynamic_pages_api: Box::new(dynamic_pages_api::DynamicPagesApiClient::new(
                configuration.clone(),
            )),
            genres_api: Box::new(genres_api::GenresApiClient::new(configuration.clone())),
            lyrics_api: Box::new(lyrics_api::LyricsApiClient::new(configuration.clone())),
            manual_artist_claims_api: Box::new(
                manual_artist_claims_api::ManualArtistClaimsApiClient::new(configuration.clone()),
            ),
            play_queues_api: Box::new(play_queues_api::PlayQueuesApiClient::new(
                configuration.clone(),
            )),
            playlists_api: Box::new(playlists_api::PlaylistsApiClient::new(
                configuration.clone(),
            )),
            providers_api: Box::new(providers_api::ProvidersApiClient::new(
                configuration.clone(),
            )),
            reactions_api: Box::new(reactions_api::ReactionsApiClient::new(
                configuration.clone(),
            )),
            saved_shares_api: Box::new(saved_shares_api::SavedSharesApiClient::new(
                configuration.clone(),
            )),
            search_results_api: Box::new(search_results_api::SearchResultsApiClient::new(
                configuration.clone(),
            )),
            search_suggestions_api: Box::new(
                search_suggestions_api::SearchSuggestionsApiClient::new(configuration.clone()),
            ),
            shares_api: Box::new(shares_api::SharesApiClient::new(configuration.clone())),
            stripe_connections_api: Box::new(
                stripe_connections_api::StripeConnectionsApiClient::new(configuration.clone()),
            ),
            track_files_api: Box::new(track_files_api::TrackFilesApiClient::new(
                configuration.clone(),
            )),
            track_manifests_api: Box::new(track_manifests_api::TrackManifestsApiClient::new(
                configuration.clone(),
            )),
            track_source_files_api: Box::new(
                track_source_files_api::TrackSourceFilesApiClient::new(configuration.clone()),
            ),
            track_statistics_api: Box::new(track_statistics_api::TrackStatisticsApiClient::new(
                configuration.clone(),
            )),
            tracks_api: Box::new(tracks_api::TracksApiClient::new(configuration.clone())),
            user_collection_folders_api: Box::new(
                user_collection_folders_api::UserCollectionFoldersApiClient::new(
                    configuration.clone(),
                ),
            ),
            user_collections_api: Box::new(user_collections_api::UserCollectionsApiClient::new(
                configuration.clone(),
            )),
            user_entitlements_api: Box::new(user_entitlements_api::UserEntitlementsApiClient::new(
                configuration.clone(),
            )),
            user_recommendations_api: Box::new(
                user_recommendations_api::UserRecommendationsApiClient::new(configuration.clone()),
            ),
            user_reports_api: Box::new(user_reports_api::UserReportsApiClient::new(
                configuration.clone(),
            )),
            users_api: Box::new(users_api::UsersApiClient::new(configuration.clone())),
            videos_api: Box::new(videos_api::VideosApiClient::new(configuration.clone())),
        }
    }
}

impl Api for ApiClient {
    fn albums_api(&self) -> &dyn albums_api::AlbumsApi {
        self.albums_api.as_ref()
    }
    fn appreciations_api(&self) -> &dyn appreciations_api::AppreciationsApi {
        self.appreciations_api.as_ref()
    }
    fn artist_biographies_api(&self) -> &dyn artist_biographies_api::ArtistBiographiesApi {
        self.artist_biographies_api.as_ref()
    }
    fn artist_claims_api(&self) -> &dyn artist_claims_api::ArtistClaimsApi {
        self.artist_claims_api.as_ref()
    }
    fn artist_roles_api(&self) -> &dyn artist_roles_api::ArtistRolesApi {
        self.artist_roles_api.as_ref()
    }
    fn artists_api(&self) -> &dyn artists_api::ArtistsApi {
        self.artists_api.as_ref()
    }
    fn artworks_api(&self) -> &dyn artworks_api::ArtworksApi {
        self.artworks_api.as_ref()
    }
    fn dynamic_pages_api(&self) -> &dyn dynamic_pages_api::DynamicPagesApi {
        self.dynamic_pages_api.as_ref()
    }
    fn genres_api(&self) -> &dyn genres_api::GenresApi {
        self.genres_api.as_ref()
    }
    fn lyrics_api(&self) -> &dyn lyrics_api::LyricsApi {
        self.lyrics_api.as_ref()
    }
    fn manual_artist_claims_api(&self) -> &dyn manual_artist_claims_api::ManualArtistClaimsApi {
        self.manual_artist_claims_api.as_ref()
    }
    fn play_queues_api(&self) -> &dyn play_queues_api::PlayQueuesApi {
        self.play_queues_api.as_ref()
    }
    fn playlists_api(&self) -> &dyn playlists_api::PlaylistsApi {
        self.playlists_api.as_ref()
    }
    fn providers_api(&self) -> &dyn providers_api::ProvidersApi {
        self.providers_api.as_ref()
    }
    fn reactions_api(&self) -> &dyn reactions_api::ReactionsApi {
        self.reactions_api.as_ref()
    }
    fn saved_shares_api(&self) -> &dyn saved_shares_api::SavedSharesApi {
        self.saved_shares_api.as_ref()
    }
    fn search_results_api(&self) -> &dyn search_results_api::SearchResultsApi {
        self.search_results_api.as_ref()
    }
    fn search_suggestions_api(&self) -> &dyn search_suggestions_api::SearchSuggestionsApi {
        self.search_suggestions_api.as_ref()
    }
    fn shares_api(&self) -> &dyn shares_api::SharesApi {
        self.shares_api.as_ref()
    }
    fn stripe_connections_api(&self) -> &dyn stripe_connections_api::StripeConnectionsApi {
        self.stripe_connections_api.as_ref()
    }
    fn track_files_api(&self) -> &dyn track_files_api::TrackFilesApi {
        self.track_files_api.as_ref()
    }
    fn track_manifests_api(&self) -> &dyn track_manifests_api::TrackManifestsApi {
        self.track_manifests_api.as_ref()
    }
    fn track_source_files_api(&self) -> &dyn track_source_files_api::TrackSourceFilesApi {
        self.track_source_files_api.as_ref()
    }
    fn track_statistics_api(&self) -> &dyn track_statistics_api::TrackStatisticsApi {
        self.track_statistics_api.as_ref()
    }
    fn tracks_api(&self) -> &dyn tracks_api::TracksApi {
        self.tracks_api.as_ref()
    }
    fn user_collection_folders_api(
        &self,
    ) -> &dyn user_collection_folders_api::UserCollectionFoldersApi {
        self.user_collection_folders_api.as_ref()
    }
    fn user_collections_api(&self) -> &dyn user_collections_api::UserCollectionsApi {
        self.user_collections_api.as_ref()
    }
    fn user_entitlements_api(&self) -> &dyn user_entitlements_api::UserEntitlementsApi {
        self.user_entitlements_api.as_ref()
    }
    fn user_recommendations_api(&self) -> &dyn user_recommendations_api::UserRecommendationsApi {
        self.user_recommendations_api.as_ref()
    }
    fn user_reports_api(&self) -> &dyn user_reports_api::UserReportsApi {
        self.user_reports_api.as_ref()
    }
    fn users_api(&self) -> &dyn users_api::UsersApi {
        self.users_api.as_ref()
    }
    fn videos_api(&self) -> &dyn videos_api::VideosApi {
        self.videos_api.as_ref()
    }
}

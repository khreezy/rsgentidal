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
            Error::ResponseError(e) => (
                "response",
                format!("status code {} content {}", e.status, e.content),
            ),
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
#[cfg(feature = "internal")]
pub mod appreciations_api;
#[cfg(feature = "internal")]
pub mod artist_biographies_api;
#[cfg(feature = "internal")]
pub mod artist_claims_api;
#[cfg(feature = "internal")]
pub mod artist_roles_api;
pub mod artists_api;
pub mod artworks_api;
#[cfg(feature = "internal")]
pub mod dynamic_pages_api;
#[cfg(feature = "internal")]
pub mod genres_api;
#[cfg(feature = "internal")]
pub mod lyrics_api;
#[cfg(feature = "internal")]
pub mod manual_artist_claims_api;
#[cfg(feature = "internal")]
pub mod play_queues_api;
pub mod playlists_api;
pub mod providers_api;
#[cfg(feature = "internal")]
pub mod reactions_api;
#[cfg(feature = "internal")]
pub mod saved_shares_api;
pub mod search_results_api;
pub mod search_suggestions_api;
#[cfg(feature = "internal")]
pub mod shares_api;
#[cfg(feature = "internal")]
pub mod stripe_connections_api;
#[cfg(feature = "internal")]
pub mod track_files_api;
#[cfg(feature = "internal")]
pub mod track_manifests_api;
#[cfg(feature = "internal")]
pub mod track_source_files_api;
#[cfg(feature = "internal")]
pub mod track_statistics_api;
pub mod tracks_api;
pub mod user_collection_folders_api;
pub mod user_collections_api;
pub mod user_entitlements_api;
pub mod user_recommendations_api;
#[cfg(feature = "internal")]
pub mod user_reports_api;
pub mod users_api;
pub mod videos_api;

pub mod configuration;

use std::sync::Arc;

pub trait Api {
    fn albums_api(&self) -> &dyn albums_api::AlbumsApi;
    #[cfg(feature = "internal")]
    fn appreciations_api(&self) -> &dyn appreciations_api::AppreciationsApi;
    #[cfg(feature = "internal")]
    fn artist_biographies_api(&self) -> &dyn artist_biographies_api::ArtistBiographiesApi;
    #[cfg(feature = "internal")]
    fn artist_claims_api(&self) -> &dyn artist_claims_api::ArtistClaimsApi;
    #[cfg(feature = "internal")]
    fn artist_roles_api(&self) -> &dyn artist_roles_api::ArtistRolesApi;
    fn artists_api(&self) -> &dyn artists_api::ArtistsApi;
    fn artworks_api(&self) -> &dyn artworks_api::ArtworksApi;
    #[cfg(feature = "internal")]
    fn dynamic_pages_api(&self) -> &dyn dynamic_pages_api::DynamicPagesApi;
    #[cfg(feature = "internal")]
    fn genres_api(&self) -> &dyn genres_api::GenresApi;
    #[cfg(feature = "internal")]
    fn lyrics_api(&self) -> &dyn lyrics_api::LyricsApi;
    #[cfg(feature = "internal")]
    fn manual_artist_claims_api(&self) -> &dyn manual_artist_claims_api::ManualArtistClaimsApi;
    #[cfg(feature = "internal")]
    fn play_queues_api(&self) -> &dyn play_queues_api::PlayQueuesApi;
    fn playlists_api(&self) -> &dyn playlists_api::PlaylistsApi;
    fn providers_api(&self) -> &dyn providers_api::ProvidersApi;
    #[cfg(feature = "internal")]
    fn reactions_api(&self) -> &dyn reactions_api::ReactionsApi;
    #[cfg(feature = "internal")]
    fn saved_shares_api(&self) -> &dyn saved_shares_api::SavedSharesApi;
    fn search_results_api(&self) -> &dyn search_results_api::SearchResultsApi;
    fn search_suggestions_api(&self) -> &dyn search_suggestions_api::SearchSuggestionsApi;
    #[cfg(feature = "internal")]
    fn shares_api(&self) -> &dyn shares_api::SharesApi;
    #[cfg(feature = "internal")]
    fn stripe_connections_api(&self) -> &dyn stripe_connections_api::StripeConnectionsApi;
    #[cfg(feature = "internal")]
    fn track_files_api(&self) -> &dyn track_files_api::TrackFilesApi;
    #[cfg(feature = "internal")]
    fn track_manifests_api(&self) -> &dyn track_manifests_api::TrackManifestsApi;
    #[cfg(feature = "internal")]
    fn track_source_files_api(&self) -> &dyn track_source_files_api::TrackSourceFilesApi;
    #[cfg(feature = "internal")]
    fn track_statistics_api(&self) -> &dyn track_statistics_api::TrackStatisticsApi;
    fn tracks_api(&self) -> &dyn tracks_api::TracksApi;
    fn user_collection_folders_api(
        &self,
    ) -> &dyn user_collection_folders_api::UserCollectionFoldersApi;
    fn user_collections_api(&self) -> &dyn user_collections_api::UserCollectionsApi;
    fn user_entitlements_api(&self) -> &dyn user_entitlements_api::UserEntitlementsApi;
    fn user_recommendations_api(&self) -> &dyn user_recommendations_api::UserRecommendationsApi;
    #[cfg(feature = "internal")]
    fn user_reports_api(&self) -> &dyn user_reports_api::UserReportsApi;
    fn users_api(&self) -> &dyn users_api::UsersApi;
    fn videos_api(&self) -> &dyn videos_api::VideosApi;
}

#[derive(Clone)]
pub struct ApiClient {
    albums_api: Arc<dyn albums_api::AlbumsApi>,
    #[cfg(feature = "internal")]
    appreciations_api: Arc<dyn appreciations_api::AppreciationsApi>,
    #[cfg(feature = "internal")]
    artist_biographies_api: Arc<dyn artist_biographies_api::ArtistBiographiesApi>,
    #[cfg(feature = "internal")]
    artist_claims_api: Arc<dyn artist_claims_api::ArtistClaimsApi>,
    #[cfg(feature = "internal")]
    artist_roles_api: Arc<dyn artist_roles_api::ArtistRolesApi>,
    artists_api: Arc<dyn artists_api::ArtistsApi>,
    artworks_api: Arc<dyn artworks_api::ArtworksApi>,
    #[cfg(feature = "internal")]
    dynamic_pages_api: Arc<dyn dynamic_pages_api::DynamicPagesApi>,
    #[cfg(feature = "internal")]
    genres_api: Arc<dyn genres_api::GenresApi>,
    #[cfg(feature = "internal")]
    lyrics_api: Arc<dyn lyrics_api::LyricsApi>,
    #[cfg(feature = "internal")]
    manual_artist_claims_api: Arc<dyn manual_artist_claims_api::ManualArtistClaimsApi>,
    #[cfg(feature = "internal")]
    play_queues_api: Arc<dyn play_queues_api::PlayQueuesApi>,
    playlists_api: Arc<dyn playlists_api::PlaylistsApi>,
    providers_api: Arc<dyn providers_api::ProvidersApi>,
    #[cfg(feature = "internal")]
    reactions_api: Arc<dyn reactions_api::ReactionsApi>,
    #[cfg(feature = "internal")]
    saved_shares_api: Arc<dyn saved_shares_api::SavedSharesApi>,
    search_results_api: Arc<dyn search_results_api::SearchResultsApi>,
    search_suggestions_api: Arc<dyn search_suggestions_api::SearchSuggestionsApi>,
    #[cfg(feature = "internal")]
    shares_api: Arc<dyn shares_api::SharesApi>,
    #[cfg(feature = "internal")]
    stripe_connections_api: Arc<dyn stripe_connections_api::StripeConnectionsApi>,
    #[cfg(feature = "internal")]
    track_files_api: Arc<dyn track_files_api::TrackFilesApi>,
    #[cfg(feature = "internal")]
    track_manifests_api: Arc<dyn track_manifests_api::TrackManifestsApi>,
    #[cfg(feature = "internal")]
    track_source_files_api: Arc<dyn track_source_files_api::TrackSourceFilesApi>,
    #[cfg(feature = "internal")]
    track_statistics_api: Arc<dyn track_statistics_api::TrackStatisticsApi>,
    tracks_api: Arc<dyn tracks_api::TracksApi>,
    user_collection_folders_api: Arc<dyn user_collection_folders_api::UserCollectionFoldersApi>,
    user_collections_api: Arc<dyn user_collections_api::UserCollectionsApi>,
    user_entitlements_api: Arc<dyn user_entitlements_api::UserEntitlementsApi>,
    user_recommendations_api: Arc<dyn user_recommendations_api::UserRecommendationsApi>,
    #[cfg(feature = "internal")]
    user_reports_api: Arc<dyn user_reports_api::UserReportsApi>,
    users_api: Arc<dyn users_api::UsersApi>,
    videos_api: Arc<dyn videos_api::VideosApi>,
}

impl ApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self {
            albums_api: Arc::new(albums_api::AlbumsApiClient::new(configuration.clone())),
            #[cfg(feature = "internal")]
            appreciations_api: Arc::new(appreciations_api::AppreciationsApiClient::new(
                configuration.clone(),
            )),
            #[cfg(feature = "internal")]
            artist_biographies_api: Arc::new(
                artist_biographies_api::ArtistBiographiesApiClient::new(configuration.clone()),
            ),
            #[cfg(feature = "internal")]
            artist_claims_api: Arc::new(artist_claims_api::ArtistClaimsApiClient::new(
                configuration.clone(),
            )),
            #[cfg(feature = "internal")]
            artist_roles_api: Arc::new(artist_roles_api::ArtistRolesApiClient::new(
                configuration.clone(),
            )),
            artists_api: Arc::new(artists_api::ArtistsApiClient::new(configuration.clone())),
            artworks_api: Arc::new(artworks_api::ArtworksApiClient::new(configuration.clone())),
            #[cfg(feature = "internal")]
            dynamic_pages_api: Arc::new(dynamic_pages_api::DynamicPagesApiClient::new(
                configuration.clone(),
            )),
            #[cfg(feature = "internal")]
            genres_api: Arc::new(genres_api::GenresApiClient::new(configuration.clone())),
            #[cfg(feature = "internal")]
            lyrics_api: Arc::new(lyrics_api::LyricsApiClient::new(configuration.clone())),
            #[cfg(feature = "internal")]
            manual_artist_claims_api: Arc::new(
                manual_artist_claims_api::ManualArtistClaimsApiClient::new(configuration.clone()),
            ),
            #[cfg(feature = "internal")]
            play_queues_api: Arc::new(play_queues_api::PlayQueuesApiClient::new(
                configuration.clone(),
            )),
            playlists_api: Arc::new(playlists_api::PlaylistsApiClient::new(
                configuration.clone(),
            )),
            providers_api: Arc::new(providers_api::ProvidersApiClient::new(
                configuration.clone(),
            )),
            #[cfg(feature = "internal")]
            reactions_api: Arc::new(reactions_api::ReactionsApiClient::new(
                configuration.clone(),
            )),
            #[cfg(feature = "internal")]
            saved_shares_api: Arc::new(saved_shares_api::SavedSharesApiClient::new(
                configuration.clone(),
            )),
            search_results_api: Arc::new(search_results_api::SearchResultsApiClient::new(
                configuration.clone(),
            )),
            search_suggestions_api: Arc::new(
                search_suggestions_api::SearchSuggestionsApiClient::new(configuration.clone()),
            ),
            #[cfg(feature = "internal")]
            shares_api: Arc::new(shares_api::SharesApiClient::new(configuration.clone())),
            #[cfg(feature = "internal")]
            stripe_connections_api: Arc::new(
                stripe_connections_api::StripeConnectionsApiClient::new(configuration.clone()),
            ),
            #[cfg(feature = "internal")]
            track_files_api: Arc::new(track_files_api::TrackFilesApiClient::new(
                configuration.clone(),
            )),
            #[cfg(feature = "internal")]
            track_manifests_api: Arc::new(track_manifests_api::TrackManifestsApiClient::new(
                configuration.clone(),
            )),
            #[cfg(feature = "internal")]
            track_source_files_api: Arc::new(
                track_source_files_api::TrackSourceFilesApiClient::new(configuration.clone()),
            ),
            #[cfg(feature = "internal")]
            track_statistics_api: Arc::new(track_statistics_api::TrackStatisticsApiClient::new(
                configuration.clone(),
            )),
            tracks_api: Arc::new(tracks_api::TracksApiClient::new(configuration.clone())),
            user_collection_folders_api: Arc::new(
                user_collection_folders_api::UserCollectionFoldersApiClient::new(
                    configuration.clone(),
                ),
            ),
            user_collections_api: Arc::new(user_collections_api::UserCollectionsApiClient::new(
                configuration.clone(),
            )),
            user_entitlements_api: Arc::new(user_entitlements_api::UserEntitlementsApiClient::new(
                configuration.clone(),
            )),
            user_recommendations_api: Arc::new(
                user_recommendations_api::UserRecommendationsApiClient::new(configuration.clone()),
            ),
            #[cfg(feature = "internal")]
            user_reports_api: Arc::new(user_reports_api::UserReportsApiClient::new(
                configuration.clone(),
            )),
            users_api: Arc::new(users_api::UsersApiClient::new(configuration.clone())),
            videos_api: Arc::new(videos_api::VideosApiClient::new(configuration.clone())),
        }
    }
}

impl Api for ApiClient {
    fn albums_api(&self) -> &dyn albums_api::AlbumsApi {
        self.albums_api.as_ref()
    }
    #[cfg(feature = "internal")]
    fn appreciations_api(&self) -> &dyn appreciations_api::AppreciationsApi {
        self.appreciations_api.as_ref()
    }
    #[cfg(feature = "internal")]
    fn artist_biographies_api(&self) -> &dyn artist_biographies_api::ArtistBiographiesApi {
        self.artist_biographies_api.as_ref()
    }
    #[cfg(feature = "internal")]
    fn artist_claims_api(&self) -> &dyn artist_claims_api::ArtistClaimsApi {
        self.artist_claims_api.as_ref()
    }
    #[cfg(feature = "internal")]
    fn artist_roles_api(&self) -> &dyn artist_roles_api::ArtistRolesApi {
        self.artist_roles_api.as_ref()
    }
    fn artists_api(&self) -> &dyn artists_api::ArtistsApi {
        self.artists_api.as_ref()
    }
    fn artworks_api(&self) -> &dyn artworks_api::ArtworksApi {
        self.artworks_api.as_ref()
    }
    #[cfg(feature = "internal")]
    fn dynamic_pages_api(&self) -> &dyn dynamic_pages_api::DynamicPagesApi {
        self.dynamic_pages_api.as_ref()
    }
    #[cfg(feature = "internal")]
    fn genres_api(&self) -> &dyn genres_api::GenresApi {
        self.genres_api.as_ref()
    }
    #[cfg(feature = "internal")]
    fn lyrics_api(&self) -> &dyn lyrics_api::LyricsApi {
        self.lyrics_api.as_ref()
    }
    #[cfg(feature = "internal")]
    fn manual_artist_claims_api(&self) -> &dyn manual_artist_claims_api::ManualArtistClaimsApi {
        self.manual_artist_claims_api.as_ref()
    }
    #[cfg(feature = "internal")]
    fn play_queues_api(&self) -> &dyn play_queues_api::PlayQueuesApi {
        self.play_queues_api.as_ref()
    }
    fn playlists_api(&self) -> &dyn playlists_api::PlaylistsApi {
        self.playlists_api.as_ref()
    }
    fn providers_api(&self) -> &dyn providers_api::ProvidersApi {
        self.providers_api.as_ref()
    }
    #[cfg(feature = "internal")]
    fn reactions_api(&self) -> &dyn reactions_api::ReactionsApi {
        self.reactions_api.as_ref()
    }
    #[cfg(feature = "internal")]
    fn saved_shares_api(&self) -> &dyn saved_shares_api::SavedSharesApi {
        self.saved_shares_api.as_ref()
    }
    fn search_results_api(&self) -> &dyn search_results_api::SearchResultsApi {
        self.search_results_api.as_ref()
    }
    fn search_suggestions_api(&self) -> &dyn search_suggestions_api::SearchSuggestionsApi {
        self.search_suggestions_api.as_ref()
    }
    #[cfg(feature = "internal")]
    fn shares_api(&self) -> &dyn shares_api::SharesApi {
        self.shares_api.as_ref()
    }
    #[cfg(feature = "internal")]
    fn stripe_connections_api(&self) -> &dyn stripe_connections_api::StripeConnectionsApi {
        self.stripe_connections_api.as_ref()
    }
    #[cfg(feature = "internal")]
    fn track_files_api(&self) -> &dyn track_files_api::TrackFilesApi {
        self.track_files_api.as_ref()
    }
    #[cfg(feature = "internal")]
    fn track_manifests_api(&self) -> &dyn track_manifests_api::TrackManifestsApi {
        self.track_manifests_api.as_ref()
    }
    #[cfg(feature = "internal")]
    fn track_source_files_api(&self) -> &dyn track_source_files_api::TrackSourceFilesApi {
        self.track_source_files_api.as_ref()
    }
    #[cfg(feature = "internal")]
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
    #[cfg(feature = "internal")]
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

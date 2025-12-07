use crate::error::Error;
use crate::types::*;
use std::time::Duration;

const DEFAULT_BASE_URL: &str = "https://api-public.cs-prod.leetify.com";
const API_KEY_HEADER: &str = "_leetify_key";
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// Builder for creating a customized `Client`
pub struct ClientBuilder {
    base_url: Option<String>,
    api_key: Option<String>,
    timeout: Option<Duration>,
    client_builder: reqwest::ClientBuilder,
}

impl ClientBuilder {
    /// Create a new builder with default settings
    pub fn new() -> Self {
        Self {
            base_url: None,
            api_key: None,
            timeout: Some(DEFAULT_TIMEOUT),
            client_builder: reqwest::Client::builder(),
        }
    }

    /// Set a custom base URL for the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use leetify::Client;
    ///
    /// let client = Client::builder()
    ///     .base_url("https://custom-api.example.com")
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// Set the API key
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use leetify::Client;
    ///
    /// let client = Client::builder()
    ///     .api_key("your-api-key")
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn api_key(mut self, key: impl Into<String>) -> Self {
        self.api_key = Some(key.into());
        self
    }

    /// Set the request timeout
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use leetify::Client;
    /// use std::time::Duration;
    ///
    /// let client = Client::builder()
    ///     .timeout(Duration::from_secs(60))
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self.client_builder = self.client_builder.timeout(timeout);
        self
    }

    /// Configure the underlying reqwest client builder
    ///
    /// This allows advanced configuration of the HTTP client.
    pub fn client_builder(mut self, builder: reqwest::ClientBuilder) -> Self {
        self.client_builder = builder;
        self
    }

    /// Build the client
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use leetify::Client;
    ///
    /// let client = Client::builder()
    ///     .api_key("your-api-key")
    ///     .build()?;
    /// # Ok::<(), leetify::Error>(())
    /// ```
    pub fn build(self) -> Result<Client, Error> {
        let client = self
            .client_builder
            .timeout(self.timeout.unwrap_or(DEFAULT_TIMEOUT))
            .build()
            .map_err(Error::Http)?;

        Ok(Client {
            client,
            base_url: self
                .base_url
                .unwrap_or_else(|| DEFAULT_BASE_URL.to_string()),
            api_key: self.api_key,
        })
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Client for interacting with the Leetify Public CS API
pub struct Client {
    client: reqwest::Client,
    base_url: String,
    api_key: Option<String>,
}

impl Client {
    /// Create a new client without an API key
    ///
    /// Requests without an API key are subject to increased rate limits.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use leetify::Client;
    ///
    /// let client = Client::new();
    /// ```
    pub fn new() -> Self {
        ClientBuilder::new()
            .build()
            .expect("Failed to create default client")
    }

    /// Create a new client with an API key
    ///
    /// API keys can be obtained at https://leetify.com/app/developer
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use leetify::Client;
    ///
    /// let client = Client::with_api_key("your-api-key".to_string());
    /// ```
    pub fn with_api_key(api_key: String) -> Self {
        ClientBuilder::new()
            .api_key(api_key)
            .build()
            .expect("Failed to create client with API key")
    }

    /// Create a builder for customizing the client configuration
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use leetify::Client;
    /// use std::time::Duration;
    ///
    /// let client = Client::builder()
    ///     .api_key("your-api-key")
    ///     .timeout(Duration::from_secs(60))
    ///     .base_url("https://custom-api.example.com")
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    /// Get player profile
    ///
    /// # Arguments
    /// * `id` - Player id (either Steam64 ID or Leetify ID)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use leetify::{Client, PlayerId, Steam64Id, LeetifyId};
    /// # async fn example() -> Result<(), leetify::Error> {
    /// let client = Client::new();
    ///
    /// // Using Steam64 ID
    /// let profile = client.get_profile(PlayerId::Steam64("76561198000000000".into())).await?;
    ///
    /// // Using Leetify ID (UUID format)
    /// let profile = client.get_profile(PlayerId::Leetify("5ea07280-2399-4c7e-88ab-f2f7db0c449f".into())).await?;
    ///
    /// // Using automatic conversion with type annotation
    /// let id: PlayerId = "76561198000000000".into();
    /// let profile = client.get_profile(id).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_profile(&self, id: impl Into<PlayerId>) -> Result<ProfileResponse, Error> {
        let id = id.into();

        let url = format!("{}/v3/profile", self.base_url);
        let query_params = self.build_profile_query_params(&id);

        let mut request = self.client.get(&url);
        if !query_params.is_empty() {
            request = request.query(&query_params);
        }
        request = self.add_api_key_header(request);

        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Get player match history
    ///
    /// # Arguments
    /// * `id` - Player id (either Steam64 ID or Leetify ID)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use leetify::{Client, PlayerId};
    /// # async fn example() -> Result<(), leetify::Error> {
    /// let client = Client::new();
    ///
    /// // Get matches by Steam64 ID
    /// let matches = client.get_profile_matches(PlayerId::Steam64("76561198000000000".into())).await?;
    ///
    /// // Get matches by Leetify ID (UUID format)
    /// let matches = client.get_profile_matches(PlayerId::Leetify("5ea07280-2399-4c7e-88ab-f2f7db0c449f".into())).await?;
    ///
    /// // Using automatic conversion with type annotation
    /// let id: PlayerId = "76561198000000000".into();
    /// let matches = client.get_profile_matches(id).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_profile_matches(
        &self,
        id: impl Into<PlayerId>,
    ) -> Result<Vec<MatchDetailsResponse>, Error> {
        let id = id.into();

        let url = format!("{}/v3/profile/matches", self.base_url);
        let query_params = self.build_profile_query_params(&id);

        let mut request = self.client.get(&url);
        if !query_params.is_empty() {
            request = request.query(&query_params);
        }
        request = self.add_api_key_header(request);

        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Get match details by game ID
    ///
    /// # Arguments
    /// * `game_id` - The game ID (Leetify match ID)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use leetify::Client;
    /// # async fn example() -> Result<(), leetify::Error> {
    /// let client = Client::new();
    ///
    /// let match_details = client.get_match_by_game_id("match-id-123".to_string()).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_match_by_game_id(
        &self,
        game_id: String,
    ) -> Result<MatchDetailsResponse, Error> {
        let url = format!("{}/v2/matches/{}", self.base_url, game_id);
        let request = self.client.get(&url);
        let request = self.add_api_key_header(request);

        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Get match details by data source and data source ID
    ///
    /// # Arguments
    /// * `data_source` - The data source (e.g., "faceit", "matchmaking")
    /// * `data_source_id` - The data source specific match ID
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use leetify::{Client, DataSource};
    /// # async fn example() -> Result<(), leetify::Error> {
    /// let client = Client::new();
    ///
    /// // Using DataSource enum
    /// let match_details = client
    ///     .get_match_by_data_source(DataSource::FACEIT, "faceit-match-id")
    ///     .await?;
    ///
    /// // Using string (will be converted to DataSource)
    /// let match_details = client
    ///     .get_match_by_data_source("matchmaking", "matchmaking-match-id")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_match_by_data_source(
        &self,
        data_source: impl Into<DataSource>,
        data_source_id: impl AsRef<str>,
    ) -> Result<MatchDetailsResponse, Error> {
        let data_source = data_source.into();
        let url = format!(
            "{}/v2/matches/{}/{}",
            self.base_url,
            data_source.as_str(),
            data_source_id.as_ref()
        );
        let request = self.client.get(&url);
        let request = self.add_api_key_header(request);

        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Validate the API key
    ///
    /// Returns:
    /// - `Ok(())` if the key is valid
    /// - `Err(Error::InvalidApiKey)` if the key is invalid or missing
    /// - `Err(Error::ServerError)` if there was a server error
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use leetify::Client;
    /// # async fn example() -> Result<(), leetify::Error> {
    /// let client = Client::with_api_key("your-api-key".to_string());
    ///
    /// match client.validate_api_key().await {
    ///     Ok(()) => println!("API key is valid"),
    ///     Err(e) => eprintln!("API key validation failed: {}", e),
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn validate_api_key(&self) -> Result<(), Error> {
        let url = format!("{}/api-key/validate", self.base_url);
        let request = self.client.get(&url);
        let request = self.add_api_key_header(request);

        let response = request.send().await?;
        let status = response.status();

        match status.as_u16() {
            200 => Ok(()),
            401 => Err(Error::InvalidApiKey),
            500 => Err(Error::ServerError),
            _ => Err(Error::Api(
                status.as_u16(),
                format!("Unexpected status code: {}", status),
            )),
        }
    }

    fn build_profile_query_params(&self, id: &PlayerId) -> Vec<(&'static str, String)> {
        match id {
            PlayerId::Steam64(id) => {
                vec![("steam64_id", id.as_ref().to_string())]
            }
            PlayerId::Leetify(id) => {
                vec![("id", id.as_ref().to_string())]
            }
        }
    }

    fn add_api_key_header(&self, request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        if let Some(ref api_key) = self.api_key {
            request.header(API_KEY_HEADER, api_key.as_str())
        } else {
            request
        }
    }

    async fn handle_response<T>(&self, response: reqwest::Response) -> Result<T, Error>
    where
        T: serde::de::DeserializeOwned,
    {
        let status = response.status();
        let response_text = response.text().await?;

        if !status.is_success() {
            let status_code = status.as_u16();
            return match status_code {
                401 => Err(Error::InvalidApiKey),
                500 => Err(Error::ServerError),
                _ => Err(Error::Api(status_code, response_text)),
            };
        }

        // Try to parse JSON, but provide better error message if it fails
        match serde_json::from_str::<T>(&response_text) {
            Ok(json) => Ok(json),
            Err(e) => {
                // If JSON parsing fails, create a more descriptive error
                // We'll wrap it in an Api error with the response text
                Err(Error::Api(
                    status.as_u16(),
                    format!(
                        "Failed to parse JSON response: {}. Response body: {}",
                        e, response_text
                    ),
                ))
            }
        }
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_id_conversion() {
        // Test Steam64 ID conversion (numeric, 17 digits)
        let id: PlayerId = "76561198283431555".into();
        assert!(matches!(id, PlayerId::Steam64(_)));

        // Test Leetify ID conversion (UUID format)
        let id: PlayerId = "5ea07280-2399-4c7e-88ab-f2f7db0c449f".into();
        assert!(matches!(id, PlayerId::Leetify(_)));

        // Test explicit Steam64 variant
        let id = PlayerId::Steam64("76561198283431555".into());
        assert!(matches!(id, PlayerId::Steam64(_)));

        // Test explicit Leetify variant
        let id = PlayerId::Leetify("5ea07280-2399-4c7e-88ab-f2f7db0c449f".into());
        assert!(matches!(id, PlayerId::Leetify(_)));

        // Test that numeric strings >= 15 digits are treated as Steam64
        let id: PlayerId = "76561198000000000".into();
        assert!(matches!(id, PlayerId::Steam64(_)));

        // Test that UUID format strings are treated as Leetify
        let id: PlayerId = "00000000-0000-0000-0000-000000000000".into();
        assert!(matches!(id, PlayerId::Leetify(_)));
    }

    #[test]
    fn test_client_builder() {
        let builder = ClientBuilder::new();
        assert!(builder.base_url.is_none());
        assert!(builder.api_key.is_none());
        assert!(builder.timeout.is_some());
    }

    #[test]
    fn test_client_builder_with_options() {
        let client = ClientBuilder::new()
            .base_url("https://test.example.com")
            .api_key("test-key")
            .timeout(Duration::from_secs(60))
            .build()
            .unwrap();

        assert_eq!(client.base_url, "https://test.example.com");
        assert_eq!(client.api_key, Some("test-key".to_string()));
    }

    #[test]
    fn test_steam64_id_conversion() {
        let id: Steam64Id = "76561198000000000".into();
        assert_eq!(id.as_ref(), "76561198000000000");
    }

    #[test]
    fn test_leetify_id_conversion() {
        let id: LeetifyId = "user-123".into();
        assert_eq!(id.as_ref(), "user-123");
    }

    #[test]
    fn test_data_source_conversion() {
        let ds: DataSource = "faceit".into();
        assert!(matches!(ds, DataSource::FACEIT));
        assert_eq!(ds.as_str(), "faceit");

        let ds: DataSource = "matchmaking".into();
        assert!(matches!(ds, DataSource::Matchmaking));
        assert_eq!(ds.as_str(), "matchmaking");

        let ds: DataSource = "other".into();
        match ds {
            DataSource::Other(s) => assert_eq!(s, "other"),
            _ => panic!("Expected Other variant"),
        }
    }
}

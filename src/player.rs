use crate::client::Client;
use crate::error::Error;
use crate::types::{MatchDetailsResponse, PlayerId, ProfileResponse};

/// High-level API for interacting with a specific player
///
/// This struct provides a convenient way to work with player data
/// without needing to pass the player id to each method call.
///
/// # Examples
///
/// ```no_run
/// # use leetify::{Client, Player, PlayerId};
/// # async fn example() -> Result<(), leetify::Error> {
/// let client = Client::new();
/// let player = Player::new(PlayerId::Steam64("76561198283431555".into()), &client);
///
/// // Get profile
/// let profile = player.profile().await?;
/// println!("Player: {}", profile.name);
///
/// // Get matches
/// let matches = player.matches().await?;
/// println!("Found {} matches", matches.len());
/// # Ok(())
/// # }
/// ```
pub struct Player<'a> {
    id: PlayerId,
    client: &'a Client,
}

impl<'a> Player<'a> {
    /// Create a new Player instance
    ///
    /// # Arguments
    /// * `id` - The player id (Steam64 ID or Leetify ID)
    /// * `client` - Reference to the Leetify client
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use leetify::{Client, Player, PlayerId};
    /// let client = Client::new();
    /// let player = Player::new(PlayerId::Steam64("76561198283431555".into()), &client);
    /// ```
    pub fn new(id: impl Into<PlayerId>, client: &'a Client) -> Self {
        Self {
            id: id.into(),
            client,
        }
    }

    /// Get the player's id
    pub fn id(&self) -> &PlayerId {
        &self.id
    }

    /// Get the player's profile
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use leetify::{Client, Player, PlayerId};
    /// # async fn example() -> Result<(), leetify::Error> {
    /// let client = Client::new();
    /// let player = Player::new(PlayerId::Steam64("76561198283431555".into()), &client);
    /// let profile = player.profile().await?;
    /// println!("Player: {}", profile.name);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn profile(&self) -> Result<ProfileResponse, Error> {
        self.client.get_profile(self.id.clone()).await
    }

    /// Get the player's match history
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use leetify::{Client, Player, PlayerId};
    /// # async fn example() -> Result<(), leetify::Error> {
    /// let client = Client::new();
    /// let player = Player::new(PlayerId::Steam64("76561198283431555".into()), &client);
    /// let matches = player.matches().await?;
    /// println!("Found {} matches", matches.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn matches(&self) -> Result<Vec<MatchDetailsResponse>, Error> {
        self.client.get_profile_matches(self.id.clone()).await
    }
}

#[cfg(feature = "player")]
impl Client {
    /// Create a Player instance for the given id
    ///
    /// This is a convenience method that creates a Player instance
    /// using this client.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use leetify::{Client, PlayerId};
    /// # async fn example() -> Result<(), leetify::Error> {
    /// let client = Client::new();
    /// let player = client.player(PlayerId::Steam64("76561198283431555".into()));
    ///
    /// let profile = player.profile().await?;
    /// let matches = player.matches().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn player(&self, id: impl Into<PlayerId>) -> crate::player::Player<'_> {
        crate::player::Player::new(id, self)
    }
}

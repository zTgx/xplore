///! This module defines the `IProfile` trait for fetching user profiles and user IDs by screen name.
use {
    crate::core::models::{profile::Profile, Result},
    async_trait::async_trait,
};

#[async_trait]
pub trait IProfile {
    ///! Fetches the profile of a user by their screen name.
    /// # Arguments
    /// * `screen_name` - The screen name of the user whose profile is to be fetched.
    /// # Returns
    /// * `Result<Profile>` - A result containing the user's profile if successful, or an error if not.
    /// # Errors
    /// Returns an error if the profile cannot be fetched, such as if the user does not exist or if there is a network issue.
    async fn get_profile(&self, screen_name: &str) -> Result<Profile>;

    ///! Fetches the user ID of a user by their screen name.
    /// # Arguments
    /// * `screen_name` - The screen name of the user whose ID is to be fetched.
    /// # Returns
    /// * `Result<String>` - A result containing the user's ID if successful, or an error if not.
    /// # Errors
    /// Returns an error if the user ID cannot be fetched, such as if the user does not exist or if there is a network issue.
    async fn get_user_id(&self, screen_name: &str) -> Result<String>;
}

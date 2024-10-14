use crate::{
    adventures::Adventures, adventures::AdventuresManager,
    adventures::CreateAdventureError, adventures::DeleteAdventureError,
    adventures::NewJourney, adventures::NewJourneyData, favorites::AddFavorite,
    favorites::DelFavorite, favorites::Favorite, favorites::FavoriteError,
    favorites::FavoritesManager, DomainError, UsersManager,
};
use repository::users::models::{MyUsers, NewMyUsers};
use serde::Serialize;
use vars::ID;

#[derive(Serialize, Debug, Clone)]
pub struct Users {
    pub id: ID,
    pub username: String,
    pub password: String,
    pub roles: Vec<String>,
}
#[derive(Serialize, Debug, Clone)]
pub struct RegistryUsers {
    pub username: String,
    pub password: String,
    pub roles: Vec<String>,
}

pub mod connection;
pub mod error;


pub use connection::Repo;

use once_cell::sync::OnceCell;



pub static REPOSITORY: OnceCell<Repo> = OnceCell::new();

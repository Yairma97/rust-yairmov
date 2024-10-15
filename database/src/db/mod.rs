pub mod connection;


pub use connection::Repo;

use once_cell::sync::OnceCell;



static REPOSITORY: OnceCell<Repo> = OnceCell::new();

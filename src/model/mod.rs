//!Model Layer
//!
//! Design
//!
//! - 	The Model layer normalizes the application's data type
//! 	structures and access.
//! -	All application code data access must go through the Model Layer.
//! -	The 'Model Manager' holds the internal states/resources
//! 	need by the ModelControllers to access data. e.g db_pool, S3 cliet, redis client.
//! - 	Model Controllers e.g TaskBmc ProjectBmc implement CRUD
//! 	and other data access methods on a given "entity"
//! 	e.g Task, Project. Bmc is for Backend Model Controller
//! - 	In frmaeworks like Axum, Tauri, 'ModelManager' are typically used as App State
//!
//!

// region:    --- Modules

mod base;
mod error;
mod store;
pub mod task;
pub mod user;

use store::{new_db_pool, Db};

pub use self::error::{Error, Result};

// endregion: --- Modules

#[derive(Clone)]
pub struct ModelManager {
	db: Db,
}

impl ModelManager {
	pub async fn new() -> Result<Self> {
		let db = new_db_pool().await?;
		Ok(ModelManager { db })
	}

	///Returns the sqlx db pool reference.
	/// (Only for the model layer)
	pub(in crate::model) fn db(&self) -> &Db {
		&self.db
	}
}

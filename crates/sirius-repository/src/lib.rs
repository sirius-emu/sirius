//! The data access layer for Sirius.
//!
//! `Repository` is the single entry point for all database access. It owns
//! one sub-repository per entity family and is cheaply cloneable.

use sirius_database::Database;

use crate::repositories::UserRepository;

pub mod models;
pub mod repositories;

/// The central repository manager.
///
/// Construct once at startup and pass as `Arc<Repository>` or clone.
/// Cloning is cheap because every sub-repository holds an `Arc` over the
/// underlying connection pool.
#[derive(Debug, Clone)]
pub struct Repository {
    pub users: UserRepository,
}

impl Repository {
    pub fn new(database: &Database) -> Self {
        let pool = database.pool().clone();

        Self {
            users: UserRepository::new(pool.clone()),
        }
    }
}

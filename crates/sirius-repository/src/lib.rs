//! The data access layer for Sirius.
//!
//! This crate contains all SQL queries and domain models.

use sirius_database::Database;

use crate::repositories::UserRepository;

pub mod models;
pub mod repositories;

/// The cenral repository manager.
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

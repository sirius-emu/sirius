//! The data access layer for Sirius.
//!
//! `Repository` is the single entry point for all database access. It owns
//! one sub-repository per entity family and is cheaply cloneable.

use crate::repositories::{RoomRepository, UserRepository};
use sirius_database::Database;
use sirius_error::{DatabaseError, SiriusError};

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
    pub rooms: RoomRepository,
}

impl Repository {
    pub fn new(database: &Database) -> Self {
        let pool = database.pool().clone();

        Self {
            users: UserRepository::new(pool.clone()),
            rooms: RoomRepository::new(pool.clone()),
        }
    }
}

/// Converts a [`sqlx::Error`] into a [`SiriusError`].
pub fn map_sqlx_error(err: sqlx::Error) -> SiriusError {
    let db_err = match err {
        sqlx::Error::RowNotFound => {
            DatabaseError::NotFound { entity: "Unknown" }
        }
        sqlx::Error::Database(db_err) => {
            if let Some(code) = db_err.code() {
                if code == "23505" {
                    // unique_violation
                    return SiriusError::Database(
                        DatabaseError::UniqueViolation {
                            field: db_err
                                .constraint()
                                .unwrap_or("unknown")
                                .to_string(),
                        },
                    );
                }
            }
            DatabaseError::QueryFailed {
                reason: db_err.message().to_string(),
            }
        }
        _ => DatabaseError::QueryFailed {
            reason: err.to_string(),
        },
    };

    SiriusError::Database(db_err)
}

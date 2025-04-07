//! List of extractors used to access databases in endpoints of the application.

mod postgres;
mod redis;

pub use postgres::DbPool;

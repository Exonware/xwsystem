// #exonware/xwsystem/rust/src/query/contracts.rs
//! #exonware/xwsystem/src/exonware/xwsystem/query/contracts.py
//! 
//! Query contracts (foundation layer).
//! 
//! These interfaces live in xwsystem to avoid circular dependencies between:
//! - xwnode (data structures / node facade)
//! - xwquery (query engine)
//! - xwdata (data facade built on xwnode + xwquery)
//! 
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 28-Dec-2025


/// Query result type alias (can be any value).
pub type QueryResult = serde_json::Value;

/// Interface for query execution providers.
///
/// Implementations live in higher-level packages (e.g., xwquery) and register
/// themselves via `exonware.xwsystem.query.registry`.
pub trait IQueryProvider: Send + Sync {
    /// Get the provider ID.
    fn provider_id(&self) -> &str;

    /// Execute a query against data.
    fn execute(
        &self,
        query: &str,
        data: serde_json::Value,
        format: Option<&str>,
        auto_detect: bool,
    ) -> QueryResult;
}

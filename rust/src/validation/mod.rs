// #exonware/xwsystem/rust/src/validation/mod.rs
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 04, 2025
//! 
//! XSystem Validation Package
//! 
//! Declarative validation with type hints, automatic coercion, and Pydantic-style models.

pub mod contracts;
pub mod declarative;
pub mod schema_discovery;
pub mod type_safety;

pub use contracts::{ISchemaValidator};

pub use declarative::{Field, ValidationError, XModel};

pub use schema_discovery::{DEFAULT_SCHEMA_VALIDATOR_ENTRYPOINT_GROUP, SchemaValidatorDiscoveryResult, available_schema_validators, discover_schema_validators, get_schema_validator, set_schema_validator};

pub use type_safety::{validate_untrusted_data};

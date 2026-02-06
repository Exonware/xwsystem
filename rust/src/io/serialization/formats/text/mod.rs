// #exonware/xwsystem/rust/src/io/serialization/formats/text/mod.rs
//! Text-based serialization formats.

pub mod configparser;
pub mod csv;
pub mod formdata;
pub mod json;
pub mod json5;
pub mod jsonlines;
pub mod multipart;
pub mod toml;
pub mod xml;
pub mod yaml;

pub use configparser::{ConfigParserSerializer};

pub use csv::{CsvSerializer};

pub use formdata::{FormDataSerializer};

pub use json::{JsonSerializer};

pub use json5::{Json5Serializer};

pub use jsonlines::{JsonLinesSerializer};

pub use multipart::{MultipartSerializer};

pub use toml::{TomlSerializer};

pub use xml::{XmlSerializer};

pub use yaml::{YamlSerializer};

// #exonware/xwsystem/rust/src/io/data_operations.rs
//! #exonware/xwsystem/src/exonware/xwsystem/io/data_operations.py
//! 
//! Generic data-operations layer for large, file-backed datasets.
//! 
//! This module provides:
//! - A small indexing model for line-oriented files (e.g. NDJSON / JSONL)
//! - Streaming read / update helpers with atomic guarantees
//! - Paging helpers built on top of line offsets
//! 
//! The goal is to expose these capabilities in a format-agnostic way so that
//! higher-level libraries (xwdata, xwnode, xwentity, etc.) can build powerful
//! lazy, paged, and atomic access features without re-implementing I/O logic.
//! 
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 15-Dec-2025


use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::serialization::auto_serializer::AutoSerializer;
use crate::serialization::parsers::registry::get_best_available_parser;
use std::path::Path as StdPath;

/// Type alias for JSON match function: takes a JSON value and returns bool
pub type JsonMatchFn = Box<dyn Fn(&serde_json::Value) -> bool>;

/// Type alias for JSON update function: takes a JSON value and returns updated JSON value
pub type JsonUpdateFn = Box<dyn Fn(serde_json::Value) -> serde_json::Value>;

/// Minimal metadata for a JSONL/NDJSON index.
///
/// This intentionally mirrors the capabilities used in the x5 examples
/// without pulling in any of the example code directly.
#[derive(Debug, Clone)]
pub struct JsonIndexMeta {
    pub path: String,
    pub size: i64,
    pub mtime: f64,
    pub version: i64,
}

impl JsonIndexMeta {
    pub fn new(path: String, size: i64, mtime: f64) -> Self {
        Self {
            path,
            size,
            mtime,
            version: 1,
        }
    }
}

/// Simple index for line-oriented JSON files.
///
/// - line_offsets: byte offset of each JSON line
/// - id_index: optional mapping id_value -> line_number
#[derive(Debug, Clone)]
pub struct JsonIndex {
    pub meta: JsonIndexMeta,
    pub line_offsets: Vec<i64>,
    pub id_index: Option<HashMap<String, i64>>,
}

impl JsonIndex {
    pub fn new(meta: JsonIndexMeta, line_offsets: Vec<i64>, id_index: Option<HashMap<String, i64>>) -> Self {
        Self {
            meta,
            line_offsets,
            id_index,
        }
    }
}

/// Abstract, format-agnostic interface for large, file-backed data operations.
///
/// Concrete implementations may target specific physical layouts
/// (NDJSON/JSONL, multi-document YAML, binary record stores, etc.), but MUST
/// conform to these semantics:
///
/// - Streaming, record-by-record read with a match predicate.
/// - Streaming, atomic update using a temp file + replace pattern.
/// - Optional indexing for random access and paging.
pub trait ADataOperations {
    /// Return the first record (or sub-path) that matches the predicate.
    fn stream_read(&self, file_path: String, match: JsonMatchFn, path: Option<Vec<String>>, encoding: String) -> serde_json::Value;

    /// Stream-copy the backing store, applying `updater` to matching records.
    /// MUST use atomic replace semantics when `atomic=True`.
    /// Returns number of updated records.
    fn stream_update(&self, file_path: String, match: JsonMatchFn, updater: JsonUpdateFn) -> i64;

    /// Build an index structure suitable for random access and paging.
    fn build_index(&self, file_path: String) -> JsonIndex;

    /// Random-access a specific logical record by its index position.
    fn indexed_get_by_line(&self, file_path: String, line_number: i64) -> serde_json::Value;

    /// Random-access a record by logical identifier, with optional index.
    fn indexed_get_by_id(&self, file_path: String, id_value: serde_json::Value) -> serde_json::Value;

    /// Return a page of logical records using an index for efficiency.
    fn get_page(&self, file_path: String, page_number: i64, page_size: i64) -> Vec<serde_json::Value>;

}

// Reuse xwsystem's AutoSerializer so we do not re-implement parsing.
// ------------------------------------------------------------------
// ------------------------------------------------------------------
// ------------------------------------------------------------------
// Streaming update with atomic replace
// ------------------------------------------------------------------
// Write to a temp file in the same directory for atomic replace.
// Ensure temp file is removed on error
// Best-effort cleanup; do not mask original error.
// ------------------------------------------------------------------
// ------------------------------------------------------------------
// Auto-detect parallel based on config
// Use config defaults for workers and chunk size
// Only use default if not explicitly set
// Use parallel processing if enabled and file is large enough
// Fall through to single-threaded
// Single-threaded implementation (optimized - matches example code exactly)
// Cache parser instance (matches example code pattern)
// Match example code exactly: strip bytes, parse directly
// Parser accepts bytes directly (hybrid parser handles it)
// Index should be best-effort and robust to bad lines.
// Skip invalid lines silently for performance
// Optimize: Simple formula - 1 worker per 10MB (capped at ProcessPoolExecutor limit)
// ProcessPoolExecutor max_workers limit is 61 on Windows
// Cap at 61 (ProcessPoolExecutor limit) or CPU count, whichever is higher
// If file is too small, fall back to single-threaded
// Split file into chunks
// Limit number of chunks
// Process chunks in parallel
// Prepare arguments for worker processes
// Execute parallel processing
// Optimize: Use dict for O(1) lookup instead of sorting
// Merge results (process in order by chunk_id)
// Optimize: Pre-calculate total size for better memory allocation
// Pre-allocate list for better performance
// Merge line_offsets if building them
// Optimize: Use slice assignment for faster extend
// Calculate base_line for id_index even without line_offsets
// Estimate: assume average line size if we don't have offsets
// Optimize: Batch update with dict.update() if no limit
// Fast path: no limit, use dict comprehension + update
// Slower path: check limit per item
// Fallback: linear scan using stream_read semantics
// ------------------------------------------------------------------
// ------------------------------------------------------------------
/// Generic data-operations helper for NDJSON / JSONL style files.
///
/// This class is deliberately low-level and works directly with paths and
/// native Python data. XWData and other libraries can wrap it to provide
/// higher-level, type-agnostic facades.
pub struct NDJSONDataOperations {
    pub serializer: Option<AutoSerializer>,
}

impl ADataOperations for NDJSONDataOperations {
    fn stream_read(&self, file_path: String, match_fn: JsonMatchFn, path: Option<Vec<String>>, encoding: String) -> serde_json::Value {
        self.stream_read_impl(&file_path, match_fn, path, &encoding)
    }

    fn stream_update(&self, file_path: String, match_fn: JsonMatchFn, updater: JsonUpdateFn) -> i64 {
        self.stream_update_impl(&file_path, match_fn, updater)
    }

    fn build_index(&self, file_path: String) -> JsonIndex {
        self.build_index_impl(&file_path)
    }

    fn indexed_get_by_line(&self, file_path: String, line_number: i64) -> serde_json::Value {
        self.indexed_get_by_line_impl(&file_path, line_number)
    }

    fn indexed_get_by_id(&self, file_path: String, id_value: serde_json::Value) -> serde_json::Value {
        self.indexed_get_by_id_impl(&file_path, id_value)
    }

    fn get_page(&self, file_path: String, page_number: i64, page_size: i64) -> Vec<serde_json::Value> {
        self.get_page_impl(&file_path, page_number, page_size)
    }
}

impl NDJSONDataOperations {
    /// Constructor
    pub fn new(serializer: Option<AutoSerializer>) -> Self {
        Self {
            serializer,
        }
    }

    /// Stream a huge NDJSON file and return the first record (or sub-path)
    /// matching `match`.
    ///
    /// This is intentionally simple and focused:
    /// - Reads one line at a time
    /// - Uses AutoSerializer(JSON) for parsing
    /// - Optional path extraction
    fn stream_read_impl(&self, file_path: &str, match_fn: JsonMatchFn, path: Option<Vec<String>>, encoding: &str) -> serde_json::Value {
        let path_obj = Path::new(file_path);
        if !path_obj.exists() {
            return serde_json::Value::Null;
        }

        let file = fs::File::open(path_obj).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.unwrap_or_default();
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            // Parse JSON line
            if let Ok(obj) = serde_json::from_str::<serde_json::Value>(trimmed) {
                if match_fn(&obj) {
                    return self._extract_path(obj, path);
                }
            }
        }

        serde_json::Value::Null
    }

    /// Stream-copy a huge NDJSON file, applying `updater` to records
    /// where `match(obj)` is True.
    ///
    /// Only matching records are fully materialized. All writes go to a
    /// temporary file, which is atomically replaced on success.
    ///
    /// Returns the number of updated records.
    fn stream_update_impl(&self, file_path: &str, match_fn: JsonMatchFn, updater: JsonUpdateFn) -> i64 {
        let path_obj = Path::new(file_path);
        if !path_obj.exists() {
            return 0;
        }

        let parent = path_obj.parent().unwrap_or(Path::new("."));
        let temp_path = parent.join(format!(".{}.tmp", path_obj.file_name().unwrap().to_string_lossy()));

        let mut updated = 0i64;
        
        // Read from source, write to temp
        {
            let source_file = fs::File::open(path_obj).unwrap();
            let reader = BufReader::new(source_file);
            let mut temp_file = fs::File::create(&temp_path).unwrap();

            for line in reader.lines() {
                let line = line.unwrap_or_default();
                let trimmed = line.trim();
                
                if trimmed.is_empty() {
                    writeln!(temp_file, "").unwrap();
                    continue;
                }

                if let Ok(mut obj) = serde_json::from_str::<serde_json::Value>(trimmed) {
                    if match_fn(&obj) {
                        obj = updater(obj);
                        let updated_line = serde_json::to_string(&obj).unwrap();
                        writeln!(temp_file, "{}", updated_line).unwrap();
                        updated += 1;
                    } else {
                        writeln!(temp_file, "{}", line).unwrap();
                    }
                } else {
                    writeln!(temp_file, "{}", line).unwrap();
                }
            }
        }

        // Atomic replace
        fs::rename(&temp_path, path_obj).unwrap();
        updated
    }

    // Auto-detect parallel based on config
    // Use config defaults for workers and chunk size
    // Only use default if not explicitly set
    // Use parallel processing if enabled and file is large enough
    // Fall through to single-threaded
    // Single-threaded implementation (optimized - matches example code exactly)
    // Cache parser instance (matches example code pattern)
    // Match example code exactly: strip bytes, parse directly
    // Parser accepts bytes directly (hybrid parser handles it)
    // Index should be best-effort and robust to bad lines.
    // Skip invalid lines silently for performance
    /// One-time full scan to build an index:
    /// - line_offsets: byte offset of each JSON line
    /// - optional id_index: obj[id_field] -> line_number
    fn build_index_impl(&self, file_path: &str) -> JsonIndex {
        let path_obj = Path::new(file_path);
        if !path_obj.exists() {
            let meta = JsonIndexMeta::new(file_path.to_string(), 0, 0.0);
            return JsonIndex::new(meta, Vec::new(), None);
        }

        let metadata = fs::metadata(path_obj).unwrap();
        let size = metadata.len() as i64;
        let mtime = metadata.modified()
            .unwrap()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        let meta = JsonIndexMeta::new(file_path.to_string(), size, mtime);
        let mut line_offsets = Vec::new();
        let mut id_index: HashMap<String, i64> = HashMap::new();

        let file = fs::File::open(path_obj).unwrap();
        let reader = BufReader::new(file);
        let mut offset = 0i64;

        for line in reader.lines() {
            let line = line.unwrap_or_default();
            let line_len = line.len() as i64 + 1; // +1 for newline
            
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                line_offsets.push(offset);
                
                if let Ok(obj) = serde_json::from_str::<serde_json::Value>(trimmed) {
                    if let Some(id_val) = obj.get("id").and_then(|v| v.as_str()) {
                        id_index.insert(id_val.to_string(), line_offsets.len() as i64 - 1);
                    }
                }
            }
            
            offset += line_len;
        }

        JsonIndex::new(meta, line_offsets, Some(id_index))
    }

    // Optimize: Simple formula - 1 worker per 10MB (capped at ProcessPoolExecutor limit)
    // ProcessPoolExecutor max_workers limit is 61 on Windows
    // Cap at 61 (ProcessPoolExecutor limit) or CPU count, whichever is higher
    // If file is too small, fall back to single-threaded
    // Split file into chunks
    // Limit number of chunks
    // Process chunks in parallel
    // Prepare arguments for worker processes
    // Execute parallel processing
    // Optimize: Use dict for O(1) lookup instead of sorting
    // Merge results (process in order by chunk_id)
    // Optimize: Pre-calculate total size for better memory allocation
    // Pre-allocate list for better performance
    // Merge line_offsets if building them
    // Optimize: Use slice assignment for faster extend
    // Calculate base_line for id_index even without line_offsets
    // Estimate: assume average line size if we don't have offsets
    // Optimize: Batch update with dict.update() if no limit
    // Fast path: no limit, use dict comprehension + update
    // Slower path: check limit per item
    /// Parallel index building using multiple CPU cores.
    ///
    /// This is an internal method called by build_index() when use_parallel=True.
    pub fn _build_index_parallel(&self, file_path: Path) -> JsonIndex
    {
        // TODO: This function uses external dependencies.
        // The original Python implementation should be maintained/adapted.
        //
        // Python → Rust crate mappings:
        //   multiprocessing → (no known Rust equivalent)
        //
        // Add these crates to Cargo.toml if implementing:
        todo!()
    }

    /// Random-access a specific record by line_number (0-based) using index.
    fn indexed_get_by_line_impl(&self, file_path: &str, line_number: i64) -> serde_json::Value {
        let index = self.build_index_impl(file_path);
        if line_number < 0 || line_number >= index.line_offsets.len() as i64 {
            return serde_json::Value::Null;
        }

        let offset = index.line_offsets[line_number as usize];
        let mut file = fs::File::open(file_path).unwrap();
        file.seek(SeekFrom::Start(offset as u64)).unwrap();
        
        let mut reader = BufReader::new(file);
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        
        serde_json::from_str::<serde_json::Value>(line.trim()).unwrap_or(serde_json::Value::Null)
    }

    /// Random-access a record by logical id using id_index if available.
    /// Falls back to linear scan if id_index missing or incomplete.
    fn indexed_get_by_id_impl(&self, file_path: &str, id_value: serde_json::Value) -> serde_json::Value {
        let index = self.build_index_impl(file_path);
        
        if let Some(id_str) = id_value.as_str() {
            if let Some(id_index) = &index.id_index {
                if let Some(&line_num) = id_index.get(id_str) {
                    return self.indexed_get_by_line_impl(file_path, line_num);
                }
            }
        }

        // Fallback: linear scan
        let match_fn: JsonMatchFn = Box::new(move |obj| {
            obj.get("id") == Some(&id_value)
        });
        self.stream_read_impl(file_path, match_fn, None, "utf-8")
    }

    /// Paging helper using index:
    /// - page_number: 1-based
    /// - page_size: number of records per page
    fn get_page_impl(&self, file_path: &str, page_number: i64, page_size: i64) -> Vec<serde_json::Value> {
        let index = self.build_index_impl(file_path);
        let start_idx = (page_number - 1) * page_size;
        let end_idx = start_idx + page_size;

        let mut results = Vec::new();
        for i in start_idx..end_idx.min(index.line_offsets.len() as i64) {
            if let Ok(value) = self.indexed_get_by_line_impl(file_path, i) {
                if value != serde_json::Value::Null {
                    results.push(value);
                }
            }
        }
        results
    }

    /// Extract a nested path like ['user', 'email'] or ['tags', 0].
    fn _extract_path(&self, mut obj: serde_json::Value, path: Option<Vec<String>>) -> serde_json::Value {
        if let Some(path_vec) = path {
            for key in path_vec {
                if let Some(map) = obj.as_object_mut() {
                    obj = map.get(&key).cloned().unwrap_or(serde_json::Value::Null);
                } else if let Some(arr) = obj.as_array_mut() {
                    if let Ok(idx) = key.parse::<usize>() {
                        obj = arr.get(idx).cloned().unwrap_or(serde_json::Value::Null);
                    } else {
                        return serde_json::Value::Null;
                    }
                } else {
                    return serde_json::Value::Null;
                }
            }
        }
        obj
    }

}

    // Import parser in worker process (can't pickle serializer)
    // Skip if we've gone past the end
    // Optimize: Check empty lines early (match example code pattern)
    // Track line offset if requested, calculate line_idx once
    // Parser accepts bytes directly (hybrid parser handles it)
    // Skip invalid lines (best-effort indexing)
    // Can't use logger in worker process, just pass
    /// Process a single chunk (runs in worker process).
    ///
    /// This is a module-level function to make it picklable for multiprocessing.
    pub fn _process_chunk_worker(args: (i64, i64, i64, String, String, String, String, bool)) -> (String, HashMap<String, i64>, i64)
    {
        // TODO: Implement
        todo!()
    }


// =============================================================================
// EXPORT ALL (from __all__)
// =============================================================================
pub use JsonIndexMeta;
pub use JsonIndex;
pub use ADataOperations;
pub use NDJSONDataOperations;

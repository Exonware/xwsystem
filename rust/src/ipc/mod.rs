// #exonware/xwsystem/rust/src/ipc/mod.rs
//! Inter-Process Communication (IPC) Module
//! ========================================
//! 
//! Production-grade IPC utilities for XSystem.
//! 
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Company: eXonware.com
//! Generated: 2025-01-27
//! 
//! This module provides:
//! - Process management and communication
//! - Shared memory abstractions
//! - Message queues and pipes
//! - Process pools with monitoring
//! - Cross-platform IPC primitives

pub mod async_fabric;
pub mod message_queue;
pub mod pipes;
pub mod process_manager;
pub mod process_pool;
pub mod shared_memory;

pub use async_fabric::{AsyncProcessFabric};

pub use message_queue::{AsyncMessageQueue, MessageQueue};

pub use pipes::{AsyncPipe, Pipe};

pub use process_manager::{ProcessInfo, ProcessManager};

pub use process_pool::{AsyncProcessPool, ProcessPool};

pub use shared_memory::{SharedData, SharedMemoryManager};

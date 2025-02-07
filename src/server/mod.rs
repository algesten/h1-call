#![allow(missing_docs)]
//!
//!
//!
//! ```text
//!        ┌──────────────────┐
//!     ┌──│   RecvRequest    │───────────────┐
//!     │  └──────────────────┘               │
//!     │            │                        │
//!     │            │                        │
//!     │            ▼                        ▼
//!     │  ┌──────────────────┐     ┌──────────────────┐
//!     │  │     RecvBody     │◀────│     Send100      │
//!     │  └──────────────────┘     └──────────────────┘
//!     │            │                        │
//!     │            │                        │
//!     │            ▼                        │
//!     └─▶┌──────────────────┐◀──────────────┘
//!        │   SendResponse   │──┐
//!        └──────────────────┘  │
//!                  │           │
//!                  │           │
//!                  ▼           │
//!        ┌──────────────────┐  │
//!        │     SendBody     │  │
//!        └──────────────────┘  │
//!                  │           │
//!                  │           │
//!                  ▼           │
//!        ┌──────────────────┐  │
//!        │     Cleanup      │◀─┘
//!        └──────────────────┘
//! ```

/// Max number of headers to parse from an HTTP requst
pub const MAX_REQUEST_HEADERS: usize = 128;

mod amended;
mod call;
mod flow;
mod holder;

pub use flow::*;

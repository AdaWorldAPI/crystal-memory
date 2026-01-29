//! Crystal: 3D quorum field and 4KB holographic compression.
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                    CRYSTAL ARCHITECTURE                      │
//! ├─────────────────────────────────────────────────────────────┤
//! │                                                             │
//! │   QuorumField                                               │
//! │   ├── 5×5×5 × 10Kbit = 156KB                               │
//! │   ├── Quorum voting (neighbor consensus)                   │
//! │   └── 2^1,250,000 configuration space                      │
//! │                                                             │
//! │   Crystal4K                                                 │
//! │   ├── 3 × 10Kbit projections = 4KB                         │
//! │   ├── Holographic boundary encoding                        │
//! │   └── 41:1 compression ratio                               │
//! │                                                             │
//! │   CrystalMemory                                             │
//! │   ├── 43K crystals × 4KB = 170MB                           │
//! │   ├── Inference via settle-into-attractor                  │
//! │   └── Learning via landscape sculpting                     │
//! │                                                             │
//! └─────────────────────────────────────────────────────────────┘
//! ```
//!
//! # Example
//!
//! ```rust
//! use ladybug::crystal::{QuorumField, Crystal4K, CrystalMemory};
//! use ladybug::Fingerprint;
//!
//! // Create a quorum field
//! let mut field = QuorumField::new(4); // 4/6 majority threshold
//!
//! // Inject a pattern
//! let pattern = Fingerprint::from_content("my concept");
//! field.inject(&pattern);
//!
//! // Let it settle into attractor
//! let (steps, converged) = field.settle(100);
//!
//! // Compress to 4KB crystal
//! let crystal = Crystal4K::from_field(&field);
//!
//! // Store in memory
//! let mut memory = CrystalMemory::new();
//! memory.add(crystal.clone());
//!
//! // Inference: query → settled attractor
//! let result = memory.infer(&crystal);
//! ```

mod field;
mod crystal4k;
mod memory;

pub use field::{QuorumField, FIELD_SIZE, FIELD_CELLS};
pub use crystal4k::Crystal4K;
pub use memory::{CrystalMemory, MemoryStats, DEFAULT_CAPACITY, DEFAULT_SETTLE_STEPS};

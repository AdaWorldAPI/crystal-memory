# Crystal Memory

**4KB Holographic Crystals: 5×5×5 quorum fields compressed to 3×10Kbit**

170MB = 4096 attractor basins navigating 2^1.25M space

## What It Does

Crystal Memory provides **holographic storage** where any fragment can reconstruct the whole:

```
┌─────────────────────────────────────────────────────────────────────────┐
│                     CRYSTAL MEMORY ARCHITECTURE                          │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│   Traditional DB:  Store → Index → Query → Retrieve                     │
│                    O(log N) lookups, index maintenance                  │
│                                                                          │
│   Crystal Memory:  Content → Fingerprint → 5×5×5 Cell → Quorum          │
│                    O(1) resonance, self-healing via redundancy          │
│                                                                          │
│   ┌─────────────────────────────────────────────────────────────────┐   │
│   │   3 copies × 10Kbit = 30Kbit per memory                         │   │
│   │   5×5×5 = 125 cells × 4096 crystals = 170MB total               │   │
│   │   Any 2-of-3 can reconstruct (quorum consensus)                 │   │
│   └─────────────────────────────────────────────────────────────────┘   │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

## Integration with ladybug-rs

Crystal Memory is an **optional extension** for ladybug-rs, providing:

1. **Holographic redundancy** - 2-of-3 quorum for error correction
2. **Attractor basins** - Content naturally clusters into 4096 stable states  
3. **Self-healing** - Corrupted data reconstructs from surviving copies

```rust
// As optional feature in ladybug-rs
use ladybug::extensions::crystal_memory::CrystalStore;

let crystal = CrystalStore::new(4096); // 4096 attractor basins
crystal.store(&fingerprint, &content);
let recovered = crystal.recall(&partial_fingerprint); // Works with noise!
```

## Relation to Crystal Family

| Repo | Purpose | When to Use |
|------|---------|-------------|
| **crystal-memory** | Holographic storage with quorum | Fault-tolerant memory |
| **crystal-savant** | Multi-pass CAM codebook | One-time training, fast lookup |
| **crystal-compress** | Semantic compression | Large context → small index |
| **spo-crystal** | 3D knowledge graph | O(1) triple queries |

## Key Insight

> The crystal doesn't "store" memories—it **becomes** them.  
> Like a hologram, any shard contains the whole.  
> Damage doesn't erase; it degrades gracefully.

## Usage

```bash
cargo add crystal-memory
```

```rust
use crystal_memory::{Crystal, QuorumField};

// Create 5×5×5 quorum field
let field = QuorumField::new();

// Store with 3-way redundancy
field.store_quorum(&key_fingerprint, &value_fingerprint);

// Recall tolerates 1 corrupted copy
let value = field.recall_quorum(&key_fingerprint)?;
```

## License

MIT OR Apache-2.0

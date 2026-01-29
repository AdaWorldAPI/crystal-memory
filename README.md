# 🐞 ladybug-rs

**Unified cognitive database in Rust. SQL + Cypher + Vector + Hamming + NARS + Counterfactuals.**

Built on Lance columnar storage. AGI operations as first-class primitives.

---

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Python / JS / etc                         │
│     db.sql()     db.cypher()     db.resonate()     db.fork()│
└─────────────────────────────┬───────────────────────────────┘
                              │ PyO3 / NAPI
┌─────────────────────────────▼───────────────────────────────┐
│                      ladybug-rs (Rust)                       │
│                                                             │
│  ┌───────────────────┐  ┌───────────────────────────────┐  │
│  │   Conventional    │  │        AGI Operations         │  │
│  │   ─────────────   │  │        ──────────────         │  │
│  │   • SQL           │  │   • resonate (Hamming sim)    │  │
│  │   • Cypher        │  │   • traverse (graph paths)    │  │
│  │   • Vector ANN    │  │   • fork + what_if            │  │
│  │   • CRUD          │  │   • NARS inference            │  │
│  └───────────────────┘  └───────────────────────────────┘  │
│                              │                              │
│  ┌───────────────────────────▼───────────────────────────┐ │
│  │                    Core Primitives                     │ │
│  │  VSA Ops (bind/bundle)  │  SIMD Hamming  │  NARS      │ │
│  └───────────────────────────┬───────────────────────────┘ │
│                              │                              │
│  ┌───────────────────────────▼───────────────────────────┐ │
│  │               Lance Columnar Storage                   │ │
│  └───────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
```

---

## Quick Start

### Rust

```rust
use ladybug::{Database, Thought, TruthValue, Deduction};

// Open database
let db = Database::open("./mydb")?;

// SQL
let results = db.sql("SELECT * FROM thoughts WHERE confidence > 0.7")?;

// Cypher (transpiled to SQL)
let results = db.cypher("MATCH (a)-[:CAUSES]->(b) RETURN b")?;

// Resonance search (Hamming similarity)
let similar = db.resonate_content("quantum physics", 0.7, 10);

// NARS inference
let premise1 = TruthValue::new(0.9, 0.9);
let premise2 = TruthValue::new(0.8, 0.8);
let conclusion = premise1.deduction(&premise2);

// Counterfactual reasoning
let what_if = db.fork()
    .apply(Change::Remove("feature_flag".into()))
    .propagate()
    .diff();
```

### Python

```python
import ladybug

# Open database
db = ladybug.open("./mydb")

# SQL
results = db.sql("SELECT * FROM thoughts")

# Resonance search
similar = db.resonate("quantum physics", threshold=0.7, limit=10)

# NARS inference
truth1 = ladybug.TruthValue(frequency=0.9, confidence=0.9)
truth2 = ladybug.TruthValue(frequency=0.8, confidence=0.8)
conclusion = truth1.deduction(truth2)
print(conclusion)  # ⟨72%, 65%⟩

# Fingerprint operations
fp1 = ladybug.Fingerprint("hello")
fp2 = ladybug.Fingerprint("world")
print(fp1.similarity(fp2))  # ~0.5 (random baseline)

# Bind (VSA composition)
red_apple = ladybug.Fingerprint("red").bind(ladybug.Fingerprint("apple"))
```

---

## Features

### 🔍 Unified Query Engine

| Query Type | Syntax | Backend |
|------------|--------|---------|
| SQL | `db.sql("SELECT ...")` | DataFusion |
| Cypher | `db.cypher("MATCH ...")` | Transpiled → SQL |
| Vector | `db.vector_search(emb, k)` | Lance ANN |
| Hamming | `db.resonate(fp, threshold)` | SIMD |

### 🧠 NARS Reasoning

```rust
// Truth values: <frequency, confidence>
let birds_fly = TruthValue::from_evidence(positive: 9.0, negative: 1.0);
let tweety_bird = TruthValue::certain_true();

// Deduction: birds fly + tweety is bird → tweety flies
let tweety_flies = birds_fly.deduction(&tweety_bird);

// Revision: combine independent evidence
let combined = evidence1.revision(&evidence2);
```

Supported inference rules:
- **Deduction**: A→B, B→C ⊢ A→C
- **Induction**: A→B, A→C ⊢ B→C  
- **Abduction**: A→B, C→B ⊢ A→C
- **Analogy**: A→B, A↔C ⊢ C→B

### 🌐 VSA Operations

```rust
// Bind: create compound representation
let red_apple = color_red.bind(&object_apple);

// Unbind: recover component
let recovered = red_apple.unbind(&color_red);  // ≈ object_apple

// Bundle: create prototype from examples
let cat_prototype = Fingerprint::bundle(&[cat1, cat2, cat3]);

// Sequence: encode ordered items
let sentence = Fingerprint::sequence(&[word1, word2, word3]);
```

### ⚡ SIMD Hamming

AVX-512/AVX2/NEON accelerated:

| Corpus | Latency | Throughput |
|--------|---------|------------|
| 10K | 150μs | 65M cmp/sec |
| 100K | 1.5ms | 65M cmp/sec |
| 1M | 15ms | 65M cmp/sec |

### 🔀 Counterfactual Reasoning

```rust
// Fork world for "what if" analysis
let alternate = db.fork()
    .apply(Change::Remove("config_flag".into()))
    .propagate();

// See what changed
let diff = alternate.diff(&db);
println!("Affected: {:?}", diff.affected_nodes);
println!("Broken chains: {:?}", diff.broken_chains);
```

---

## Installation

### From crates.io (coming soon)

```bash
cargo add ladybug
```

### From source

```bash
git clone https://github.com/AdaWorldAPI/ladybug-rs
cd ladybug-rs
cargo build --release
```

### Python bindings

```bash
pip install ladybug
# or
maturin develop --features python
```

---

## Performance Targets

| Operation | Target |
|-----------|--------|
| Single Hamming | 20ns |
| Batch 1M | 15ms |
| NARS inference | 50ns |
| World fork | 1μs (COW) |
| SQL simple | 1ms |
| Cypher 5-hop | 5ms |

---

## Project Structure

```
ladybug-rs/
├── src/
│   ├── lib.rs           # Crate entry point
│   ├── core/            # Fingerprints, SIMD, VSA
│   ├── nars/            # Truth values, inference
│   ├── cognitive/       # Thought, Concept, Style
│   ├── graph/           # Edges, traversal
│   ├── world/           # Counterfactuals
│   ├── query/           # SQL/Cypher
│   ├── storage/         # Lance integration
│   └── python/          # PyO3 bindings
└── Cargo.toml
```

---

## Related

- [LadybugDB](https://github.com/AdaWorldAPI/ladybugdb) - Python prototype
- [LanceDB](https://lancedb.com/) - Storage foundation
- [OpenNARS](https://github.com/opennars/opennars) - NARS reference

---

## 🔮 Crystal Memory: 4KB Holographic Compression

Compress 156KB quorum fields into 4KB crystals via axis projections.

```
┌─────────────────────────────────────────────────────────────┐
│                    CRYSTAL ARCHITECTURE                      │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│   QuorumField: 5×5×5 × 10Kbit = 156KB                      │
│   ↓ XOR-fold per axis                                       │
│   Crystal4K: 3 × 10Kbit = 4KB                              │
│                                                             │
│   Compression: 41:1                                         │
│   Configuration space: 2^1,250,000                         │
│   Storage: 170MB → 43,000 attractor basins                 │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### Rust Usage

```rust
use ladybug::crystal::{QuorumField, Crystal4K, CrystalMemory};
use ladybug::Fingerprint;

// Create quorum field (5×5×5 lattice)
let mut field = QuorumField::new(4);  // 4/6 quorum threshold

// Inject pattern and settle into attractor
let pattern = Fingerprint::from_content("my concept");
field.inject(&pattern);
let (steps, converged) = field.settle(100);

// Compress to 4KB crystal
let crystal = Crystal4K::from_field(&field);

// Build memory with 43K crystals (170MB)
let mut memory = CrystalMemory::new();
memory.add(crystal.clone());

// Inference: query → settled attractor
let result = memory.infer(&crystal);

// Learning: sculpt attractor toward target
memory.learn(&input_crystal, &target_crystal, 0.1);
```

### The Paradigm Shift

| Traditional | Crystal Memory |
|-------------|----------------|
| Store weights | Shape landscape |
| Compute f(x) | Settle into basin |
| Backprop gradients | Hebbian sculpting |
| O(params) inference | O(ticks) inference |

The knowledge isn't stored. It's **shaped**.

---

## License

Apache 2.0

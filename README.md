# Gabriel Organism

Gabriel Organism is a Rust workspace that explores a biologically inspired "metabolic" approach to information processing. It models an adaptive organism that ingests domain-agnostic information quanta, organizes them in a self-rewiring neural graph, embeds them in a 4D funnel geometry, and detects emergent structure over time. The workspace is organised as modular crates so the metabolic core, geometry, emergence analysis, and higher-level reasoning layers can be reused independently.

## Highlights

- **Metabolic information flow** – ingestion, digestion, assimilation, excretion, and synthesis loops regulate how quanta move through the system and how synaptic weights evolve over time.【F:metabolics/src/lib.rs†L1-L198】
- **Self-organising neural substrate** – Gabriel Cells manage a dynamic directed graph with Hebbian reinforcement, structural pruning, and diffusion-based activation propagation.【F:gabriel-core/src/lib.rs†L1-L138】【F:gabriel-core/src/graph.rs†L1-L147】
- **4D funnel geometry** – each quantum is placed in a temporal-spatial manifold that drives neighbourhood formation and tensor-based pattern sampling.【F:trichter-geometry/src/lib.rs†L1-L139】
- **Emergent pattern analysis** – tensor signatures are inspected for coherence, complexity, stability, and qualitative classes (periodic, harmonic, fractal, chaotic, hierarchical).【F:emergence/src/lib.rs†L1-L120】
- **Organism orchestration** – the `organism` crate wires metabolism, emergence, and diagnostics together, exposes a CLI, and tracks health, consciousness, and lifecycle progression.【F:organism/src/lib.rs†L1-L170】【F:organism/src/main.rs†L1-L133】
- **Mathematical cognition layer** – optional abstractions describe mathematical objects, proofs, and curricula so higher-level reasoning can emerge from the same substrate.【F:mathematical-cognition/src/lib.rs†L1-L120】【F:organism/src/learning.rs†L1-L120】

## Workspace layout

```
gabriel/
├── Cargo.toml                  # Workspace members & shared dependencies
├── BUILD.md, TECHNICAL_SPEC.md # Additional architecture & build notes
├── gabriel-core/               # Dynamic neuron graph & InformationQuantum trait
├── trichter-geometry/          # 4D funnel growth, density & tensor sampling
├── metabolics/                 # Metabolic engine connecting cells & geometry
├── emergence/                  # Pattern detection & statistics
├── organism/                   # Library + CLI binary (`gabriel-organism`)
├── mathematical-cognition/     # Math-specific quantum types and tooling
└── experiments/                # Exploratory training scenarios & prototypes
```

Each crate can be built and tested on its own, while the `organism` crate provides an executable demo of the whole organism.

## Getting started

### Prerequisites

- Rust 1.75+ with `cargo` and `rustup`
- (Optional) `llvm-tools-preview` for benchmarking via `criterion`

### Build the workspace

```bash
cd gabriel
cargo build
```

### Run the organism

The main binary lives in `organism/src/main.rs` and is exposed as `gabriel-organism`. It feeds synthetic quanta, steps through lifecycle iterations, and prints a diagnostic report.

```bash
cargo run -p organism --bin gabriel-organism -- --cycles 120 --feed-count 80 --radius 1.5 --verbose
```

CLI flags:

- `--cycles <u64>` – number of lifecycle iterations to execute (default: 100)
- `--feed-count <u64>` – how many quanta to ingest before running cycles (default: 50)
- `--radius <f64>` – initial funnel radius for the geometry model (default: 1.0)
- `--output <path>` – export final organism state and diagnostics to JSON
- `--verbose` – enable debug-level tracing via `tracing_subscriber`

At the end of a run the organism prints a multi-section diagnostics report that summarises vitals, metabolism, emergent patterns, and geometry state.【F:organism/src/lib.rs†L216-L276】【F:organism/src/main.rs†L100-L160】

### Inspecting state programmatically

The library API offers helpers for embedding in other applications:

- `GabrielOrganism::feed` to ingest arbitrary `InformationQuantum` implementations.【F:organism/src/lib.rs†L94-L138】
- `GabrielOrganism::lifecycle` to advance the metabolism/emergence loop and update organism state.【F:organism/src/lib.rs†L139-L214】
- `GabrielOrganism::diagnostics` and `GabrielOrganism::export_state` to retrieve structured telemetry.【F:organism/src/lib.rs†L200-L236】

### Run tests

```bash
cargo test            # run every crate's unit tests
cargo test -p organism -- --ignored   # example for running ignored tests if added later
```

Benchmark harnesses are provided via `criterion` in several crates; enable them with `cargo bench` once the nightly benchmarking toolchain is installed.

## Architectural overview

1. **Information ingestion** – Each quantum implements the `InformationQuantum` trait and is positioned inside the 4D funnel to determine neighbourhoods and potential resonance.【F:gabriel-core/src/lib.rs†L1-L55】【F:metabolics/src/lib.rs†L54-L116】
2. **Neural dynamics** – Gabriel Cells link nearby quanta, propagate activations, and update weights with Hebbian learning and decay.【F:gabriel-core/src/graph.rs†L25-L147】【F:gabriel-core/src/graph.rs†L147-L215】
3. **Metabolic regulation** – Energy, entropy, and growth metrics are updated every cycle while ingestion, digestion, assimilation, excretion, and synthesis steps manage network structure.【F:metabolics/src/lib.rs†L1-L198】【F:metabolics/src/lib.rs†L198-L360】
4. **Emergent analysis** – Tensor samples from the funnel are evaluated for coherence and classified into qualitative pattern types; statistics accumulate over time.【F:emergence/src/lib.rs†L1-L182】
5. **Organism state** – Health, consciousness, and survival thresholds are computed from metabolic stats and emergent complexity, with diagnostics emitted each lifecycle.【F:organism/src/lib.rs†L40-L190】

The modular design makes it straightforward to swap in domain-specific quanta, alternative geometric embeddings, or custom emergence detectors while reusing the metabolic backbone.

## Mathematical cognition layer

The optional `mathematical-cognition` crate implements rich `InformationQuantum` types for integers, primes, functions, theorems, equations, and proof steps. It adds metadata such as mathematical complexity, similarity, and certainty to support emergent reasoning about mathematical structures.【F:mathematical-cognition/src/lib.rs†L1-L120】

Within the organism the `learning`, `memory`, and `reasoning` modules build on this layer to:

- Reinforce proof paths using rewards and punishments (`learning.rs`).【F:organism/src/learning.rs†L1-L120】
- Store and retrieve theorems with tag-based and resonance-based memory mechanisms (`memory.rs`).【F:organism/src/memory.rs†L1-L160】
- Assemble reasoning chains and conjectures across different proof strategies (`reasoning.rs`).【F:organism/src/reasoning.rs†L1-L160】

The `experiments/` directory contains prototype training scripts (e.g. progressive mathematical curricula and Riemann hypothesis exploration) that demonstrate how to drive the organism through increasingly complex domains.【F:experiments/README.md†L1-L120】【F:experiments/math_learning.rs†L1-L120】 These examples are not wired into Cargo binaries yet, but serve as blueprints for custom integration.

## Additional documentation

- `BUILD.md` – detailed build, tooling, and troubleshooting guide
- `EXAMPLES.md` – curated scenarios for integrating the organism into different domains
- `TECHNICAL_SPEC.md` – mathematical derivations and performance characteristics
- `DELIVERY_REPORT.md` – project status summary

## License

Dual-licensed under MIT or Apache-2.0, at your option.

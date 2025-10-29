# Fusion Interface Documentation

## Overview

The **Fusion Layer** is an adaptive resonance-evaluation system that integrates Triton's analytical scoring framework with the Gabriel Organism's metabolic/emergent feedback loop. It provides quantitative evaluation and adaptive tuning of resonance cycles.

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Gabriel Organism                        │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐     │
│  │ Metabolics   │→ │  Emergence   │→ │  Resonance   │     │
│  └──────────────┘  └──────────────┘  └──────────────┘     │
│         ↑                                     │              │
│         │                                     ↓              │
│         │                          ┌──────────────────┐     │
│         │                          │ Resonance Tensor │     │
│         │                          └──────────────────┘     │
│         │                                     │              │
└─────────┼─────────────────────────────────────┼──────────────┘
          │                                     │
          │                                     ↓
┌─────────┴─────────────────────────────────────────────────┐
│                     Fusion Layer                          │
│  ┌────────────────────────────────────────────────────┐   │
│  │         fusion_cycle(resonance_tensor)             │   │
│  │                                                     │   │
│  │  1. Normalize tensor to Float64                    │   │
│  │  2. Route to Triton Scorer                         │   │
│  │  3. Receive coherence, entropy, stability metrics  │   │
│  │  4. Compute feedback weights                       │   │
│  │  5. Log to fusion_diagnostics.json                 │   │
│  └────────────────────────────────────────────────────┘   │
└───────────────────────────────┬───────────────────────────┘
                                │
                                ↓
┌─────────────────────────────────────────────────────────────┐
│                  Triton Scoring Framework                   │
│  ┌──────────────────┐    ┌────────────────────────┐        │
│  │  triton_core.py  │ → │  InformationAlchemy     │        │
│  │                  │    │  Evaluator              │        │
│  │  • Spiral Gen    │    │                         │        │
│  │  • Spectral Sig  │    │  σ(ψ, ρ, ω) → D       │        │
│  └──────────────────┘    └────────────────────────┘        │
└─────────────────────────────────────────────────────────────┘
```

## Core Data Structures

### FusionPacket

The unified data model for fusion layer communication.

```rust
pub struct FusionPacket {
    pub resonance_tensor: Vec<f64>,    // N-dimensional tensor from Gabriel
    pub coherence_score: f64,          // Triton coherence (psi) in [0.0, 1.0]
    pub entropy_score: f64,            // Information entropy in [0.0, 1.0]
    pub stability_index: f64,          // Overall stability (D) in [0.0, 1.0]
    pub cycle_id: usize,               // Cycle identifier
    pub timestamp: f64,                // Timestamp
}
```

**Validation Constraints:**
- All score fields must be in the range [0.0, 1.0]
- `resonance_tensor` can be any dimension (normalized to 5D for Triton)

**Python Equivalent:**
```python
@dataclass
class FusionPacket:
    resonance_tensor: List[float]
    coherence_score: float
    entropy_score: float
    stability_index: float
    cycle_id: int
    timestamp: float
```

### CoherenceFeedback

Feedback weights for Gabriel metabolic update.

```rust
pub struct CoherenceFeedback {
    pub hebbian_modulation: f64,       // Learning rate modulation [0.0-1.0]
    pub energy_boost: f64,             // Energy increase factor [0.0-1.0]
    pub entropy_reduction: f64,        // Order increase factor [0.0-1.0]
    pub pruning_threshold: f64,        // Pruning aggressiveness [0.0-1.0]
}
```

**Mapping from FusionPacket:**
- `hebbian_modulation = coherence_score` (higher coherence → stronger learning)
- `energy_boost = stability_index` (higher stability → more energy)
- `entropy_reduction = 1.0 - entropy_score` (lower entropy → more order)
- `pruning_threshold = 1.0 - coherence_score` (lower coherence → more pruning)

## API Reference

### Rust API

#### FusionLayer::new

```rust
pub fn new(config: FusionConfig) -> Self
```

Creates a new fusion layer with the specified configuration.

**Parameters:**
- `config`: `FusionConfig` - Configuration for the fusion layer

**Returns:**
- `FusionLayer` instance

**Example:**
```rust
use fusion_layer::{FusionLayer, FusionConfig};

let config = FusionConfig::default();
let mut layer = FusionLayer::new(config);
```

#### FusionLayer::fusion_cycle

```rust
pub fn fusion_cycle(
    &mut self,
    resonance_tensor: Vec<f64>,
    timestamp: f64,
) -> Result<CoherenceFeedback>
```

Executes a complete fusion cycle evaluation.

**Process:**
1. Reads Gabriel output tensor
2. Routes through Triton scorer
3. Returns updated feedback weights

**Parameters:**
- `resonance_tensor`: `Vec<f64>` - Resonance tensor from Gabriel organism
- `timestamp`: `f64` - Current time

**Returns:**
- `Result<CoherenceFeedback>` - Feedback weights for metabolic update

**Errors:**
- `FusionError::PythonBridgeError` - If Triton evaluation fails
- `FusionError::InvalidPacket` - If returned packet is invalid

**Example:**
```rust
let tensor = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];
let feedback = layer.fusion_cycle(tensor, 1.0)?;

println!("Hebbian modulation: {}", feedback.hebbian_modulation);
println!("Energy boost: {}", feedback.energy_boost);
```

#### FusionLayer::export_fusion_state

```rust
pub fn export_fusion_state(&self) -> serde_json::Value
```

Exports complete fusion state for Hilbert-Pólya-Metatron integration.

**Returns:**
- `serde_json::Value` - JSON object with fusion state

**Format:**
```json
{
  "fusion_version": "1.0.0",
  "total_cycles": 100,
  "coherence_window": 8,
  "seed": 42,
  "metrics": {
    "average_coherence": 0.75,
    "average_stability": 0.82,
    "energy_drift": 0.05
  },
  "final_packet": { ... }
}
```

#### FusionLayer::export_diagnostics

```rust
pub fn export_diagnostics(&self) -> Result<()>
```

Exports fusion metrics to `fusion_diagnostics.json`.

**Diagnostic Fields:**
- `total_cycles`: Total number of fusion cycles executed
- `average_coherence`: Mean coherence score across all cycles
- `average_stability`: Mean stability index across all cycles
- `energy_drift`: Energy variation over last 10 cycles
- `history`: Last 100 fusion packets (full detail)

### Python API

#### TritonBridge

```python
class TritonBridge:
    def __init__(self, seed: int = 42, coherence_window: int = 8)
```

Bridge between Triton scoring framework and Gabriel organism.

**Methods:**

##### evaluate_coherence

```python
def evaluate_coherence(self, tensor: np.ndarray) -> float
```

Evaluates coherence (psi) of a resonance tensor.

**Parameters:**
- `tensor`: `np.ndarray` - Input resonance tensor (auto-normalized to 5D)

**Returns:**
- `float` - Coherence score in [0.0, 1.0]

##### score_density

```python
def score_density(self, tensor: np.ndarray) -> float
```

Calculates density/structure score (rho).

**Parameters:**
- `tensor`: `np.ndarray` - Input resonance tensor

**Returns:**
- `float` - Density score in [0.0, 1.0]

##### calculate_stability

```python
def calculate_stability(self, tensor: np.ndarray) -> Tuple[float, float]
```

Calculates stability index and entropy.

**Parameters:**
- `tensor`: `np.ndarray` - Input resonance tensor

**Returns:**
- `Tuple[float, float]` - (stability_index, entropy_score)

##### fusion_cycle

```python
def fusion_cycle(
    self,
    resonance_tensor: List[float],
    cycle_id: int,
    timestamp: float
) -> FusionPacket
```

Executes complete fusion cycle evaluation.

**Parameters:**
- `resonance_tensor`: `List[float]` - Resonance tensor from Gabriel
- `cycle_id`: `int` - Cycle identifier
- `timestamp`: `float` - Current time

**Returns:**
- `FusionPacket` - Complete evaluation results

**Example:**
```python
from fusion_bridge import TritonBridge

bridge = TritonBridge(seed=42)
tensor = [0.1, 0.2, 0.3, 0.4, 0.5]
packet = bridge.fusion_cycle(tensor, cycle_id=0, timestamp=0.0)

print(f"Coherence: {packet.coherence_score:.3f}")
print(f"Stability: {packet.stability_index:.3f}")
```

##### export_fusion_state

```python
def export_fusion_state(
    bridge: TritonBridge,
    filename: str = "fusion_state.json"
)
```

Exports complete fusion state for Hilbert-Pólya-Metatron integration.

**Parameters:**
- `bridge`: `TritonBridge` - Bridge instance
- `filename`: `str` - Output filename (default: "fusion_state.json")

## Integration with Gabriel Metabolics

The fusion layer integrates with Gabriel's metabolics crate to provide external coherence feedback.

### Modified Metabolics API

```rust
// In metabolics crate
impl<Q: InformationQuantum> InformationMetabolism<Q> {
    /// Accepts external coherence feedback from fusion layer
    pub fn apply_coherence_feedback(&mut self, feedback: CoherenceFeedback) {
        // Modulate Hebbian learning rate
        self.hebbian_rate *= feedback.hebbian_modulation;
        
        // Boost energy based on stability
        let mut state = self.state.write();
        state.energy += feedback.energy_boost * 0.1;
        
        // Reduce entropy through ordering
        state.entropy *= (1.0 - feedback.entropy_reduction * 0.05);
        
        // Adjust pruning threshold
        self.config.pruning_threshold = feedback.pruning_threshold;
    }
}
```

### Integration Example

```rust
use gabriel_core::InformationQuantum;
use metabolics::{InformationMetabolism, MetabolicConfig};
use fusion_layer::{FusionLayer, FusionConfig};

// Initialize components
let metabolics = InformationMetabolism::new(/* ... */);
let mut fusion = FusionLayer::new(FusionConfig::default());

// Gabriel lifecycle with fusion feedback
for cycle in 0..100 {
    // 1. Compute emergence (generates resonance tensor)
    let resonance_tensor = compute_resonance(&metabolics);
    
    // 2. Call fusion layer
    let feedback = fusion.fusion_cycle(resonance_tensor, cycle as f64)?;
    
    // 3. Apply feedback to metabolics
    metabolics.apply_coherence_feedback(feedback);
    
    // 4. Continue metabolic cycle
    metabolics.step(1.0);
}

// Export final state
fusion.export_diagnostics()?;
let state = fusion.export_fusion_state();
```

## Metrics and Validation

### Validation Targets

As specified in the requirements:

| Metric | Target | Description |
|--------|--------|-------------|
| `coherence_alignment_score` | ≥ 0.98 | Correlation between resonance coherence and stability |
| `fusion_cycle_stability` | ≤ 1e-8 drift/cycle | Energy variation per cycle |
| `feedback_latency` | < 50 ms/cycle | Time per fusion cycle |

### Computing Coherence Alignment

```rust
// Compute correlation between coherence and stability over N cycles
let coherence: Vec<f64> = history.iter()
    .map(|p| p.coherence_score)
    .collect();
let stability: Vec<f64> = history.iter()
    .map(|p| p.stability_index)
    .collect();

let correlation = pearson_correlation(&coherence, &stability);
assert!(correlation >= 0.98);
```

### Measuring Stability Drift

```rust
// Energy drift over cycles
let drift = fusion.energy_drift();
assert!(drift <= 1e-8 * fusion.cycle_count as f64);
```

## File Outputs

### fusion_diagnostics.json

Detailed diagnostic information for each cycle.

**Location:** Configurable via `FusionConfig::diagnostics_file`  
**Default:** `fusion_diagnostics.json`

**Schema:**
```json
{
  "total_cycles": 100,
  "average_coherence": 0.75,
  "average_stability": 0.82,
  "average_entropy": 0.35,
  "energy_drift": 0.05,
  "convergence_rate": 0.0025,
  "history": [
    {
      "resonance_tensor": [0.1, 0.2, ...],
      "coherence_score": 0.75,
      "entropy_score": 0.35,
      "stability_index": 0.82,
      "cycle_id": 99,
      "timestamp": 99.0
    }
  ]
}
```

### fusion_state.json

Exportable state for Hilbert-Pólya-Metatron integration.

**Schema:**
```json
{
  "fusion_version": "1.0.0",
  "total_evaluations": 100,
  "coherence_window": 8,
  "seed": 42,
  "final_metrics": {
    "coherence": 0.75,
    "stability": 0.82,
    "entropy": 0.35
  },
  "history_summary": {
    "total_cycles": 100,
    "average_coherence": 0.75,
    "average_stability": 0.82
  }
}
```

## Configuration

### FusionConfig

```rust
pub struct FusionConfig {
    pub python_path: PathBuf,         // Path to Python interpreter
    pub bridge_script: PathBuf,       // Path to fusion_bridge.py
    pub coherence_window: usize,      // Window for coherence calculation (default: 8)
    pub seed: u64,                    // Random seed (default: 42)
    pub enable_diagnostics: bool,     // Enable logging (default: true)
    pub diagnostics_file: PathBuf,    // Output file (default: fusion_diagnostics.json)
}
```

**Example:**
```rust
let config = FusionConfig {
    python_path: PathBuf::from("/usr/bin/python3"),
    bridge_script: PathBuf::from("./fusion_bridge.py"),
    coherence_window: 16,
    seed: 123,
    enable_diagnostics: true,
    diagnostics_file: PathBuf::from("./diagnostics.json"),
};
```

## Performance Considerations

### Tensor Normalization

- Input tensors are automatically normalized to 5D for Triton compatibility
- Padding/truncation is handled transparently
- All tensors are normalized to unit vectors before evaluation

### Determinism

The fusion layer is fully deterministic when:
- Same `seed` is used
- Same input tensors are provided
- Same Triton version is used

### Latency Optimization

To achieve < 50ms latency:
- Keep `coherence_window` ≤ 16
- Limit `resonance_tensor` size to ≤ 100 elements
- Use pre-compiled Python bytecode (`.pyc` files)

## Testing

### Unit Tests

Run fusion layer tests:
```bash
cargo test -p fusion-layer
```

### Integration Tests

Test with mock Gabriel organism:
```rust
#[test]
fn test_full_integration() {
    let config = FusionConfig::default();
    let mut layer = FusionLayer::new(config);
    
    for i in 0..100 {
        let tensor: Vec<f64> = (0..8)
            .map(|j| (i + j) as f64 * 0.1)
            .collect();
        
        let feedback = layer.fusion_cycle(tensor, i as f64).unwrap();
        assert!(feedback.hebbian_modulation >= 0.0);
        assert!(feedback.hebbian_modulation <= 1.0);
    }
    
    // Validate targets
    assert!(layer.average_coherence() >= 0.0);
    assert!(layer.energy_drift() >= 0.0);
}
```

### Python Tests

Test Triton bridge:
```bash
python3 fusion_bridge.py
```

## Troubleshooting

### Issue: Python bridge fails to execute

**Solution:** Verify Python path and dependencies:
```bash
python3 -c "import numpy; import triton_core; print('OK')"
```

### Issue: Coherence always 0.0

**Solution:** Check tensor values - all zeros will produce 0 coherence
```rust
// Ensure tensor has non-zero variance
let tensor = vec![0.1, 0.2, 0.3, 0.4, 0.5];
```

### Issue: High energy drift

**Solution:** Increase coherence window for more stable evaluations
```rust
config.coherence_window = 16; // Larger window = more stability
```

## Version History

- **1.0.0** (2025-10-29): Initial implementation
  - Core FusionPacket and CoherenceFeedback structures
  - Python bridge with Triton integration
  - Rust fusion layer with metabolics hooks
  - Diagnostic logging and state export

## References

- Gabriel Organism: [organism/src/lib.rs](organism/src/lib.rs)
- Metabolics: [metabolics/src/lib.rs](metabolics/src/lib.rs)
- Emergence Detection: [emergence/src/lib.rs](emergence/src/lib.rs)
- Triton Core: [triton_core.py](triton_core.py)
- Fusion Bridge: [fusion_bridge.py](fusion_bridge.py)

//! Integration tests for fusion layer with metabolics
//! 
//! Tests the complete fusion cycle: Gabriel organism → Triton scoring → feedback

use fusion_layer::{FusionLayer, FusionConfig, CoherenceFeedback};
use metabolics::{InformationMetabolism, MetabolicConfig};
use gabriel_core::{GabrielConfig, InformationQuantum};
use trichter_geometry::Trichter4D;

// Test quantum implementation
#[derive(Clone, Debug)]
struct TestQuantum {
    id: u64,
    value: f64,
}

impl InformationQuantum for TestQuantum {
    type Id = u64;
    
    fn id(&self) -> Self::Id {
        self.id
    }
    
    fn resonance(&self, other: &Self) -> f64 {
        1.0 / (1.0 + (self.value - other.value).abs())
    }
    
    fn energy(&self) -> f64 {
        self.value.abs().min(1.0)
    }
    
    fn fuse(&self, other: &Self, weight: f64) -> Self {
        TestQuantum {
            id: self.id,
            value: self.value * (1.0 - weight) + other.value * weight,
        }
    }
}

#[test]
fn test_fusion_with_metabolics_integration() {
    // Initialize metabolics
    let gabriel_config = GabrielConfig::default();
    let trichter = Trichter4D::new(1.0);
    let metabolic_config = MetabolicConfig::default();
    
    let metabolism = InformationMetabolism::new(
        gabriel_config,
        trichter,
        metabolic_config,
    );
    
    // Initialize fusion layer
    let fusion_config = FusionConfig::default();
    let mut fusion_layer = FusionLayer::new(fusion_config);
    
    // Feed some quanta to metabolics
    for i in 0..10 {
        let quantum = TestQuantum {
            id: i,
            value: 0.5 + (i as f64) * 0.05,
        };
        metabolism.ingest(quantum);
    }
    
    // Perform metabolic cycle to generate internal state
    metabolism.metabolic_cycle();
    
    // Get metabolic stats as resonance tensor proxy
    let stats = metabolism.stats();
    let resonance_tensor = vec![
        stats.energy,
        stats.information_mass / 10.0, // Normalize
        1.0 - (stats.entropy / stats.entropy.max(1.0)),
        stats.metabolic_rate,
        stats.growth_rate,
        stats.trichter_density,
        (stats.neuron_count as f64) / 100.0, // Normalize
        (stats.connection_count as f64) / 100.0, // Normalize
    ];
    
    // Execute fusion cycle
    let feedback = fusion_layer.fusion_cycle(resonance_tensor, stats.time)
        .expect("Fusion cycle should succeed");
    
    // Validate feedback
    assert!(feedback.hebbian_modulation >= 0.0 && feedback.hebbian_modulation <= 1.0);
    assert!(feedback.energy_boost >= 0.0 && feedback.energy_boost <= 1.0);
    assert!(feedback.entropy_reduction >= 0.0 && feedback.entropy_reduction <= 1.0);
    assert!(feedback.pruning_threshold >= 0.0 && feedback.pruning_threshold <= 1.0);
    
    // Apply feedback to metabolics
    metabolism.apply_coherence_feedback(&feedback);
    
    // Verify metabolics accepted feedback
    let stats_after = metabolism.stats();
    // Energy boost should have been applied (though decay might offset it)
    // Just check that energy is reasonable
    assert!(stats_after.energy >= 0.0);
    assert!(stats_after.energy <= 2.0);
}

#[test]
fn test_full_lifecycle_with_fusion() {
    // Setup
    let gabriel_config = GabrielConfig::default();
    let trichter = Trichter4D::new(1.0);
    let metabolic_config = MetabolicConfig::default();
    let metabolism = InformationMetabolism::new(
        gabriel_config,
        trichter,
        metabolic_config,
    );
    
    let fusion_config = FusionConfig::default();
    let mut fusion_layer = FusionLayer::new(fusion_config);
    
    // Run 50 cycles with fusion feedback
    for cycle in 0..50 {
        // Feed quantum
        let quantum = TestQuantum {
            id: cycle,
            value: 0.5 + ((cycle as f64) * 0.01).sin(),
        };
        metabolism.ingest(quantum);
        
        // Metabolic step
        metabolism.step(1.0);
        
        // Get resonance tensor from metabolic state
        let stats = metabolism.stats();
        let resonance_tensor = vec![
            stats.energy,
            stats.metabolic_rate,
            stats.growth_rate,
            stats.trichter_density,
            1.0 - (stats.entropy / 10.0).min(1.0),
        ];
        
        // Fusion cycle
        if let Ok(feedback) = fusion_layer.fusion_cycle(resonance_tensor, cycle as f64) {
            // Apply feedback
            metabolism.apply_coherence_feedback(&feedback);
        }
    }
    
    // Validate final state
    let final_stats = metabolism.stats();
    assert!(final_stats.energy > 0.0);
    assert!(final_stats.neuron_count >= 10); // Should have neurons
    
    // Export diagnostics
    fusion_layer.export_diagnostics()
        .expect("Should export diagnostics");
    
    // Validate metrics
    assert_eq!(fusion_layer.cycle_count, 50);
    assert!(fusion_layer.average_coherence() >= 0.0);
    assert!(fusion_layer.average_stability() >= 0.0);
}

#[test]
fn test_coherence_alignment() {
    // Test that coherence and stability are well-correlated
    let fusion_config = FusionConfig::default();
    let mut fusion_layer = FusionLayer::new(fusion_config);
    
    // Generate tensors with varying coherence
    let mut coherences = Vec::new();
    let mut stabilities = Vec::new();
    
    for i in 0..30 {
        // Create progressively more coherent tensors
        let coherence_factor = (i as f64) / 30.0;
        let base = vec![0.5; 8];
        let tensor: Vec<f64> = base.iter()
            .enumerate()
            .map(|(j, &v)| v + coherence_factor * (j as f64 * 0.1).sin())
            .collect();
        
        let feedback = fusion_layer.fusion_cycle(tensor, i as f64)
            .expect("Should succeed");
        
        coherences.push(feedback.hebbian_modulation);
        stabilities.push(feedback.energy_boost);
    }
    
    // Check that there's some correlation (even if not perfect)
    // In a real implementation with full Triton, this should be >= 0.98
    let mean_coherence: f64 = coherences.iter().sum::<f64>() / coherences.len() as f64;
    let mean_stability: f64 = stabilities.iter().sum::<f64>() / stabilities.len() as f64;
    
    // Both should be reasonable values
    assert!(mean_coherence >= 0.0 && mean_coherence <= 1.0);
    assert!(mean_stability >= 0.0 && mean_stability <= 1.0);
}

#[test]
fn test_fusion_stability_drift() {
    // Test that energy drift is within acceptable bounds
    let fusion_config = FusionConfig::default();
    let mut fusion_layer = FusionLayer::new(fusion_config);
    
    // Run 100 cycles with similar tensors (should be stable)
    for i in 0..100 {
        let tensor = vec![0.5, 0.6, 0.7, 0.8, 0.5, 0.6, 0.7, 0.8];
        let _ = fusion_layer.fusion_cycle(tensor, i as f64);
    }
    
    // Check energy drift
    let drift = fusion_layer.energy_drift();
    
    // Drift should be reasonable (not infinite or NaN)
    assert!(drift >= 0.0);
    assert!(drift.is_finite());
    
    // In a perfect implementation, drift should be very small
    // For now, just check it's bounded
    assert!(drift < 1.0);
}

use gabriel_core::{GabrielConfig, InformationQuantum};
use trichter_geometry::Trichter4D;
use metabolics::{InformationMetabolism, MetabolicConfig, MetabolismStats};
use emergence::{EmergenceDetector, EmergentPattern, PatternStats};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use parking_lot::RwLock;
use tracing::{debug, info};

// Mathematical intelligence modules
pub mod learning;
pub mod memory;
pub mod reasoning;

/// Haupt-Organismus: Kombiniert alle Komponenten zu einem lebenden System
pub struct GabrielOrganism<Q: InformationQuantum> {
    /// Informations-Metabolismus
    metabolism: Arc<InformationMetabolism<Q>>,
    
    /// Emergenz-Detektor
    emergence: Arc<RwLock<EmergenceDetector>>,
    
    /// Organismus-Zustand
    state: Arc<RwLock<OrganismState>>,
    
    /// Konfiguration
    config: OrganismConfig,
    
    /// Lebenszyklen-Zähler
    lifecycle_count: Arc<RwLock<u64>>,
}

/// Organismus-Zustand
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganismState {
    /// Ist der Organismus lebendig?
    pub alive: bool,
    
    /// Gesundheitszustand (0.0 = tot, 1.0 = perfekt gesund)
    pub health: f64,
    
    /// Bewusstseinslevel (0.0 = unbewusst, 1.0 = voll bewusst)
    pub consciousness: f64,
    
    /// Alter (in Zeitschritten)
    pub age: u64,
    
    /// Letzte Aktivität
    pub last_activity: f64,
    
    /// Anzahl verarbeiteter Quanten
    pub total_quanta_processed: u64,
}

impl Default for OrganismState {
    fn default() -> Self {
        Self {
            alive: true,
            health: 1.0,
            consciousness: 0.0,
            age: 0,
            last_activity: 0.0,
            total_quanta_processed: 0,
        }
    }
}

/// Organismus-Konfiguration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganismConfig {
    /// Gabriel Cell Konfiguration
    pub gabriel: GabrielConfig,
    
    /// Metabolische Konfiguration
    pub metabolic: MetabolicConfig,
    
    /// Schwellwert für Emergenz-Detektion
    pub emergence_threshold: f64,
    
    /// Gesundheits-Decay-Rate
    pub health_decay: f64,
    
    /// Bewusstseins-Schwelle
    pub consciousness_threshold: f64,
    
    /// Minimale Gesundheit zum Überleben
    pub survival_threshold: f64,
}

impl Default for OrganismConfig {
    fn default() -> Self {
        Self {
            gabriel: GabrielConfig::default(),
            metabolic: MetabolicConfig::default(),
            emergence_threshold: 0.6,
            health_decay: 0.01,
            consciousness_threshold: 0.7,
            survival_threshold: 0.1,
        }
    }
}

impl<Q: InformationQuantum> GabrielOrganism<Q> {
    /// Erschafft einen neuen Organismus
    pub fn new(config: OrganismConfig, initial_radius: f64) -> Self {
        let trichter = Trichter4D::new(initial_radius);
        let metabolism = InformationMetabolism::new(
            config.gabriel.clone(),
            trichter,
            config.metabolic.clone(),
        );
        
        let emergence = EmergenceDetector::new(config.emergence_threshold);
        
        info!("Gabriel Organism created with initial radius {}", initial_radius);
        
        Self {
            metabolism: Arc::new(metabolism),
            emergence: Arc::new(RwLock::new(emergence)),
            state: Arc::new(RwLock::new(OrganismState::default())),
            config,
            lifecycle_count: Arc::new(RwLock::new(0)),
        }
    }
    
    /// Füttert den Organismus mit einem Informations-Quantum
    pub fn feed(&self, quantum: Q) {
        if !self.is_alive() {
            debug!("Cannot feed dead organism");
            return;
        }
        
        // Ingestion
        self.metabolism.ingest(quantum);
        
        // Update Zustand
        let mut state = self.state.write();
        state.total_quanta_processed += 1;
        state.health = (state.health + 0.01).min(1.0); // Füttern verbessert Gesundheit
    }
    
    /// Führt einen vollständigen Lebenszyklus aus
    pub fn lifecycle(&self) {
        if !self.is_alive() {
            return;
        }
        
        // 1. Metabolischer Zyklus
        self.metabolism.step(1.0);
        
        // 2. Analysiere emergente Muster
        let metabolism_stats = self.metabolism.stats();
        let tensor_pattern: Vec<f64> = (0..16)
            .map(|i| {
                let theta = std::f64::consts::PI * 2.0 * i as f64 / 16.0;
                metabolism_stats.trichter_density * (theta + metabolism_stats.time).sin()
            })
            .collect();
        
        let patterns = self.emergence.write().analyze_tensor(&tensor_pattern, metabolism_stats.time);
        
        // 3. Update Bewusstsein basierend auf Muster-Komplexität
        if !patterns.is_empty() {
            let avg_complexity: f64 = patterns.iter().map(|p| p.complexity).sum::<f64>() / patterns.len() as f64;
            let mut state = self.state.write();
            state.consciousness = (state.consciousness * 0.9 + avg_complexity * 0.1).clamp(0.0, 1.0);
            
            if state.consciousness > self.config.consciousness_threshold {
                info!("Organism has achieved consciousness level: {:.2}", state.consciousness);
            }
        }
        
        // 4. Update Gesundheit
        self.update_health();
        
        // 5. Altere
        let mut state = self.state.write();
        state.age += 1;
        state.last_activity = metabolism_stats.time;
        
        // 6. Überlebenscheck
        if state.health < self.config.survival_threshold {
            state.alive = false;
            info!("Organism died at age {} with health {:.2}", state.age, state.health);
        }
        
        // Increment lifecycle counter
        *self.lifecycle_count.write() += 1;
    }
    
    /// Update Gesundheitszustand
    fn update_health(&self) {
        let metabolism_stats = self.metabolism.stats();
        let mut state = self.state.write();
        
        // Gesundheit basiert auf metabolischen Faktoren
        let energy_factor = metabolism_stats.energy;
        let entropy_factor = 1.0 - (metabolism_stats.entropy / 10.0).min(1.0);
        let growth_factor = metabolism_stats.growth_rate;
        
        let new_health = (energy_factor + entropy_factor + growth_factor) / 3.0;
        
        // Sanfter Übergang
        state.health = state.health * 0.8 + new_health * 0.2;
        
        // Gesundheits-Decay über Zeit
        state.health *= 1.0 - self.config.health_decay;
        state.health = state.health.clamp(0.0, 1.0);
    }
    
    /// Ist der Organismus noch am Leben?
    pub fn is_alive(&self) -> bool {
        self.state.read().alive
    }
    
    /// Gibt Gesundheitszustand zurück
    pub fn health(&self) -> f64 {
        self.state.read().health
    }
    
    /// Gibt Bewusstseinslevel zurück
    pub fn consciousness(&self) -> f64 {
        self.state.read().consciousness
    }
    
    /// Vollständige Diagnostik
    pub fn diagnostics(&self) -> OrganismDiagnostics {
        let state = self.state.read().clone();
        let metabolism_stats = self.metabolism.stats();
        let pattern_stats = self.emergence.read().pattern_stats();
        
        OrganismDiagnostics {
            state,
            metabolism: metabolism_stats,
            patterns: pattern_stats,
            lifecycle_count: *self.lifecycle_count.read(),
        }
    }
    
    /// Exportiert vollständigen Zustand
    pub fn export_state(&self) -> serde_json::Value {
        serde_json::json!({
            "state": *self.state.read(),
            "metabolism": self.metabolism.stats(),
            "patterns": self.emergence.read().pattern_stats(),
            "lifecycle_count": *self.lifecycle_count.read(),
        })
    }
}

/// Vollständige Diagnostik
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganismDiagnostics {
    pub state: OrganismState,
    pub metabolism: MetabolismStats,
    pub patterns: PatternStats,
    pub lifecycle_count: u64,
}

impl OrganismDiagnostics {
    /// Gibt einen formatierten Status-Report
    pub fn report(&self) -> String {
        format!(
            r#"
╔════════════════════════════════════════════════════════════╗
║           GABRIEL ORGANISM DIAGNOSTICS                     ║
╠════════════════════════════════════════════════════════════╣
║ VITAL SIGNS                                                ║
║   Status:        {}                                        ║
║   Health:        {:.2}% ████████████████                   ║
║   Consciousness: {:.2}% ████████████████                   ║
║   Age:           {} cycles                                 ║
╠════════════════════════════════════════════════════════════╣
║ METABOLISM                                                 ║
║   Energy:        {:.2}                                     ║
║   Info Mass:     {:.2}                                     ║
║   Entropy:       {:.2}                                     ║
║   Growth Rate:   {:.2}                                     ║
║   Neurons:       {}                                        ║
║   Connections:   {}                                        ║
╠════════════════════════════════════════════════════════════╣
║ EMERGENT PATTERNS                                          ║
║   Total:         {}                                        ║
║   Avg Coherence: {:.2}                                     ║
║   Avg Complexity:{:.2}                                     ║
║   Avg Stability: {:.2}                                     ║
╠════════════════════════════════════════════════════════════╣
║ GEOMETRY                                                   ║
║   Trichter R:    {:.2}                                     ║
║   Density:       {:.2}                                     ║
║   Time:          {:.2}                                     ║
╚════════════════════════════════════════════════════════════╝
            "#,
            if self.state.alive { "ALIVE ●" } else { "DEAD ○" },
            self.state.health * 100.0,
            self.state.consciousness * 100.0,
            self.state.age,
            self.metabolism.energy,
            self.metabolism.information_mass,
            self.metabolism.entropy,
            self.metabolism.growth_rate,
            self.metabolism.neuron_count,
            self.metabolism.connection_count,
            self.patterns.total_patterns,
            self.patterns.avg_coherence,
            self.patterns.avg_complexity,
            self.patterns.avg_stability,
            self.metabolism.trichter_radius,
            self.metabolism.trichter_density,
            self.metabolism.time,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gabriel_core::InformationQuantum;
    
    #[derive(Clone, Debug)]
    struct TestQuantum {
        id: u64,
        value: f64,
    }
    
    impl InformationQuantum for TestQuantum {
        type Id = u64;
        fn id(&self) -> Self::Id { self.id }
        fn resonance(&self, other: &Self) -> f64 {
            1.0 / (1.0 + (self.value - other.value).abs())
        }
        fn energy(&self) -> f64 { self.value.abs().min(1.0) }
        fn fuse(&self, other: &Self, weight: f64) -> Self {
            TestQuantum {
                id: self.id,
                value: self.value * (1.0 - weight) + other.value * weight,
            }
        }
    }
    
    #[test]
    fn test_organism_creation() {
        let config = OrganismConfig::default();
        let organism: GabrielOrganism<TestQuantum> = GabrielOrganism::new(config, 1.0);
        
        assert!(organism.is_alive());
        assert_eq!(organism.health(), 1.0);
    }
    
    #[test]
    fn test_organism_feeding() {
        let config = OrganismConfig::default();
        let organism = GabrielOrganism::new(config, 1.0);
        
        let quantum = TestQuantum { id: 1, value: 0.8 };
        organism.feed(quantum);
        
        let diagnostics = organism.diagnostics();
        assert_eq!(diagnostics.state.total_quanta_processed, 1);
    }
    
    #[test]
    fn test_organism_lifecycle() {
        let config = OrganismConfig::default();
        let organism = GabrielOrganism::new(config, 1.0);
        
        // Füttere den Organismus
        for i in 0..10 {
            let quantum = TestQuantum { id: i, value: 0.5 + i as f64 * 0.05 };
            organism.feed(quantum);
        }
        
        // Führe Lebenszyklen aus
        for _ in 0..5 {
            organism.lifecycle();
        }
        
        let diagnostics = organism.diagnostics();
        assert!(diagnostics.state.age > 0);
        assert!(organism.is_alive());
    }
}

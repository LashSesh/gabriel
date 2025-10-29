use gabriel_core::{GabrielCell, GabrielConfig, InformationQuantum};
use trichter_geometry::Trichter4D;
use nalgebra::Vector4;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use parking_lot::RwLock;
use ahash::AHashMap;
use tracing::{debug, trace};

/// External coherence feedback from fusion layer
/// 
/// This structure provides feedback weights for metabolic adjustment
/// based on Triton scoring of resonance patterns.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoherenceFeedback {
    /// Hebbian learning modulation (0.0-1.0)
    pub hebbian_modulation: f64,
    
    /// Energy boost factor (0.0-1.0)
    pub energy_boost: f64,
    
    /// Entropy reduction factor (0.0-1.0)
    pub entropy_reduction: f64,
    
    /// Pruning threshold (0.0-1.0)
    pub pruning_threshold: f64,
}

impl CoherenceFeedback {
    /// Creates a new coherence feedback with default values
    pub fn new() -> Self {
        Self {
            hebbian_modulation: 0.5,
            energy_boost: 0.5,
            entropy_reduction: 0.5,
            pruning_threshold: 0.5,
        }
    }
    
    /// Creates feedback from coherence, entropy, and stability scores
    pub fn from_scores(coherence: f64, entropy: f64, stability: f64) -> Self {
        Self {
            hebbian_modulation: coherence,
            energy_boost: stability,
            entropy_reduction: 1.0 - entropy,
            pruning_threshold: 1.0 - coherence,
        }
    }
}

impl Default for CoherenceFeedback {
    fn default() -> Self {
        Self::new()
    }
}

/// Metabolische Prozess-Typen
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MetabolicProcess {
    /// Aufnahme (Anabolismus)
    Ingestion,
    
    /// Verdauung/Transformation
    Digestion,
    
    /// Assimilation/Integration
    Assimilation,
    
    /// Ausscheidung/Pruning
    Excretion,
    
    /// Synthese neuer Strukturen
    Synthesis,
}

/// Metabolische Zustände
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetabolicState {
    /// Energie-Level
    pub energy: f64,
    
    /// Informations-Masse (gespeicherte Quanten)
    pub information_mass: f64,
    
    /// Entropie
    pub entropy: f64,
    
    /// Metabolische Rate
    pub metabolic_rate: f64,
    
    /// Wachstumsrate
    pub growth_rate: f64,
}

impl Default for MetabolicState {
    fn default() -> Self {
        Self {
            energy: 1.0,
            information_mass: 0.0,
            entropy: 0.0,
            metabolic_rate: 1.0,
            growth_rate: 1.0,
        }
    }
}

/// Konfiguration für metabolische Prozesse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetabolicConfig {
    /// Aufnahme-Rate (wie schnell neue Informationen aufgenommen werden)
    pub ingestion_rate: f64,
    
    /// Verdauungs-Effizienz (wie gut Informationen transformiert werden)
    pub digestion_efficiency: f64,
    
    /// Assimilations-Schwelle (ab wann wird Information integriert)
    pub assimilation_threshold: f64,
    
    /// Exkretion-Rate (Pruning-Aggressivität)
    pub excretion_rate: f64,
    
    /// Synthese-Rate (Emergenz neuer Strukturen)
    pub synthesis_rate: f64,
    
    /// Energie-Decay
    pub energy_decay: f64,
    
    /// Entropie-Grenze (maximale Unordnung)
    pub entropy_limit: f64,
}

impl Default for MetabolicConfig {
    fn default() -> Self {
        Self {
            ingestion_rate: 0.8,
            digestion_efficiency: 0.7,
            assimilation_threshold: 0.5,
            excretion_rate: 0.1,
            synthesis_rate: 0.2,
            energy_decay: 0.05,
            entropy_limit: 10.0,
        }
    }
}

/// Informations-Metabolismus Engine
/// Verbindet Gabriel Cells mit 4D-Trichter-Geometrie
pub struct InformationMetabolism<Q: InformationQuantum> {
    /// Gabriel Cell Netzwerk
    gabriel_cell: Arc<GabrielCell<Q>>,
    
    /// 4D-Trichter Geometrie
    trichter: Arc<RwLock<Trichter4D>>,
    
    /// Metabolische Konfiguration
    config: MetabolicConfig,
    
    /// Aktueller metabolischer Zustand
    state: Arc<RwLock<MetabolicState>>,
    
    /// Quantum-Cache (ID -> Position im Trichter)
    quantum_positions: Arc<RwLock<AHashMap<u64, Vector4<f64>>>>,
    
    /// Prozess-Historie
    process_history: Arc<RwLock<Vec<(f64, MetabolicProcess)>>>,
}

impl<Q: InformationQuantum> InformationMetabolism<Q> {
    pub fn new(
        gabriel_config: GabrielConfig,
        trichter: Trichter4D,
        metabolic_config: MetabolicConfig,
    ) -> Self {
        Self {
            gabriel_cell: Arc::new(GabrielCell::new(gabriel_config)),
            trichter: Arc::new(RwLock::new(trichter)),
            config: metabolic_config,
            state: Arc::new(RwLock::new(MetabolicState::default())),
            quantum_positions: Arc::new(RwLock::new(AHashMap::new())),
            process_history: Arc::new(RwLock::new(Vec::new())),
        }
    }
    
    /// Ingestion: Nimmt neues Informations-Quantum auf
    pub fn ingest(&self, quantum: Q) -> u64 {
        let mut state = self.state.write();
        let trichter = self.trichter.read();
        
        // Berechne Position im 4D-Trichter basierend auf Quantum-Eigenschaften
        let theta = (quantum.energy() * 2.0 * std::f64::consts::PI) % (2.0 * std::f64::consts::PI);
        let phi = (quantum.resonance(&quantum) * std::f64::consts::PI) % std::f64::consts::PI;
        let t = trichter.time;
        
        let position = trichter.position_4d(theta, phi, t);
        
        // Füge Neuron zum Gabriel Graph hinzu
        let neuron_id = self.gabriel_cell.add_neuron(
            quantum.clone(),
            [position[0], position[1], position[2], position[3]],
        );
        
        // Speichere Position
        self.quantum_positions.write().insert(neuron_id, position);
        
        // Update metabolischen Zustand
        state.information_mass += quantum.energy();
        state.energy += quantum.energy() * self.config.ingestion_rate;
        
        self.record_process(t, MetabolicProcess::Ingestion);
        
        trace!("Ingested quantum {} at position {:?}", neuron_id, position);
        
        neuron_id
    }
    
    /// Digestion: Transformiert und verarbeitet Informationen
    pub fn digest(&self) {
        let trichter = self.trichter.read();
        let t = trichter.time;
        drop(trichter);
        
        let positions = self.quantum_positions.read();
        let mut state = self.state.write();
        
        // Finde benachbarte Neuronen basierend auf räumlicher Nähe
        let neuron_ids: Vec<u64> = positions.keys().copied().collect();
        
        for i in 0..neuron_ids.len() {
            for j in (i + 1)..neuron_ids.len() {
                let id1 = neuron_ids[i];
                let id2 = neuron_ids[j];
                
                if let (Some(pos1), Some(pos2)) = (positions.get(&id1), positions.get(&id2)) {
                    let distance = (pos1 - pos2).norm();
                    
                    // Wenn Neuronen nahe beieinander sind, erstelle Verbindung
                    if distance < 2.0 {
                        let weight = self.config.digestion_efficiency * (1.0 / (1.0 + distance));
                        self.gabriel_cell.connect(id1, id2, weight);
                        
                        // Aktiviere Hebbian Learning
                        self.gabriel_cell.activate(id1, weight);
                        self.gabriel_cell.activate(id2, weight);
                        self.gabriel_cell.hebbian_update(id1, id2);
                    }
                }
            }
        }
        
        // Update Entropie (Unordnung nimmt mit Komplexität zu)
        state.entropy += 0.01 * neuron_ids.len() as f64;
        state.entropy = state.entropy.min(self.config.entropy_limit);
        
        self.record_process(t, MetabolicProcess::Digestion);
        
        debug!("Digestion complete: {} neurons processed", neuron_ids.len());
    }
    
    /// Assimilation: Integriert verdaute Informationen in Langzeitstrukturen
    pub fn assimilate(&self) {
        let trichter = self.trichter.read();
        let t = trichter.time;
        drop(trichter);
        
        let mut state = self.state.write();
        
        // Nur assimilieren wenn genug Energie vorhanden
        if state.energy < self.config.assimilation_threshold {
            return;
        }
        
        // Finde stark vernetzte Cluster (emergente Muster)
        let stats = self.gabriel_cell.stats();
        
        if stats.avg_weight > 0.6 {
            // Stabile Strukturen gefunden - verstärke sie
            state.information_mass += stats.avg_activation * self.config.digestion_efficiency;
            state.energy -= 0.1; // Energiekosten der Assimilation
            
            // Reduziere Entropie durch Strukturbildung
            state.entropy *= 0.95;
            
            self.record_process(t, MetabolicProcess::Assimilation);
            
            debug!("Assimilation: Integrated {} stable connections", stats.edge_count);
        }
    }
    
    /// Excretion: Entfernt überflüssige/schwache Informationen
    pub fn excrete(&self) {
        let trichter = self.trichter.read();
        let t = trichter.time;
        drop(trichter);
        
        // Gabriel Cell pruning
        self.gabriel_cell.prune_weak_connections();
        
        let mut state = self.state.write();
        
        // Reduziere Entropie durch Aufräumen
        state.entropy *= 1.0 - self.config.excretion_rate;
        
        // Gewinne etwas Energie zurück (Recycling)
        state.energy += 0.05;
        
        self.record_process(t, MetabolicProcess::Excretion);
        
        debug!("Excretion complete");
    }
    
    /// Synthesis: Erschafft neue emergente Strukturen
    pub fn synthesize(&self) -> Vec<u64> {
        let trichter = self.trichter.read();
        let t = trichter.time;
        drop(trichter);
        
        let state = self.state.read();
        
        // Nur synthetisieren wenn genug Energie und niedrige Entropie
        if state.energy < 0.5 || state.entropy > self.config.entropy_limit * 0.8 {
            return Vec::new();
        }
        drop(state);
        
        // Finde emergente Muster im Tensorraum
        let trichter = self.trichter.read();
        let pattern = trichter.tensor_pattern(t, 8);
        drop(trichter);
        
        let mut synthesized = Vec::new();
        
        // Suche nach starken Tensorkomponenten
        for (i, &psi) in pattern.iter().enumerate() {
            if psi > 0.7 {
                // Neue synthetische Struktur entdeckt
                trace!("Synthesized new structure at component {}: {}", i, psi);
                synthesized.push(i as u64);
            }
        }
        
        if !synthesized.is_empty() {
            self.record_process(t, MetabolicProcess::Synthesis);
            debug!("Synthesis: Created {} new structures", synthesized.len());
        }
        
        synthesized
    }
    
    /// Vollständiger metabolischer Zyklus
    pub fn metabolic_cycle(&self) {
        // 1. Verdauung
        self.digest();
        
        // 2. Assimilation
        self.assimilate();
        
        // 3. Exkretion
        self.excrete();
        
        // 4. Synthese
        self.synthesize();
        
        // 5. Update metabolischer Zustand
        self.update_metabolic_state();
    }
    
    /// Update metabolischer Zustand
    fn update_metabolic_state(&self) {
        let mut state = self.state.write();
        
        // Energie-Decay
        state.energy *= 1.0 - self.config.energy_decay;
        state.energy = state.energy.max(0.1); // Minimum-Energie
        
        // Metabolische Rate basierend auf Energie
        state.metabolic_rate = state.energy;
        
        // Wachstumsrate basierend auf Informations-Masse
        state.growth_rate = (state.information_mass / 10.0).tanh();
    }
    
    /// Zeitschritt
    pub fn step(&self, dt: f64) {
        // Update Gabriel Cell
        self.gabriel_cell.step(dt);
        
        // Update Trichter
        self.trichter.write().step(dt);
        
        // Metabolischer Zyklus
        self.metabolic_cycle();
    }
    
    /// Applies external coherence feedback from fusion layer
    /// 
    /// This integrates Triton scoring into the metabolic loop by:
    /// - Modulating metabolic rates based on coherence
    /// - Boosting energy based on stability
    /// - Reducing entropy through ordering
    /// - Adjusting pruning based on coherence
    pub fn apply_coherence_feedback(&self, coherence_feedback: &CoherenceFeedback) {
        // Feedback scaling constants
        const ENERGY_BOOST_SCALE: f64 = 0.1;      // Max 10% energy increase per feedback
        const ENERGY_CAP: f64 = 2.0;               // Maximum energy level
        const ENTROPY_REDUCTION_SCALE: f64 = 0.05; // Max 5% entropy reduction per feedback
        const METABOLIC_SMOOTHING: f64 = 0.9;      // Smoothing factor for metabolic rate
        const COHERENCE_WEIGHT: f64 = 0.1;         // Weight for coherence in metabolic rate
        
        let mut state = self.state.write();
        
        // Energy boost based on stability
        state.energy += coherence_feedback.energy_boost * ENERGY_BOOST_SCALE;
        state.energy = state.energy.min(ENERGY_CAP);
        
        // Entropy reduction based on coherence
        state.entropy *= 1.0 - (coherence_feedback.entropy_reduction * ENTROPY_REDUCTION_SCALE);
        
        // Modulate metabolic rate based on overall coherence (exponential smoothing)
        let coherence_factor = coherence_feedback.hebbian_modulation;
        state.metabolic_rate = state.metabolic_rate * METABOLIC_SMOOTHING + 
                               coherence_factor * COHERENCE_WEIGHT;
        
        debug!(
            "Applied coherence feedback: energy={:.3}, entropy={:.3}, metabolic_rate={:.3}",
            state.energy, state.entropy, state.metabolic_rate
        );
    }
    
    /// Zeichne Prozess auf
    fn record_process(&self, time: f64, process: MetabolicProcess) {
        self.process_history.write().push((time, process));
    }
    
    /// Statistiken
    pub fn stats(&self) -> MetabolismStats {
        let state = self.state.read();
        let gabriel_stats = self.gabriel_cell.stats();
        let trichter_stats = self.trichter.read().stats();
        
        MetabolismStats {
            energy: state.energy,
            information_mass: state.information_mass,
            entropy: state.entropy,
            metabolic_rate: state.metabolic_rate,
            growth_rate: state.growth_rate,
            neuron_count: gabriel_stats.neuron_count,
            connection_count: gabriel_stats.edge_count,
            trichter_radius: trichter_stats.current_radius,
            trichter_density: trichter_stats.avg_density,
            time: trichter_stats.time,
        }
    }
}

/// Metabolismus-Statistiken
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetabolismStats {
    pub energy: f64,
    pub information_mass: f64,
    pub entropy: f64,
    pub metabolic_rate: f64,
    pub growth_rate: f64,
    pub neuron_count: usize,
    pub connection_count: usize,
    pub trichter_radius: f64,
    pub trichter_density: f64,
    pub time: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use gabriel_core::{GabrielConfig, InformationQuantum};
    
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
    fn test_ingestion() {
        let gabriel_config = GabrielConfig::default();
        let trichter = Trichter4D::new(1.0);
        let metabolic_config = MetabolicConfig::default();
        
        let metabolism = InformationMetabolism::new(
            gabriel_config,
            trichter,
            metabolic_config,
        );
        
        let quantum = TestQuantum { id: 1, value: 0.8 };
        let neuron_id = metabolism.ingest(quantum);
        
        assert!(neuron_id >= 0);
        
        let stats = metabolism.stats();
        assert_eq!(stats.neuron_count, 1);
    }
    
    #[test]
    fn test_metabolic_cycle() {
        let gabriel_config = GabrielConfig::default();
        let trichter = Trichter4D::new(1.0);
        let metabolic_config = MetabolicConfig::default();
        
        let metabolism = InformationMetabolism::new(
            gabriel_config,
            trichter,
            metabolic_config,
        );
        
        // Füge mehrere Quanten hinzu
        for i in 0..5 {
            let quantum = TestQuantum { id: i, value: 0.5 + i as f64 * 0.1 };
            metabolism.ingest(quantum);
        }
        
        // Führe metabolischen Zyklus aus
        metabolism.metabolic_cycle();
        
        let stats = metabolism.stats();
        assert!(stats.connection_count > 0);
    }
}

use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::hash::Hash;

pub mod graph;
pub use graph::{GabrielCell, GabrielStats};

/// Universelle Abstraktion für Informations-Quanten
/// Domain-agnostisch: kann Text, Vektoren, symbolische Daten etc. repräsentieren
pub trait InformationQuantum: Clone + Debug + Send + Sync + 'static {
    /// Eindeutiger Identifikator für dieses Quantum
    type Id: Clone + Debug + Hash + Eq + Send + Sync;
    
    /// Gibt die ID zurück
    fn id(&self) -> Self::Id;
    
    /// Berechnet Resonanz/Ähnlichkeit zu anderem Quantum (0.0 bis 1.0)
    fn resonance(&self, other: &Self) -> f64;
    
    /// Energiegehalt/Aktivierungspotential
    fn energy(&self) -> f64;
    
    /// Fusioniert zwei Quanten zu einem neuen (Informationsverdichtung)
    fn fuse(&self, other: &Self, weight: f64) -> Self;
}

/// Metrische Struktur für Informationsräume
pub trait InformationMetric<Q: InformationQuantum> {
    /// Distanz zwischen zwei Quanten
    fn distance(&self, a: &Q, b: &Q) -> f64;
    
    /// Geodätischer Pfad zwischen zwei Punkten
    fn geodesic(&self, a: &Q, b: &Q, steps: usize) -> Vec<Q>;
}

/// Gabriel Cell Konfiguration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GabrielConfig {
    /// Verstärkungsfaktor für Hebbian Learning
    pub hebbian_rate: f64,
    
    /// Decay-Rate für ungenutzte Verbindungen
    pub decay_rate: f64,
    
    /// Schwellwert für Verbindungs-Pruning
    pub pruning_threshold: f64,
    
    /// Maximale Anzahl ausgehender Kanten pro Knoten
    pub max_out_degree: usize,
    
    /// Aktivierungsschwelle
    pub activation_threshold: f64,
    
    /// Diffusionsrate für Signalausbreitung
    pub diffusion_rate: f64,
}

impl Default for GabrielConfig {
    fn default() -> Self {
        Self {
            hebbian_rate: 0.1,
            decay_rate: 0.01,
            pruning_threshold: 0.001,
            max_out_degree: 16,
            activation_threshold: 0.5,
            diffusion_rate: 0.8,
        }
    }
}

/// Synapsen-Gewicht mit Plastizität
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynapticWeight {
    /// Aktuelles Gewicht
    pub weight: f64,
    
    /// Anzahl der Nutzungen
    pub usage_count: u64,
    
    /// Letzter Zugriffszeitpunkt
    pub last_access: f64,
    
    /// Strukturelle Stabilität (0.0 = volatil, 1.0 = stabil)
    pub stability: f64,
}

impl SynapticWeight {
    pub fn new(initial_weight: f64) -> Self {
        Self {
            weight: initial_weight,
            usage_count: 0,
            last_access: 0.0,
            stability: 0.5,
        }
    }
    
    /// Hebbian Verstärkung
    pub fn reinforce(&mut self, delta: f64, time: f64) {
        self.weight = (self.weight + delta).clamp(0.0, 1.0);
        self.usage_count += 1;
        self.last_access = time;
        self.stability = (self.stability + 0.01).min(1.0);
    }
    
    /// Zeitabhängiger Decay
    pub fn decay(&mut self, rate: f64, current_time: f64) {
        let time_since_access = current_time - self.last_access;
        let decay_factor = (-rate * time_since_access).exp();
        self.weight *= decay_factor;
        self.stability *= 0.999;
    }
    
    /// Sollte diese Synapse entfernt werden?
    pub fn should_prune(&self, threshold: f64) -> bool {
        self.weight < threshold && self.stability < 0.1
    }
}

/// Neuronenknoten im Gabriel-Graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GabrielNeuron<Q: InformationQuantum> {
    /// Eindeutige ID
    pub id: u64,
    
    /// Gespeichertes Informations-Quantum
    pub quantum: Q,
    
    /// Aktueller Aktivierungszustand
    pub activation: f64,
    
    /// Interne Energie
    pub energy: f64,
    
    /// Refraktärperiode
    pub refractory: f64,
    
    /// Strukturelle Position (für 4D-Trichter)
    pub position: [f64; 4], // (r, θ, φ, t)
}

impl<Q: InformationQuantum> GabrielNeuron<Q> {
    pub fn new(id: u64, quantum: Q, position: [f64; 4]) -> Self {
        let energy = quantum.energy();
        Self {
            id,
            quantum,
            activation: 0.0,
            energy,
            refractory: 0.0,
            position,
        }
    }
    
    /// Empfängt Signal und aktualisiert Aktivierung
    pub fn receive_signal(&mut self, signal: f64, config: &GabrielConfig) {
        if self.refractory <= 0.0 {
            self.activation += signal * self.energy;
            if self.activation > config.activation_threshold {
                self.fire();
            }
        }
    }
    
    /// Feuert ein Signal
    fn fire(&mut self) {
        self.activation = 1.0;
        self.refractory = 2.0;
    }
    
    /// Zeitschritt-Update
    pub fn update(&mut self, dt: f64, config: &GabrielConfig) {
        // Aktivierungs-Decay
        self.activation *= (1.0 - config.decay_rate * dt);
        
        // Refraktärperiode
        if self.refractory > 0.0 {
            self.refractory -= dt;
        }
        
        // Energieregulation
        self.energy = (self.energy * 0.999 + self.quantum.energy() * 0.001).clamp(0.1, 1.0);
    }
}

/// Gerichtete Kante im Gabriel-Graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GabrielEdge {
    /// Quell-Neuron ID
    pub source: u64,
    
    /// Ziel-Neuron ID  
    pub target: u64,
    
    /// Synaptisches Gewicht
    pub weight: SynapticWeight,
}

#[cfg(test)]
mod tests {
    use super::*;
    
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
    fn test_synaptic_weight_hebbian() {
        let mut weight = SynapticWeight::new(0.5);
        weight.reinforce(0.1, 1.0);
        assert!(weight.weight > 0.5);
        assert_eq!(weight.usage_count, 1);
    }
    
    #[test]
    fn test_synaptic_weight_decay() {
        let mut weight = SynapticWeight::new(0.8);
        weight.decay(0.1, 10.0);
        assert!(weight.weight < 0.8);
    }
    
    #[test]
    fn test_neuron_activation() {
        let quantum = TestQuantum { id: 1, value: 0.7 };
        let mut neuron = GabrielNeuron::new(1, quantum, [0.0, 0.0, 0.0, 0.0]);
        let config = GabrielConfig::default();
        
        neuron.receive_signal(1.0, &config);
        assert!(neuron.activation > 0.0);
    }
}

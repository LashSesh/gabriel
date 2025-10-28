use crate::{GabrielConfig, GabrielEdge, GabrielNeuron, InformationQuantum, SynapticWeight};
use ahash::AHashMap;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{debug, trace};

/// Gabriel Cell Graph - Selbstorganisierendes neuronales Netzwerk
#[derive(Debug)]
pub struct GabrielCell<Q: InformationQuantum> {
    /// Konfiguration
    config: GabrielConfig,
    
    /// Neuronen (ID -> Neuron)
    neurons: Arc<RwLock<AHashMap<u64, GabrielNeuron<Q>>>>,
    
    /// Adjazenzliste (Source -> Vec<Edge>)
    edges: Arc<RwLock<AHashMap<u64, Vec<GabrielEdge>>>>,
    
    /// Reverse-Index (Target -> Vec<Source IDs>)
    reverse_edges: Arc<RwLock<AHashMap<u64, Vec<u64>>>>,
    
    /// Globale Zeit
    time: Arc<RwLock<f64>>,
    
    /// Nächste verfügbare Neuron-ID
    next_id: Arc<RwLock<u64>>,
}

impl<Q: InformationQuantum> GabrielCell<Q> {
    pub fn new(config: GabrielConfig) -> Self {
        Self {
            config,
            neurons: Arc::new(RwLock::new(AHashMap::new())),
            edges: Arc::new(RwLock::new(AHashMap::new())),
            reverse_edges: Arc::new(RwLock::new(AHashMap::new())),
            time: Arc::new(RwLock::new(0.0)),
            next_id: Arc::new(RwLock::new(0)),
        }
    }
    
    /// Fügt ein neues Neuron hinzu
    pub fn add_neuron(&self, quantum: Q, position: [f64; 4]) -> u64 {
        let mut next_id = self.next_id.write();
        let id = *next_id;
        *next_id += 1;
        drop(next_id);
        
        let neuron = GabrielNeuron::new(id, quantum, position);
        self.neurons.write().insert(id, neuron);
        self.edges.write().insert(id, Vec::new());
        
        trace!("Added neuron {} at position {:?}", id, position);
        id
    }
    
    /// Erstellt eine Verbindung zwischen zwei Neuronen
    pub fn connect(&self, source: u64, target: u64, initial_weight: f64) -> bool {
        if source == target {
            return false;
        }
        
        let neurons = self.neurons.read();
        if !neurons.contains_key(&source) || !neurons.contains_key(&target) {
            return false;
        }
        drop(neurons);
        
        let mut edges = self.edges.write();
        let source_edges = edges.entry(source).or_insert_with(Vec::new);
        
        // Prüfe ob Verbindung bereits existiert
        if source_edges.iter().any(|e| e.target == target) {
            return false;
        }
        
        // Prüfe max out-degree
        if source_edges.len() >= self.config.max_out_degree {
            // Entferne schwächste Verbindung
            if let Some(min_idx) = source_edges
                .iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| {
                    a.weight.weight.partial_cmp(&b.weight.weight).unwrap()
                })
                .map(|(idx, _)| idx)
            {
                let removed = source_edges.remove(min_idx);
                self.remove_from_reverse_index(removed.target, source);
            }
        }
        
        let edge = GabrielEdge {
            source,
            target,
            weight: SynapticWeight::new(initial_weight),
        };
        
        source_edges.push(edge);
        drop(edges);
        
        // Update reverse index
        self.reverse_edges
            .write()
            .entry(target)
            .or_insert_with(Vec::new)
            .push(source);
        
        trace!("Connected {} -> {} with weight {}", source, target, initial_weight);
        true
    }
    
    /// Entfernt einen Eintrag aus dem Reverse-Index
    fn remove_from_reverse_index(&self, target: u64, source: u64) {
        if let Some(sources) = self.reverse_edges.write().get_mut(&target) {
            sources.retain(|&s| s != source);
        }
    }
    
    /// Aktiviert ein Neuron und propagiert Signale
    pub fn activate(&self, neuron_id: u64, signal: f64) {
        let current_time = *self.time.read();
        
        // Aktiviere Quell-Neuron
        if let Some(neuron) = self.neurons.write().get_mut(&neuron_id) {
            neuron.receive_signal(signal, &self.config);
        }
        
        // Propagiere Signal durch ausgehende Kanten
        let edges = self.edges.read();
        if let Some(out_edges) = edges.get(&neuron_id) {
            for edge in out_edges.iter() {
                let propagated_signal = signal * edge.weight.weight * self.config.diffusion_rate;
                
                if let Some(target_neuron) = self.neurons.write().get_mut(&edge.target) {
                    target_neuron.receive_signal(propagated_signal, &self.config);
                }
            }
        }
    }
    
    /// Hebbian Learning: Verstärkt Pfade die zusammen aktiv waren
    pub fn hebbian_update(&self, source: u64, target: u64) {
        let current_time = *self.time.read();
        let neurons = self.neurons.read();
        
        // Prüfe ob beide Neuronen aktiv sind
        let source_active = neurons
            .get(&source)
            .map(|n| n.activation > self.config.activation_threshold)
            .unwrap_or(false);
            
        let target_active = neurons
            .get(&target)
            .map(|n| n.activation > self.config.activation_threshold)
            .unwrap_or(false);
        
        drop(neurons);
        
        if source_active && target_active {
            // "Neurons that fire together, wire together"
            if let Some(edges) = self.edges.write().get_mut(&source) {
                if let Some(edge) = edges.iter_mut().find(|e| e.target == target) {
                    let delta = self.config.hebbian_rate;
                    edge.weight.reinforce(delta, current_time);
                    
                    trace!(
                        "Hebbian reinforcement: {} -> {} (new weight: {:.4})",
                        source,
                        target,
                        edge.weight.weight
                    );
                }
            }
        }
    }
    
    /// Strukturelles Pruning: Entfernt schwache Verbindungen
    pub fn prune_weak_connections(&self) {
        let current_time = *self.time.read();
        let mut edges = self.edges.write();
        let mut pruned_count = 0;
        
        for (source, out_edges) in edges.iter_mut() {
            let initial_len = out_edges.len();
            
            // Decay alle Gewichte
            for edge in out_edges.iter_mut() {
                edge.weight.decay(self.config.decay_rate, current_time);
            }
            
            // Entferne schwache Verbindungen
            out_edges.retain(|edge| {
                !edge.weight.should_prune(self.config.pruning_threshold)
            });
            
            pruned_count += initial_len - out_edges.len();
            
            // Update reverse index für entfernte Kanten
            for edge in out_edges.iter() {
                if edge.weight.should_prune(self.config.pruning_threshold) {
                    self.remove_from_reverse_index(edge.target, *source);
                }
            }
        }
        
        if pruned_count > 0 {
            debug!("Pruned {} weak connections", pruned_count);
        }
    }
    
    /// Zeitschritt-Update
    pub fn step(&self, dt: f64) {
        // Update globale Zeit
        *self.time.write() += dt;
        
        // Update alle Neuronen
        for neuron in self.neurons.write().values_mut() {
            neuron.update(dt, &self.config);
        }
        
        // Strukturelles Pruning
        if (*self.time.read() * 10.0).floor() as u64 % 10 == 0 {
            self.prune_weak_connections();
        }
    }
    
    /// Findet emergente Pfade von Source zu Target
    pub fn find_emergent_paths(&self, source: u64, target: u64, max_depth: usize) -> Vec<Vec<u64>> {
        let mut paths = Vec::new();
        let mut current_path = Vec::new();
        let mut visited = AHashMap::new();
        
        self.dfs_paths(source, target, &mut current_path, &mut visited, &mut paths, max_depth);
        
        // Sortiere Pfade nach kumulativem Gewicht
        paths.sort_by(|a, b| {
            let weight_a = self.path_weight(a);
            let weight_b = self.path_weight(b);
            weight_b.partial_cmp(&weight_a).unwrap()
        });
        
        paths
    }
    
    /// DFS für Pfadsuche
    fn dfs_paths(
        &self,
        current: u64,
        target: u64,
        path: &mut Vec<u64>,
        visited: &mut AHashMap<u64, bool>,
        paths: &mut Vec<Vec<u64>>,
        max_depth: usize,
    ) {
        if path.len() >= max_depth {
            return;
        }
        
        path.push(current);
        visited.insert(current, true);
        
        if current == target {
            paths.push(path.clone());
        } else {
            let edges = self.edges.read();
            if let Some(out_edges) = edges.get(&current) {
                for edge in out_edges.iter() {
                    if !visited.get(&edge.target).copied().unwrap_or(false) {
                        self.dfs_paths(edge.target, target, path, visited, paths, max_depth);
                    }
                }
            }
        }
        
        path.pop();
        visited.insert(current, false);
    }
    
    /// Berechnet kumulatives Gewicht eines Pfades
    fn path_weight(&self, path: &[u64]) -> f64 {
        let edges = self.edges.read();
        let mut total_weight = 1.0;
        
        for window in path.windows(2) {
            if let Some(out_edges) = edges.get(&window[0]) {
                if let Some(edge) = out_edges.iter().find(|e| e.target == window[1]) {
                    total_weight *= edge.weight.weight;
                }
            }
        }
        
        total_weight
    }
    
    /// Statistiken über das Netzwerk
    pub fn stats(&self) -> GabrielStats {
        let neurons = self.neurons.read();
        let edges = self.edges.read();
        
        let neuron_count = neurons.len();
        let edge_count: usize = edges.values().map(|v| v.len()).sum();
        
        let avg_weight: f64 = if edge_count > 0 {
            edges
                .values()
                .flat_map(|v| v.iter())
                .map(|e| e.weight.weight)
                .sum::<f64>()
                / edge_count as f64
        } else {
            0.0
        };
        
        let avg_activation: f64 = if neuron_count > 0 {
            neurons.values().map(|n| n.activation).sum::<f64>() / neuron_count as f64
        } else {
            0.0
        };
        
        GabrielStats {
            neuron_count,
            edge_count,
            avg_weight,
            avg_activation,
            time: *self.time.read(),
        }
    }
}

/// Netzwerk-Statistiken
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GabrielStats {
    pub neuron_count: usize,
    pub edge_count: usize,
    pub avg_weight: f64,
    pub avg_activation: f64,
    pub time: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::InformationQuantum;
    
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
    fn test_gabriel_cell_basic() {
        let config = GabrielConfig::default();
        let cell = GabrielCell::new(config);
        
        let q1 = TestQuantum { id: 1, value: 0.5 };
        let q2 = TestQuantum { id: 2, value: 0.7 };
        
        let n1 = cell.add_neuron(q1, [0.0, 0.0, 0.0, 0.0]);
        let n2 = cell.add_neuron(q2, [1.0, 0.0, 0.0, 0.1]);
        
        assert!(cell.connect(n1, n2, 0.5));
        
        let stats = cell.stats();
        assert_eq!(stats.neuron_count, 2);
        assert_eq!(stats.edge_count, 1);
    }
    
    #[test]
    fn test_hebbian_learning() {
        let config = GabrielConfig::default();
        let cell = GabrielCell::new(config);
        
        let q1 = TestQuantum { id: 1, value: 0.8 };
        let q2 = TestQuantum { id: 2, value: 0.9 };
        
        let n1 = cell.add_neuron(q1, [0.0, 0.0, 0.0, 0.0]);
        let n2 = cell.add_neuron(q2, [1.0, 0.0, 0.0, 0.1]);
        
        cell.connect(n1, n2, 0.3);
        
        // Aktiviere beide Neuronen
        cell.activate(n1, 1.0);
        cell.activate(n2, 1.0);
        
        let initial_weight = cell.edges.read()
            .get(&n1).unwrap()[0].weight.weight;
        
        // Hebbian Update
        cell.hebbian_update(n1, n2);
        
        let new_weight = cell.edges.read()
            .get(&n1).unwrap()[0].weight.weight;
        
        assert!(new_weight > initial_weight);
    }
}

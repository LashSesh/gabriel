# Anwendungsbeispiele

## 1. Text-Analyse & NLP

```rust
use organism::{GabrielOrganism, OrganismConfig};
use gabriel_core::InformationQuantum;
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct TextQuantum {
    id: u64,
    text: String,
    embeddings: Vec<f64>, // z.B. von BERT/GPT
    metadata: HashMap<String, String>,
}

impl InformationQuantum for TextQuantum {
    type Id = u64;
    
    fn id(&self) -> Self::Id {
        self.id
    }
    
    fn resonance(&self, other: &Self) -> f64 {
        // Cosine Similarity der Embeddings
        cosine_similarity(&self.embeddings, &other.embeddings)
    }
    
    fn energy(&self) -> f64 {
        // Energie basiert auf Text-Länge und semantischer Dichte
        let length_factor = (self.text.len() as f64 / 1000.0).tanh();
        let embedding_norm = l2_norm(&self.embeddings);
        (length_factor + embedding_norm) / 2.0
    }
    
    fn fuse(&self, other: &Self, weight: f64) -> Self {
        // Fusioniere Embeddings
        let fused_embeddings: Vec<f64> = self.embeddings
            .iter()
            .zip(other.embeddings.iter())
            .map(|(&a, &b)| a * (1.0 - weight) + b * weight)
            .collect();
        
        TextQuantum {
            id: self.id,
            text: format!("{} {}", self.text, other.text),
            embeddings: fused_embeddings,
            metadata: self.metadata.clone(),
        }
    }
}

fn cosine_similarity(a: &[f64], b: &[f64]) -> f64 {
    let dot: f64 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f64 = a.iter().map(|x| x * x).sum::<f64>().sqrt();
    let norm_b: f64 = b.iter().map(|x| x * x).sum::<f64>().sqrt();
    
    if norm_a == 0.0 || norm_b == 0.0 {
        0.0
    } else {
        dot / (norm_a * norm_b)
    }
}

fn l2_norm(v: &[f64]) -> f64 {
    v.iter().map(|x| x * x).sum::<f64>().sqrt()
}

fn main() {
    let config = OrganismConfig::default();
    let organism = GabrielOrganism::<TextQuantum>::new(config, 1.0);
    
    // Füttere mit Dokumenten
    let documents = vec![
        "Künstliche Intelligenz revolutioniert die Technologie",
        "Machine Learning ermöglicht neue Anwendungen",
        "Deep Learning nutzt neuronale Netze",
        // ... mehr Dokumente
    ];
    
    for (i, doc) in documents.iter().enumerate() {
        let quantum = TextQuantum {
            id: i as u64,
            text: doc.to_string(),
            embeddings: compute_embeddings(doc), // Ihre Embedding-Funktion
            metadata: HashMap::new(),
        };
        organism.feed(quantum);
    }
    
    // Lebenszyklen für emergente Muster
    for _ in 0..100 {
        organism.lifecycle();
    }
    
    // Diagnostik zeigt semantische Cluster
    let diagnostics = organism.diagnostics();
    println!("Gefundene Muster: {}", diagnostics.patterns.total_patterns);
}

fn compute_embeddings(_text: &str) -> Vec<f64> {
    // Placeholder - in Praxis: BERT, GPT, etc.
    vec![0.5; 768]
}
```

---

## 2. Zeitserien-Analyse

```rust
use organism::{GabrielOrganism, OrganismConfig};
use gabriel_core::InformationQuantum;
use chrono::{DateTime, Utc};

#[derive(Clone, Debug)]
struct TimeSeriesQuantum {
    id: u64,
    timestamp: DateTime<Utc>,
    values: Vec<f64>, // Multivariate Zeitreihe
    features: Vec<f64>, // Extrahierte Features (FFT, Statistiken, etc.)
}

impl InformationQuantum for TimeSeriesQuantum {
    type Id = u64;
    
    fn id(&self) -> Self::Id {
        self.id
    }
    
    fn resonance(&self, other: &Self) -> f64 {
        // DTW (Dynamic Time Warping) oder Kreuzkorrelation
        cross_correlation(&self.values, &other.values)
    }
    
    fn energy(&self) -> f64 {
        // Energie = Varianz + Trend-Stärke
        let variance = calculate_variance(&self.values);
        let trend = calculate_trend(&self.values);
        (variance + trend.abs()) / 2.0
    }
    
    fn fuse(&self, other: &Self, weight: f64) -> Self {
        let fused_values: Vec<f64> = self.values
            .iter()
            .zip(other.values.iter())
            .map(|(&a, &b)| a * (1.0 - weight) + b * weight)
            .collect();
        
        TimeSeriesQuantum {
            id: self.id,
            timestamp: self.timestamp,
            values: fused_values,
            features: vec![],
        }
    }
}

fn cross_correlation(a: &[f64], b: &[f64]) -> f64 {
    // Simplified - echte Implementierung würde FFT nutzen
    let len = a.len().min(b.len());
    let mut max_corr = 0.0;
    
    for lag in 0..len/2 {
        let mut corr = 0.0;
        for i in 0..len-lag {
            corr += a[i] * b[i+lag];
        }
        max_corr = max_corr.max(corr);
    }
    
    max_corr / len as f64
}

fn calculate_variance(data: &[f64]) -> f64 {
    let mean = data.iter().sum::<f64>() / data.len() as f64;
    data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / data.len() as f64
}

fn calculate_trend(data: &[f64]) -> f64 {
    if data.len() < 2 {
        return 0.0;
    }
    // Linear regression slope
    let n = data.len() as f64;
    let sum_x: f64 = (0..data.len()).map(|i| i as f64).sum();
    let sum_y: f64 = data.iter().sum();
    let sum_xy: f64 = data.iter().enumerate().map(|(i, &y)| i as f64 * y).sum();
    let sum_x2: f64 = (0..data.len()).map(|i| (i as f64).powi(2)).sum();
    
    (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x.powi(2))
}

fn main() {
    let config = OrganismConfig::default();
    let organism = GabrielOrganism::<TimeSeriesQuantum>::new(config, 1.0);
    
    // Streaming Zeitserien-Daten
    for window in sliding_window_data() {
        let quantum = TimeSeriesQuantum {
            id: window.id,
            timestamp: window.timestamp,
            values: window.values,
            features: extract_features(&window.values),
        };
        organism.feed(quantum);
        
        // Regelmäßige Lebenszyklen
        organism.lifecycle();
    }
    
    // Emergente Muster = Anomalien, Periodizitäten, etc.
    let diagnostics = organism.diagnostics();
    println!("Consciousness: {:.2}%", diagnostics.state.consciousness * 100.0);
}

fn sliding_window_data() -> Vec<TimeSeriesQuantum> {
    vec![] // Placeholder
}

fn extract_features(_values: &[f64]) -> Vec<f64> {
    vec![0.0; 10] // Placeholder
}
```

---

## 3. Blockchain Forensik (PHOSPHOROS Integration)

```rust
use organism::{GabrielOrganism, OrganismConfig};
use gabriel_core::InformationQuantum;

#[derive(Clone, Debug)]
struct TransactionQuantum {
    id: u64,
    tx_hash: String,
    from: String,
    to: String,
    value: f64,
    timestamp: u64,
    gas_used: u64,
    // Graph-Features
    degree_centrality: f64,
    betweenness: f64,
    clustering_coeff: f64,
}

impl InformationQuantum for TransactionQuantum {
    type Id = u64;
    
    fn id(&self) -> Self::Id {
        self.id
    }
    
    fn resonance(&self, other: &Self) -> f64 {
        // Transaktionen resonieren wenn sie:
        // 1. Gleiche Adressen involvieren
        // 2. Ähnliche Werte haben
        // 3. Zeitlich nah sind
        
        let address_similarity = if self.from == other.from || self.to == other.to {
            1.0
        } else {
            0.0
        };
        
        let value_similarity = 1.0 / (1.0 + (self.value - other.value).abs() / self.value.max(other.value));
        let time_similarity = 1.0 / (1.0 + ((self.timestamp as f64 - other.timestamp as f64).abs() / 86400.0));
        
        (address_similarity + value_similarity + time_similarity) / 3.0
    }
    
    fn energy(&self) -> f64 {
        // Energie = Transaktionswert + Graph-Zentralität
        let value_factor = (self.value / 1000.0).tanh();
        let centrality_factor = (self.degree_centrality + self.betweenness) / 2.0;
        
        (value_factor + centrality_factor) / 2.0
    }
    
    fn fuse(&self, other: &Self, weight: f64) -> Self {
        TransactionQuantum {
            id: self.id,
            tx_hash: self.tx_hash.clone(),
            from: self.from.clone(),
            to: self.to.clone(),
            value: self.value * (1.0 - weight) + other.value * weight,
            timestamp: self.timestamp,
            gas_used: self.gas_used,
            degree_centrality: self.degree_centrality * (1.0 - weight) + other.degree_centrality * weight,
            betweenness: self.betweenness * (1.0 - weight) + other.betweenness * weight,
            clustering_coeff: self.clustering_coeff * (1.0 - weight) + other.clustering_coeff * weight,
        }
    }
}

fn main() {
    let config = OrganismConfig::default();
    let organism = GabrielOrganism::<TransactionQuantum>::new(config, 1.0);
    
    // Füttere mit Blockchain-Daten
    for tx in fetch_transactions() {
        let quantum = TransactionQuantum {
            id: tx.id,
            tx_hash: tx.hash,
            from: tx.from,
            to: tx.to,
            value: tx.value,
            timestamp: tx.timestamp,
            gas_used: tx.gas,
            degree_centrality: compute_centrality(&tx),
            betweenness: compute_betweenness(&tx),
            clustering_coeff: compute_clustering(&tx),
        };
        
        organism.feed(quantum);
    }
    
    // Emergente Muster = verdächtige Cluster, Geldwäsche-Netzwerke
    for _ in 0..100 {
        organism.lifecycle();
    }
    
    let diagnostics = organism.diagnostics();
    println!("Gefundene verdächtige Muster: {}", diagnostics.patterns.total_patterns);
}

fn fetch_transactions() -> Vec<TransactionQuantum> {
    vec![] // Placeholder
}

fn compute_centrality(_tx: &TransactionQuantum) -> f64 { 0.5 }
fn compute_betweenness(_tx: &TransactionQuantum) -> f64 { 0.5 }
fn compute_clustering(_tx: &TransactionQuantum) -> f64 { 0.5 }
```

---

## 4. Bioinformatik (DNA-Sequenzen)

```rust
use organism::{GabrielOrganism, OrganismConfig};
use gabriel_core::InformationQuantum;

#[derive(Clone, Debug)]
struct DNAQuantum {
    id: u64,
    sequence: String, // ATCG
    k_mers: Vec<String>, // k-mer Decomposition
    gc_content: f64,
    complexity: f64,
}

impl InformationQuantum for DNAQuantum {
    type Id = u64;
    
    fn id(&self) -> Self::Id {
        self.id
    }
    
    fn resonance(&self, other: &Self) -> f64 {
        // Smith-Waterman oder Needleman-Wunsch Alignment
        // Simplified: k-mer overlap
        let overlap = self.k_mers.iter()
            .filter(|kmer| other.k_mers.contains(kmer))
            .count();
        
        overlap as f64 / self.k_mers.len().max(other.k_mers.len()) as f64
    }
    
    fn energy(&self) -> f64 {
        // Energie = GC-Content + Komplexität
        (self.gc_content + self.complexity) / 2.0
    }
    
    fn fuse(&self, other: &Self, weight: f64) -> Self {
        // Consensus sequence
        let fused_sequence = if weight > 0.5 {
            other.sequence.clone()
        } else {
            self.sequence.clone()
        };
        
        DNAQuantum {
            id: self.id,
            sequence: fused_sequence.clone(),
            k_mers: extract_kmers(&fused_sequence, 6),
            gc_content: self.gc_content * (1.0 - weight) + other.gc_content * weight,
            complexity: self.complexity * (1.0 - weight) + other.complexity * weight,
        }
    }
}

fn extract_kmers(seq: &str, k: usize) -> Vec<String> {
    if seq.len() < k {
        return vec![];
    }
    
    (0..=seq.len()-k)
        .map(|i| seq[i..i+k].to_string())
        .collect()
}

fn calculate_gc_content(seq: &str) -> f64 {
    let gc_count = seq.chars().filter(|&c| c == 'G' || c == 'C').count();
    gc_count as f64 / seq.len() as f64
}

fn main() {
    let config = OrganismConfig::default();
    let organism = GabrielOrganism::<DNAQuantum>::new(config, 1.0);
    
    // Füttere mit DNA-Sequenzen
    let sequences = vec![
        "ATCGATCGATCG",
        "GCTAGCTAGCTA",
        "TTAATTAATTAA",
        // ... mehr Sequenzen
    ];
    
    for (i, seq) in sequences.iter().enumerate() {
        let quantum = DNAQuantum {
            id: i as u64,
            sequence: seq.to_string(),
            k_mers: extract_kmers(seq, 6),
            gc_content: calculate_gc_content(seq),
            complexity: 0.5, // Shannon-Entropie der Sequenz
        };
        organism.feed(quantum);
    }
    
    // Emergente Muster = konservierte Regionen, Motive
    for _ in 0..100 {
        organism.lifecycle();
    }
}
```

---

## 5. Multi-Modal Learning (Vision + Text)

```rust
use organism::{GabrielOrganism, OrganismConfig};
use gabriel_core::InformationQuantum;

#[derive(Clone, Debug)]
struct MultiModalQuantum {
    id: u64,
    image_features: Vec<f64>, // CNN features
    text_features: Vec<f64>,  // BERT features
    modality_weights: (f64, f64), // (image_weight, text_weight)
}

impl InformationQuantum for MultiModalQuantum {
    type Id = u64;
    
    fn id(&self) -> Self::Id {
        self.id
    }
    
    fn resonance(&self, other: &Self) -> f64 {
        // Multi-modal similarity
        let img_sim = cosine_similarity(&self.image_features, &other.image_features);
        let txt_sim = cosine_similarity(&self.text_features, &other.text_features);
        
        // Gewichtete Kombination
        img_sim * self.modality_weights.0 + txt_sim * self.modality_weights.1
    }
    
    fn energy(&self) -> f64 {
        let img_energy = l2_norm(&self.image_features);
        let txt_energy = l2_norm(&self.text_features);
        
        (img_energy + txt_energy) / 2.0
    }
    
    fn fuse(&self, other: &Self, weight: f64) -> Self {
        let fused_img: Vec<f64> = self.image_features
            .iter()
            .zip(other.image_features.iter())
            .map(|(&a, &b)| a * (1.0 - weight) + b * weight)
            .collect();
        
        let fused_txt: Vec<f64> = self.text_features
            .iter()
            .zip(other.text_features.iter())
            .map(|(&a, &b)| a * (1.0 - weight) + b * weight)
            .collect();
        
        MultiModalQuantum {
            id: self.id,
            image_features: fused_img,
            text_features: fused_txt,
            modality_weights: self.modality_weights,
        }
    }
}

fn cosine_similarity(a: &[f64], b: &[f64]) -> f64 {
    let dot: f64 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f64 = a.iter().map(|x| x * x).sum::<f64>().sqrt();
    let norm_b: f64 = b.iter().map(|x| x * x).sum::<f64>().sqrt();
    
    if norm_a == 0.0 || norm_b == 0.0 {
        0.0
    } else {
        dot / (norm_a * norm_b)
    }
}

fn l2_norm(v: &[f64]) -> f64 {
    v.iter().map(|x| x * x).sum::<f64>().sqrt()
}
```

---

## Integration mit MEF-Core

```rust
// Integration mit Vektor-Datenbank
use organism::{GabrielOrganism, OrganismConfig};

fn integrate_with_mef_core() {
    let organism = GabrielOrganism::new(OrganismConfig::default(), 1.0);
    
    // 1. Empfange Vektoren von MEF-Core
    let vectors = fetch_from_mef_core();
    
    // 2. Konvertiere zu Quanten
    for vec in vectors {
        let quantum = UniversalQuantum::from_vector(&vec);
        organism.feed(quantum);
    }
    
    // 3. Lebenszyklen
    organism.lifecycle();
    
    // 4. Emergente Muster zurück zu MEF-Core
    let patterns = organism.diagnostics().patterns;
    store_patterns_to_mef_core(patterns);
}

fn fetch_from_mef_core() -> Vec<Vec<f64>> {
    vec![] // Placeholder
}

fn store_patterns_to_mef_core(_patterns: emergence::PatternStats) {
    // Speichere in MEF-Core
}
```

---

Diese Beispiele zeigen die **volle Flexibilität** und **Domain-Agnostizität** des Systems!

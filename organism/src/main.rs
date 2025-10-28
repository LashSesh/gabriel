use organism::{GabrielOrganism, OrganismConfig};
use gabriel_core::InformationQuantum;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use tracing::{info, Level};
use tracing_subscriber;
use clap::Parser;
use anyhow::Result;
use std::time::Duration;

/// Universelles Informations-Quantum
/// Domain-agnostisch: Kann Text, Zahlen, Vektoren etc. repräsentieren
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UniversalQuantum {
    /// Eindeutiger Hash
    id: u64,
    
    /// Rohdaten (bytes)
    data: Vec<u8>,
    
    /// Metadaten
    metadata: QuantumMetadata,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QuantumMetadata {
    /// Typ-Hint (optional)
    pub type_hint: String,
    
    /// Zeitstempel der Erstellung
    pub timestamp: f64,
    
    /// Priorität (0.0 = niedrig, 1.0 = hoch)
    pub priority: f64,
}

impl UniversalQuantum {
    /// Erstellt Quantum aus beliebigen Bytes
    pub fn from_bytes(data: Vec<u8>, type_hint: &str) -> Self {
        let id = Self::compute_hash(&data);
        Self {
            id,
            data: data.clone(),
            metadata: QuantumMetadata {
                type_hint: type_hint.to_string(),
                timestamp: 0.0,
                priority: 0.5,
            },
        }
    }
    
    /// Erstellt Quantum aus Text
    pub fn from_text(text: &str) -> Self {
        Self::from_bytes(text.as_bytes().to_vec(), "text")
    }
    
    /// Erstellt Quantum aus Zahlen-Vektor
    pub fn from_vector(vec: &[f64]) -> Self {
        let bytes: Vec<u8> = vec
            .iter()
            .flat_map(|&f| f.to_le_bytes())
            .collect();
        Self::from_bytes(bytes, "vector")
    }
    
    /// Berechnet Hash
    fn compute_hash(data: &[u8]) -> u64 {
        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        hasher.finish()
    }
    
    /// Konvertiert zu f64-Vektor (für numerische Operationen)
    fn as_float_vector(&self) -> Vec<f64> {
        self.data
            .chunks(8)
            .filter_map(|chunk| {
                if chunk.len() == 8 {
                    let mut bytes = [0u8; 8];
                    bytes.copy_from_slice(chunk);
                    Some(f64::from_le_bytes(bytes))
                } else {
                    None
                }
            })
            .collect()
    }
}

impl InformationQuantum for UniversalQuantum {
    type Id = u64;
    
    fn id(&self) -> Self::Id {
        self.id
    }
    
    fn resonance(&self, other: &Self) -> f64 {
        // Berechne Ähnlichkeit basierend auf Byte-Übereinstimmung
        let len = self.data.len().min(other.data.len());
        if len == 0 {
            return 0.0;
        }
        
        let matches = self.data.iter()
            .zip(other.data.iter())
            .take(len)
            .filter(|(a, b)| a == b)
            .count();
        
        matches as f64 / len as f64
    }
    
    fn energy(&self) -> f64 {
        // Energie basiert auf Datenmenge und Priorität
        let size_factor = (self.data.len() as f64 / 1000.0).tanh();
        let priority_factor = self.metadata.priority;
        
        (size_factor + priority_factor) / 2.0
    }
    
    fn fuse(&self, other: &Self, weight: f64) -> Self {
        // Fusioniere Daten (gewichteter Durchschnitt wenn numerisch)
        let self_vec = self.as_float_vector();
        let other_vec = other.as_float_vector();
        
        let fused_vec: Vec<f64> = if !self_vec.is_empty() && !other_vec.is_empty() {
            let len = self_vec.len().min(other_vec.len());
            self_vec.iter()
                .zip(other_vec.iter())
                .take(len)
                .map(|(&a, &b)| a * (1.0 - weight) + b * weight)
                .collect()
        } else {
            // Fallback: Byte-weise Fusion
            self.data.iter()
                .zip(other.data.iter())
                .map(|(&a, &b)| ((a as f64 * (1.0 - weight) + b as f64 * weight) as u8))
                .collect::<Vec<u8>>()
                .chunks(8)
                .filter_map(|chunk| {
                    if chunk.len() == 8 {
                        let mut bytes = [0u8; 8];
                        bytes.copy_from_slice(chunk);
                        Some(f64::from_le_bytes(bytes))
                    } else {
                        None
                    }
                })
                .collect()
        };
        
        UniversalQuantum::from_vector(&fused_vec)
    }
}

/// CLI Argumente
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Anzahl der Lebenszyklen
    #[arg(short, long, default_value_t = 100)]
    cycles: u64,
    
    /// Anzahl der Quanten zum Füttern
    #[arg(short, long, default_value_t = 50)]
    feed_count: u64,
    
    /// Initiale Trichter-Radius
    #[arg(short, long, default_value_t = 1.0)]
    radius: f64,
    
    /// Output-Datei für Diagnostik (JSON)
    #[arg(short, long)]
    output: Option<String>,
    
    /// Verbose Logging
    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    // Setup Logging
    let level = if args.verbose { Level::DEBUG } else { Level::INFO };
    tracing_subscriber::fmt()
        .with_max_level(level)
        .with_target(false)
        .init();
    
    info!("╔════════════════════════════════════════════════════════════╗");
    info!("║       GABRIEL ORGANISM - Metabolic Information System      ║");
    info!("╚════════════════════════════════════════════════════════════╝");
    
    // Erstelle Organismus
    let config = OrganismConfig::default();
    let organism = GabrielOrganism::<UniversalQuantum>::new(config, args.radius);
    
    info!("\n🌱 Organism created with initial radius: {}", args.radius);
    
    // Füttere den Organismus
    info!("\n🍽️  Feeding organism with {} quanta...", args.feed_count);
    for i in 0..args.feed_count {
        // Erstelle diverse Quanten
        let quantum = match i % 4 {
            0 => UniversalQuantum::from_text(&format!("Information packet {}", i)),
            1 => UniversalQuantum::from_vector(&[(i as f64 * 0.1).sin(), (i as f64 * 0.2).cos()]),
            2 => UniversalQuantum::from_bytes(vec![i as u8; 16], "binary"),
            _ => UniversalQuantum::from_text(&format!("Knowledge unit {}", i)),
        };
        
        organism.feed(quantum);
        
        if i % 10 == 0 {
            info!("  Fed {} / {} quanta", i + 1, args.feed_count);
        }
    }
    
    // Lebenszyklen
    info!("\n🔄 Running {} lifecycle iterations...", args.cycles);
    for cycle in 0..args.cycles {
        organism.lifecycle();
        
        if cycle % 10 == 0 {
            let diagnostics = organism.diagnostics();
            info!(
                "  Cycle {}/{} - Health: {:.1}%, Consciousness: {:.1}%, Neurons: {}",
                cycle + 1,
                args.cycles,
                diagnostics.state.health * 100.0,
                diagnostics.state.consciousness * 100.0,
                diagnostics.metabolism.neuron_count
            );
        }
        
        if !organism.is_alive() {
            info!("\n💀 Organism died at cycle {}", cycle);
            break;
        }
        
        // Simuliere Zeitverzögerung
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    
    // Finale Diagnostik
    let diagnostics = organism.diagnostics();
    println!("\n{}", diagnostics.report());
    
    // Export zu JSON wenn gewünscht
    if let Some(output_path) = args.output {
        let state_json = organism.export_state();
        std::fs::write(&output_path, serde_json::to_string_pretty(&state_json)?)?;
        info!("\n💾 State exported to: {}", output_path);
    }
    
    if organism.is_alive() {
        info!("\n✅ Organism survived {} cycles!", args.cycles);
        info!("   Final Health: {:.1}%", organism.health() * 100.0);
        info!("   Final Consciousness: {:.1}%", organism.consciousness() * 100.0);
    } else {
        info!("\n⚠️  Organism did not survive the full duration");
    }
    
    Ok(())
}

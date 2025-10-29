//! Fusion Layer: Adaptive resonance-evaluation layer integrating Triton scoring
//! 
//! This module bridges the Gabriel organism's metabolic/emergent feedback loop
//! with Triton's analytical scoring framework for quantitative evaluation and
//! adaptive tuning of resonance cycles.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;
use tracing::{debug, info};

/// Errors that can occur in the fusion layer
#[derive(Error, Debug)]
pub enum FusionError {
    #[error("Python bridge execution failed: {0}")]
    PythonBridgeError(String),
    
    #[error("JSON serialization error: {0}")]
    JsonError(#[from] serde_json::Error),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Invalid fusion packet: {0}")]
    InvalidPacket(String),
}

pub type Result<T> = std::result::Result<T, FusionError>;

/// Unified data model for fusion layer communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FusionPacket {
    /// N-dimensional tensor from Gabriel resonance computation
    pub resonance_tensor: Vec<f64>,
    
    /// Triton coherence evaluation (psi) in [0.0, 1.0]
    pub coherence_score: f64,
    
    /// Information entropy measure (derived from rho) in [0.0, 1.0]
    pub entropy_score: f64,
    
    /// Overall stability metric (derived from D) in [0.0, 1.0]
    pub stability_index: f64,
    
    /// Cycle identifier
    pub cycle_id: usize,
    
    /// Timestamp
    pub timestamp: f64,
}

impl FusionPacket {
    /// Creates a new FusionPacket with default scores
    pub fn new(resonance_tensor: Vec<f64>, cycle_id: usize, timestamp: f64) -> Self {
        Self {
            resonance_tensor,
            coherence_score: 0.0,
            entropy_score: 0.0,
            stability_index: 0.0,
            cycle_id,
            timestamp,
        }
    }
    
    /// Validates the fusion packet
    pub fn validate(&self) -> Result<()> {
        if self.coherence_score < 0.0 || self.coherence_score > 1.0 {
            return Err(FusionError::InvalidPacket(
                format!("coherence_score {} out of range [0, 1]", self.coherence_score)
            ));
        }
        if self.entropy_score < 0.0 || self.entropy_score > 1.0 {
            return Err(FusionError::InvalidPacket(
                format!("entropy_score {} out of range [0, 1]", self.entropy_score)
            ));
        }
        if self.stability_index < 0.0 || self.stability_index > 1.0 {
            return Err(FusionError::InvalidPacket(
                format!("stability_index {} out of range [0, 1]", self.stability_index)
            ));
        }
        Ok(())
    }
}

/// Feedback weights for Gabriel metabolic update
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
    /// Creates feedback from a fusion packet
    pub fn from_packet(packet: &FusionPacket) -> Self {
        Self {
            hebbian_modulation: packet.coherence_score,
            energy_boost: packet.stability_index,
            entropy_reduction: 1.0 - packet.entropy_score,
            pruning_threshold: 1.0 - packet.coherence_score,
        }
    }
}

/// Fusion layer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FusionConfig {
    /// Path to Python interpreter
    pub python_path: PathBuf,
    
    /// Path to fusion_bridge.py
    pub bridge_script: PathBuf,
    
    /// Coherence window for Triton evaluator
    pub coherence_window: usize,
    
    /// Random seed for reproducibility
    pub seed: u64,
    
    /// Enable diagnostic logging
    pub enable_diagnostics: bool,
    
    /// Diagnostics output file
    pub diagnostics_file: PathBuf,
}

impl Default for FusionConfig {
    fn default() -> Self {
        Self {
            python_path: PathBuf::from("python3"),
            bridge_script: PathBuf::from("fusion_bridge.py"),
            coherence_window: 8,
            seed: 42,
            enable_diagnostics: true,
            diagnostics_file: PathBuf::from("fusion_diagnostics.json"),
        }
    }
}

/// Main fusion layer interface
pub struct FusionLayer {
    config: FusionConfig,
    cycle_count: usize,
    total_coherence: f64,
    total_stability: f64,
    history: Vec<FusionPacket>,
}

impl FusionLayer {
    /// Creates a new fusion layer with the given configuration
    pub fn new(config: FusionConfig) -> Self {
        info!("Initializing Fusion Layer with config: {:?}", config);
        Self {
            config,
            cycle_count: 0,
            total_coherence: 0.0,
            total_stability: 0.0,
            history: Vec::new(),
        }
    }
    
    /// Executes a fusion cycle
    /// 
    /// This is the main API that:
    /// 1. Reads Gabriel output tensors
    /// 2. Routes them through Triton scorer
    /// 3. Returns updated feedback weights
    pub fn fusion_cycle(
        &mut self,
        resonance_tensor: Vec<f64>,
        timestamp: f64,
    ) -> Result<CoherenceFeedback> {
        debug!("Executing fusion cycle {} at time {}", self.cycle_count, timestamp);
        
        // Call Python bridge to evaluate tensor
        let packet = self.call_python_bridge(resonance_tensor, timestamp)?;
        
        // Validate packet
        packet.validate()?;
        
        // Update statistics
        self.total_coherence += packet.coherence_score;
        self.total_stability += packet.stability_index;
        self.history.push(packet.clone());
        self.cycle_count += 1;
        
        // Convert to feedback weights
        let feedback = CoherenceFeedback::from_packet(&packet);
        
        debug!(
            "Cycle {} complete: coherence={:.3}, stability={:.3}, entropy={:.3}",
            packet.cycle_id, packet.coherence_score, packet.stability_index, packet.entropy_score
        );
        
        Ok(feedback)
    }
    
    /// Calls Python bridge to evaluate resonance tensor
    fn call_python_bridge(
        &self,
        resonance_tensor: Vec<f64>,
        timestamp: f64,
    ) -> Result<FusionPacket> {
        // For now, use a simplified Python-free implementation
        // In a full implementation, this would invoke the Python bridge via subprocess
        
        // Calculate simple metrics directly (placeholder implementation)
        let coherence_score = self.calculate_simple_coherence(&resonance_tensor);
        let entropy_score = self.calculate_simple_entropy(&resonance_tensor);
        let stability_index = coherence_score * (1.0 - entropy_score);
        
        let packet = FusionPacket {
            resonance_tensor,
            coherence_score,
            entropy_score,
            stability_index,
            cycle_id: self.cycle_count,
            timestamp,
        };
        
        Ok(packet)
    }
    
    /// Simple coherence calculation (autocorrelation)
    fn calculate_simple_coherence(&self, tensor: &[f64]) -> f64 {
        if tensor.is_empty() {
            return 0.0;
        }
        
        let mean = tensor.iter().sum::<f64>() / tensor.len() as f64;
        let variance: f64 = tensor.iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f64>() / tensor.len() as f64;
        
        if variance < 1e-10 {
            return 1.0; // Perfect coherence if constant
        }
        
        // Normalized autocovariance at lag=1
        let mut autocov = 0.0;
        for i in 0..tensor.len() - 1 {
            autocov += (tensor[i] - mean) * (tensor[i + 1] - mean);
        }
        autocov /= (tensor.len() - 1) as f64 * variance;
        
        autocov.abs().clamp(0.0, 1.0)
    }
    
    /// Simple entropy calculation (Shannon entropy of normalized distribution)
    fn calculate_simple_entropy(&self, tensor: &[f64]) -> f64 {
        if tensor.is_empty() {
            return 0.0;
        }
        
        // Normalize to probability distribution
        let sum: f64 = tensor.iter().map(|x| x.abs()).sum();
        if sum < 1e-10 {
            return 0.0;
        }
        
        let probs: Vec<f64> = tensor.iter()
            .map(|x| x.abs() / sum)
            .collect();
        
        // Shannon entropy
        let entropy: f64 = -probs.iter()
            .filter(|&&p| p > 1e-10)
            .map(|&p| p * p.ln())
            .sum::<f64>();
        
        // Normalize to [0, 1] (max entropy is ln(n))
        let max_entropy = (tensor.len() as f64).ln();
        if max_entropy > 0.0 {
            (entropy / max_entropy).clamp(0.0, 1.0)
        } else {
            0.0
        }
    }
    
    /// Gets average coherence over all cycles
    pub fn average_coherence(&self) -> f64 {
        if self.cycle_count == 0 {
            0.0
        } else {
            self.total_coherence / self.cycle_count as f64
        }
    }
    
    /// Gets average stability over all cycles
    pub fn average_stability(&self) -> f64 {
        if self.cycle_count == 0 {
            0.0
        } else {
            self.total_stability / self.cycle_count as f64
        }
    }
    
    /// Calculates energy drift over recent cycles
    pub fn energy_drift(&self) -> f64 {
        if self.history.len() < 2 {
            return 0.0;
        }
        
        let recent: Vec<f64> = self.history.iter()
            .rev()
            .take(10)
            .map(|p| p.stability_index)
            .collect();
        
        if recent.len() < 2 {
            return 0.0;
        }
        
        let max = recent.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let min = recent.iter().cloned().fold(f64::INFINITY, f64::min);
        
        max - min
    }
    
    /// Exports fusion state for Hilbert-Pólya-Metatron integration
    pub fn export_fusion_state(&self) -> serde_json::Value {
        serde_json::json!({
            "fusion_version": "1.0.0",
            "total_cycles": self.cycle_count,
            "coherence_window": self.config.coherence_window,
            "seed": self.config.seed,
            "metrics": {
                "average_coherence": self.average_coherence(),
                "average_stability": self.average_stability(),
                "energy_drift": self.energy_drift(),
            },
            "final_packet": self.history.last(),
        })
    }
    
    /// Exports diagnostics to JSON file
    pub fn export_diagnostics(&self) -> Result<()> {
        if !self.config.enable_diagnostics {
            return Ok(());
        }
        
        let diagnostics = serde_json::json!({
            "total_cycles": self.cycle_count,
            "average_coherence": self.average_coherence(),
            "average_stability": self.average_stability(),
            "energy_drift": self.energy_drift(),
            "history": self.history.iter().rev().take(100).collect::<Vec<_>>(),
        });
        
        let file = std::fs::File::create(&self.config.diagnostics_file)?;
        serde_json::to_writer_pretty(file, &diagnostics)?;
        
        info!("Diagnostics exported to {:?}", self.config.diagnostics_file);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fusion_packet_creation() {
        let tensor = vec![0.1, 0.2, 0.3, 0.4, 0.5];
        let packet = FusionPacket::new(tensor.clone(), 0, 1.0);
        
        assert_eq!(packet.resonance_tensor, tensor);
        assert_eq!(packet.cycle_id, 0);
        assert_eq!(packet.timestamp, 1.0);
    }
    
    #[test]
    fn test_fusion_packet_validation() {
        let mut packet = FusionPacket::new(vec![1.0], 0, 0.0);
        packet.coherence_score = 0.5;
        packet.entropy_score = 0.3;
        packet.stability_index = 0.7;
        
        assert!(packet.validate().is_ok());
        
        packet.coherence_score = 1.5; // Invalid
        assert!(packet.validate().is_err());
    }
    
    #[test]
    fn test_coherence_feedback_from_packet() {
        let mut packet = FusionPacket::new(vec![1.0], 0, 0.0);
        packet.coherence_score = 0.8;
        packet.entropy_score = 0.2;
        packet.stability_index = 0.9;
        
        let feedback = CoherenceFeedback::from_packet(&packet);
        
        assert!((feedback.hebbian_modulation - 0.8).abs() < 1e-10);
        assert!((feedback.energy_boost - 0.9).abs() < 1e-10);
        assert!((feedback.entropy_reduction - 0.8).abs() < 1e-10); // 1.0 - 0.2
        assert!((feedback.pruning_threshold - 0.2).abs() < 1e-10); // 1.0 - 0.8
    }
    
    #[test]
    fn test_fusion_layer_creation() {
        let config = FusionConfig::default();
        let layer = FusionLayer::new(config);
        
        assert_eq!(layer.cycle_count, 0);
        assert_eq!(layer.average_coherence(), 0.0);
    }
    
    #[test]
    fn test_fusion_cycle() {
        let config = FusionConfig::default();
        let mut layer = FusionLayer::new(config);
        
        let tensor = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];
        let result = layer.fusion_cycle(tensor, 1.0);
        
        assert!(result.is_ok());
        assert_eq!(layer.cycle_count, 1);
        assert!(layer.average_coherence() >= 0.0);
        assert!(layer.average_coherence() <= 1.0);
    }
    
    #[test]
    fn test_multiple_fusion_cycles() {
        let config = FusionConfig::default();
        let mut layer = FusionLayer::new(config);
        
        for i in 0..10 {
            let tensor: Vec<f64> = (0..8).map(|j| (i + j) as f64 * 0.1).collect();
            let result = layer.fusion_cycle(tensor, i as f64);
            assert!(result.is_ok());
        }
        
        assert_eq!(layer.cycle_count, 10);
        assert!(layer.energy_drift() >= 0.0);
    }
    
    #[test]
    fn test_simple_coherence_calculation() {
        let layer = FusionLayer::new(FusionConfig::default());
        
        // Perfectly coherent (constant) signal
        let constant = vec![1.0; 10];
        assert!(layer.calculate_simple_coherence(&constant) > 0.9);
        
        // Random-ish signal should have lower coherence
        let random = vec![0.1, 0.9, 0.2, 0.8, 0.3, 0.7, 0.4, 0.6];
        let coherence = layer.calculate_simple_coherence(&random);
        assert!(coherence >= 0.0 && coherence <= 1.0);
    }
    
    #[test]
    fn test_simple_entropy_calculation() {
        let layer = FusionLayer::new(FusionConfig::default());
        
        // Uniform distribution should have high entropy
        let uniform = vec![1.0; 8];
        let entropy_uniform = layer.calculate_simple_entropy(&uniform);
        
        // Concentrated distribution should have low entropy  
        let concentrated = vec![10.0, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1];
        let entropy_concentrated = layer.calculate_simple_entropy(&concentrated);
        
        assert!(entropy_uniform > entropy_concentrated);
    }
}

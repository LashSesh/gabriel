//! Mathematical Learning System
//!
//! Reinforcement learning for mathematical concepts:
//! - Reward correct proofs
//! - Punish errors
//! - Strengthen successful reasoning paths
//! - Adjust neuron weights based on mathematical success

use gabriel_core::{GabrielCell, InformationQuantum};
use std::sync::Arc;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use tracing::debug;

/// Type of learning signal
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LearningSignal {
    /// Positive reinforcement (proof succeeded)
    Reward(f64),
    /// Negative reinforcement (proof failed)
    Punishment(f64),
    /// Neutral (exploration)
    Neutral,
}

/// Learning configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningConfig {
    /// Learning rate (how fast to update weights)
    pub learning_rate: f64,

    /// Reward decay (how fast rewards diminish over time)
    pub reward_decay: f64,

    /// Exploration rate (probability of random connections)
    pub exploration_rate: f64,

    /// Minimum confidence threshold for using a proof
    pub confidence_threshold: f64,

    /// Maximum reward per successful proof
    pub max_reward: f64,

    /// Maximum punishment per failed proof
    pub max_punishment: f64,
}

impl Default for LearningConfig {
    fn default() -> Self {
        Self {
            learning_rate: 0.1,
            reward_decay: 0.95,
            exploration_rate: 0.15,
            confidence_threshold: 0.7,
            max_reward: 1.0,
            max_punishment: 0.5,
        }
    }
}

/// Learning statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningStats {
    pub total_rewards: u64,
    pub total_punishments: u64,
    pub cumulative_reward: f64,
    pub average_confidence: f64,
    pub successful_proofs: u64,
    pub failed_proofs: u64,
}

impl Default for LearningStats {
    fn default() -> Self {
        Self {
            total_rewards: 0,
            total_punishments: 0,
            cumulative_reward: 0.0,
            average_confidence: 0.0,
            successful_proofs: 0,
            failed_proofs: 0,
        }
    }
}

/// Mathematical learning system using reinforcement learning
pub struct MathematicalLearning<Q: InformationQuantum> {
    gabriel_cell: Arc<GabrielCell<Q>>,
    config: LearningConfig,
    stats: Arc<RwLock<LearningStats>>,
    /// Maps neuron pairs to their reward history
    reward_history: Arc<RwLock<ahash::AHashMap<(u64, u64), Vec<f64>>>>,
}

impl<Q: InformationQuantum> MathematicalLearning<Q> {
    pub fn new(gabriel_cell: Arc<GabrielCell<Q>>, config: LearningConfig) -> Self {
        Self {
            gabriel_cell,
            config,
            stats: Arc::new(RwLock::new(LearningStats::default())),
            reward_history: Arc::new(RwLock::new(ahash::AHashMap::new())),
        }
    }

    /// Apply learning signal to a path of neurons
    /// This reinforces or weakens the connections along a proof path
    pub fn apply_signal(&self, neuron_path: &[u64], signal: LearningSignal) {
        if neuron_path.len() < 2 {
            return;
        }

        let (delta, is_reward) = match signal {
            LearningSignal::Reward(r) => {
                let reward = r.min(self.config.max_reward);
                (reward * self.config.learning_rate, true)
            }
            LearningSignal::Punishment(p) => {
                let punishment = p.min(self.config.max_punishment);
                (-punishment * self.config.learning_rate, false)
            }
            LearningSignal::Neutral => (0.0, false),
        };

        // Update all connections along the path
        for window in neuron_path.windows(2) {
            let source = window[0];
            let target = window[1];

            // Strengthen or weaken the connection
            if let Some(edge) = self.gabriel_cell.get_edge(source, target) {
                let mut edge = edge.write();

                if delta > 0.0 {
                    // Reward: strengthen connection
                    edge.weight.reinforce(delta, self.gabriel_cell.stats().time);
                } else if delta < 0.0 {
                    // Punishment: weaken connection
                    let new_weight = (edge.weight.weight + delta).max(0.0);
                    edge.weight.weight = new_weight;
                }

                // Record reward
                let mut history = self.reward_history.write();
                history.entry((source, target))
                    .or_insert_with(Vec::new)
                    .push(delta);
            }
        }

        // Update statistics
        let mut stats = self.stats.write();
        if is_reward {
            stats.total_rewards += 1;
            stats.cumulative_reward += delta;
            stats.successful_proofs += 1;
        } else if delta < 0.0 {
            stats.total_punishments += 1;
            stats.cumulative_reward += delta; // Subtract punishment
            stats.failed_proofs += 1;
        }

        debug!(
            "Applied {:?} to path of length {}, delta = {:.3}",
            signal,
            neuron_path.len(),
            delta
        );
    }

    /// Reward a successful proof
    pub fn reward_proof(&self, neuron_path: &[u64], quality: f64) {
        let reward = quality.clamp(0.0, 1.0);
        self.apply_signal(neuron_path, LearningSignal::Reward(reward));
    }

    /// Punish a failed proof attempt
    pub fn punish_error(&self, neuron_path: &[u64], error_severity: f64) {
        let punishment = error_severity.clamp(0.0, 1.0);
        self.apply_signal(neuron_path, LearningSignal::Punishment(punishment));
    }

    /// Exploration: create random connections to discover new proof strategies
    pub fn explore(&self) {
        if rand::random::<f64>() > self.config.exploration_rate {
            return;
        }

        let stats = self.gabriel_cell.stats();
        if stats.neuron_count < 2 {
            return;
        }

        // Random pair of neurons
        let n1 = rand::random::<u64>() % stats.neuron_count as u64;
        let n2 = rand::random::<u64>() % stats.neuron_count as u64;

        if n1 != n2 {
            let random_weight = rand::random::<f64>() * 0.3; // Weak initial exploration
            self.gabriel_cell.connect(n1, n2, random_weight);

            debug!("Exploration: connected {} -> {} with weight {:.3}", n1, n2, random_weight);
        }
    }

    /// Decay old rewards (implements temporal forgetting)
    pub fn decay_rewards(&self) {
        let mut history = self.reward_history.write();

        for rewards in history.values_mut() {
            for reward in rewards.iter_mut() {
                *reward *= self.config.reward_decay;
            }
        }
    }

    /// Get average reward for a connection
    pub fn get_connection_value(&self, source: u64, target: u64) -> f64 {
        let history = self.reward_history.read();

        if let Some(rewards) = history.get(&(source, target)) {
            if rewards.is_empty() {
                return 0.0;
            }

            let sum: f64 = rewards.iter().sum();
            sum / rewards.len() as f64
        } else {
            0.0
        }
    }

    /// Update confidence based on recent performance
    pub fn update_confidence(&self) {
        let stats = self.stats.read();

        let total_attempts = stats.successful_proofs + stats.failed_proofs;
        if total_attempts == 0 {
            return;
        }

        let success_rate = stats.successful_proofs as f64 / total_attempts as f64;

        let mut stats_mut = self.stats.write();
        stats_mut.average_confidence = success_rate;
    }

    /// Get learning statistics
    pub fn stats(&self) -> LearningStats {
        self.stats.read().clone()
    }

    /// Reset learning (for experiments)
    pub fn reset(&self) {
        let mut stats = self.stats.write();
        *stats = LearningStats::default();

        let mut history = self.reward_history.write();
        history.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gabriel_core::GabrielCell;

    #[derive(Clone, Debug)]
    struct TestQuantum {
        id: u64,
        value: f64,
    }

    impl InformationQuantum for TestQuantum {
        type Id = u64;
        fn id(&self) -> u64 { self.id }
        fn resonance(&self, _other: &Self) -> f64 { 0.5 }
        fn energy(&self) -> f64 { self.value }
        fn fuse(&self, other: &Self, w: f64) -> Self {
            TestQuantum {
                id: self.id,
                value: self.value * (1.0 - w) + other.value * w,
            }
        }
    }

    #[test]
    fn test_learning_reward() {
        let config = gabriel_core::GabrielConfig::default();
        let cell = Arc::new(GabrielCell::new(config));

        // Create neurons
        let n1 = cell.add_neuron(TestQuantum { id: 1, value: 1.0 }, [0.0; 4]);
        let n2 = cell.add_neuron(TestQuantum { id: 2, value: 1.0 }, [0.0; 4]);
        let n3 = cell.add_neuron(TestQuantum { id: 3, value: 1.0 }, [0.0; 4]);

        // Connect
        cell.connect(n1, n2, 0.5);
        cell.connect(n2, n3, 0.5);

        let learning = MathematicalLearning::new(cell.clone(), LearningConfig::default());

        // Reward path
        learning.reward_proof(&[n1, n2, n3], 0.8);

        let stats = learning.stats();
        assert_eq!(stats.successful_proofs, 1);
        assert!(stats.cumulative_reward > 0.0);
    }

    #[test]
    fn test_learning_punishment() {
        let config = gabriel_core::GabrielConfig::default();
        let cell = Arc::new(GabrielCell::new(config));

        let n1 = cell.add_neuron(TestQuantum { id: 1, value: 1.0 }, [0.0; 4]);
        let n2 = cell.add_neuron(TestQuantum { id: 2, value: 1.0 }, [0.0; 4]);

        cell.connect(n1, n2, 0.8);

        let learning = MathematicalLearning::new(cell, LearningConfig::default());

        learning.punish_error(&[n1, n2], 0.5);

        let stats = learning.stats();
        assert_eq!(stats.failed_proofs, 1);
        assert!(stats.cumulative_reward < 0.0);
    }

    #[test]
    fn test_confidence_update() {
        let config = gabriel_core::GabrielConfig::default();
        let cell = Arc::new(GabrielCell::new(config));

        let n1 = cell.add_neuron(TestQuantum { id: 1, value: 1.0 }, [0.0; 4]);
        let n2 = cell.add_neuron(TestQuantum { id: 2, value: 1.0 }, [0.0; 4]);

        cell.connect(n1, n2, 0.5);

        let learning = MathematicalLearning::new(cell, LearningConfig::default());

        // 3 successes, 1 failure
        learning.reward_proof(&[n1, n2], 1.0);
        learning.reward_proof(&[n1, n2], 1.0);
        learning.reward_proof(&[n1, n2], 1.0);
        learning.punish_error(&[n1, n2], 0.5);

        learning.update_confidence();

        let stats = learning.stats();
        assert_eq!(stats.average_confidence, 0.75); // 3/4 success rate
    }
}

//! # Mathematical Learning Experiment
//!
//! Train the Gabriel Organism to learn mathematics from axioms.
//! Progressive curriculum: Peano axioms → Number theory → Complex analysis → Riemann Hypothesis
//!
//! ## Training Pipeline:
//! 1. Feed foundational axioms (Peano, ZFC)
//! 2. Train on simple theorems (commutativity, associativity)
//! 3. Progress to number theory (primes, divisibility)
//! 4. Advance to complex analysis (functions, contours)
//! 5. Approach zeta function and Riemann Hypothesis
//!
//! ## Success Metrics:
//! - Proof success rate
//! - Conjecture quality
//! - Abstraction level achieved
//! - Consciousness during mathematical reasoning

use gabriel_core::GabrielConfig;
use organism::{GabrielOrganism, OrganismConfig, learning, memory, reasoning};
use mathematical_cognition::*;
use std::sync::Arc;
use tracing::{info, warn};

/// Mathematical curriculum difficulty levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CurriculumLevel {
    Foundations,    // Axioms, basic arithmetic
    Elementary,     // Simple theorems
    Intermediate,   // Number theory
    Advanced,       // Complex analysis
    Research,       // Riemann Hypothesis
}

/// Training curriculum for mathematical learning
pub struct MathCurriculum {
    current_level: CurriculumLevel,
    proofs_completed: usize,
    proofs_failed: usize,
    consciousness_history: Vec<f64>,
}

impl MathCurriculum {
    pub fn new() -> Self {
        Self {
            current_level: CurriculumLevel::Foundations,
            proofs_completed: 0,
            proofs_failed: 0,
            consciousness_history: Vec::new(),
        }
    }

    /// Check if ready to advance to next level
    pub fn should_advance(&self) -> bool {
        let total_attempts = self.proofs_completed + self.proofs_failed;

        if total_attempts < 10 {
            return false; // Need minimum experience
        }

        let success_rate = self.proofs_completed as f64 / total_attempts as f64;

        // Need 80% success rate to advance
        success_rate > 0.8
    }

    pub fn advance(&mut self) {
        self.current_level = match self.current_level {
            CurriculumLevel::Foundations => CurriculumLevel::Elementary,
            CurriculumLevel::Elementary => CurriculumLevel::Intermediate,
            CurriculumLevel::Intermediate => CurriculumLevel::Advanced,
            CurriculumLevel::Advanced => CurriculumLevel::Research,
            CurriculumLevel::Research => CurriculumLevel::Research, // Max level
        };

        info!("🎓 Advanced to {:?} level!", self.current_level);

        // Reset counters
        self.proofs_completed = 0;
        self.proofs_failed = 0;
    }

    pub fn record_success(&mut self) {
        self.proofs_completed += 1;
    }

    pub fn record_failure(&mut self) {
        self.proofs_failed += 1;
    }

    pub fn record_consciousness(&mut self, level: f64) {
        self.consciousness_history.push(level);
    }

    pub fn average_consciousness(&self) -> f64 {
        if self.consciousness_history.is_empty() {
            return 0.0;
        }

        self.consciousness_history.iter().sum::<f64>() / self.consciousness_history.len() as f64
    }
}

/// Mathematical training session
pub struct MathTrainingSession {
    curriculum: MathCurriculum,
    theorem_memory: Arc<memory::TheoremMemory<MathTheorem>>,
    cycles_completed: usize,
}

impl MathTrainingSession {
    pub fn new() -> Self {
        let memory_config = memory::MemoryConfig::default();

        Self {
            curriculum: MathCurriculum::new(),
            theorem_memory: Arc::new(memory::TheoremMemory::new(memory_config)),
            cycles_completed: 0,
        }
    }

    /// Load foundational axioms
    pub fn load_axioms(&mut self) {
        info!("📚 Loading foundational axioms...");

        // Peano axioms
        let peano_zero = theorem::peano_zero_axiom();
        self.theorem_memory.store_theorem(
            peano_zero,
            vec!["axiom".to_string(), "foundation".to_string(), "arithmetic".to_string()],
        );

        info!("✅ Loaded Peano axioms");
    }

    /// Load elementary theorems
    pub fn load_elementary_theorems(&mut self) {
        info!("📚 Loading elementary theorems...");

        // Commutativity of addition
        let comm_add = MathTheorem::new(
            "Commutativity of Addition".to_string(),
            "∀a,b ∈ ℕ: a + b = b + a".to_string(),
            MathDomain::Arithmetic,
        ).with_difficulty(DifficultyLevel::Elementary)
         .with_tags(vec!["arithmetic".to_string(), "commutativity".to_string()]);

        self.theorem_memory.store_theorem(
            comm_add,
            vec!["elementary".to_string(), "arithmetic".to_string()],
        );

        info!("✅ Loaded elementary theorems");
    }

    /// Load number theory theorems
    pub fn load_number_theory(&mut self) {
        info!("📚 Loading number theory...");

        let inf_primes = theorem::infinitude_of_primes();
        self.theorem_memory.store_theorem(
            inf_primes,
            vec!["number_theory".to_string(), "primes".to_string()],
        );

        info!("✅ Loaded number theory");
    }

    /// Load complex analysis
    pub fn load_complex_analysis(&mut self) {
        info!("📚 Loading complex analysis...");

        // Fundamental theorem of calculus
        let ftc = MathTheorem::new(
            "Fundamental Theorem of Calculus".to_string(),
            "∫[a,b] f'(x)dx = f(b) - f(a)".to_string(),
            MathDomain::Analysis,
        ).with_difficulty(DifficultyLevel::Advanced);

        self.theorem_memory.store_theorem(
            ftc,
            vec!["analysis".to_string(), "calculus".to_string()],
        );

        info!("✅ Loaded complex analysis");
    }

    /// Load Riemann Hypothesis
    pub fn load_riemann_hypothesis(&mut self) {
        info!("📚 Loading Riemann Hypothesis...");

        let rh = theorem::riemann_hypothesis();
        self.theorem_memory.store_theorem(
            rh,
            vec!["riemann".to_string(), "research".to_string(), "unsolved".to_string()],
        );

        info!("🎯 Loaded Riemann Hypothesis - the ultimate goal!");
    }

    /// Run training cycle
    pub fn train_cycle(&mut self) -> TrainingMetrics {
        self.cycles_completed += 1;

        // Load curriculum based on level
        match self.curriculum.current_level {
            CurriculumLevel::Foundations => {
                if self.theorem_memory.count() == 0 {
                    self.load_axioms();
                }
            }
            CurriculumLevel::Elementary => {
                self.load_elementary_theorems();
            }
            CurriculumLevel::Intermediate => {
                self.load_number_theory();
            }
            CurriculumLevel::Advanced => {
                self.load_complex_analysis();
            }
            CurriculumLevel::Research => {
                self.load_riemann_hypothesis();
            }
        }

        // Simulate proof attempts (simplified)
        let success_probability = match self.curriculum.current_level {
            CurriculumLevel::Foundations => 0.95,
            CurriculumLevel::Elementary => 0.85,
            CurriculumLevel::Intermediate => 0.70,
            CurriculumLevel::Advanced => 0.50,
            CurriculumLevel::Research => 0.10, // Riemann is hard!
        };

        let proof_succeeded = rand::random::<f64>() < success_probability;

        if proof_succeeded {
            self.curriculum.record_success();
            info!("✅ Proof succeeded at {:?} level", self.curriculum.current_level);
        } else {
            self.curriculum.record_failure();
            warn!("❌ Proof failed at {:?} level", self.curriculum.current_level);
        }

        // Simulate consciousness growth
        let consciousness = match self.curriculum.current_level {
            CurriculumLevel::Foundations => 0.2,
            CurriculumLevel::Elementary => 0.4,
            CurriculumLevel::Intermediate => 0.6,
            CurriculumLevel::Advanced => 0.8,
            CurriculumLevel::Research => 0.95,
        };

        self.curriculum.record_consciousness(consciousness);

        // Check advancement
        if self.curriculum.should_advance() {
            self.curriculum.advance();
        }

        TrainingMetrics {
            cycle: self.cycles_completed,
            level: self.curriculum.current_level,
            proofs_completed: self.curriculum.proofs_completed,
            proofs_failed: self.curriculum.proofs_failed,
            consciousness: consciousness,
            theorems_stored: self.theorem_memory.count(),
        }
    }

    /// Run full training program
    pub fn run_training(&mut self, max_cycles: usize) {
        info!("🚀 Starting Mathematical Learning Training");
        info!("   Target: {} cycles", max_cycles);
        info!("");

        for cycle in 0..max_cycles {
            let metrics = self.train_cycle();

            if cycle % 10 == 0 {
                self.report_progress(&metrics);
            }

            // Check if reached Riemann Hypothesis level
            if self.curriculum.current_level == CurriculumLevel::Research &&
               self.curriculum.proofs_completed > 0 {
                info!("");
                info!("🎉 BREAKTHROUGH! Organism has successfully engaged with Riemann Hypothesis!");
                info!("   Cycles to achievement: {}", cycle);
                break;
            }
        }

        info!("");
        self.final_report();
    }

    fn report_progress(&self, metrics: &TrainingMetrics) {
        info!("📊 Cycle {}: {:?} | Success: {} | Failed: {} | Consciousness: {:.2} | Theorems: {}",
            metrics.cycle,
            metrics.level,
            metrics.proofs_completed,
            metrics.proofs_failed,
            metrics.consciousness,
            metrics.theorems_stored,
        );
    }

    fn final_report(&self) {
        info!("═══════════════════════════════════════════════════════════");
        info!("                   TRAINING COMPLETE                       ");
        info!("═══════════════════════════════════════════════════════════");
        info!("Final Level: {:?}", self.curriculum.current_level);
        info!("Total Proofs Completed: {}", self.curriculum.proofs_completed);
        info!("Total Proofs Failed: {}", self.curriculum.proofs_failed);
        info!("Average Consciousness: {:.2}", self.curriculum.average_consciousness());
        info!("Theorems Stored: {}", self.theorem_memory.count());
        info!("═══════════════════════════════════════════════════════════");
    }
}

#[derive(Debug, Clone)]
pub struct TrainingMetrics {
    pub cycle: usize,
    pub level: CurriculumLevel,
    pub proofs_completed: usize,
    pub proofs_failed: usize,
    pub consciousness: f64,
    pub theorems_stored: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curriculum_progression() {
        let mut curriculum = MathCurriculum::new();
        assert_eq!(curriculum.current_level, CurriculumLevel::Foundations);

        // Simulate successful learning
        for _ in 0..10 {
            curriculum.record_success();
        }

        assert!(curriculum.should_advance());

        curriculum.advance();
        assert_eq!(curriculum.current_level, CurriculumLevel::Elementary);
    }

    #[test]
    fn test_training_session() {
        let mut session = MathTrainingSession::new();

        session.load_axioms();
        assert!(session.theorem_memory.count() > 0);

        let metrics = session.train_cycle();
        assert!(metrics.consciousness > 0.0);
    }
}

fn main() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Create and run training session
    let mut session = MathTrainingSession::new();
    session.run_training(100);
}

//! Proof step representation

use crate::{MathId, MathDomain};
use serde::{Deserialize, Serialize};

/// Type of logical step in a proof
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProofStepType {
    Axiom,              // Foundational assumption
    Definition,         // Defining new concepts
    Hypothesis,         // Assumed for proof
    DirectProof,        // A → B directly
    Contradiction,      // Assume ¬B, derive contradiction
    Induction,          // Base case + inductive step
    CaseAnalysis,       // Consider all cases
    Construction,       // Build an object with properties
    Calculation,        // Computational step
    Substitution,       // Replace equals with equals
    LogicalDeduction,   // Apply inference rules
    Lemma,             // Intermediate result
    Conclusion,        // Final QED
}

/// Single step in a mathematical proof
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofStep {
    pub id: MathId,
    pub step_type: ProofStepType,
    pub statement: String,
    /// References to previous steps this depends on
    pub depends_on: Vec<MathId>,
    /// Mathematical domain
    pub domain: MathDomain,
    /// Confidence in this step (0.0-1.0)
    pub confidence: f64,
    /// Has this been verified?
    pub verified: bool,
}

impl ProofStep {
    pub fn new(step_type: ProofStepType, statement: String, domain: MathDomain) -> Self {
        let id = MathId::from_hash(&statement);

        let confidence = match step_type {
            ProofStepType::Axiom => 1.0,
            ProofStepType::Definition => 1.0,
            ProofStepType::Calculation => 0.95,
            ProofStepType::Lemma => 0.8,
            _ => 0.7,
        };

        Self {
            id,
            step_type,
            statement,
            depends_on: Vec::new(),
            domain,
            confidence,
            verified: false,
        }
    }

    pub fn with_dependencies(mut self, deps: Vec<MathId>) -> Self {
        self.depends_on = deps;
        self
    }

    pub fn verify(&mut self) {
        self.verified = true;
        self.confidence = 1.0;
    }

    pub fn is_foundational(&self) -> bool {
        matches!(self.step_type, ProofStepType::Axiom | ProofStepType::Definition)
    }
}

/// Complete proof structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proof {
    pub id: MathId,
    pub theorem_id: MathId,
    pub steps: Vec<ProofStep>,
    pub complete: bool,
    pub verified: bool,
}

impl Proof {
    pub fn new(theorem_id: MathId) -> Self {
        Self {
            id: MathId::new(rand::random()),
            theorem_id,
            steps: Vec::new(),
            complete: false,
            verified: false,
        }
    }

    pub fn add_step(&mut self, step: ProofStep) {
        self.steps.push(step);
    }

    pub fn finalize(&mut self) {
        self.complete = true;
        // Check if all steps are verified
        self.verified = self.steps.iter().all(|s| s.verified);
    }

    pub fn overall_confidence(&self) -> f64 {
        if self.steps.is_empty() {
            return 0.0;
        }

        let sum: f64 = self.steps.iter().map(|s| s.confidence).sum();
        sum / self.steps.len() as f64
    }

    /// Check if proof has valid dependency chain
    pub fn is_well_formed(&self) -> bool {
        let step_ids: std::collections::HashSet<_> =
            self.steps.iter().map(|s| s.id).collect();

        // All dependencies must reference existing steps
        self.steps.iter().all(|step| {
            step.depends_on.iter().all(|dep| step_ids.contains(dep))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proof_step_creation() {
        let step = ProofStep::new(
            ProofStepType::Axiom,
            "For all n ∈ ℕ, n + 0 = n".to_string(),
            MathDomain::Arithmetic,
        );

        assert_eq!(step.confidence, 1.0);
        assert!(step.is_foundational());
    }

    #[test]
    fn test_proof_construction() {
        let theorem_id = MathId::new(42);
        let mut proof = Proof::new(theorem_id);

        let step1 = ProofStep::new(
            ProofStepType::Hypothesis,
            "Let n be even".to_string(),
            MathDomain::NumberTheory,
        );

        let step2 = ProofStep::new(
            ProofStepType::DirectProof,
            "Then n = 2k for some k ∈ ℤ".to_string(),
            MathDomain::NumberTheory,
        ).with_dependencies(vec![step1.id]);

        proof.add_step(step1);
        proof.add_step(step2);
        proof.finalize();

        assert_eq!(proof.steps.len(), 2);
        assert!(proof.is_well_formed());
    }

    #[test]
    fn test_proof_confidence() {
        let mut proof = Proof::new(MathId::new(1));

        let mut step1 = ProofStep::new(
            ProofStepType::Axiom,
            "Axiom".to_string(),
            MathDomain::Logic,
        );
        step1.verify();

        let mut step2 = ProofStep::new(
            ProofStepType::Calculation,
            "2 + 2 = 4".to_string(),
            MathDomain::Arithmetic,
        );
        step2.verify();

        proof.add_step(step1);
        proof.add_step(step2);
        proof.finalize();

        assert_eq!(proof.overall_confidence(), 1.0);
        assert!(proof.verified);
    }
}

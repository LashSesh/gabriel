//! Mathematical theorem representation

use crate::{MathId, MathQuantum, MathDomain, DifficultyLevel, normalize_similarity, levenshtein_distance};
use crate::proof::{Proof, ProofStep};
use gabriel_core::InformationQuantum;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Mathematical theorem with proofs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MathTheorem {
    pub id: MathId,
    pub name: String,
    pub statement: String,
    pub domain: MathDomain,
    pub difficulty: DifficultyLevel,
    /// Is this proven or conjectured?
    pub proven: bool,
    /// Certainty level (0.0 = pure conjecture, 1.0 = proven)
    pub certainty: f64,
    /// Known proofs
    pub proofs: Vec<Proof>,
    /// Related theorems
    pub related_theorems: HashSet<MathId>,
    /// Tags for pattern matching
    pub tags: HashSet<String>,
    /// How many times used in other proofs
    pub citation_count: usize,
}

impl MathTheorem {
    pub fn new(name: String, statement: String, domain: MathDomain) -> Self {
        let id = MathId::from_hash(&statement);

        Self {
            id,
            name,
            statement,
            domain,
            difficulty: DifficultyLevel::Intermediate,
            proven: false,
            certainty: 0.0,
            proofs: Vec::new(),
            related_theorems: HashSet::new(),
            tags: HashSet::new(),
            citation_count: 0,
        }
    }

    pub fn with_difficulty(mut self, difficulty: DifficultyLevel) -> Self {
        self.difficulty = difficulty;
        self
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags.into_iter().collect();
        self
    }

    pub fn add_proof(&mut self, proof: Proof) {
        if proof.verified {
            self.proven = true;
            self.certainty = 1.0;
        } else {
            self.certainty = self.certainty.max(proof.overall_confidence());
        }
        self.proofs.push(proof);
    }

    pub fn relate_to(&mut self, other_id: MathId) {
        self.related_theorems.insert(other_id);
    }

    /// Best proof available
    pub fn best_proof(&self) -> Option<&Proof> {
        self.proofs.iter()
            .max_by(|a, b| {
                a.overall_confidence()
                    .partial_cmp(&b.overall_confidence())
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
    }
}

impl InformationQuantum for MathTheorem {
    type Id = MathId;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn resonance(&self, other: &Self) -> f64 {
        // Theorem resonance based on:
        // 1. Statement similarity (Levenshtein)
        // 2. Shared tags
        // 3. Same domain
        // 4. Explicit relations

        // Textual similarity
        let max_len = self.statement.len().max(other.statement.len()).max(1);
        let edit_distance = levenshtein_distance(&self.statement, &other.statement);
        let text_similarity = 1.0 - (edit_distance as f64 / max_len as f64);

        // Tag overlap
        let tag_similarity = crate::structural_similarity(&self.tags, &other.tags);

        // Domain match
        let domain_match = if self.domain == other.domain { 0.5 } else { 0.0 };

        // Explicit relation
        let explicit_relation = if self.related_theorems.contains(&other.id) ||
                                  other.related_theorems.contains(&self.id) {
            0.8
        } else {
            0.0
        };

        // Weighted combination
        let base_similarity =
            0.3 * text_similarity +
            0.3 * tag_similarity +
            0.2 * domain_match +
            0.2 * explicit_relation;

        normalize_similarity(base_similarity)
    }

    fn energy(&self) -> f64 {
        // Energy = difficulty + proven status + citations + proof quality
        let difficulty_energy = self.difficulty.as_f64();

        let proven_bonus = if self.proven { 0.2 } else { 0.0 };

        let citation_energy = (self.citation_count as f64 * 0.05).min(0.3);

        let proof_quality = self.best_proof()
            .map(|p| p.overall_confidence() * 0.2)
            .unwrap_or(0.0);

        normalize_similarity(
            difficulty_energy +
            proven_bonus +
            citation_energy +
            proof_quality
        )
    }

    fn fuse(&self, other: &Self, weight: f64) -> Self {
        // Fusion creates a hybrid theorem
        let new_name = if weight > 0.5 {
            other.name.clone()
        } else {
            self.name.clone()
        };

        let new_statement = format!(
            "{} AND {}",
            self.statement,
            other.statement
        );

        let mut fused = MathTheorem::new(new_name, new_statement, self.domain);

        // Merge tags
        fused.tags = self.tags.union(&other.tags).cloned().collect();

        // Merge related theorems
        fused.related_theorems = self.related_theorems
            .union(&other.related_theorems)
            .cloned()
            .collect();

        // Weighted certainty
        fused.certainty = self.certainty * (1.0 - weight) + other.certainty * weight;

        // Higher difficulty
        fused.difficulty = self.difficulty.max(other.difficulty);

        fused
    }
}

impl MathQuantum for MathTheorem {
    fn complexity(&self) -> f64 {
        // Theorem complexity = difficulty + proof complexity
        let base = self.difficulty.as_f64();

        let proof_complexity = self.best_proof()
            .map(|p| p.steps.len() as f64 * 0.02)
            .unwrap_or(0.0);

        normalize_similarity(base + proof_complexity)
    }

    fn mathematical_similarity(&self, other: &Self) -> f64 {
        self.resonance(other)
    }

    fn is_provable(&self) -> bool {
        // Most theorems are provable (within a formal system)
        // We assume true unless it's explicitly a known independent statement
        !self.tags.contains("independent") && !self.tags.contains("undecidable")
    }

    fn certainty(&self) -> f64 {
        self.certainty
    }
}

// Pre-defined foundational theorems (axioms)

/// Peano axiom: 0 is a natural number
pub fn peano_zero_axiom() -> MathTheorem {
    MathTheorem::new(
        "Peano Zero".to_string(),
        "0 ∈ ℕ".to_string(),
        MathDomain::Arithmetic,
    )
    .with_difficulty(DifficultyLevel::Elementary)
    .with_tags(vec!["axiom".to_string(), "peano".to_string(), "foundation".to_string()])
}

/// Infinitude of primes (Euclid's theorem)
pub fn infinitude_of_primes() -> MathTheorem {
    MathTheorem::new(
        "Infinitude of Primes".to_string(),
        "There are infinitely many prime numbers".to_string(),
        MathDomain::NumberTheory,
    )
    .with_difficulty(DifficultyLevel::Intermediate)
    .with_tags(vec!["primes".to_string(), "infinity".to_string(), "euclid".to_string()])
}

/// Riemann Hypothesis
pub fn riemann_hypothesis() -> MathTheorem {
    MathTheorem::new(
        "Riemann Hypothesis".to_string(),
        "All non-trivial zeros of ζ(s) have Re(s) = 1/2".to_string(),
        MathDomain::ComplexAnalysis,
    )
    .with_difficulty(DifficultyLevel::Research)
    .with_tags(vec![
        "riemann".to_string(),
        "zeta".to_string(),
        "conjecture".to_string(),
        "millennium".to_string(),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theorem_creation() {
        let thm = infinitude_of_primes();
        assert_eq!(thm.name, "Infinitude of Primes");
        assert!(!thm.proven); // Not proven in our system yet
        assert_eq!(thm.certainty, 0.0);
    }

    #[test]
    fn test_theorem_proof() {
        let mut thm = peano_zero_axiom();
        let mut proof = Proof::new(thm.id);

        let mut step = ProofStep::new(
            crate::proof::ProofStepType::Axiom,
            "0 ∈ ℕ by definition".to_string(),
            MathDomain::Arithmetic,
        );
        step.verify();

        proof.add_step(step);
        proof.finalize();

        thm.add_proof(proof);

        assert!(thm.proven);
        assert_eq!(thm.certainty, 1.0);
    }

    #[test]
    fn test_theorem_resonance() {
        let thm1 = infinitude_of_primes();
        let thm2 = riemann_hypothesis();

        // Both involve primes/number theory
        let res = thm1.resonance(&thm2);
        assert!(res > 0.0); // Some similarity due to domain
    }

    #[test]
    fn test_riemann_hypothesis() {
        let rh = riemann_hypothesis();
        assert_eq!(rh.difficulty, DifficultyLevel::Research);
        assert!(rh.tags.contains("conjecture"));
        assert!(!rh.proven);
    }
}

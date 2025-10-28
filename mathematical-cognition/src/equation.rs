//! Mathematical equation representation

use crate::{MathId, MathQuantum, MathDomain, normalize_similarity, levenshtein_distance};
use gabriel_core::InformationQuantum;
use serde::{Deserialize, Serialize};

/// Type of equation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EquationType {
    Algebraic,       // Polynomial equations
    Differential,    // ODEs, PDEs
    Functional,      // Equations of functions
    Transcendental,  // Involving transcendental functions
    Diophantine,     // Integer solutions
}

/// Mathematical equation to solve
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MathEquation {
    pub id: MathId,
    pub equation_type: EquationType,
    pub expression: String,
    pub domain: MathDomain,
    /// Has this been solved?
    pub solved: bool,
    /// Known solutions (as strings for now)
    pub solutions: Vec<String>,
    /// Difficulty in solving
    pub difficulty: f64,
}

impl MathEquation {
    pub fn new(equation_type: EquationType, expression: String, domain: MathDomain) -> Self {
        let id = MathId::from_hash(&expression);

        let difficulty = match equation_type {
            EquationType::Algebraic => 0.3,
            EquationType::Diophantine => 0.5,
            EquationType::Transcendental => 0.6,
            EquationType::Differential => 0.7,
            EquationType::Functional => 0.8,
        };

        Self {
            id,
            equation_type,
            expression,
            domain,
            solved: false,
            solutions: Vec::new(),
            difficulty,
        }
    }

    pub fn add_solution(&mut self, solution: String) {
        self.solutions.push(solution);
        self.solved = true;
    }

    pub fn solution_count(&self) -> usize {
        self.solutions.len()
    }
}

impl InformationQuantum for MathEquation {
    type Id = MathId;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn resonance(&self, other: &Self) -> f64 {
        // Equations resonate based on:
        // 1. Same type
        // 2. Syntactic similarity
        // 3. Same domain

        let type_match = if self.equation_type == other.equation_type {
            0.5
        } else {
            0.0
        };

        let max_len = self.expression.len().max(other.expression.len()).max(1);
        let edit_distance = levenshtein_distance(&self.expression, &other.expression);
        let text_similarity = 1.0 - (edit_distance as f64 / max_len as f64);

        let domain_match = if self.domain == other.domain { 0.3 } else { 0.0 };

        normalize_similarity(
            0.4 * type_match +
            0.4 * text_similarity +
            0.2 * domain_match
        )
    }

    fn energy(&self) -> f64 {
        // Energy = difficulty + solution status
        let base_difficulty = self.difficulty;

        let solved_bonus = if self.solved { 0.2 } else { 0.0 };

        let solution_richness = (self.solutions.len() as f64 * 0.05).min(0.2);

        normalize_similarity(base_difficulty + solved_bonus + solution_richness)
    }

    fn fuse(&self, other: &Self, weight: f64) -> Self {
        // Fusion creates a system of equations
        let new_expression = format!(
            "{{ {} , {} }}",
            self.expression,
            other.expression
        );

        let new_type = if weight > 0.5 {
            other.equation_type
        } else {
            self.equation_type
        };

        let new_difficulty = self.difficulty * (1.0 - weight) + other.difficulty * weight;

        let mut fused = MathEquation::new(new_type, new_expression, self.domain);
        fused.difficulty = new_difficulty;

        fused
    }
}

impl MathQuantum for MathEquation {
    fn complexity(&self) -> f64 {
        self.difficulty
    }

    fn mathematical_similarity(&self, other: &Self) -> f64 {
        self.resonance(other)
    }

    fn is_provable(&self) -> bool {
        // Solvability depends on type
        match self.equation_type {
            EquationType::Algebraic => true,  // Galois theory
            EquationType::Diophantine => false, // Undecidable in general
            _ => true, // Most others have algorithms
        }
    }

    fn certainty(&self) -> f64 {
        if self.solved {
            1.0
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equation_creation() {
        let eq = MathEquation::new(
            EquationType::Algebraic,
            "x^2 - 2 = 0".to_string(),
            MathDomain::Algebra,
        );

        assert!(!eq.solved);
        assert_eq!(eq.solutions.len(), 0);
    }

    #[test]
    fn test_equation_solution() {
        let mut eq = MathEquation::new(
            EquationType::Algebraic,
            "x^2 - 4 = 0".to_string(),
            MathDomain::Algebra,
        );

        eq.add_solution("x = 2".to_string());
        eq.add_solution("x = -2".to_string());

        assert!(eq.solved);
        assert_eq!(eq.solution_count(), 2);
        assert_eq!(eq.certainty(), 1.0);
    }

    #[test]
    fn test_equation_resonance() {
        let eq1 = MathEquation::new(
            EquationType::Algebraic,
            "x^2 = 4".to_string(),
            MathDomain::Algebra,
        );

        let eq2 = MathEquation::new(
            EquationType::Algebraic,
            "x^2 = 9".to_string(),
            MathDomain::Algebra,
        );

        let eq3 = MathEquation::new(
            EquationType::Differential,
            "dy/dx = y".to_string(),
            MathDomain::Analysis,
        );

        // Similar algebraic equations
        assert!(eq1.resonance(&eq2) > 0.7);

        // Different types
        assert!(eq1.resonance(&eq3) < 0.3);
    }
}

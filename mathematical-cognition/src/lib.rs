//! # Mathematical Cognition
//!
//! Mathematical intelligence layer for Gabriel Organism.
//! Represents mathematical objects (numbers, equations, theorems, proofs) as Information Quanta
//! with emergent learning capabilities.
//!
//! ## Philosophy
//! Mathematics is not programmed - it EMERGES through metabolic learning.
//! The organism discovers mathematical truth through pattern recognition,
//! resonance, and self-organization.

use gabriel_core::InformationQuantum;
use num_complex::Complex64;
use num_traits::{Num, One, Zero};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

// Re-exports
pub use integer::MathInteger;
pub use prime::MathPrime;
pub use complex::MathComplex;
pub use function::MathFunction;
pub use theorem::MathTheorem;
pub use equation::MathEquation;
pub use proof::{Proof, ProofStep};

pub mod integer;
pub mod prime;
pub mod complex;
pub mod function;
pub mod theorem;
pub mod equation;
pub mod proof;

/// Core trait for all mathematical objects in the Gabriel system.
/// Extends InformationQuantum with mathematical-specific operations.
pub trait MathQuantum: InformationQuantum {
    /// Mathematical complexity - how deep is this concept?
    /// Simple arithmetic: 0.1-0.3
    /// Number theory: 0.4-0.6
    /// Complex analysis: 0.7-0.9
    /// Deep theorems (Riemann): 0.9-1.0
    fn complexity(&self) -> f64;

    /// Mathematical similarity - detects isomorphisms and analogies
    /// 1.0 = mathematically identical
    /// 0.7-0.9 = structurally similar (e.g., addition and multiplication as monoids)
    /// 0.4-0.6 = analogous (e.g., integers and polynomials)
    /// 0.0-0.3 = unrelated
    fn mathematical_similarity(&self, other: &Self) -> f64;

    /// Can this object be proven/verified?
    fn is_provable(&self) -> bool;

    /// Certainty level (0.0 = conjecture, 1.0 = proven)
    fn certainty(&self) -> f64;
}

/// Unique identifier for mathematical objects
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MathId(pub u64);

impl MathId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    /// Generate from hash of content
    pub fn from_hash<T: Hash>(content: &T) -> Self {
        use std::collections::hash_map::DefaultHasher;
        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        Self(hasher.finish())
    }
}

/// Represents categories of mathematical knowledge
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MathDomain {
    Arithmetic,      // Basic number operations
    NumberTheory,    // Primes, divisibility, modular arithmetic
    Algebra,         // Groups, rings, fields
    Analysis,        // Limits, continuity, differentiation
    ComplexAnalysis, // Complex functions, contour integration
    Topology,        // Continuous deformations, spaces
    SetTheory,       // Foundations, cardinality
    Logic,           // Proof theory, model theory
}

/// Difficulty level of mathematical concept
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum DifficultyLevel {
    Elementary = 1,  // Basic arithmetic
    Intermediate = 2,// Algebra, basic calculus
    Advanced = 3,    // Real/complex analysis
    Graduate = 4,    // Specialized topics
    Research = 5,    // Open problems, cutting edge
}

impl DifficultyLevel {
    pub fn as_f64(&self) -> f64 {
        match self {
            Self::Elementary => 0.2,
            Self::Intermediate => 0.4,
            Self::Advanced => 0.6,
            Self::Graduate => 0.8,
            Self::Research => 1.0,
        }
    }
}

/// Calculate Levenshtein distance between two strings (for symbolic similarity)
pub fn levenshtein_distance(a: &str, b: &str) -> usize {
    let len_a = a.len();
    let len_b = b.len();

    if len_a == 0 {
        return len_b;
    }
    if len_b == 0 {
        return len_a;
    }

    let mut matrix = vec![vec![0; len_b + 1]; len_a + 1];

    for i in 0..=len_a {
        matrix[i][0] = i;
    }
    for j in 0..=len_b {
        matrix[0][j] = j;
    }

    for (i, ca) in a.chars().enumerate() {
        for (j, cb) in b.chars().enumerate() {
            let cost = if ca == cb { 0 } else { 1 };
            matrix[i + 1][j + 1] = (matrix[i][j + 1] + 1)
                .min(matrix[i + 1][j] + 1)
                .min(matrix[i][j] + cost);
        }
    }

    matrix[len_a][len_b]
}

/// Normalize similarity score to 0.0-1.0
pub fn normalize_similarity(raw_similarity: f64) -> f64 {
    raw_similarity.clamp(0.0, 1.0)
}

/// Calculate structural similarity between two sets of properties
pub fn structural_similarity<T: Hash + Eq>(set_a: &HashSet<T>, set_b: &HashSet<T>) -> f64 {
    if set_a.is_empty() && set_b.is_empty() {
        return 1.0;
    }

    let intersection_size = set_a.intersection(set_b).count();
    let union_size = set_a.union(set_b).count();

    if union_size == 0 {
        0.0
    } else {
        intersection_size as f64 / union_size as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_levenshtein_distance() {
        assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
        assert_eq!(levenshtein_distance("", "abc"), 3);
        assert_eq!(levenshtein_distance("abc", "abc"), 0);
    }

    #[test]
    fn test_structural_similarity() {
        let mut set_a = HashSet::new();
        set_a.insert(1);
        set_a.insert(2);
        set_a.insert(3);

        let mut set_b = HashSet::new();
        set_b.insert(2);
        set_b.insert(3);
        set_b.insert(4);

        let sim = structural_similarity(&set_a, &set_b);
        assert!((sim - 0.5).abs() < 0.01); // 2 common / 4 total = 0.5
    }

    #[test]
    fn test_difficulty_level_ordering() {
        assert!(DifficultyLevel::Elementary < DifficultyLevel::Research);
        assert_eq!(DifficultyLevel::Advanced.as_f64(), 0.6);
    }
}

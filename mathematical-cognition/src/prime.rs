//! Prime number representation with number-theoretic properties

use crate::{MathId, MathQuantum, normalize_similarity};
use gabriel_core::InformationQuantum;
use serde::{Deserialize, Serialize};

/// Prime number with rich mathematical structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MathPrime {
    pub id: MathId,
    pub value: u64,
    /// Index in prime sequence (2 is 1st, 3 is 2nd, etc.)
    pub prime_index: usize,
    /// Twin prime status
    pub is_twin: bool,
    /// Sophie Germain prime (p where 2p+1 is also prime)
    pub is_sophie_germain: bool,
    /// Mersenne prime (2^n - 1)
    pub is_mersenne: bool,
    /// Usage count in successful number theory proofs
    pub theorem_count: usize,
}

impl MathPrime {
    pub fn new(value: u64) -> Option<Self> {
        if !is_prime_u64(value) {
            return None;
        }

        let prime_index = prime_counting_approx(value);
        let is_twin = is_twin_prime(value);
        let is_sophie_germain = is_sophie_germain_prime(value);
        let is_mersenne = is_mersenne_prime(value);

        Some(Self {
            id: MathId::from_hash(&value),
            value,
            prime_index,
            is_twin,
            is_sophie_germain,
            is_mersenne,
            theorem_count: 0,
        })
    }

    /// Prime gap to next prime
    pub fn gap(&self) -> u64 {
        next_prime(self.value + 1) - self.value
    }
}

fn is_prime_u64(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let limit = (n as f64).sqrt() as u64;
    let mut i = 5;
    while i <= limit {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }

    true
}

fn next_prime(n: u64) -> u64 {
    let mut candidate = n;
    while !is_prime_u64(candidate) {
        candidate += 1;
    }
    candidate
}

fn is_twin_prime(p: u64) -> bool {
    if p < 3 {
        return false;
    }
    is_prime_u64(p - 2) || is_prime_u64(p + 2)
}

fn is_sophie_germain_prime(p: u64) -> bool {
    is_prime_u64(2 * p + 1)
}

fn is_mersenne_prime(p: u64) -> bool {
    // Check if p = 2^n - 1 for some n
    let n_plus_1 = p + 1;
    n_plus_1.count_ones() == 1 && is_prime_u64(p)
}

fn prime_counting_approx(n: u64) -> usize {
    // Approximate π(n) using prime number theorem: n / ln(n)
    if n < 2 {
        return 0;
    }
    (n as f64 / (n as f64).ln()) as usize
}

impl InformationQuantum for MathPrime {
    type Id = MathId;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn resonance(&self, other: &Self) -> f64 {
        // Primes resonate based on:
        // 1. Shared special properties (twin, Sophie Germain, Mersenne)
        // 2. Prime gap similarity
        // 3. Proximity in prime sequence

        let mut property_matches = 0.0;
        if self.is_twin && other.is_twin {
            property_matches += 0.2;
        }
        if self.is_sophie_germain && other.is_sophie_germain {
            property_matches += 0.3;
        }
        if self.is_mersenne && other.is_mersenne {
            property_matches += 0.3; // Mersenne primes are rare!
        }

        // Gap similarity
        let gap_diff = (self.gap() as f64 - other.gap() as f64).abs();
        let gap_similarity = 1.0 / (1.0 + gap_diff / 10.0);

        // Sequence proximity
        let index_diff = (self.prime_index as f64 - other.prime_index as f64).abs();
        let sequence_proximity = 1.0 / (1.0 + index_diff / 100.0);

        normalize_similarity(
            0.4 * property_matches +
            0.3 * gap_similarity +
            0.3 * sequence_proximity
        )
    }

    fn energy(&self) -> f64 {
        // Prime energy = rarity + special properties + theorem usage
        let rarity = (self.value as f64).ln() / 20.0; // Larger primes are rarer

        let mut special_bonus = 0.0;
        if self.is_twin {
            special_bonus += 0.1;
        }
        if self.is_sophie_germain {
            special_bonus += 0.15;
        }
        if self.is_mersenne {
            special_bonus += 0.25; // Mersenne primes are very special
        }

        let theorem_experience = (self.theorem_count as f64 * 0.05).min(0.3);

        normalize_similarity(0.4 + rarity + special_bonus + theorem_experience)
    }

    fn fuse(&self, other: &Self, weight: f64) -> Self {
        // Fusing primes: find nearest prime to weighted average
        let avg_value = ((self.value as f64 * (1.0 - weight)) +
                        (other.value as f64 * weight)) as u64;

        // Find nearest prime
        let nearest_prime = if is_prime_u64(avg_value) {
            avg_value
        } else {
            next_prime(avg_value)
        };

        MathPrime::new(nearest_prime).unwrap_or_else(|| self.clone())
    }
}

impl MathQuantum for MathPrime {
    fn complexity(&self) -> f64 {
        // Primes are fundamental but increase in complexity with size
        let magnitude = (self.value as f64).ln() / 15.0;
        let special_complexity = if self.is_mersenne { 0.3 } else { 0.0 };

        normalize_similarity(0.5 + magnitude + special_complexity)
    }

    fn mathematical_similarity(&self, other: &Self) -> f64 {
        self.resonance(other)
    }

    fn is_provable(&self) -> bool {
        // Primality is decidable (though computationally hard for large primes)
        true
    }

    fn certainty(&self) -> f64 {
        // Primes are certain once verified
        1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_creation() {
        let p = MathPrime::new(17).unwrap();
        assert_eq!(p.value, 17);

        assert!(MathPrime::new(4).is_none()); // Not prime
    }

    #[test]
    fn test_twin_primes() {
        let p1 = MathPrime::new(11).unwrap();
        let p2 = MathPrime::new(13).unwrap();

        assert!(p1.is_twin);
        assert!(p2.is_twin);
    }

    #[test]
    fn test_mersenne_primes() {
        let m1 = MathPrime::new(3).unwrap();  // 2^2 - 1
        let m2 = MathPrime::new(7).unwrap();  // 2^3 - 1
        let m3 = MathPrime::new(31).unwrap(); // 2^5 - 1

        assert!(m1.is_mersenne);
        assert!(m2.is_mersenne);
        assert!(m3.is_mersenne);

        let not_mersenne = MathPrime::new(11).unwrap();
        assert!(!not_mersenne.is_mersenne);
    }

    #[test]
    fn test_prime_resonance() {
        let p1 = MathPrime::new(11).unwrap();
        let p2 = MathPrime::new(13).unwrap();
        let p3 = MathPrime::new(97).unwrap();

        // Twin primes have higher resonance
        assert!(p1.resonance(&p2) > p1.resonance(&p3));
    }
}

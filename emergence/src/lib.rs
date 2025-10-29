use ndarray::{Array1, Array2, ArrayView1};
use num_complex::Complex64;
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;
use statrs::statistics::{Data, OrderStatistics, Statistics, Distribution};

/// Emergenz-Detektor für Tensorprodukt-Muster
#[derive(Debug)]
pub struct EmergenceDetector {
    /// Schwellwert für Mustererkennung
    threshold: f64,
    
    /// Historie erkannter Muster
    pattern_history: Vec<EmergentPattern>,
    
    /// Fourier-Transformation Cache
    fft_cache: Option<Vec<Complex64>>,
}

/// Emergentes Muster
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergentPattern {
    /// Eindeutige ID
    pub id: u64,
    
    /// Tensorprodukt-Signatur
    pub signature: Vec<f64>,
    
    /// Kohärenz-Maß (0.0 = zufällig, 1.0 = perfekt kohärent)
    pub coherence: f64,
    
    /// Komplexität (Shannon-Entropie)
    pub complexity: f64,
    
    /// Stabilität über Zeit
    pub stability: f64,
    
    /// Zeitstempel der Entdeckung
    pub discovered_at: f64,
    
    /// Typ des Musters
    pub pattern_type: PatternType,
}

/// Muster-Typen
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PatternType {
    /// Periodisch (wiederkehrend)
    Periodic,
    
    /// Hierarchisch (verschachtelt)
    Hierarchical,
    
    /// Chaotisch (sensibel aber deterministisch)
    Chaotic,
    
    /// Fraktal (selbstähnlich)
    Fractal,
    
    /// Harmonisch (Resonanz-basiert)
    Harmonic,
    
    /// Unbekannt
    Unknown,
}

impl EmergenceDetector {
    pub fn new(threshold: f64) -> Self {
        Self {
            threshold,
            pattern_history: Vec::new(),
            fft_cache: None,
        }
    }
    
    /// Analysiert Tensorprodukt auf emergente Muster
    pub fn analyze_tensor(&mut self, tensor: &[f64], time: f64) -> Vec<EmergentPattern> {
        let mut patterns = Vec::new();
        
        // 1. Kohärenz-Analyse
        let coherence = self.calculate_coherence(tensor);
        
        if coherence > self.threshold {
            // 2. Komplexitäts-Berechnung
            let complexity = self.shannon_entropy(tensor);
            
            // 3. Muster-Klassifikation
            let pattern_type = self.classify_pattern(tensor);
            
            // 4. Stabilität (vergleiche mit Historie)
            let stability = self.calculate_stability(tensor);
            
            let pattern = EmergentPattern {
                id: self.pattern_history.len() as u64,
                signature: tensor.to_vec(),
                coherence,
                complexity,
                stability,
                discovered_at: time,
                pattern_type,
            };
            
            patterns.push(pattern.clone());
            self.pattern_history.push(pattern);
        }
        
        patterns
    }
    
    /// Berechnet Kohärenz (wie geordnet ist das Muster)
    fn calculate_coherence(&self, tensor: &[f64]) -> f64 {
        if tensor.is_empty() {
            return 0.0;
        }
        
        // Autokorrelation als Kohärenz-Maß
        let mean = tensor.iter().sum::<f64>() / tensor.len() as f64;
        let variance: f64 = tensor.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / tensor.len() as f64;
        
        if variance < 1e-10 {
            return 1.0; // Perfekt konstant = maximale Kohärenz
        }
        
        // Normalisierte Autokorrelation bei Lag=1
        let mut autocorr = 0.0;
        for i in 0..tensor.len() - 1 {
            autocorr += (tensor[i] - mean) * (tensor[i + 1] - mean);
        }
        autocorr /= (tensor.len() - 1) as f64 * variance;
        
        autocorr.abs().clamp(0.0, 1.0)
    }
    
    /// Shannon-Entropie als Komplexitätsmaß
    fn shannon_entropy(&self, tensor: &[f64]) -> f64 {
        if tensor.is_empty() {
            return 0.0;
        }
        
        // Normalisiere zu Wahrscheinlichkeitsverteilung
        let sum: f64 = tensor.iter().map(|x| x.abs()).sum();
        if sum < 1e-10 {
            return 0.0;
        }
        
        let probs: Vec<f64> = tensor.iter().map(|x| x.abs() / sum).collect();
        
        // Berechne Entropie
        -probs
            .iter()
            .filter(|&&p| p > 1e-10)
            .map(|&p| p * p.ln())
            .sum::<f64>()
    }
    
    /// Klassifiziert Muster-Typ
    fn classify_pattern(&self, tensor: &[f64]) -> PatternType {
        if tensor.is_empty() {
            return PatternType::Unknown;
        }
        
        // Periodizität durch FFT-Peaks
        let is_periodic = self.detect_periodicity(tensor);
        if is_periodic {
            return PatternType::Periodic;
        }
        
        // Harmonische Resonanz
        let is_harmonic = self.detect_harmonics(tensor);
        if is_harmonic {
            return PatternType::Harmonic;
        }
        
        // Selbstähnlichkeit (Fraktal)
        let is_fractal = self.detect_self_similarity(tensor);
        if is_fractal {
            return PatternType::Fractal;
        }
        
        // Lyapunov-Exponent für Chaos
        let is_chaotic = self.detect_chaos(tensor);
        if is_chaotic {
            return PatternType::Chaotic;
        }
        
        // Hierarchische Struktur
        let is_hierarchical = self.detect_hierarchy(tensor);
        if is_hierarchical {
            return PatternType::Hierarchical;
        }
        
        PatternType::Unknown
    }
    
    /// Detektiert Periodizität
    fn detect_periodicity(&self, tensor: &[f64]) -> bool {
        if tensor.len() < 4 {
            return false;
        }
        
        // Einfacher Peak-Detection in Autokorrelation
        let mut autocorr = vec![0.0; tensor.len() / 2];
        let mean = tensor.iter().sum::<f64>() / tensor.len() as f64;
        
        for lag in 1..autocorr.len() {
            let mut sum = 0.0;
            for i in 0..tensor.len() - lag {
                sum += (tensor[i] - mean) * (tensor[i + lag] - mean);
            }
            autocorr[lag] = sum / (tensor.len() - lag) as f64;
        }
        
        // Finde signifikante Peaks
        let max_autocorr = autocorr.iter().cloned().fold(0.0, f64::max);
        autocorr.iter().filter(|&&x| x > max_autocorr * 0.7).count() > 2
    }
    
    /// Detektiert harmonische Resonanz
    fn detect_harmonics(&self, tensor: &[f64]) -> bool {
        if tensor.len() < 8 {
            return false;
        }
        
        // Prüfe auf ganzzahlige Frequenzverhältnisse
        let mut data = Data::new(tensor.to_vec());
        let mean = data.mean().unwrap_or(0.0);
        let std_dev = data.std_dev().unwrap_or(1.0);
        
        if std_dev < 1e-10 {
            return false;
        }
        
        // Z-Score Normalisierung
        let normalized: Vec<f64> = tensor.iter().map(|&x| (x - mean) / std_dev).collect();
        
        // Suche nach harmonischen Beziehungen
        let peaks: Vec<f64> = normalized
            .iter()
            .enumerate()
            .filter(|(_, &v)| v > 1.0)
            .map(|(i, _)| i as f64)
            .collect();
        
        if peaks.len() < 2 {
            return false;
        }
        
        // Prüfe Verhältnisse
        for i in 0..peaks.len() - 1 {
            for j in i + 1..peaks.len() {
                let ratio = peaks[j] / peaks[i];
                // Prüfe auf ganzzahlige oder einfache rationale Verhältnisse
                if (ratio.round() - ratio).abs() < 0.1 {
                    return true;
                }
            }
        }
        
        false
    }
    
    /// Detektiert Selbstähnlichkeit (Fraktal)
    fn detect_self_similarity(&self, tensor: &[f64]) -> bool {
        if tensor.len() < 16 {
            return false;
        }
        
        // Box-Counting für fraktale Dimension
        let n = tensor.len();
        let half = n / 2;
        
        // Vergleiche Muster bei verschiedenen Skalen
        let correlation_full_half = self.correlation(&tensor[0..n], &tensor[0..half]);
        let correlation_half_quarter = if half >= 4 {
            self.correlation(&tensor[0..half], &tensor[0..half / 2])
        } else {
            0.0
        };
        
        // Selbstähnlich wenn Korrelationen über Skalen ähnlich sind
        (correlation_full_half - correlation_half_quarter).abs() < 0.3
            && correlation_full_half > 0.5
    }
    
    /// Detektiert chaotisches Verhalten
    fn detect_chaos(&self, tensor: &[f64]) -> bool {
        if tensor.len() < 10 {
            return false;
        }
        
        // Approximativer Lyapunov-Exponent
        let mut divergences = Vec::new();
        
        for i in 0..tensor.len() - 5 {
            let diff = (tensor[i + 5] - tensor[i]).abs();
            if diff > 1e-10 {
                divergences.push(diff.ln());
            }
        }
        
        if divergences.is_empty() {
            return false;
        }
        
        let avg_divergence = divergences.iter().sum::<f64>() / divergences.len() as f64;
        
        // Positiver Lyapunov-Exponent deutet auf Chaos hin
        avg_divergence > 0.0
    }
    
    /// Detektiert hierarchische Struktur
    fn detect_hierarchy(&self, tensor: &[f64]) -> bool {
        if tensor.len() < 8 {
            return false;
        }
        
        // Wavelet-ähnliche Dekomposition in Level
        let mid = tensor.len() / 2;
        
        let level1_var = self.variance(&tensor[0..mid]);
        let level2_var = self.variance(&tensor[mid..]);
        
        // Hierarchisch wenn verschiedene Level unterschiedliche Varianz haben
        let ratio = (level1_var / (level2_var + 1e-10)).max(level2_var / (level1_var + 1e-10));
        
        ratio > 2.0
    }
    
    /// Berechnet Stabilität (Vergleich mit Muster-Historie)
    fn calculate_stability(&self, tensor: &[f64]) -> f64 {
        if self.pattern_history.is_empty() {
            return 0.5; // Neutral bei erstem Muster
        }
        
        // Vergleiche mit letzten N Mustern
        let n = 5.min(self.pattern_history.len());
        let recent_patterns = &self.pattern_history[self.pattern_history.len() - n..];
        
        let similarities: Vec<f64> = recent_patterns
            .iter()
            .map(|p| self.correlation(tensor, &p.signature))
            .collect();
        
        if similarities.is_empty() {
            return 0.5;
        }
        
        similarities.iter().sum::<f64>() / similarities.len() as f64
    }
    
    /// Korrelation zwischen zwei Signalen
    fn correlation(&self, a: &[f64], b: &[f64]) -> f64 {
        let len = a.len().min(b.len());
        if len == 0 {
            return 0.0;
        }
        
        let mean_a = a.iter().take(len).sum::<f64>() / len as f64;
        let mean_b = b.iter().take(len).sum::<f64>() / len as f64;
        
        let mut cov = 0.0;
        let mut var_a = 0.0;
        let mut var_b = 0.0;
        
        for i in 0..len {
            let da = a[i] - mean_a;
            let db = b[i] - mean_b;
            cov += da * db;
            var_a += da * da;
            var_b += db * db;
        }
        
        if var_a < 1e-10 || var_b < 1e-10 {
            return 0.0;
        }
        
        (cov / (var_a * var_b).sqrt()).clamp(-1.0, 1.0)
    }
    
    /// Varianz
    fn variance(&self, data: &[f64]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }
        
        let mean = data.iter().sum::<f64>() / data.len() as f64;
        data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / data.len() as f64
    }
    
    /// Gibt Statistiken über erkannte Muster
    pub fn pattern_stats(&self) -> PatternStats {
        let total = self.pattern_history.len();
        
        if total == 0 {
            return PatternStats::default();
        }
        
        let avg_coherence = self.pattern_history.iter().map(|p| p.coherence).sum::<f64>() / total as f64;
        let avg_complexity = self.pattern_history.iter().map(|p| p.complexity).sum::<f64>() / total as f64;
        let avg_stability = self.pattern_history.iter().map(|p| p.stability).sum::<f64>() / total as f64;
        
        // Zähle Muster-Typen
        let mut type_counts = std::collections::HashMap::new();
        for pattern in &self.pattern_history {
            *type_counts.entry(pattern.pattern_type).or_insert(0) += 1;
        }
        
        PatternStats {
            total_patterns: total,
            avg_coherence,
            avg_complexity,
            avg_stability,
            type_distribution: type_counts,
        }
    }
}

/// Muster-Statistiken
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternStats {
    pub total_patterns: usize,
    pub avg_coherence: f64,
    pub avg_complexity: f64,
    pub avg_stability: f64,
    pub type_distribution: std::collections::HashMap<PatternType, usize>,
}

impl Default for PatternStats {
    fn default() -> Self {
        Self {
            total_patterns: 0,
            avg_coherence: 0.0,
            avg_complexity: 0.0,
            avg_stability: 0.0,
            type_distribution: std::collections::HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_coherence_calculation() {
        let detector = EmergenceDetector::new(0.5);
        
        // Perfekt periodisches Signal
        let periodic: Vec<f64> = (0..16).map(|i| (i as f64 * PI / 4.0).sin()).collect();
        let coherence = detector.calculate_coherence(&periodic);
        
        assert!(coherence > 0.3, "Periodic signal should have high coherence");
    }
    
    #[test]
    fn test_shannon_entropy() {
        let detector = EmergenceDetector::new(0.5);
        
        // Gleichverteilung sollte hohe Entropie haben
        let uniform = vec![1.0, 1.0, 1.0, 1.0];
        let entropy_uniform = detector.shannon_entropy(&uniform);
        
        // Konzentrierte Verteilung sollte niedrige Entropie haben
        let concentrated = vec![10.0, 0.1, 0.1, 0.1];
        let entropy_concentrated = detector.shannon_entropy(&concentrated);
        
        assert!(entropy_uniform > entropy_concentrated);
    }
    
    #[test]
    fn test_pattern_classification() {
        let detector = EmergenceDetector::new(0.5);
        
        // Periodisches Signal
        let periodic: Vec<f64> = (0..32).map(|i| (i as f64 * PI / 4.0).sin()).collect();
        let pattern_type = detector.classify_pattern(&periodic);
        
        assert!(
            pattern_type == PatternType::Periodic || pattern_type == PatternType::Harmonic,
            "Should detect periodic/harmonic pattern"
        );
    }
}

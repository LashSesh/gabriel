use nalgebra::{Matrix4, Vector4};
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// 4D-Trichter: r(t,θ) = r₀ + f(t) · g(θ)
/// Modelliert Informationsakkumulation über Zeit mit Winkelabhängigkeit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trichter4D {
    /// Initiale Radius r₀
    pub r0: f64,
    
    /// Zeitliche Wachstumsfunktion Parameter
    pub growth_params: GrowthParams,
    
    /// Winkel-Modulationsfunktion Parameter
    pub angular_params: AngularParams,
    
    /// Informationsdichte Parameter
    pub density_params: DensityParams,
    
    /// Aktuelle Zeit
    pub time: f64,
}

/// Parameter für zeitliches Wachstum f(t)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrowthParams {
    /// Logarithmische Komponente
    pub log_scale: f64,
    
    /// Lineare Komponente
    pub linear_scale: f64,
    
    /// Spiralen-Frequenz
    pub spiral_frequency: f64,
    
    /// Dämpfung
    pub damping: f64,
}

impl Default for GrowthParams {
    fn default() -> Self {
        Self {
            log_scale: 1.0,
            linear_scale: 0.5,
            spiral_frequency: 0.1,
            damping: 0.01,
        }
    }
}

/// Parameter für Winkel-Modulation g(θ)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AngularParams {
    /// Anzahl der Sektoren
    pub sectors: usize,
    
    /// Sektor-Gewichte (für heterogene Verteilung)
    pub sector_weights: Vec<f64>,
    
    /// Periodizität
    pub periodicity: f64,
}

impl Default for AngularParams {
    fn default() -> Self {
        Self {
            sectors: 8,
            sector_weights: vec![1.0; 8],
            periodicity: 1.0,
        }
    }
}

/// Parameter für Informationsdichte ρ(r,θ,t)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DensityParams {
    /// Basis-Dichte ρ₀
    pub rho0: f64,
    
    /// Dichte-Änderung Δρ(t)
    pub delta_scale: f64,
    
    /// Räumliche Variation
    pub spatial_variation: f64,
}

impl Default for DensityParams {
    fn default() -> Self {
        Self {
            rho0: 1.0,
            delta_scale: 0.1,
            spatial_variation: 0.05,
        }
    }
}

impl Trichter4D {
    pub fn new(r0: f64) -> Self {
        Self {
            r0,
            growth_params: GrowthParams::default(),
            angular_params: AngularParams::default(),
            density_params: DensityParams::default(),
            time: 0.0,
        }
    }
    
    /// Berechnet Radius r(t,θ) für gegebene Zeit und Winkel
    pub fn radius(&self, t: f64, theta: f64) -> f64 {
        let f_t = self.growth_function(t);
        let g_theta = self.angular_modulation(theta);
        self.r0 + f_t * g_theta
    }
    
    /// Zeitliche Wachstumsfunktion f(t)
    fn growth_function(&self, t: f64) -> f64 {
        let p = &self.growth_params;
        
        // Logarithmische Komponente für anfängliches schnelles Wachstum
        let log_component = p.log_scale * (1.0 + t).ln();
        
        // Lineare Komponente für konstantes Wachstum
        let linear_component = p.linear_scale * t;
        
        // Spiralen-Komponente für oszillierende Dynamik
        let spiral_component = (p.spiral_frequency * t).sin() * 0.1;
        
        // Dämpfung für Langzeit-Stabilität
        let damping_factor = (-p.damping * t).exp();
        
        (log_component + linear_component + spiral_component) * damping_factor
    }
    
    /// Winkel-Modulationsfunktion g(θ)
    fn angular_modulation(&self, theta: f64) -> f64 {
        let p = &self.angular_params;
        
        // Normalisiere θ auf [0, 2π]
        let theta_norm = ((theta % (2.0 * PI)) + 2.0 * PI) % (2.0 * PI);
        
        // Sektor-Index
        let sector_size = 2.0 * PI / p.sectors as f64;
        let sector_idx = (theta_norm / sector_size).floor() as usize % p.sectors;
        
        // Sektor-spezifisches Gewicht
        let sector_weight = p.sector_weights.get(sector_idx).copied().unwrap_or(1.0);
        
        // Periodische Modulation
        let periodic_mod = (p.periodicity * theta_norm).cos() * 0.2 + 1.0;
        
        sector_weight * periodic_mod
    }
    
    /// Informationsdichte ρ(r,θ,t)
    pub fn density(&self, r: f64, theta: f64, t: f64) -> f64 {
        let p = &self.density_params;
        
        // Basisdichte
        let base = p.rho0;
        
        // Zeitliche Änderung
        let delta_rho = p.delta_scale * t.sin();
        
        // Räumliche Variation (Abfall mit Radius)
        let spatial_mod = 1.0 / (1.0 + p.spatial_variation * r);
        
        // Winkel-abhängige Variation
        let angular_mod = 1.0 + 0.1 * (3.0 * theta).cos();
        
        (base + delta_rho) * spatial_mod * angular_mod
    }
    
    /// 4D-Position: (r, θ, φ, t) für sphärische Koordinaten
    pub fn position_4d(&self, theta: f64, phi: f64, t: f64) -> Vector4<f64> {
        let r = self.radius(t, theta);
        Vector4::new(r, theta, phi, t)
    }
    
    /// Konvertiert 4D-Position zu kartesischen Koordinaten (x, y, z, t)
    pub fn to_cartesian(&self, pos: Vector4<f64>) -> Vector4<f64> {
        let r = pos[0];
        let theta = pos[1];
        let phi = pos[2];
        let t = pos[3];
        
        let x = r * theta.sin() * phi.cos();
        let y = r * theta.sin() * phi.sin();
        let z = r * theta.cos();
        
        Vector4::new(x, y, z, t)
    }
    
    /// Berechnet Tensorprodukt für Musterbildung
    /// Ψ_total = ⊗ᵢ₌₁ᴺ Ψᵢ(t)
    pub fn tensor_pattern(&self, t: f64, n_components: usize) -> Vec<f64> {
        let mut pattern = Vec::with_capacity(n_components);
        
        for i in 0..n_components {
            let theta = 2.0 * PI * i as f64 / n_components as f64;
            let r = self.radius(t, theta);
            let rho = self.density(r, theta, t);
            
            // Tensorkomponente
            let psi_i = (r * rho).sqrt();
            pattern.push(psi_i);
        }
        
        pattern
    }
    
    /// Metrischer Tensor für Informationsgeometrie
    pub fn metric_tensor(&self, pos: Vector4<f64>) -> Matrix4<f64> {
        let r = pos[0];
        let theta = pos[1];
        
        // Simplified metric for 4D funnel geometry
        // g_μν für (r, θ, φ, t)
        let mut g = Matrix4::zeros();
        
        // g_rr
        g[(0, 0)] = 1.0;
        
        // g_θθ
        g[(1, 1)] = r * r;
        
        // g_φφ
        g[(2, 2)] = r * r * theta.sin().powi(2);
        
        // g_tt (zeitliche Komponente)
        g[(3, 3)] = 1.0 + self.growth_params.spiral_frequency.powi(2);
        
        g
    }
    
    /// Christoffel-Symbole für geodätische Bewegung
    pub fn christoffel_symbols(&self, pos: Vector4<f64>) -> Vec<[[[f64; 4]; 4]; 4]> {
        let r = pos[0];
        let theta = pos[1];
        
        let mut gamma = vec![[[[0.0; 4]; 4]; 4]; 4];
        
        // Γʳ_θθ = -r
        gamma[0][1][1] = -r;
        
        // Γʳ_φφ = -r sin²(θ)
        gamma[0][2][2] = -r * theta.sin().powi(2);
        
        // Γᶿ_rθ = Γᶿ_θr = 1/r
        if r > 0.0 {
            gamma[1][0][1] = 1.0 / r;
            gamma[1][1][0] = 1.0 / r;
        }
        
        // Γᶿ_φφ = -sin(θ)cos(θ)
        gamma[1][2][2] = -theta.sin() * theta.cos();
        
        // Γᵠ_rφ = Γᵠ_φr = 1/r
        if r > 0.0 {
            gamma[2][0][2] = 1.0 / r;
            gamma[2][2][0] = 1.0 / r;
        }
        
        // Γᵠ_θφ = Γᵠ_φθ = cot(θ)
        if theta.sin().abs() > 1e-10 {
            let cot_theta = theta.cos() / theta.sin();
            gamma[2][1][2] = cot_theta;
            gamma[2][2][1] = cot_theta;
        }
        
        gamma
    }
    
    /// Berechnet geodätische Linie (optimaler Pfad im Informationsraum)
    pub fn geodesic(&self, start: Vector4<f64>, end: Vector4<f64>, steps: usize) -> Vec<Vector4<f64>> {
        let mut path = Vec::with_capacity(steps);
        
        for i in 0..steps {
            let s = i as f64 / (steps - 1) as f64;
            
            // Einfache lineare Interpolation (für komplexere Geodäten: Integrate Christoffel symbols)
            let pos = start * (1.0 - s) + end * s;
            path.push(pos);
        }
        
        path
    }
    
    /// Update Zeitschritt
    pub fn step(&mut self, dt: f64) {
        self.time += dt;
    }
}

/// Trichter-Statistiken
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrichterStats {
    pub current_radius: f64,
    pub avg_density: f64,
    pub total_volume: f64,
    pub time: f64,
}

impl Trichter4D {
    pub fn stats(&self) -> TrichterStats {
        let current_radius = self.radius(self.time, 0.0);
        
        // Durchschnittliche Dichte über alle Winkel
        let n_samples = 100;
        let avg_density: f64 = (0..n_samples)
            .map(|i| {
                let theta = 2.0 * PI * i as f64 / n_samples as f64;
                let r = self.radius(self.time, theta);
                self.density(r, theta, self.time)
            })
            .sum::<f64>()
            / n_samples as f64;
        
        // Approximatives Volumen (vereinfacht)
        let total_volume = 4.0 / 3.0 * PI * current_radius.powi(3);
        
        TrichterStats {
            current_radius,
            avg_density,
            total_volume,
            time: self.time,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    
    #[test]
    fn test_trichter_radius_growth() {
        let trichter = Trichter4D::new(1.0);
        
        let r0 = trichter.radius(0.0, 0.0);
        let r1 = trichter.radius(10.0, 0.0);
        
        assert!(r1 > r0, "Radius should grow over time");
    }
    
    #[test]
    fn test_angular_modulation() {
        let trichter = Trichter4D::new(1.0);
        
        let r1 = trichter.radius(5.0, 0.0);
        let r2 = trichter.radius(5.0, PI);
        
        // Radius sollte winkelabhängig sein
        assert_ne!(r1, r2);
    }
    
    #[test]
    fn test_density_calculation() {
        let trichter = Trichter4D::new(1.0);
        
        let rho = trichter.density(1.0, 0.0, 0.0);
        assert!(rho > 0.0);
    }
    
    #[test]
    fn test_cartesian_conversion() {
        let trichter = Trichter4D::new(1.0);
        
        let pos_4d = Vector4::new(1.0, PI / 2.0, 0.0, 0.0);
        let cart = trichter.to_cartesian(pos_4d);
        
        // Bei θ=π/2, φ=0 sollte z≈0 sein
        assert_relative_eq!(cart[2], 0.0, epsilon = 1e-10);
    }
    
    #[test]
    fn test_tensor_pattern() {
        let trichter = Trichter4D::new(1.0);
        
        let pattern = trichter.tensor_pattern(0.0, 8);
        assert_eq!(pattern.len(), 8);
        assert!(pattern.iter().all(|&x| x >= 0.0));
    }
}

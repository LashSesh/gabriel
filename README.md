# Gabriel Organismus

Ein domänenagnostisches, biologisch inspiriertes kognitives System zur emergenten Informationsverarbeitung.

## Überblick

Gabriel ist ein sophistiziertes Informationsverarbeitungssystem, das biologische Prinzipien auf die Wissensverarbeitung überträgt. Das System behandelt **beliebige Informationstypen** einheitlich durch eine universelle Abstraktion (`InformationQuantum`) und ermöglicht emergentes Lernen ohne explizite Programmierung von Logik oder Strukturen.

### Kernprinzipien

- **Domänenagnostik**: Ein einziges Trait (`InformationQuantum`) ermöglicht die Verarbeitung beliebiger Informationstypen – von mathematischen Objekten über Text bis hin zu Vektoren
- **Emergentes Lernen**: Strukturen und Muster entstehen durch Hebbsches Lernen und Selbstorganisation, nicht durch explizite Programmierung
- **Biologischer Realismus**: Metabolische Zyklen, Energieerhaltung und neuronale Dynamik nach biologischem Vorbild
- **Mathematische Fundierung**: 4D-Trichtergeometrie, Fourier-Analyse, Geodäten und Tensorprodukte

---

## Architektur

Das System ist als Rust-Workspace mit sieben spezialisierten Crates organisiert, ergänzt durch eine Python-Schicht für analytisches Scoring.

```
gabriel/
├── gabriel-core/           # Neuronales Substrat & InformationQuantum-Trait
├── trichter-geometry/      # 4D-Trichtergeometrie für Informationsakkumulation
├── metabolics/             # Metabolischer Zyklus der Informationsverarbeitung
├── emergence/              # Mustererkennung und Klassifikation
├── organism/               # Zentraler Orchestrator mit Lernen, Gedächtnis, Reasoning
├── mathematical-cognition/ # Mathematische Domäneninstanziierung
├── fusion-layer/           # Python-Rust-Brücke für Triton-Scoring
└── python/                 # Triton-Alchemie-Engine (Analytisches Framework)
```

---

## Kernmodule

### Gabriel-Core

Das Fundament des gesamten Systems. Definiert die universelle Informationsabstraktion und das selbstorganisierende neuronale Netzwerk.

**InformationQuantum-Trait** – Die zentrale Abstraktion:

```rust
pub trait InformationQuantum: Clone + Debug + Send + Sync {
    type Id: Clone + Debug + Hash + Eq + Send + Sync;

    fn id(&self) -> Self::Id;                    // Eindeutiger Bezeichner
    fn resonance(&self, other: &Self) -> f64;    // Ähnlichkeit [0, 1]
    fn energy(&self) -> f64;                     // Aktivierungspotential
    fn fuse(&self, other: &Self, weight: f64) -> Self;  // Kombination
}
```

**GabrielCell** – Selbstorganisierendes neuronales Netzwerk:
- `GabrielNeuron`: Speichert Informationsquantum + Aktivierungszustand + Energie + Position im 4D-Raum
- `GabrielEdge`: Synaptische Verbindungen mit Hebbscher Plastizität
- `SynapticWeight`: Modelliert Lernen mit Verstärkung, Zerfall und Pruning-Schwellwerten
- **Hebbsches Lernen**: "Neuronen, die gemeinsam feuern, verdrahten sich"
- **Strukturelles Pruning**: Entfernt schwache Verbindungen über Zeit

### Trichter-Geometrie

Mathematischer Raum für Informationsakkumulation in 4D.

**Geometrische Grundlage**:
```
r(t,θ) = r₀ + f(t)·g(θ)
```

- **Zeitabhängiges Wachstum**: Logarithmische + lineare + spiralförmige Komponenten
- **Winkelmodulation**: Sektorgewichtete Verteilung
- **Dichtefelder**: Informationsdichte ρ(r,θ,t)
- **Metriktensor**: Geodätische Pfadberechnung für optimale Routen durch den Informationsraum

### Metabolics

Biologisch inspirierter Stoffwechsel für Informationsverarbeitung.

**Fünf metabolische Phasen**:

1. **Ingestion** – Aufnahme neuer Informationsquanten
2. **Digestion** – Erzeugung neuronaler Verbindungen basierend auf räumlicher Nähe
3. **Assimilation** – Integration stabiler Strukturen in Langzeitwissen
4. **Excretion** – Pruning schwacher/redundanter Informationen
5. **Synthese** – Entdeckung emergenter Strukturen

**CoherenceFeedback**: Externe Abstimmung aus dem Triton-Scoring-Framework zur dynamischen Anpassung metabolischer Raten.

### Emergence

Mustererkennung und -klassifikation durch Tensoranalyse.

**Erkannte Mustertypen**:
- **Periodisch**: Wiederkehrende Muster
- **Harmonisch**: Resonanzbasierte Beziehungen
- **Fraktal**: Selbstähnliche Strukturen
- **Chaotisch**: Sensitive Abhängigkeit (Ljapunow-Exponent)
- **Hierarchisch**: Mehrstufige Organisation

**Methoden**: Fourier-Analyse, Autokorrelation, Shannon-Entropie

### Organism

Zentraler Koordinator, der alle Komponenten integriert.

**Drei kognitive Subsysteme**:

1. **Lernen** (`learning.rs`): Verstärkungslernen für Beweis-Erfolg
2. **Gedächtnis** (`memory.rs`): Langzeitspeicher mit Zerfall und Konsolidierung
3. **Reasoning** (`reasoning.rs`): Musterbasierte Konjekturgenerierung

**Lebenszyklus-Mechanik**:
- Gesundheit, Bewusstsein, Altersfortschritt
- Überlebensschwellwert
- Awareness emergenter Muster

### Mathematical-Cognition

Instanziierung des domänenagnostischen Systems für mathematische Objekte.

**Mathematische Typen**:

| Typ | Beschreibung |
|-----|--------------|
| `MathInteger` | Zahlen mit Eigenschaften (gerade, ungerade, prim, zusammengesetzt) |
| `MathPrime` | Spezielle Primeigenschaften (Zwilling, Sophie Germain, Mersenne) |
| `MathComplex` | Komplexe Zahlen |
| `MathFunction` | Funktionsobjekte mit Definitions-/Wertebereichen |
| `MathTheorem` | Theoremrepräsentation |
| `MathEquation` | Gleichungen als Quanten |
| `Proof` | Beweisobjekte mit Beweisschritten |

**Resonanzberechnung für MathInteger**:
- 50% geteilte Eigenschaften (beide prim, beide gerade, etc.)
- 30% numerische Nähe
- 20% Teilbarkeitsbeziehungen

### Fusion-Layer

Brücke zwischen Rust und Python für analytisches Scoring.

**Komponenten**:
- `FusionLayer`: Integration von Triton-Scoring mit metabolischem Feedback
- `FusionPacket`: Einheitliches Datenmodell für Tensor → Scores

**Berechnete Metriken**:
- **Kohärenz**: Signalstabilität (Autokorrelation)
- **Entropie**: Shannon-Entropie der Verteilungen
- **Stabilität**: Komposit-Metrik (Kohärenz × (1 - Entropie))

### Python-Schicht (Triton-Alchemie)

Fortgeschrittenes analytisches Framework.

**Kernkonzepte**:
- **TRITON-Alchemie-Engine**: 5D-Spiralexploration mit Spektralfeldmessung
- **SpectralSignature**: σ = (ψ, ρ, ω) für Kohärenz, Dichte, Frequenz
- **Ouroboros-Feedback**: Selbstreferentielles Momentum für Spiralnavigation
- **Solve et Coagula**: Phasenübergänge und Merkaba-Gate-Evaluation

---

## Datenfluss

```
┌─────────────────────────────────────────────────────────────────┐
│                    FEEDING-PHASE                                │
├─────────────────────────────────────────────────────────────────┤
│  Quantum → InformationMetabolism.ingest()                       │
│         → GabrielCell.add_neuron() + Trichter4D-Position        │
│         → Energie + Position gespeichert                        │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│                   LIFECYCLE-PHASE                               │
├─────────────────────────────────────────────────────────────────┤
│  Metabolism.step() →                                            │
│    digest()      [Proximity-basierte Verbindungen]              │
│    assimilate()  [Stabile Strukturintegration]                  │
│    excrete()     [Pruning]                                      │
│    synthesize()  [Emergente Strukturentdeckung]                 │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│                   REASONING-PHASE                               │
├─────────────────────────────────────────────────────────────────┤
│  GabrielOrganism.lifecycle() →                                  │
│    EmergenceDetector.analyze_tensor()                           │
│    MathematicalReasoning.detect_pattern()                       │
│    LearningSignal [Reward/Punishment]                           │
│    MathematicalLearning.apply_signal()                          │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│                   FEEDBACK-PHASE                                │
├─────────────────────────────────────────────────────────────────┤
│  FusionLayer.fusion_cycle() →                                   │
│    Triton-Evaluation (Python)                                   │
│    FusionPacket-Erstellung                                      │
│    CoherenceFeedback → InformationMetabolism                    │
│    Metabolische Raten angepasst                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Verwendung

### Voraussetzungen

- Rust 1.75+ mit `cargo`
- Python 3.9+ (für Triton-Komponenten)
- Optional: `llvm-tools-preview` für Benchmarking

### Bauen

```bash
cargo build --release
```

### Organismus ausführen

```bash
cargo run -p organism --bin gabriel-organism -- \
    --cycles 120 \
    --feed-count 80 \
    --radius 1.5 \
    --verbose
```

**CLI-Optionen**:

| Flag | Beschreibung | Standard |
|------|--------------|----------|
| `--cycles <u64>` | Anzahl der Lebenszyklusiterationen | 100 |
| `--feed-count <u64>` | Anzahl der zu ingestierenden Quanten | 50 |
| `--radius <f64>` | Initialer Trichterradius | 1.0 |
| `--output <path>` | Export des Endzustands als JSON | - |
| `--verbose` | Debug-Level Tracing aktivieren | false |

### Programmatische Integration

```rust
use organism::GabrielOrganism;
use gabriel_core::InformationQuantum;

// Organismus erstellen
let mut organism = GabrielOrganism::new(config);

// Informationen füttern
organism.feed(quantum);

// Lebenszyklus ausführen
organism.lifecycle();

// Diagnostik abrufen
let diagnostics = organism.diagnostics();
```

### Tests ausführen

```bash
cargo test                           # Alle Tests
cargo test -p mathematical-cognition # Nur mathematische Kognition
cargo bench                          # Benchmarks (mit Criterion)
```

---

## Domänenagnostisches Design

Das System ist **bewusst domänenneutral** konzipiert:

### 1. Generisches Quantum-Interface

Funktioniert mit beliebigen Typen, die `InformationQuantum` implementieren:
- `MathInteger`, `MathPrime`, `MathTheorem`
- `UniversalQuantum` (Bytes-basiert, für beliebige Daten)
- Eigene Typen durch einfache Trait-Implementierung

### 2. Pluggable Evaluatoren

```python
class SpectralEvaluator(ABC):
    @abstractmethod
    def evaluate(self, tensor: np.ndarray) -> SpectralSignature:
        pass
```

Neue Evaluatoren können ohne Änderung des Kerns hinzugefügt werden.

### 3. Generischer Metabolischer Zyklus

- Arbeitet auf beliebigen Quantum-Typen
- Digestion nutzt räumliche Nähe (Trichter-4D-Koordinaten)
- Keine Annahmen über die Art der Information

### 4. Mathematik als Spezialfall

Die mathematische Kognition ist **nicht** in den Kern eingebaut:
- Eigenständiges Domänenmodul
- Instanziiert `InformationQuantum` für mathematische Objekte
- Dieselben Mechanismen funktionieren für andere Domänen

---

## Neue Domäne hinzufügen

Um eine neue Domäne zu unterstützen, implementieren Sie lediglich `InformationQuantum`:

```rust
#[derive(Clone, Debug)]
pub struct MeineDomäne {
    id: String,
    daten: Vec<f64>,
}

impl InformationQuantum for MeineDomäne {
    type Id = String;

    fn id(&self) -> Self::Id {
        self.id.clone()
    }

    fn resonance(&self, other: &Self) -> f64 {
        // Domänenspezifische Ähnlichkeitsberechnung
        kosinusähnlichkeit(&self.daten, &other.daten)
    }

    fn energy(&self) -> f64 {
        // Domänenspezifisches Aktivierungspotential
        self.daten.iter().map(|x| x.abs()).sum::<f64>()
    }

    fn fuse(&self, other: &Self, weight: f64) -> Self {
        // Domänenspezifische Kombination
        MeineDomäne {
            id: format!("{}+{}", self.id, other.id),
            daten: self.daten.iter()
                .zip(&other.daten)
                .map(|(a, b)| a * (1.0 - weight) + b * weight)
                .collect(),
        }
    }
}
```

Die gesamte kognitive Maschinerie – neuronale Dynamik, metabolische Zyklen, Mustererkennung – funktioniert dann automatisch mit der neuen Domäne.

---

## Laufzeitverhalten

### Organismus-Lebenszyklus

1. **Erstellung**: Initialisierung mit zufälligen Neuronen und Trichtergeometrie
2. **Fütterung**: Aufnahme von Informationsquanten → Speicherung als Neuronen
3. **Verarbeitung**: Metabolischer Zyklus verarbeitet und verfeinert Information
4. **Mustererkennung**: Emergence-Detektor identifiziert kohärente Muster
5. **Lernen**: Erfolgreiche Muster verstärken neuronale Gewichte
6. **Alterung**: Gesundheit zerfällt, Gedächtnis konsolidiert, Neuronen refaktorieren
7. **Tod**: Organismus stirbt bei Gesundheit < Überlebensschwellwert

### Mathematisches Lernbeispiel

```
Eingabe: Integer 12
  ↓
Erkannte Eigenschaften: gerade, zusammengesetzt, positiv
  ↓
Resonanz mit: 6 (Teiler), 24 (Vielfaches), 13 (Nachbar)
  ↓
Bei erfolgreichem Beweis: Gewicht(12→nächster_Knoten) steigt
  ↓
Über Zeit: Organismus lernt "gerade zusammengesetzte Zahlen
           erscheinen oft in bestimmten Beweismustern"
```

---

## Design-Entscheidungen

### Rust für den Kern

- **Performance**: Kompilierzeit-Polymorphismus ohne vtables
- **Typsicherheit**: Generische Parameter `<Q: InformationQuantum>` durchgehend
- **Nebenläufigkeit**: `Arc<RwLock<>>` für Interior Mutability

### Python für Analytik

- Komplexe analytische Algorithmen (FFT, Statistik)
- Schnelle Iteration bei experimentellen Evaluatoren
- NumPy/SciPy-Ökosystem

### Modularer Workspace

- Klare Modulgrenzen
- Unabhängig wiederverwendbare Komponenten
- Einzelne Crates können separat gebaut und getestet werden

---

## Systemcharakteristika

### Stärken

- **Echte Domänenagnostik**: Eine Codebasis für beliebige Informationstypen
- **Emergentes Lernen**: Keine explizite Logikprogrammierung, Strukturen emergieren
- **Biologischer Realismus**: Hebbsche Regeln, metabolische Zyklen, Energieerhaltung
- **Mathematische Sophistikation**: Fourier-Analyse, Geodäten, Tensorprodukte
- **Produktionsreif**: Umfassende Fehlerbehandlung, Logging, Testing

### Neuartige Konzepte

- **4D-Trichtergeometrie**: Informationsakkumulation mit Winkelsektoren
- **Ouroboros-Feedback-Schleife**: Selbstreferentielle Navigation im Spiralraum
- **Solve et Coagula**: Alchemistische Phasenübergänge (flüssig ↔ fest)
- **Merkaba-Gate**: Multi-Konditions-Emergenz-Gating
- **Einheitliche Spektralsignatur**: σ = (ψ, ρ, ω) für universelle Evaluation

---

## Lizenz

Apache-2.0 / MIT (Dual-Lizenz)

---

## Autor

Sebastian Klemm

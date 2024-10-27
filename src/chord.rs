use crate::Rng;

pub const ROOTS: &'static [&'static str] =
    &["A", "A♯", "B", "C", "C♯", "D", "D♯", "E", "F", "F♯", "G", "G♯"];

pub struct ChordType {
    /// Interval pattern for base triad or 7th chord
    intervals: (u8, u8, u8),

    /// Full name of the chord type
    name: &'static str,

    /// Base suffix for the chord type
    base_suffix: &'static str,

    /// Whether this chord type allows extensions
    allows_extensions: bool,

    /// List of allowed alterations for this chord type
    allowed_alterations: &'static [&'static str],
}

/// Various chord types with corresponding intervals and names
pub const CHORD_TYPES: &'static [ChordType] = &[
    ChordType { intervals: (4, 3, 4), name: "Major",
                base_suffix: "maj7", allows_extensions: true,
                allowed_alterations: &["♯11", "♭13"] },
    ChordType { intervals: (3, 4, 3), name: "Minor",
                base_suffix: "m7", allows_extensions: true,
                allowed_alterations: &["♭9", "♯11"] },
    ChordType { intervals: (4, 3, 3), name: "Dominant",
                base_suffix: "7", allows_extensions: true,
                allowed_alterations: &["♭9", "♯9", "♯11", "♭13"] },
    ChordType { intervals: (3, 3, 3), name: "Diminished",
                base_suffix: "dim7", allows_extensions: false,
                allowed_alterations: &[] },
    ChordType { intervals: (3, 3, 4), name: "Half-Diminished",
                base_suffix: "m7♭5", allows_extensions: true,
                allowed_alterations: &["♯9", "♯11"] },
    ChordType { intervals: (3, 4, 4), name: "Minor-Major",
                base_suffix: "minmaj7", allows_extensions: false,
                allowed_alterations: &["♯11"] },
    ChordType { intervals: (4, 4, 3), name: "Augmented Major",
                base_suffix: "augmaj7", allows_extensions: true,
                allowed_alterations: &["♯11"] },
    ChordType { intervals: (3, 3, 5), name: "Diminished Major",
                base_suffix: "dimmaj7", allows_extensions: false,
                allowed_alterations: &[] },
];

/// Extensions available for complex chords
pub const EXTENSIONS: [&'static str; 3] = ["9", "11", "13"];

/// Randomly selects a root note
pub fn random_root(rng: &mut Rng) -> &'static str {
    ROOTS[rng.next() % ROOTS.len()]
}

/// Randomly selects a chord type
pub fn random_chord_type(rng: &mut Rng) -> &'static ChordType {
    &CHORD_TYPES[rng.next() % CHORD_TYPES.len()]
}

/// Generates a random chord with optional complexity flag
pub fn random_chord(rng: &mut Rng, complex: bool) -> String {
    let root = random_root(rng);
    let chord_type = random_chord_type(rng);
    let mut chord_str = format!("{}{}", root, chord_type.base_suffix);

    // If complex chords are allowed
    // and the chord type permits extensions or alterations
    if complex && (chord_type.allows_extensions ||
            !chord_type.allowed_alterations.is_empty()) {
        // Optionally add an extension (e.g., 9th, 11th, 13th)
        if chord_type.allows_extensions {
            let extension = EXTENSIONS[rng.next() % EXTENSIONS.len()];
            chord_str.push_str(&format!(" {}", extension));
        }

        // Optionally add up to two alterations if allowed for this chord type
        if !chord_type.allowed_alterations.is_empty() {
            let num_alterations = rng.next() % 3; // 0 to 2 alterations
            let mut alterations = Vec::new();
            for _ in 0..num_alterations {
                let alteration = chord_type.allowed_alterations[
                    rng.next() % chord_type.allowed_alterations.len()];
                if !alterations.contains(&alteration) {
                    alterations.push(alteration);
                }
            }
            if !alterations.is_empty() {
                chord_str.push_str(" (");
                chord_str.push_str(&alterations.join(", "));
                chord_str.push(')');
            }
        }
    }

    // Include the interval structure for reference
    // "Diminished Major".len() == 16
    format!("{:>16} | {:?} | {}",
            chord_type.name, chord_type.intervals, chord_str)
}

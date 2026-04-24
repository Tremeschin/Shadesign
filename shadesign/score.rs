use crate::*;

#[derive(SmartDefault, Debug, PartialEq, Eq)]
pub struct Score {

    /// Comparing word A
    pub A: String,

    /// Comparing word B
    pub B: String,

    /// How many characters overlaps:
    /// - (sha-des + des-ign) = 3
    /// - (des-ign + sha-des) = 0
    pub overlap: usize,

    /// Whether both words have even length (center point)
    pub even: bool,

    /// Whether the overlap point is exactly in the middle
    /// - (sha-des + des-ign)    = true
    /// - (out-smart + smart-er) = false
    pub symmetric: bool,
}

impl Score {
    pub fn compute(A: String, B: String) -> Option<Self> {
        let mut score = Score::default();

        // Compute the overlap (utf-8 indices!)
        for (i, _) in B.char_indices() {
            if i >= A.len() {
                break;
            }
            if A.ends_with(&B[..i]) {
                score.overlap = i;
            }
        }

        // Skip boring overlaps
        if score.overlap <= 2 {
            return None;
        }

        // Check even-ness
        if A.len().is_multiple_of(2) {
            if B.len().is_multiple_of(2) {
                score.even = true;
            }
        }

        // Check symmetry
        if A.len() == score.overlap * 2 {
            if B.len() == score.overlap * 2 {
                score.symmetric = true;
            }
        }

        score.A = A;
        score.B = B;
        Some(score)
    }
}

/* -------------------------------------------------------------------------- */

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Score {
    fn cmp(&self, other: &Self) -> Ordering {
        self.overlap.cmp(&other.overlap)
    }
}

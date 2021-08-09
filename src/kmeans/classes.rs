use std::ops::{Index, IndexMut};

/// Maximum difference between 2 classes to consider them similar (see [`is_stable`]).
pub const CLASS_SIMILARITY_THRESHOLD: usize = 3;

/// Classes structure.
pub struct Classes {
    vector: Vec<u8>,
}

impl Classes {
    /// Initialize the class vector with equidistant classes.
    pub fn new(classes: usize) -> Self {
        let mut class = Self {
            vector: Vec::with_capacity(classes)
        };

        let class_base: u8 = (255 / classes) as u8;

        for i in 0..classes {
            let class_val = (class_base * i as u8) + (class_base / 2) as u8;
            class.vector.push(class_val);
        }

        return class;
    }

    /// Clone a classes object.
    pub fn clone(&self) -> Self {
        Self {
            vector: self.vector.clone(),
        }
    }

    /// Vector length
    pub fn len(&self) -> usize {
        self.vector.len()
    }

    /// Returns true if the classes are close enough.
    pub fn is_stable(&self, rhs: &Self) -> bool {
        for index in 0..self.vector.len() {
            let diff = i32::abs(self[index] as i32 - rhs[index] as i32);
            if diff as usize >= CLASS_SIMILARITY_THRESHOLD {
                return false;
            }
        }
        return true;
    }

    pub fn center(&self, bound_min: usize, bound_max: usize) -> u8 {
        let (&lhs, &rhs) = (&self.vector[bound_min], &self.vector[bound_max]);
        let (min, max) = (u8::min(lhs, rhs), u8::max(lhs, rhs));
        min + (max - min) / 2
    }
}

/// Immutable vector operator `[]`.
impl Index<usize> for Classes {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        &self.vector[index]
    }
}

/// Mutable vector operator `[]`.
impl IndexMut<usize> for Classes {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.vector[index]
    }
}

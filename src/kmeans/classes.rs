use std::ops::{Index, IndexMut};

/// Class amount.
pub const CLASS_NUM      : usize    = 4;
/// Class base value.
pub const CLASS_BASE     : u8       = (255 / CLASS_NUM) as u8;
/// Maximum difference between 2 classes to consider them similar (see [`is_stable`]).
pub const CLASS_THRESHOLD: usize    = 3;

/// Classes structure.
pub struct Classes
{
    array: [u8; CLASS_NUM]
}

impl Classes
{
    /// Initialize the class array with equidistant classes.
    pub fn init() -> Classes
    {
        let mut class = Classes{ array: [0; CLASS_NUM] };
        for i in 0..CLASS_NUM
        {
            class[i] = (CLASS_BASE * i as u8) + (CLASS_BASE / 2) as u8;
        }
        return class;
    }

    /// Clone a classes object.
    pub fn clone(&self) -> Classes
    {
        Classes
        {
            array: self.array.clone()
        }
    }
}

/// Immutable array operator `[]`.
impl Index<usize> for Classes
{
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output
    {
        &self.array[index]
    }
}

/// Mutable array operator `[]`.
impl IndexMut<usize> for Classes
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output
    {
        &mut self.array[index]
    }
}
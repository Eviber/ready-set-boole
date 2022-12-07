#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OptionBool {
    False,
    True,
    DontCare,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Row {
    pub values: Vec<OptionBool>,
    pub id: Vec<usize>,
}

use std::fmt;
impl fmt::Debug for OptionBool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OptionBool::True => write!(f, "1"),
            OptionBool::False => write!(f, "0"),
            OptionBool::DontCare => write!(f, "-"),
        }
    }
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for v in &self.values {
            write!(f, "{:?}", v)?;
        }
        Ok(())
    }
}

impl From<bool> for OptionBool {
    fn from(b: bool) -> Self {
        if b {
            OptionBool::True
        } else {
            OptionBool::False
        }
    }
}

impl From<&Row> for u32 {
    fn from(row: &Row) -> Self {
        row.values.iter().rev().fold(0, |acc, x| {
            (acc << 1)
                | match x {
                    OptionBool::True => 1,
                    _ => 0,
                }
        })
    }
}

impl Row {
    pub fn new(id: usize, width: usize) -> Row {
        let mut values = vec![OptionBool::False; width];
        (0..width).for_each(|i| {
            values[i] = OptionBool::from((id >> (width - i - 1)) & 1 == 1);
        });
        Row {
            values,
            id: vec![id],
        }
    }

    /// get a bitfield for the care bits
    fn care(&self) -> u32 {
        let mut res = 0;
        for (i, v) in self.values.iter().enumerate() {
            if *v != OptionBool::DontCare {
                res |= 1 << (self.values.len() - i - 1);
            }
        }
        res
    }

    /// get the bit difference between two rows
    fn diff(&self, other: &Row) -> u32 {
        u32::from(self) ^ u32::from(other)
    }

    /// mark the desired bits as dont care
    fn mark(&mut self, mask: u32) {
        for (i, v) in self.values.iter_mut().enumerate().rev() {
            if (mask >> i) & 1 == 1 {
                *v = OptionBool::DontCare;
            }
        }
    }

    /// merge two rows
    pub fn merge(&self, other: &Row) -> Row {
        let mut res = self.clone();
        res.mark(self.diff(other));
        res.id.extend_from_slice(&other.id);
        res
    }

    pub fn can_merge(&self, other: &Row) -> bool {
        self.care() == other.care() && self.diff(other).count_ones() == 1
    }
}

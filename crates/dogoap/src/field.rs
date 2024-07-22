use bevy_reflect::*;
use std::fmt::Display;
use std::hash::Hash;
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Reflect, Clone, Debug, PartialOrd, Copy)]
pub enum Field {
    Bool(bool),
    I64(i64),
    F64(f64),
    Enum(usize)
}

impl Hash for Field {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
        match self {
            Field::Bool(b) => b.hash(state),
            Field::I64(i) => i.hash(state),
            Field::F64(f) => f.to_bits().hash(state),
            Field::Enum(u) => u.hash(state),
        }
    }
}

impl PartialEq for Field {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Bool(l0), Self::Bool(r0)) => l0 == r0,
            (Self::I64(l0), Self::I64(r0)) => l0 == r0,
            (Self::F64(l0), Self::F64(r0)) => l0 == r0,
            (Self::Enum(l0), Self::Enum(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl Eq for Field {}

impl Field {
    pub fn from_bool(value: bool) -> Self {
        Field::Bool(value)
    }
    pub fn from_i64(value: i64) -> Self {
        Field::I64(value)
    }
    pub fn from_f64(value: f64) -> Self {
        Field::F64(value)
    }
    pub fn from_enum(value: usize) -> Self {
        Field::Enum(value)
    }

    pub fn distance(&self, other: &Field) -> u64 {
        match (self, other) {
            (Field::Bool(a), Field::Bool(b)) => if a == b { 0 } else { 1 },
            (Field::I64(a), Field::I64(b)) => (a - b).abs() as u64,
            (Field::F64(a), Field::F64(b)) => (a - b).abs() as u64,
            (Field::Enum(a), Field::Enum(b)) => if a == b { 0 } else { 1 },
            _ => panic!("Cannot calculate distance between different Field types"),
        }
    }
}

impl From<bool> for Field {
    fn from(v: bool) -> Self {
        Field::Bool(v)
    }
}

// impl From<usize> for Field {
//     fn from(v: usize) -> Self {
//         Field::Enum(v)
//     }
// }

impl From<i64> for Field {
    fn from(v: i64) -> Self {
        Field::I64(v)
    }
}

impl From<f64> for Field {
    fn from(v: f64) -> Self {
        Field::F64(v)
    }
}



impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool(v) => {
                write!(f, "Field:Bool({})", v)
            }
            Self::I64(v) => {
                write!(f, "Field:I64({})", v)
            }
            Self::F64(v) => {
                write!(f, "Field:F64({})", v)
            }
            Self::Enum(v) => {
                write!(f, "Field:Enum({})", v)
            }
        }
    }
}

impl Add for &Field {
    type Output = Field;

    fn add(self, other: &Field) -> Field {
        match (self, other) {
            (Field::I64(a), Field::I64(b)) => Field::I64(a + b),
            (Field::F64(a), Field::F64(b)) => Field::F64(a + b),
            _ => panic!("Unsupported addition between Field variants, {:?} - {:?}", self, other),
        }
    }
}

impl Add for Field {
    type Output = Field;

    #[allow(clippy::op_ref)]
    fn add(self, other: Field) -> Field {
        &self + &other
    }
}

impl Sub for &Field {
    type Output = Field;

    fn sub(self, other: &Field) -> Field {
        match (self, other) {
            (Field::I64(a), Field::I64(b)) => Field::I64(a - b),
            (Field::F64(a), Field::F64(b)) => Field::F64(a - b),
            _ => panic!("Unsupported negation between Field variants, {:?} - {:?}", self, other),
        }
    }
}

impl Sub for Field {
    type Output = Field;

    #[allow(clippy::op_ref)]
    fn sub(self, other: Field) -> Field {
        &self - &other
    }
}

impl AddAssign for Field {
    fn add_assign(&mut self, rhs: Self) {
        match self {
            Self::I64(ref mut v1) => {
                match rhs {
                    Self::I64(v2) => {
                        *v1 += v2;
                    },
                    _ => panic!("Unimplemented! Tried to remove {:?} from {:?}", self, rhs)
                }
            },
            Self::F64(ref mut v1) => {
                match rhs {
                    Self::F64(v2) => {
                        *v1 += v2;
                    },
                    _ => panic!("Unimplemented! Tried to remove {:?} from {:?}", self, rhs)
                }
            },
            _ => panic!("Unimplemented! Tried to remove {:?} from {:?}", self, rhs)
        }
    }
}

impl SubAssign for Field {
    fn sub_assign(&mut self, rhs: Self) {
        match self {
            Self::I64(ref mut v1) => {
                match rhs {
                    Self::I64(v2) => {
                        *v1 -= v2;
                    },
                    _ => panic!("Unimplemented! Tried to remove {:?} from {:?}", self, rhs)
                }
            },
            Self::F64(ref mut v1) => {
                match rhs {
                    Self::F64(v2) => {
                        *v1 -= v2;
                    },
                    _ => panic!("Unimplemented! Tried to remove {:?} from {:?}", self, rhs)
                }
            },
            _ => panic!("Unimplemented! Tried to remove {:?} from {:?}", self, rhs)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;
    #[test]
    fn test_from_bool() {
        let field = Field::from(true);
        assert_eq!(field, Field::from_bool(true));
    }

    #[test]
    fn test_from_i64() {
        let field = Field::from(123 as i64);
        assert_eq!(field, Field::from_i64(123 as i64));
    }

    #[test]
    fn test_from_f64() {
        let field = Field::from(123 as f64);
        assert_eq!(field, Field::from_f64(123 as f64));
    }

    #[test]
    fn test_equality() {
        assert_eq!(Field::from(true), Field::from_bool(true));
        assert_eq!(Field::from(666), Field::from_i64(666));
        assert_eq!(Field::from(666.666), Field::from_f64(666.666));
    }

    #[test]
    fn test_greater_than() {
        // Int
        assert!(Field::from(100) > Field::from(10));
        assert!(Field::from(1) > Field::from(0));

        // Float
        assert!(Field::from(1.2) > Field::from(1.1));
    }

    #[test]
    fn test_greater_than_equals() {
        // Int
        assert!(Field::from(100) >= Field::from(10));
        assert!(Field::from(1) >= Field::from(0));
        assert!(Field::from(100) >= Field::from(100));
        assert!(!(Field::from(100) >= Field::from(101)));

        // Float
        assert!(Field::from(1.1) >= Field::from(1.1));
        assert!(Field::from(1.2) >= Field::from(1.15));
    }

    #[test]
    fn test_distance() {
        assert_eq!(Field::Bool(true).distance(&Field::Bool(true)), 0);
        assert_eq!(Field::Bool(false).distance(&Field::Bool(false)), 0);
        assert_eq!(Field::Bool(true).distance(&Field::Bool(false)), 1);
        assert_eq!(Field::Bool(false).distance(&Field::Bool(true)), 1);

        assert_eq!(Field::I64(0).distance(&Field::I64(0)), 0);
        assert_eq!(Field::I64(0).distance(&Field::I64(10)), 10);
        assert_eq!(Field::I64(5).distance(&Field::I64(-5)), 10);
        assert_eq!(Field::I64(10).distance(&Field::I64(10)), 0);
        assert_eq!(Field::I64(10).distance(&Field::I64(0)), 10);
        assert_eq!(Field::I64(-5).distance(&Field::I64(5)), 10);

        assert_eq!(Field::F64(0.0).distance(&Field::F64(0.0)), 0);
        assert_eq!(Field::F64(1.5).distance(&Field::F64(1.5)), 0);
        assert_eq!(Field::F64(0.0).distance(&Field::F64(1.5)), 1);
        assert_eq!(Field::F64(1.5).distance(&Field::F64(0.0)), 1);
        assert_eq!(Field::F64(-2.5).distance(&Field::F64(2.5)), 5);
        assert_eq!(Field::F64(2.5).distance(&Field::F64(-2.5)), 5);
        assert_eq!(Field::F64(2.88).distance(&Field::F64(1.03)), 1);

        assert_eq!(Field::Enum(0).distance(&Field::Enum(0)), 0);
        assert_eq!(Field::Enum(1).distance(&Field::Enum(1)), 0);
        assert_eq!(Field::Enum(0).distance(&Field::Enum(1)), 1);
        assert_eq!(Field::Enum(1).distance(&Field::Enum(0)), 1);
        assert_eq!(Field::Enum(1).distance(&Field::Enum(5)), 1);
    }
}

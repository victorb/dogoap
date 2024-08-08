use bevy_reflect::*;
use std::fmt::Display;
use std::hash::Hash;
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Reflect, Clone, Debug, PartialOrd, Copy)]
pub enum Datum {
    Bool(bool),
    I64(i64),
    F64(f64),
    Enum(usize),
}

impl Hash for Datum {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
        match self {
            Datum::Bool(b) => b.hash(state),
            Datum::I64(i) => i.hash(state),
            Datum::F64(f) => f.to_bits().hash(state),
            Datum::Enum(u) => u.hash(state),
        }
    }
}

impl PartialEq for Datum {
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

impl Eq for Datum {}

impl Datum {
    pub fn distance(&self, other: &Datum) -> u64 {
        let ret = match (self, other) {
            (Datum::Bool(a), Datum::Bool(b)) => {
                if a == b {
                    0
                } else {
                    1
                }
            }
            (Datum::I64(a), Datum::I64(b)) => (a - b).unsigned_abs(),
            (Datum::F64(a), Datum::F64(b)) => (a - b).abs() as u64,
            (Datum::Enum(a), Datum::Enum(b)) => {
                if a == b {
                    0
                } else {
                    1
                }
            }
            _ => panic!("Cannot calculate distance between different Datum types"),
        };
        // ret * 10 // TODO remove, testing to increase distance to prevent oscillation
        ret
    }
}

impl Display for Datum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool(v) => {
                write!(f, "Datum:Bool({})", v)
            }
            Self::I64(v) => {
                write!(f, "Datum:I64({})", v)
            }
            Self::F64(v) => {
                write!(f, "Datum:F64({})", v)
            }
            Self::Enum(v) => {
                write!(f, "Datum:Enum({})", v)
            }
        }
    }
}

impl Add for &Datum {
    type Output = Datum;

    fn add(self, other: &Datum) -> Datum {
        match (self, other) {
            (Datum::I64(a), Datum::I64(b)) => Datum::I64(a + b),
            (Datum::F64(a), Datum::F64(b)) => Datum::F64(a + b),
            _ => panic!(
                "Unsupported addition between Datum variants, {:?} - {:?}",
                self, other
            ),
        }
    }
}

impl Add for Datum {
    type Output = Datum;

    #[allow(clippy::op_ref)]
    fn add(self, other: Datum) -> Datum {
        &self + &other
    }
}

impl Sub for &Datum {
    type Output = Datum;

    fn sub(self, other: &Datum) -> Datum {
        match (self, other) {
            (Datum::I64(a), Datum::I64(b)) => Datum::I64(a - b),
            (Datum::F64(a), Datum::F64(b)) => Datum::F64(a - b),
            _ => panic!(
                "Unsupported negation between Datum variants, {:?} - {:?}",
                self, other
            ),
        }
    }
}

impl Sub for Datum {
    type Output = Datum;

    #[allow(clippy::op_ref)]
    fn sub(self, other: Datum) -> Datum {
        &self - &other
    }
}

impl AddAssign for Datum {
    fn add_assign(&mut self, rhs: Self) {
        match self {
            Self::I64(ref mut v1) => match rhs {
                Self::I64(v2) => {
                    *v1 += v2;
                }
                _ => panic!("Unimplemented! Tried to remove {:?} from {:?}", self, rhs),
            },
            Self::F64(ref mut v1) => match rhs {
                Self::F64(v2) => {
                    *v1 += v2;
                }
                _ => panic!("Unimplemented! Tried to remove {:?} from {:?}", self, rhs),
            },
            _ => panic!("Unimplemented! Tried to remove {:?} from {:?}", self, rhs),
        }
    }
}

impl SubAssign for Datum {
    fn sub_assign(&mut self, rhs: Self) {
        match self {
            Self::I64(ref mut v1) => match rhs {
                Self::I64(v2) => {
                    *v1 -= v2;
                }
                _ => panic!("Unimplemented! Tried to remove {:?} from {:?}", self, rhs),
            },
            Self::F64(ref mut v1) => match rhs {
                Self::F64(v2) => {
                    *v1 -= v2;
                }
                _ => panic!("Unimplemented! Tried to remove {:?} from {:?}", self, rhs),
            },
            _ => panic!("Unimplemented! Tried to remove {:?} from {:?}", self, rhs),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;
    #[test]
    fn test_equality() {
        assert_eq!(Datum::Bool(true), Datum::Bool(true));
        assert_eq!(Datum::I64(666), Datum::I64(666));
        assert_eq!(Datum::F64(666.666), Datum::F64(666.666));
    }

    #[test]
    fn test_greater_than() {
        // Int
        assert!(Datum::I64(100) > Datum::I64(10));
        assert!(Datum::I64(1) > Datum::I64(0));

        // Float
        assert!(Datum::F64(1.2) > Datum::F64(1.1));
    }

    #[test]
    fn test_greater_than_equals() {
        // Int
        assert!(Datum::I64(100) >= Datum::I64(10));
        assert!(Datum::I64(1) >= Datum::I64(0));
        assert!(Datum::I64(100) >= Datum::I64(100));
        assert!(!(Datum::I64(100) >= Datum::I64(101)));

        // Float
        assert!(Datum::F64(1.1) >= Datum::F64(1.1));
        assert!(Datum::F64(1.2) >= Datum::F64(1.15));
    }

    #[test]
    fn test_distance() {
        assert_eq!(Datum::Bool(true).distance(&Datum::Bool(true)), 0);
        assert_eq!(Datum::Bool(false).distance(&Datum::Bool(false)), 0);
        assert_eq!(Datum::Bool(true).distance(&Datum::Bool(false)), 1);
        assert_eq!(Datum::Bool(false).distance(&Datum::Bool(true)), 1);

        assert_eq!(Datum::I64(0).distance(&Datum::I64(0)), 0);
        assert_eq!(Datum::I64(0).distance(&Datum::I64(10)), 10);
        assert_eq!(Datum::I64(5).distance(&Datum::I64(-5)), 10);
        assert_eq!(Datum::I64(10).distance(&Datum::I64(10)), 0);
        assert_eq!(Datum::I64(10).distance(&Datum::I64(0)), 10);
        assert_eq!(Datum::I64(-5).distance(&Datum::I64(5)), 10);

        assert_eq!(Datum::F64(0.0).distance(&Datum::F64(0.0)), 0);
        assert_eq!(Datum::F64(1.5).distance(&Datum::F64(1.5)), 0);
        assert_eq!(Datum::F64(0.0).distance(&Datum::F64(1.5)), 1);
        assert_eq!(Datum::F64(1.5).distance(&Datum::F64(0.0)), 1);
        assert_eq!(Datum::F64(-2.5).distance(&Datum::F64(2.5)), 5);
        assert_eq!(Datum::F64(2.5).distance(&Datum::F64(-2.5)), 5);
        assert_eq!(Datum::F64(2.88).distance(&Datum::F64(1.03)), 1);

        assert_eq!(Datum::Enum(0).distance(&Datum::Enum(0)), 0);
        assert_eq!(Datum::Enum(1).distance(&Datum::Enum(1)), 0);
        assert_eq!(Datum::Enum(0).distance(&Datum::Enum(1)), 1);
        assert_eq!(Datum::Enum(1).distance(&Datum::Enum(0)), 1);
        assert_eq!(Datum::Enum(1).distance(&Datum::Enum(5)), 1);
    }
}

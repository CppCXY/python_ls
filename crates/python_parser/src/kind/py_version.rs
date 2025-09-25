use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyVersionNumber {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl PyVersionNumber {
    #[allow(unused)]
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<Self> {
        let mut iter = s.split('.').map(|it| it.parse::<u32>().unwrap_or(0));
        let major = iter.next().unwrap_or(0);
        let minor = iter.next().unwrap_or(0);
        let patch = iter.next().unwrap_or(0);
        Some(Self {
            major,
            minor,
            patch,
        })
    }
}

impl PartialOrd for PyVersionNumber {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PyVersionNumber {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.major
            .cmp(&other.major)
            .then_with(|| self.minor.cmp(&other.minor))
            .then_with(|| self.patch.cmp(&other.patch))
    }
}

impl fmt::Display for PyVersionNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            PyVersionNumber { major, minor, .. } => write!(f, "Python {}.{}", major, minor),
        }
    }
}

#[allow(unused)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LuaVersionCondition {
    Eq(PyVersionNumber),
    Gte(PyVersionNumber),
    Lte(PyVersionNumber),
}

#[allow(unused)]
impl LuaVersionCondition {
    pub fn check(&self, version: &PyVersionNumber) -> bool {
        match self {
            LuaVersionCondition::Eq(v) => version == v,
            LuaVersionCondition::Gte(v) => version >= v,
            LuaVersionCondition::Lte(v) => version <= v,
        }
    }
}

impl fmt::Display for LuaVersionCondition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LuaVersionCondition::Eq(v) => write!(f, "{}", v),
            LuaVersionCondition::Gte(v) => write!(f, ">= {}", v),
            LuaVersionCondition::Lte(v) => write!(f, "<= {}", v),
        }
    }
}

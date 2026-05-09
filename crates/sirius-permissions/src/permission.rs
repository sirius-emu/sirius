#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Permission {
    Ambassador,
}

impl Permission {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ambassador => "social.ambassador",
        }
    }

    pub const ALL: &'static [Self] = &[Self::Ambassador];
}

impl std::fmt::Display for Permission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Type-safe permission keys.
///
/// Each variant maps to a namespaced string key via [`as_str`]. The format is
/// `namespace.permission_name`. These keys are stored in the `permissions_rank_permissions` table
/// and looked up at runtime by [`PermissionsManager`].
///
/// # Adding a new permission
/// 1. Add a variant to this enum.
/// 2. Add a corresponding `as_str` arm.
/// 3. Add the variant to [`ALL`].
/// 4. Insert the key into the database for the appropriate ranks.
///
/// [`as_str`]: Permission::as_str
/// [`ALL`]: Permission::ALL
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Permission {
    /// Marks the user as a hotel ambassador.
    Ambassador,
}

impl Permission {
    /// Returns the namespaced string key stored in the database.
    ///
    /// Format: `namespace.permission_name`
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ambassador => "social.ambassador",
        }
    }

    /// All permission variants.
    ///
    /// Useful for seeding the database or iterating over every
    /// known permission in tests.
    pub const ALL: &'static [Self] = &[Self::Ambassador];
}

impl std::fmt::Display for Permission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PermissionSetting {
    Disallowed,
    Allowed,
    RoomOwner,
}

impl PermissionSetting {
    #[inline]
    pub fn is_granted(self, is_room_owner: bool) -> bool {
        match self {
            Self::Allowed => true,
            Self::RoomOwner => is_room_owner,
            Self::Disallowed => false,
        }
    }
}

impl From<i16> for PermissionSetting {
    fn from(val: i16) -> Self {
        match val {
            1 => Self::Allowed,
            2 => Self::RoomOwner,
            _ => Self::Disallowed,
        }
    }
}

impl std::fmt::Display for PermissionSetting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Disallowed => write!(f, "disallowed"),
            Self::Allowed => write!(f, "allowed"),
            Self::RoomOwner => write!(f, "room_owner"),
        }
    }
}

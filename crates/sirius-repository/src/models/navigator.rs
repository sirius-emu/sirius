use crate::models::Room;

#[derive(Debug, Clone)]
pub struct SearchResultBlock {
    pub search_code: String,
    pub text: String,
    pub action: i32,
    pub is_closed: bool,
    pub view_mode: i32,
    pub rooms: Vec<Room>,
}

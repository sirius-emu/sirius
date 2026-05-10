use sirius_repository::Repository;
use sirius_repository::models::Room;

pub struct NavigatorService {
    _repo: Repository,
}

#[derive(Debug, Clone)]
pub struct NavigatorBlock {
    pub search_code: String,
    pub text: String,
    pub action: i32,
    pub is_closed: bool,
    pub view_mode: i32,
    pub rooms: Vec<Room>,
}

impl NavigatorService {
    pub fn new(repo: Repository) -> Self {
        Self { _repo: repo }
    }

    pub fn get_search_results(
        &self,
        view: &str,
        query: &str,
    ) -> Vec<NavigatorBlock> {
        let mut blocks = Vec::new();

        if !query.is_empty() {
            blocks.push(NavigatorBlock {
                search_code: "query".to_string(),
                text: query.to_string(),
                action: 0,
                is_closed: false,
                view_mode: 0,
                rooms: vec![],
            });
            return blocks;
        }

        match view {
            "official_view" => {
                blocks.push(NavigatorBlock {
                    search_code: "official_view".to_string(),
                    text: "".to_string(),
                    action: 0,
                    is_closed: false,
                    view_mode: 0,
                    rooms: vec![],
                });
            }
            "hotel_view" => {
                blocks.push(NavigatorBlock {
                    search_code: "popular".to_string(),
                    text: "".to_string(),
                    action: 0,
                    is_closed: false,
                    view_mode: 0,
                    rooms: vec![],
                });
            }
            "roomads_view" => {
                blocks.push(NavigatorBlock {
                    search_code: "highest_score".to_string(),
                    text: "".to_string(),
                    action: 0,
                    is_closed: false,
                    view_mode: 0,
                    rooms: vec![],
                });
            }
            "myworld_view" => {
                blocks.push(NavigatorBlock {
                    search_code: "my".to_string(),
                    text: "".to_string(),
                    action: 0,
                    is_closed: false,
                    view_mode: 0,
                    rooms: vec![],
                });
                blocks.push(NavigatorBlock {
                    search_code: "favorites".to_string(),
                    text: "".to_string(),
                    action: 0,
                    is_closed: true,
                    view_mode: 0,
                    rooms: vec![],
                });
            }
            _ => {}
        }

        blocks
    }
}

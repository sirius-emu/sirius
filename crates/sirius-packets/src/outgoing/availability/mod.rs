mod closed_and_opens;
mod closes_and_opens_at;
mod maintenance;
mod status;
mod time;
mod will_close_in_minutes;

pub use closed_and_opens::HotelClosedAndOpensComposer;
pub use closes_and_opens_at::HotelClosesAndOpensAtComposer;
pub use maintenance::MaintenanceStatusComposer;
pub use status::AvailabilityStatusComposer;
pub use time::AvailabilityTimeComposer;
pub use will_close_in_minutes::HotelWillCloseInMinutes;

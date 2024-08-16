mod error;
mod health;
pub mod item;

use arch_utils::arcbox::ArcBox;

pub use error::Error;
pub use health::HealthApi;
pub use item::ItemApi;

pub struct ArchApi {
    pub health_api: ArcBox<dyn HealthApi>,
    pub item_api: ArcBox<dyn ItemApi>,
}

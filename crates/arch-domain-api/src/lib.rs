mod health;

pub mod error;
pub use error::Error;

use arch_utils::arcbox::ArcBox;
pub use health::HealthApi;

pub struct ArchApi {
    pub health_api: ArcBox<dyn HealthApi>,
}

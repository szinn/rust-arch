mod error;
mod health;

use arch_utils::arcbox::ArcBox;

pub use error::Error;
pub use health::HealthApi;

pub struct ArchApi {
    pub health_api: ArcBox<dyn HealthApi>,
}

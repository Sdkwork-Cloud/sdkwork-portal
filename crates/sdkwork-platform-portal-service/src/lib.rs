pub mod domain;
pub mod ports;
pub mod service;

pub use domain::{PortalPreference, PortalPreferenceSummary, UpdatePortalPreferenceCommand};
pub use ports::PortalRepository;
pub use service::PortalService;

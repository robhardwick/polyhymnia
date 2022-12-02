#[cfg(target_os = "none")]
pub use defmt::{debug, error, info, trace, warn};

#[cfg(not(target_os = "none"))]
pub use log::{debug, error, info, trace, warn};

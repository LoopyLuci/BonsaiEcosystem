pub mod discovery;
pub mod menu;
pub mod error;

pub use discovery::{BonsaiApp, AppCategory, discover_apps};
pub use menu::AppMenu;
pub use error::{MenuError, Result};

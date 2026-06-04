pub mod frontend;

use bonsai_language_frontend::LanguageFrontend;

pub fn register_javascript() {
    tracing::info!("JavaScript language support initialized");
}

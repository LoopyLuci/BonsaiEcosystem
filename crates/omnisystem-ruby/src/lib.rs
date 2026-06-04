pub mod frontend;

use bonsai_language_frontend::LanguageFrontend;

pub fn register_ruby() {
    tracing::info!("Ruby language support initialized");
}

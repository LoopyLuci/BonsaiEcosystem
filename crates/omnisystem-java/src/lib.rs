pub mod frontend;

use bonsai_language_frontend::LanguageFrontend;

pub fn register_java() {
    tracing::info!("Java language support initialized");
}

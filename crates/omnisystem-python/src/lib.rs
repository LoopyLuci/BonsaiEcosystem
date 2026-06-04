pub mod frontend;

use bonsai_language_frontend::LanguageFrontend;

pub fn register_python() {
    tracing::info!("Python language support initialized");
}

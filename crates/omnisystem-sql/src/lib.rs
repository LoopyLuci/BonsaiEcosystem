pub mod frontend;

use bonsai_language_frontend::LanguageFrontend;

pub fn register_sql() {
    tracing::info!("SQL language support initialized");
}

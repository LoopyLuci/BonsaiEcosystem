use bonsai_language_frontend::LanguageFrontend;
use frontend::RustFrontend;

mod frontend;

inventory::submit! {
    bonsai_language_frontend::LanguageRegistration {
        name: "Rust",
        factory: || Box::new(RustFrontend::new()),
    }
}

use language_system::LanguageFrontend;
use frontend::RustFrontend;

mod frontend;

inventory::submit! {
    language_system::LanguageRegistration {
        name: "Rust",
        factory: || Box::new(RustFrontend::new()),
    }
}

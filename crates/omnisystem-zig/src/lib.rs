use bonsai_language_frontend::LanguageFrontend;
use frontend::ZigFrontend;

mod frontend;

inventory::submit! {
    bonsai_language_frontend::LanguageRegistration {
        name: "Zig",
        factory: || Box::new(ZigFrontend::new()),
    }
}

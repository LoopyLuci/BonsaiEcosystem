use language_system::LanguageFrontend;
use frontend::ZigFrontend;

mod frontend;

inventory::submit! {
    language_system::LanguageRegistration {
        name: "Zig",
        factory: || Box::new(ZigFrontend::new()),
    }
}

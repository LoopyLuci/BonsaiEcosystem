use language_system::LanguageFrontend;
use frontend::TypeScriptFrontend;

mod frontend;

inventory::submit! {
    language_system::LanguageRegistration {
        name: "TypeScript",
        factory: || Box::new(TypeScriptFrontend::new()),
    }
}

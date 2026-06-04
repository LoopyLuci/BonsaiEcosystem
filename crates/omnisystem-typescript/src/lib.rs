use bonsai_language_frontend::LanguageFrontend;
use frontend::TypeScriptFrontend;

mod frontend;

inventory::submit! {
    bonsai_language_frontend::LanguageRegistration {
        name: "TypeScript",
        factory: || Box::new(TypeScriptFrontend::new()),
    }
}

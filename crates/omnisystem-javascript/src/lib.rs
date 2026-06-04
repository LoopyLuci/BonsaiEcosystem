use bonsai_language_frontend::LanguageFrontend;
use frontend::JavaScriptFrontend;

mod frontend;

inventory::submit! {
    bonsai_language_frontend::LanguageRegistration {
        name: "JavaScript",
        factory: || Box::new(JavaScriptFrontend::new()),
    }
}

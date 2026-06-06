use language_system::LanguageFrontend;
use frontend::JavaScriptFrontend;

mod frontend;

inventory::submit! {
    language_system::LanguageRegistration {
        name: "JavaScript",
        factory: || Box::new(JavaScriptFrontend::new()),
    }
}

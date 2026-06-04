use bonsai_language_frontend::LanguageFrontend;
use frontend::GoFrontend;

mod frontend;

inventory::submit! {
    bonsai_language_frontend::LanguageRegistration {
        name: "Go",
        factory: || Box::new(GoFrontend::new()),
    }
}

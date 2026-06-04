use bonsai_language_frontend::LanguageFrontend;
use frontend::HTMLFrontend;

mod frontend;

inventory::submit! {
    bonsai_language_frontend::LanguageRegistration {
        name: "HTML",
        factory: || Box::new(HTMLFrontend::new()),
    }
}

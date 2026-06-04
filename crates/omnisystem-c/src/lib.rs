use bonsai_language_frontend::LanguageFrontend;
use frontend::CFrontend;

mod frontend;

inventory::submit! {
    bonsai_language_frontend::LanguageRegistration {
        name: "C",
        factory: || Box::new(CFrontend::new()),
    }
}

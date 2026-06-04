use bonsai_language_frontend::LanguageFrontend;
use frontend::DFrontend;

mod frontend;

inventory::submit! {
    bonsai_language_frontend::LanguageRegistration {
        name: "D",
        factory: || Box::new(DFrontend::new()),
    }
}

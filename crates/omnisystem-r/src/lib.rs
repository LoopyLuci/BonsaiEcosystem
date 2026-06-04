use bonsai_language_frontend::LanguageFrontend;
use frontend::RFrontend;

mod frontend;

inventory::submit! {
    bonsai_language_frontend::LanguageRegistration {
        name: "R",
        factory: || Box::new(RFrontend::new()),
    }
}

use bonsai_language_frontend::LanguageFrontend;
use frontend::SchemeFrontend;

mod frontend;

inventory::submit! {
    bonsai_language_frontend::LanguageRegistration {
        name: "Scheme",
        factory: || Box::new(SchemeFrontend::new()),
    }
}

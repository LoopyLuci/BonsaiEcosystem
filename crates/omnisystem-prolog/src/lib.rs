use bonsai_language_frontend::LanguageFrontend;
use frontend::PrologFrontend;

mod frontend;

inventory::submit! {
    bonsai_language_frontend::LanguageRegistration {
        name: "Prolog",
        factory: || Box::new(PrologFrontend::new()),
    }
}

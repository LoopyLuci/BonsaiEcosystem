use bonsai_language_frontend::LanguageFrontend;
use frontend::LispFrontend;

mod frontend;

inventory::submit! {
    bonsai_language_frontend::LanguageRegistration {
        name: "Lisp",
        factory: || Box::new(LispFrontend::new()),
    }
}

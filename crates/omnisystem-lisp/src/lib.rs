use language_system::LanguageFrontend;
use frontend::LispFrontend;

mod frontend;

inventory::submit! {
    language_system::LanguageRegistration {
        name: "Lisp",
        factory: || Box::new(LispFrontend::new()),
    }
}

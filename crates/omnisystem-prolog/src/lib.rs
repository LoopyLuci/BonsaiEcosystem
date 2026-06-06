use language_system::LanguageFrontend;
use frontend::PrologFrontend;

mod frontend;

inventory::submit! {
    language_system::LanguageRegistration {
        name: "Prolog",
        factory: || Box::new(PrologFrontend::new()),
    }
}

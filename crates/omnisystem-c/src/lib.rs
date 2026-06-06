use language_system::LanguageFrontend;
use frontend::CFrontend;

mod frontend;

inventory::submit! {
    language_system::LanguageRegistration {
        name: "C",
        factory: || Box::new(CFrontend::new()),
    }
}

use language_system::LanguageFrontend;
use frontend::SchemeFrontend;

mod frontend;

inventory::submit! {
    language_system::LanguageRegistration {
        name: "Scheme",
        factory: || Box::new(SchemeFrontend::new()),
    }
}

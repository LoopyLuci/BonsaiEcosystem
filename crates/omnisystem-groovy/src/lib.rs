use language_system::LanguageFrontend;
use frontend::GroovyFrontend;

mod frontend;

inventory::submit! {
    language_system::LanguageRegistration {
        name: "Groovy",
        factory: || Box::new(GroovyFrontend::new()),
    }
}

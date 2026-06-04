use bonsai_language_frontend::LanguageFrontend;
use frontend::GroovyFrontend;

mod frontend;

inventory::submit! {
    bonsai_language_frontend::LanguageRegistration {
        name: "Groovy",
        factory: || Box::new(GroovyFrontend::new()),
    }
}

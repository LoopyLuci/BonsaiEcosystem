use bonsai_language_frontend::LanguageFrontend;
use frontend::XMLFrontend;

mod frontend;

inventory::submit! {
    bonsai_language_frontend::LanguageRegistration {
        name: "XML",
        factory: || Box::new(XMLFrontend::new()),
    }
}

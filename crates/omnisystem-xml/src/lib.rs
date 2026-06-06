use language_system::LanguageFrontend;
use frontend::XMLFrontend;

mod frontend;

inventory::submit! {
    language_system::LanguageRegistration {
        name: "XML",
        factory: || Box::new(XMLFrontend::new()),
    }
}

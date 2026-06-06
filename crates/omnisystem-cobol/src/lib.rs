use language_system::LanguageFrontend;
use frontend::COBOLFrontend;

mod frontend;

inventory::submit! {
    language_system::LanguageRegistration {
        name: "COBOL",
        factory: || Box::new(COBOLFrontend::new()),
    }
}

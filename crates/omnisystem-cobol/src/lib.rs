use bonsai_language_frontend::LanguageFrontend;
use frontend::COBOLFrontend;

mod frontend;

inventory::submit! {
    bonsai_language_frontend::LanguageRegistration {
        name: "COBOL",
        factory: || Box::new(COBOLFrontend::new()),
    }
}

use bonsai_language_frontend::LanguageFrontend;
use frontend::PHPFrontend;

mod frontend;

inventory::submit! {
    bonsai_language_frontend::LanguageRegistration {
        name: "PHP",
        factory: || Box::new(PHPFrontend::new()),
    }
}

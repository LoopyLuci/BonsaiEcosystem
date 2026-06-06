use language_system::LanguageFrontend;
use frontend::PHPFrontend;

mod frontend;

inventory::submit! {
    language_system::LanguageRegistration {
        name: "PHP",
        factory: || Box::new(PHPFrontend::new()),
    }
}

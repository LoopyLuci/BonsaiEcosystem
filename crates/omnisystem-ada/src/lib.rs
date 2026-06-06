use language_system::LanguageFrontend;
use frontend::AdaFrontend;

mod frontend;

inventory::submit! {
    language_system::LanguageRegistration {
        name: "Ada",
        factory: || Box::new(AdaFrontend::new()),
    }
}

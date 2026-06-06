use language_system::LanguageFrontend;
use frontend::DFrontend;

mod frontend;

inventory::submit! {
    language_system::LanguageRegistration {
        name: "D",
        factory: || Box::new(DFrontend::new()),
    }
}

use language_system::LanguageFrontend;
use frontend::NimFrontend;

mod frontend;

inventory::submit! {
    language_system::LanguageRegistration {
        name: "Nim",
        factory: || Box::new(NimFrontend::new()),
    }
}

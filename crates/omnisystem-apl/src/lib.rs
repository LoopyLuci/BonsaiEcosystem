use language_system::LanguageFrontend;
use frontend::APLFrontend;

mod frontend;

inventory::submit! {
    language_system::LanguageRegistration {
        name: "APL",
        factory: || Box::new(APLFrontend::new()),
    }
}

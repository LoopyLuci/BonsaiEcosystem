use language_system::LanguageFrontend;
use frontend::Objective-CFrontend;

mod frontend;

inventory::submit! {
    language_system::LanguageRegistration {
        name: "Objective-C",
        factory: || Box::new(Objective-CFrontend::new()),
    }
}

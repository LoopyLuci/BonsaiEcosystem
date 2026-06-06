use language_system::LanguageFrontend;
use frontend::GoFrontend;

mod frontend;

inventory::submit! {
    language_system::LanguageRegistration {
        name: "Go",
        factory: || Box::new(GoFrontend::new()),
    }
}

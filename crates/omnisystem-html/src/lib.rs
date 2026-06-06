use language_system::LanguageFrontend;
use frontend::HTMLFrontend;

mod frontend;

inventory::submit! {
    language_system::LanguageRegistration {
        name: "HTML",
        factory: || Box::new(HTMLFrontend::new()),
    }
}

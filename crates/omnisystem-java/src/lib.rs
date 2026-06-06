use language_system::LanguageFrontend;
use frontend::JavaFrontend;

mod frontend;

inventory::submit! {
    language_system::LanguageRegistration {
        name: "Java",
        factory: || Box::new(JavaFrontend::new()),
    }
}

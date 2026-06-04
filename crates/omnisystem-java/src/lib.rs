use bonsai_language_frontend::LanguageFrontend;
use frontend::JavaFrontend;

mod frontend;

inventory::submit! {
    bonsai_language_frontend::LanguageRegistration {
        name: "Java",
        factory: || Box::new(JavaFrontend::new()),
    }
}

use bonsai_language_frontend::LanguageFrontend;
use frontend::AdaFrontend;

mod frontend;

inventory::submit! {
    bonsai_language_frontend::LanguageRegistration {
        name: "Ada",
        factory: || Box::new(AdaFrontend::new()),
    }
}

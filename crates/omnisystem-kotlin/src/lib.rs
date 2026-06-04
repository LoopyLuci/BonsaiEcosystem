use bonsai_language_frontend::LanguageFrontend;
use frontend::KotlinFrontend;

mod frontend;

inventory::submit! {
    bonsai_language_frontend::LanguageRegistration {
        name: "Kotlin",
        factory: || Box::new(KotlinFrontend::new()),
    }
}

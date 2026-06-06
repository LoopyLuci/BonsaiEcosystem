use language_system::LanguageFrontend;
use frontend::KotlinFrontend;

mod frontend;

inventory::submit! {
    language_system::LanguageRegistration {
        name: "Kotlin",
        factory: || Box::new(KotlinFrontend::new()),
    }
}

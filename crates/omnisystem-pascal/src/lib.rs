use language_system::LanguageFrontend;
use frontend::PascalFrontend;

mod frontend;

inventory::submit! {
    language_system::LanguageRegistration {
        name: "Pascal",
        factory: || Box::new(PascalFrontend::new()),
    }
}

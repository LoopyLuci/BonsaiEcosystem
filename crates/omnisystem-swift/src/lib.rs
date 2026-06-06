use language_system::LanguageFrontend;
use frontend::SwiftFrontend;

mod frontend;

inventory::submit! {
    language_system::LanguageRegistration {
        name: "Swift",
        factory: || Box::new(SwiftFrontend::new()),
    }
}

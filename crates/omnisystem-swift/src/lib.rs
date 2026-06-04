use bonsai_language_frontend::LanguageFrontend;
use frontend::SwiftFrontend;

mod frontend;

inventory::submit! {
    bonsai_language_frontend::LanguageRegistration {
        name: "Swift",
        factory: || Box::new(SwiftFrontend::new()),
    }
}

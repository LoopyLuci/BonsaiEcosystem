use bonsai_language_frontend::LanguageFrontend;
use frontend::PascalFrontend;

mod frontend;

inventory::submit! {
    bonsai_language_frontend::LanguageRegistration {
        name: "Pascal",
        factory: || Box::new(PascalFrontend::new()),
    }
}

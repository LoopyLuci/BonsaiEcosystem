use bonsai_language_frontend::LanguageFrontend;
use frontend::NimFrontend;

mod frontend;

inventory::submit! {
    bonsai_language_frontend::LanguageRegistration {
        name: "Nim",
        factory: || Box::new(NimFrontend::new()),
    }
}

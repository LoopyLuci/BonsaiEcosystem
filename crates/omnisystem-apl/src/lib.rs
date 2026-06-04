use bonsai_language_frontend::LanguageFrontend;
use frontend::APLFrontend;

mod frontend;

inventory::submit! {
    bonsai_language_frontend::LanguageRegistration {
        name: "APL",
        factory: || Box::new(APLFrontend::new()),
    }
}

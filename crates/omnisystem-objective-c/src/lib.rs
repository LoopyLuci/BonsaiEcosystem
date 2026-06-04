use bonsai_language_frontend::LanguageFrontend;
use frontend::Objective-CFrontend;

mod frontend;

inventory::submit! {
    bonsai_language_frontend::LanguageRegistration {
        name: "Objective-C",
        factory: || Box::new(Objective-CFrontend::new()),
    }
}

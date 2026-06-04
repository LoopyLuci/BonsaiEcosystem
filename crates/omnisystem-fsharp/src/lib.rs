use bonsai_language_frontend::LanguageFrontend;
use frontend::F#Frontend;

mod frontend;

inventory::submit! {
    bonsai_language_frontend::LanguageRegistration {
        name: "F#",
        factory: || Box::new(F#Frontend::new()),
    }
}

use bonsai_language_frontend::LanguageFrontend;
use frontend::HaskellFrontend;

mod frontend;

inventory::submit! {
    bonsai_language_frontend::LanguageRegistration {
        name: "Haskell",
        factory: || Box::new(HaskellFrontend::new()),
    }
}

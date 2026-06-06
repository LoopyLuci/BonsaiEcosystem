use language_system::LanguageFrontend;
use frontend::HaskellFrontend;

mod frontend;

inventory::submit! {
    language_system::LanguageRegistration {
        name: "Haskell",
        factory: || Box::new(HaskellFrontend::new()),
    }
}

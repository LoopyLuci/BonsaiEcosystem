use language_system::LanguageFrontend;
use frontend::F#Frontend;

mod frontend;

inventory::submit! {
    language_system::LanguageRegistration {
        name: "F#",
        factory: || Box::new(F#Frontend::new()),
    }
}

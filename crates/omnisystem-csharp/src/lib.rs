use language_system::LanguageFrontend;
use frontend::C#Frontend;

mod frontend;

inventory::submit! {
    language_system::LanguageRegistration {
        name: "C#",
        factory: || Box::new(C#Frontend::new()),
    }
}

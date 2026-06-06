use language_system::LanguageFrontend;
use frontend::RFrontend;

mod frontend;

inventory::submit! {
    language_system::LanguageRegistration {
        name: "R",
        factory: || Box::new(RFrontend::new()),
    }
}

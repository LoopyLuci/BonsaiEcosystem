use language_system::LanguageFrontend;
use frontend::VB.NETFrontend;

mod frontend;

inventory::submit! {
    language_system::LanguageRegistration {
        name: "VB.NET",
        factory: || Box::new(VB.NETFrontend::new()),
    }
}

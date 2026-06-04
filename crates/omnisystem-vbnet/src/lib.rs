use bonsai_language_frontend::LanguageFrontend;
use frontend::VB.NETFrontend;

mod frontend;

inventory::submit! {
    bonsai_language_frontend::LanguageRegistration {
        name: "VB.NET",
        factory: || Box::new(VB.NETFrontend::new()),
    }
}

use bonsai_language_frontend::LanguageFrontend;
use frontend::C++Frontend;

mod frontend;

inventory::submit! {
    bonsai_language_frontend::LanguageRegistration {
        name: "C++",
        factory: || Box::new(C++Frontend::new()),
    }
}

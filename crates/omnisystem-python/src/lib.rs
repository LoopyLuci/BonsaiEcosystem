use bonsai_language_frontend::LanguageFrontend;
use frontend::PythonFrontend;

mod frontend;

inventory::submit! {
    bonsai_language_frontend::LanguageRegistration {
        name: "Python",
        factory: || Box::new(PythonFrontend::new()),
    }
}

use language_system::LanguageFrontend;
use frontend::PythonFrontend;

mod frontend;

inventory::submit! {
    language_system::LanguageRegistration {
        name: "Python",
        factory: || Box::new(PythonFrontend::new()),
    }
}

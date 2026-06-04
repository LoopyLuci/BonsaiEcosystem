use bonsai_language_frontend::LanguageFrontend;
use frontend::ScalaFrontend;

mod frontend;

inventory::submit! {
    bonsai_language_frontend::LanguageRegistration {
        name: "Scala",
        factory: || Box::new(ScalaFrontend::new()),
    }
}

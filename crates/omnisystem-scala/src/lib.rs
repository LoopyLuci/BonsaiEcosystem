use language_system::LanguageFrontend;
use frontend::ScalaFrontend;

mod frontend;

inventory::submit! {
    language_system::LanguageRegistration {
        name: "Scala",
        factory: || Box::new(ScalaFrontend::new()),
    }
}

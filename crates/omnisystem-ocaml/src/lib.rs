use language_system::LanguageFrontend;
use frontend::OCamlFrontend;

mod frontend;

inventory::submit! {
    language_system::LanguageRegistration {
        name: "OCaml",
        factory: || Box::new(OCamlFrontend::new()),
    }
}

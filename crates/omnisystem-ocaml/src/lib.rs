use bonsai_language_frontend::LanguageFrontend;
use frontend::OCamlFrontend;

mod frontend;

inventory::submit! {
    bonsai_language_frontend::LanguageRegistration {
        name: "OCaml",
        factory: || Box::new(OCamlFrontend::new()),
    }
}

use bonsai_language_frontend::LanguageFrontend;
use frontend::PerlFrontend;

mod frontend;

inventory::submit! {
    bonsai_language_frontend::LanguageRegistration {
        name: "Perl",
        factory: || Box::new(PerlFrontend::new()),
    }
}

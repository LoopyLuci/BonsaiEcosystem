use language_system::LanguageFrontend;
use frontend::PerlFrontend;

mod frontend;

inventory::submit! {
    language_system::LanguageRegistration {
        name: "Perl",
        factory: || Box::new(PerlFrontend::new()),
    }
}

use language_system::LanguageFrontend;
use frontend::FORTRAN77Frontend;

mod frontend;

inventory::submit! {
    language_system::LanguageRegistration {
        name: "FORTRAN 77",
        factory: || Box::new(FORTRAN77Frontend::new()),
    }
}

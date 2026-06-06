use language_system::LanguageFrontend;
use frontend::FORTRAN90Frontend;

mod frontend;

inventory::submit! {
    language_system::LanguageRegistration {
        name: "FORTRAN 90",
        factory: || Box::new(FORTRAN90Frontend::new()),
    }
}

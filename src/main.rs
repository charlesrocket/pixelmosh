#[cfg(feature = "cli")]
mod cli;
#[cfg(feature = "gui")]
mod gui;

fn main() {
    if std::env::args().count() == 1 && cfg!(feature = "gui") {
        #[cfg(feature = "gui")]
        gui::start();
    } else {
        #[cfg(feature = "cli")]
        cli::start();
    }
}

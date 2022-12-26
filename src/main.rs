mod cli;

#[cfg(feature = "gui")]
mod gui;

fn main() {
    if std::env::args().count() == 1 {
        #[cfg(feature = "gui")]
        gui::start();
    } else {
        cli::start();
    }
}

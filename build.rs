fn main() {
    #[cfg(feature = "gui")]
    glib_build_tools::compile_resources(
        "src/resources",
        "src/resources/resources.gresource.xml",
        "pixelmosh.gresource",
    );
}

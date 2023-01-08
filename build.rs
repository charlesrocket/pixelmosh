fn main() {
    #[cfg(feature = "gui")]
    glib_build_tools::compile_resources(
        "resources",
        "resources/resources.gresource.xml",
        "pixelmosh.gresource",
    );
}

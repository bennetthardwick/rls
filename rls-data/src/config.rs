/// Used to configure save-analysis.
#[cfg_attr(feature = "serialize-serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serialize-rustc", derive(RustcDecodable, RustcEncodable))]
#[derive(Debug, Clone, Default)]
pub struct Config {
    /// File to output save-analysis data to.
    pub output_file: Option<String>,
    /// Include all documentation for items. (If `false`, only includes the
    /// summary (first paragraph) for each item).
    pub full_docs: bool,
    /// If true only includes data for public items in a crate (useful for
    /// library crates).
    pub pub_only: bool,
    /// If true only includes data for items reachable from the crate root.
    pub reachable_only: bool,
    /// True if and only if the analysed crate is part of the standard Rust distro.
    pub distro_crate: bool,
    /// Include signature information.
    pub signatures: bool,
    /// Include experimental borrow data.
    pub borrow_data: bool,
}

pub struct StdLibrary {
    pub manifest: File,
    pub files: Vec<File>,
}

pub struct File {
    pub path: String,
    pub contents: &'static [u8],
}

impl File {
    #[must_use]
    pub fn new(
        path: &str,
        contents: &'static [u8],
    ) -> Self {
        Self {
            path: path.to_owned(),
            contents,
        }
    }
}

impl Default for StdLibrary {
    fn default() -> Self {
        Self::new()
    }
}

impl StdLibrary {
    #[must_use]
    pub fn new() -> Self {
        let files = vec![File::new(
            "/std/package.wesl",
            include_bytes!("../std/package.wesl"),
        )];
        Self {
            manifest: File::new("/std/wesl.toml", include_bytes!("../std/wesl.toml")),
            files,
        }
    }
}

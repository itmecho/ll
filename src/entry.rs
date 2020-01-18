use std::fmt;

#[derive(Debug)]
pub struct Entry {
    name: String,
    hidden: bool,
    dir: bool,
    perms: String,
}

impl Entry {
    pub fn from_dir_entry(e: std::fs::DirEntry) -> Entry {
        let name = e.file_name();
        let name = name.to_string_lossy().to_string();

        let hidden = name.starts_with(".");

        let metadata = e.metadata().unwrap();
        let dir = metadata.is_dir();

        Entry {
            name: name,
            hidden: hidden,
            dir: dir,
            perms: String::new(),
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn is_dir(&self) -> bool {
        self.dir
    }

    pub fn is_hidden(&self) -> bool {
        self.hidden
    }
}

impl std::fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prefix = if self.dir { "" } else { "" };

        write!(f, "{} {}", prefix, self.name)
    }
}

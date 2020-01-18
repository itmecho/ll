use std::fmt;
use std::os::unix::fs::PermissionsExt;

const ROWN: u32 = 0o400;
const WOWN: u32 = 0o200;
const XOWN: u32 = 0o100;
const RGRP: u32 = ROWN >> 3;
const WGRP: u32 = WOWN >> 3;
const XGRP: u32 = XOWN >> 3;
const ROTH: u32 = RGRP >> 3;
const WOTH: u32 = WGRP >> 3;
const XOTH: u32 = XGRP >> 3;

#[derive(Debug)]
pub struct Entry {
    name: String,
    hidden: bool,
    dir: bool,
    perms: u32,
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
            perms: metadata.permissions().mode(),
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

    pub fn perm_string(&self) -> String {
        let mut s = String::new();

        s.push_str(if self.dir { "d" } else { "-" });
        s.push_str(if (self.perms & ROWN) != 0 { "r" } else { "-" });
        s.push_str(if (self.perms & WOWN) != 0 { "w" } else { "-" });
        s.push_str(if (self.perms & XOWN) != 0 { "x" } else { "-" });
        s.push_str(if (self.perms & RGRP) != 0 { "r" } else { "-" });
        s.push_str(if (self.perms & WGRP) != 0 { "w" } else { "-" });
        s.push_str(if (self.perms & XGRP) != 0 { "x" } else { "-" });
        s.push_str(if (self.perms & ROTH) != 0 { "r" } else { "-" });
        s.push_str(if (self.perms & WOTH) != 0 { "w" } else { "-" });
        s.push_str(if (self.perms & XOTH) != 0 { "x" } else { "-" });

        s
    }
}

impl std::fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prefix = if self.dir { "" } else { "" };

        write!(f, "{} ({}) {}", prefix, self.perm_string(), self.name)
    }
}

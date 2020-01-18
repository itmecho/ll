use std::fmt;
use std::fs;

#[derive(Debug)]
struct Entry {
    name: String,
    hidden: bool,
    dir: bool,
    perms: String,
}

impl std::fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prefix = if self.dir { "" } else { "" };

        write!(f, "{} {}", prefix, self.name)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let items = fs::read_dir(".")?;

    let entries: Vec<Entry> = items
        .map(|i| -> Entry {
            let i = i.unwrap();

            let name = i.file_name();
            let name = name.to_string_lossy().to_string();

            let hidden = name.starts_with(".");

            let metadata = i.metadata().unwrap();
            let dir = metadata.is_dir();

            Entry {
                name: name,
                hidden: hidden,
                dir: dir,
                perms: String::new(),
            }
        })
        .collect();

    let mut dirs: Vec<&Entry> = entries.iter().filter(|e| -> bool { e.dir }).collect();
    let mut files: Vec<&Entry> = entries.iter().filter(|e| -> bool { !e.dir }).collect();

    dirs.sort_by(|a, b| a.name.cmp(&b.name));
    files.sort_by(|a, b| a.name.cmp(&b.name));

    for d in dirs {
        println!("{}", d);
    }

    for f in files {
        println!("{}", f);
    }

    Ok(())
}

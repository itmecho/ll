use std::fs;

mod entry;
use entry::Entry;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let items = fs::read_dir(".")?;

    let entries: Vec<Entry> = items
        .map(|e| e.unwrap())
        .map(|e| Entry::from_dir_entry(e))
        .collect();

    let mut dirs: Vec<&Entry> = entries.iter().filter(|e| -> bool { e.is_dir() }).collect();
    let mut files: Vec<&Entry> = entries.iter().filter(|e| -> bool { !e.is_dir() }).collect();

    dirs.sort_by(|a, b| a.name().cmp(&b.name()));
    files.sort_by(|a, b| a.name().cmp(&b.name()));

    for d in dirs {
        println!("{}", d);
    }

    for f in files {
        println!("{}", f);
    }

    Ok(())
}

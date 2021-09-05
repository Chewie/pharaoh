use walkdir::{WalkDir, DirEntry};

fn handle_entry(entry : DirEntry) {
    println!("Running tests for {}", entry.path().display());
}

fn main() {
    let entries : Vec<DirEntry> = WalkDir::new(".")
        .min_depth(1)
        .into_iter()
        .filter_entry(|e| e.path().extension().unwrap() == "yaml")
        .filter_map(|e| e.ok())
        .collect();

    if entries.is_empty() {
        println!("No test case found. Exiting.");
        return;
    }
    for entry in entries {
        handle_entry(entry);
    }
}

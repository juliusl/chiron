use walkdir::WalkDir;

/// Find all yaml files in a given directory
pub fn yaml_files(dir: String) -> Vec<String> {
    // Walk the root directory and find where all the CURRENT files are
    let mut paths = vec![];
    for entry in WalkDir::new(&dir)
        .follow_links(false)
        .contents_first(true)
        .into_iter()
    {
        match entry {
            Ok(path) => {
                if path
                    .file_name()
                    .to_str()
                    .map(|s| s.ends_with("yml") || s.ends_with("yaml"))
                    .unwrap_or(false)
                {
                    let str_path = path.clone();
                    if let Some(s) = str_path.path().to_str(){
                        paths.push(s.to_owned());
                    }
                }
            }
            Err(err) => eprintln!("{}", err),
        }
    }

    return paths;
}


// TODO: Docs
#[crabtime::function]
#[macro_export]
fn add_smithy_files(pattern!($projection:literal, $plugin:literal): _) {
    #![dependency(walkdir = "2.5.0")]
    use walkdir::WalkDir;

    let smithy_path = concat!(env!("SMITHY_OUTPUT_DIR"), "/", $projection, "/", $plugin, "/");
    for _path in WalkDir::new(smithy_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| format!("\"{}\"", e.path().to_owned().display()))
    {
        crabtime::output! {
            include!({{_path}});
        }
    }
}

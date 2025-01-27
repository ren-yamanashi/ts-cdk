use include_dir::{include_dir, Dir};

// templatesディレクトリを静的アセットとして組み込む
pub static TEMPLATES: Dir = include_dir!("$CARGO_MANIFEST_DIR/templates");
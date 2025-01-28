use include_dir::{include_dir, Dir};

// NOTE: Incorporate the templates directory as a static asset.
pub static TEMPLATES: Dir = include_dir!("$CARGO_MANIFEST_DIR/templates");
use crate::commands::init::ProjectConfig;
use crate::commands::init::Linter;
use crate::commands::init::TestTool;
use crate::commands::init::Formatter;
use anyhow::Result;

// NOTE: templateとして出力するファイルパスを格納するstruct
pub struct TemplateFilePaths {
    pub file_paths: Vec<String>,
}

pub fn generate_template_file_paths(config: &ProjectConfig) -> Result<()> {
    let package_json_path = format!("templates/package.json");
    
    // package.jsonの内容を読み込む
    let mut content = std::fs::read_to_string(&package_json_path)?;
    
    // プロジェクト名をkebab-caseに変換
    let kebab_case_name = to_kebab_case(&config.name);
    
    // %project-name%をkebab-caseに変換したconfig.nameで置き換える
    content = content.replace("%project-name%", &kebab_case_name);
    
    // テストコマンドの置き換え
    let test_command = match config.test_tool {
        TestTool::Vitest => "vitest --run",
        TestTool::Jest => "jest",
        TestTool::None => "jest",
    };
    content = content.replace("%test_command%", test_command);
    
    // リントコマンドの置き換え
    let lint_command = match config.linter {
        Linter::EsLint => "eslint --config eslint.config.mjs",
        Linter::Biome => "biome lint",
        Linter::None => "eslint --config eslint.config.mjs",
    };
    content = content.replace("%lint_command%", lint_command);
    
    // フォーマットコマンドの置き換え
    let format_command = match config.formatter {
        Formatter::Prettier => "prettier --write \"**/*.ts\" --ignore-path .",
        Formatter::Biome => "biome format",
        Formatter::None => "prettier --write \"**/*.ts\" --ignore-path .",
    };
    content = content.replace("%format_command%", format_command);
    
    println!("Modified package.json content:\n{}", content);
    
    Ok(())
}

fn to_kebab_case(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && c.is_uppercase() {
            result.push('-');
        }
        result.push(c.to_lowercase().next().unwrap());
    }
    result.replace(' ', "-").replace('_', "-")
}

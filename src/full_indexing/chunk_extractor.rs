use tree_sitter::Parser;
use tree_sitter_rust;
use tree_sitter_python;
use tree_sitter_javascript;

#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Hash)]
pub enum Language {
    Rust = 0,
    Python = 1,
    JavaScript = 2
}

impl Language {
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext {
            "rs" => Some(Language::Rust),
            "py" => Some(Language::Python),
            "js" => Some(Language::JavaScript),
            _ => None,
        }
    }
    pub fn to_tree_sitter_language(self) -> tree_sitter::Language {
        match self {
            Language::Rust => tree_sitter_rust::LANGUAGE.into(),
            Language::Python => tree_sitter_python::LANGUAGE.into(),
            Language::JavaScript => tree_sitter_javascript::LANGUAGE.into(),
        }
    }
}

pub(crate) fn extract_chunks(content: &str, language: Language) -> Vec<String> {
    let mut parser = parser_for(language);
    let tree = parser.parse(content, None).unwrap();
    extract_functions_and_classes(content, tree)
}

fn extract_functions_and_classes(source: &str, tree: tree_sitter::Tree) -> Vec<String> {
    let root = tree.root_node();
    let mut chunks = Vec::new();

    fn visit(node: tree_sitter::Node, source: &str, chunks: &mut Vec<String>) {
        // adjust kinds per language
        let kind = node.kind();

        if kind == "function_item" || kind == "class_definition" {
            if let Ok(text) = node.utf8_text(source.as_bytes()) {
                chunks.push(text.to_string());
            }
        }

        for i in 0..node.child_count() {
            if let Some(child) = node.child(i.try_into().unwrap()) {
                visit(child, source, chunks);
            }
        }
    }

    visit(root, source, &mut chunks);
    chunks
}
fn parser_for(lang: Language) -> Parser {
    let mut parser = Parser::new();
    parser.set_language(&lang.to_tree_sitter_language()).unwrap();
    parser
}

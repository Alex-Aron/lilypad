use crate::block_editor::text_range::{TextPoint, TextRange};
use crate::vscode;

#[derive(Debug, PartialEq, Clone)]
pub struct Documentation {
    pub message: String,
    pub range: TextRange,
    pub hover_info: HoverInfo,
}

#[derive(PartialEq, Debug, Clone)]
pub enum BlockType {
    CodeBlock,
    RegularBlock,
}
#[derive(PartialEq, Debug, Clone)]
pub struct HoverInfo {
    pub language: String,
    pub all_blocks: Vec<(String, BlockType)>,
}

fn create_hover_info(info: &str) -> HoverInfo {
    let mut lang = "";
    let mut lang_end = "";
    if info.find("python").is_some() {
        lang_end = "py";
        lang = "python";
    } else if info.find("rust").is_some() {
        lang_end = "rs";
        lang = "rust";
    } else if info.find("csharp").is_some() {
        lang_end = "cs";
        lang = "csharp";
    }
    let re = regex::Regex::new(r"<.*?>").unwrap();
    let mut parsed_info = re.replace_all(info, "").to_string();
    parsed_info = parsed_info.replace("&nbsp;", " ").to_string();
    let blocks: Vec<_> = parsed_info.split("```").collect();
    let mut all_blocks: Vec<(String, BlockType)> = vec![];
    for i in blocks {
        if i.len() >= 9 && i[0..8].find(lang).is_some() {
            let temp = i.replace(lang, "").to_string();
            if temp.len() == 0 {
                continue;
            }
            all_blocks.push((i.replace(lang, "").trim().to_string(), BlockType::CodeBlock));
        } else {
            if i.len() == 0 {
                continue;
            }
            all_blocks.push((i.trim().to_string(), BlockType::RegularBlock));
        }
    }

    HoverInfo {
        language: lang_end.to_string(),
        all_blocks: all_blocks,
    }
}

impl Documentation {
    pub fn set_hover(&mut self, message: String, range: TextRange) {
        self.message = message;
        self.range = range;
        self.hover_info = create_hover_info(&self.message);
    }
    pub fn request_hover(&mut self, line: usize, col: usize) {
        vscode::request_hover(line, col);
    }

    pub fn new() -> Documentation {
        Documentation {
            message: " ".to_string(),
            range: TextRange::ZERO,
            hover_info: HoverInfo {
                language: " ".to_string(),
                all_blocks: vec![],
            },
        }
    }

    #[allow(dead_code)]
    pub fn example() -> Documentation {
        Documentation {
            message: "example Documentation".to_string(),
            range: TextRange::new(TextPoint::new(2, 18), TextPoint::new(2, 25)),
            hover_info: HoverInfo {
                language: "rs".to_string(),
                all_blocks: vec![(
                    String::from("fn main() {\n    println!(\"Hello, world!\");\n}"),
                    BlockType::CodeBlock,
                )],
            },
        }
    }
}

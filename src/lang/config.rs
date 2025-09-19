use egui::Color32;

use crate::{block_editor::BlockType, theme::syntax::*};

pub struct LanguageConfig {
    /// Name of the language. Used as an ID and potentially for UI
    pub name: &'static str,

    /// Tree-sitter language
    ts_lang: tree_sitter_language::LanguageFn,

    /// Tree-sitter highlight query
    pub(super) highlight_query: &'static str,

    /// The character that starts a new scope (so should increase the indent)
    pub new_scope_char: NewScopeChar,

    /// Assigns a node a block type to draw
    node_categorizer: fn(&tree_sitter::Node) -> Option<BlockType>,

    /// The IDs for a string, and the start and end. Used for pseudo-selections
    pub string_node_ids: StringNodeIDs,

    /// Snippets to use for the palette. Must end with a newline.
    pub palettes: &'static [Palette],

    /// The highlight names to recognize and their associated colors
    pub highlight: &'static [(&'static str, Color32)],
}

impl LanguageConfig {
    pub fn for_file(file_name: &str) -> &'static LanguageConfig {
        match file_name.split('.').next_back() {
            Some("py") => &PYTHON_LANGUAGE,
            Some("java") => &JAVA_LANGUAGE,
            Some("cpp") | Some("h") | Some("hpp") => &CPP_LANGUAGE,
            Some("cs") => &CS_LANGUAGE,
            Some("rs") => &RUST_LANGUAGE,
            Some("v") | Some("vh") => &VERILOG_LANGUAGE,
            Some("sv") | Some("svh") => &SYSTEMVERILOG_LANGUAGE,
            _ => &PYTHON_LANGUAGE, // TODO: plain text mode?
        }
    }

    pub fn tree_sitter(&self) -> tree_sitter::Language {
        tree_sitter::Language::new(self.ts_lang)
    }

    pub fn categorize_node(&self, node: &tree_sitter::Node) -> Option<BlockType> {
        (self.node_categorizer)(node)
    }
}

pub struct Palette {
    pub name: &'static str,
    pub snippets: &'static [Snippet],
}

impl Palette {
    pub const fn new(name: &'static str, snippets: &'static [Snippet]) -> Palette {
        Palette { name, snippets }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum NewScopeChar {
    Colon,
    Brace,
    Begin,
}

impl NewScopeChar {
    pub const fn char(&self) -> char {
        match self {
            NewScopeChar::Colon => ':',
            NewScopeChar::Brace => '{',
            NewScopeChar::Begin => 'b', // 'b' for begin - this is used for newline insertion logic
        }
    }
}

#[derive(Clone, Copy)]
pub struct StringNodeIDs {
    pub string: u16,
    pub string_bounds: &'static [u16],
}

pub struct Snippet {
    pub id: &'static str,
    pub source: &'static str,
}

impl Snippet {
    pub const fn new(id: &'static str, source: &'static str) -> Snippet {
        Snippet { id, source }
    }
}

const PYTHON_LANGUAGE: LanguageConfig = LanguageConfig {
    name: "python",
    ts_lang: tree_sitter_python::LANGUAGE,
    highlight_query: tree_sitter_python::HIGHLIGHTS_QUERY,
    new_scope_char: NewScopeChar::Colon,
    node_categorizer: |node| {
        use BlockType::*;

        match node.kind() {
            // scopes
            "class_definition" => Some(Object),
            "function_definition" => Some(FunctionDef),
            "while_statement" => Some(While),
            "if_statement" => Some(If),
            "for_statement" => Some(For),
            "try_statement" => Some(Try),

            // normal expressions
            // TODO: check exhaustiveness
            "import_statement" => Some(Generic),
            "import_from_statement" => Some(Generic),
            "expression_statement" => Some(Generic),
            "continue_statement" => Some(Generic),
            "break_statement" => Some(Generic),
            "pass_statement" => Some(Generic),
            "return_statement" => Some(Generic),

            // comments
            "comment" => Some(Comment),

            // dividers to keep generics from merging
            "else_clause" => Some(Divider),
            "elif_clause" => Some(Divider),
            "except_clause" => Some(Divider),

            // do not handle the rest
            _ => None,
        }
    },
    string_node_ids: StringNodeIDs {
        string: 232,
        string_bounds: &[104, 107], // 104 is string start, 107 is string end
    },
    palettes: &[
        Palette::new(
            "General",
            &[
                Snippet::new("import_module", "import module\n"),
                Snippet::new("import_from", "from module import thing\n"),
                Snippet::new("import_as", "import module as name\n"),
                Snippet::new("var_assign", "val = 0\n"),
                Snippet::new("var_assign_string", "val = \"Hello world\"\n"),
                Snippet::new("var_assign_list", "val = [1, 2, 3]\n"),
                Snippet::new("var_assign_dict", "val = {'a': 1, 'b': 'tw0'}\n"),
                Snippet::new("var_assign_tuple", "val = (False, 1, 2.0, '3')\n"),
                Snippet::new("var_assign_set", "val = {1, 2, 3}\n"),
            ],
        ),
        Palette::new(
            "Classes",
            &[
                Snippet::new(
                    "class_declaration",
                    "class ClassName:\n    def __init__(self, param):\n        pass\n",
                ),
                Snippet::new("instance_method", "def method(self, param):\n    pass\n"),
                Snippet::new(
                    "static_method",
                    "@staticmethod\ndef method(param):\n    pass\n",
                ),
                Snippet::new("class_instance", "instance = ClassName()\n"),
            ],
        ),
        Palette::new(
            "Control",
            &[
                Snippet::new("for", "for item in range(0, 10):\n    pass\n"),
                Snippet::new("while", "while 0 == 0:\n    pass\n"),
                Snippet::new("break", "break\n"),
                Snippet::new("continue", "continue\n"),
                Snippet::new("if", "if 0 < 0:\n    pass\n"),
                Snippet::new("if_else", "if 0 < 0:\n    pass\nelse:\n    pass\n"),
                Snippet::new(
                    "if_elif_else",
                    "if 0 < 0:\n    pass\nelif 0 > 0:\n    pass\nelse:\n    pass\n",
                ),
                Snippet::new(
                    "try",
                    "try:\n    pass\nexcept:\n    pass\nelse:\n    pass\nfinally:\n    pass\n",
                ),
            ],
        ),
        Palette::new(
            "Functions",
            &[
                Snippet::new("function_def", "def function(args):\n    return\n"),
                Snippet::new("function_call", "function(args) \n"),
                Snippet::new("return_val", "return value\n"),
                Snippet::new("return", "return\n"),
            ],
        ),
        Palette::new(
            "Logic",
            &[
                Snippet::new("equals", "a == b\n"),
                Snippet::new("not_equals", "a != b\n"),
                Snippet::new("greater_than", "a > b\n"),
                Snippet::new("less_than", "a < b\n"),
                Snippet::new("greater_than_or_equal", "a >= b\n"),
                Snippet::new("less_than_or_equal", "a <= b\n"),
                Snippet::new("and", "a and b\n"),
                Snippet::new("or", "a or b\n"),
                Snippet::new("not", "not a\n"),
                Snippet::new("in", "a in b\n"),
                Snippet::new("is", "a is b\n"),
            ],
        ),
        Palette::new(
            "Arithmetic",
            &[
                Snippet::new("add", "a + b\n"),
                Snippet::new("subtract", "a - b\n"),
                Snippet::new("multiply", "a * b\n"),
                Snippet::new("divide", "a / b\n"),
                Snippet::new("modulo", "a % b\n"),
                Snippet::new("exponent", "a ** b\n"),
                Snippet::new("floor_divide", "a // b\n"),
            ],
        ),
    ],
    highlight: STANDARD_HIGHLIGHT,
};

const JAVA_LANGUAGE: LanguageConfig = LanguageConfig {
    name: "java",
    ts_lang: tree_sitter_java::LANGUAGE,
    highlight_query: tree_sitter_java::HIGHLIGHTS_QUERY,
    new_scope_char: NewScopeChar::Brace,
    node_categorizer: |node| {
        use BlockType::*;

        match node.kind() {
            // scopes
            "class_declaration" => Some(Object),
            "interface_declaration" => Some(Object),
            "method_declaration" => Some(FunctionDef),
            "while_statement" => Some(While),
            "if_statement" => {
                // the java grammar treats else if as else, if_statement
                // so check that is isn't that
                if node.prev_sibling().map_or("", |s| s.kind()) == "else" {
                    None
                } else {
                    Some(If)
                }
            }
            "for_statement" => Some(For),
            "try_statement" => Some(Try),

            // normal expressions (incomplete)
            "import_declaration" => Some(Generic),
            "expression_statement" => Some(Generic),
            "local_variable_declaration" => {
                // don't create a block for a for loop's variable declaration
                if node.parent().map_or("", |p| p.kind()) == "for_statement" {
                    None
                } else {
                    Some(Generic)
                }
            }
            "field_declaration" => Some(Generic),
            "return_statement" => Some(Generic),
            "assert_statement" => Some(Generic),

            // comments
            "line_comment" => Some(Comment),
            "block_comment" => Some(Comment),

            // dividers to keep generics from merging
            "block" => Some(Divider),

            // do not handle the rest
            _ => None,
        }
    },
    string_node_ids: StringNodeIDs {
        string: 141,
        string_bounds: &[11, 12], // 11 is single quote, 12 is double quote
    },
    palettes: &[Palette::new(
        "General",
        &[
            Snippet::new(
                "if",
                "if (condition) {\n    \n} else if (condition) {\n    \n} else {\n    \n}\n",
            ),
            Snippet::new(
                "class",
                "public class MyClass {\n    public MyClass() {\n        \n    }\n}\n",
            ),
            Snippet::new("while", "while (condition) {\n    \n}\n"),
            Snippet::new("method", "public void myMethod() {\n    \n}\n"),
            Snippet::new(
                "try",
                "try {\n    \n} catch (Exception e) {\n    \n} finally {\n    \n}\n",
            ),
        ],
    )],
    highlight: STANDARD_HIGHLIGHT,
};

const CS_LANGUAGE: LanguageConfig = LanguageConfig {
    name: "c#",
    ts_lang: tree_sitter_c_sharp::LANGUAGE,
    highlight_query: tree_sitter_c_sharp::HIGHLIGHTS_QUERY,
    new_scope_char: NewScopeChar::Brace,
    node_categorizer: |node| {
        use BlockType::*;

        match node.kind() {
            // scopes
            "class_declaration" => Some(Object),
            "method_declaration" => Some(FunctionDef),
            "while_statement" => Some(While),
            "if_statement" => {
                if node.prev_sibling().map_or("", |s| s.kind()) == "else" {
                    None
                } else {
                    Some(If)
                }
            }
            "for_statement" => Some(For),
            "try_statement" => Some(Try),
            "switch_statement" => Some(Switch),
            "switch_section" => Some(Divider),
            "import_declaration" => Some(Generic),
            "local_decleration_statement" => {
                // don't create a block for a for loop's variable declaration
                if node.parent().map_or("", |p| p.kind()) == "for_statement" {
                    None
                } else {
                    Some(Generic)
                }
            }
            "field_declaration" => Some(Generic),
            "break_statement" => Some(Generic),
            "return_statement" => Some(Generic),
            "assert_statement" => Some(Generic),
            "local_function_statement" => Some(Generic),
            "expression_statement" => Some(Generic),

            "using_directive" => Some(Generic),
            // comments
            "line_comment" => Some(Comment),
            "block_comment" => Some(Comment),

            // dividers to keep generics from merging
            "block" => Some(Divider),

            // do not handle the rest
            _ => None,
        }
    },
    string_node_ids: StringNodeIDs {
        string: 141,
        string_bounds: &[11, 12], // 11 is single quote, 12 is double quote
    },
    palettes: &[Palette::new(
        "General",
        &[
            Snippet::new(
                "if",
                "if (condition) {\n    \n} else if (condition) {\n    \n} else {\n    \n}\n",
            ),
            Snippet::new(
                "class",
                "public class MyClass {\n    public MyClass() {\n        \n    }\n}\n",
            ),
            Snippet::new("while", "while (condition) {\n    \n}\n"),
            Snippet::new("func", "public void myFunction() {\n    \n}\n"),
            Snippet::new(
                "try",
                "try {\n    \n} catch (Exception e) {\n    \n} finally {\n    \n}\n",
            ),
        ],
    )],
    highlight: STANDARD_HIGHLIGHT,
};

const CPP_LANGUAGE: LanguageConfig = LanguageConfig {
    name: "cpp",
    ts_lang: tree_sitter_cpp::LANGUAGE,
    highlight_query: tree_sitter_cpp::HIGHLIGHT_QUERY,
    new_scope_char: NewScopeChar::Brace,
    node_categorizer: |node| {
        use BlockType::*;

        match node.kind() {
            // scopes
            "class_specifier" => Some(Object),
            "struct_specifier" => Some(Object),
            "abstract_function_declarator" => Some(Object),
            "function_definition" => {
                // create one box around a template function
                if node.parent().map_or("", |s| s.kind()) == "template_declaration" {
                    None
                } else {
                    Some(FunctionDef)
                }
            }
            "while_statement" => Some(While),
            "if_statement" => {
                if node.prev_sibling().map_or("", |s| s.kind()) == "else" {
                    None
                } else {
                    Some(If)
                }
            }
            "for_statement" => Some(For),
            "try_statement" => Some(Try),
            "template_declaration" => Some(FunctionDef),

            // normal expressions (incomplete)
            "preproc_include" => Some(Generic),
            "expression_statement" => Some(Generic),
            "continue_statement" => Some(Generic),
            "break_statement" => Some(Generic),
            "pass_statement" => Some(Generic),
            "local_variable_declaration" => {
                // don't create a block for a for loop's variable declaration
                if node.parent().map_or("", |p| p.kind()) == "for_statement" {
                    None
                } else {
                    Some(Generic)
                }
            }

            // comments
            "comment" => Some(Comment),

            // dividers to keep generics from merging
            "else_clause" => Some(Divider),
            "except_clause" => Some(Divider),

            // do not handle the rest
            _ => None,
        }
    },
    string_node_ids: StringNodeIDs {
        string: 360,
        string_bounds: &[162],
    },
    palettes: &[Palette::new("General", &[])],
    highlight: STANDARD_HIGHLIGHT,
};

const RUST_LANGUAGE: LanguageConfig = LanguageConfig {
    name: "rust",
    ts_lang: tree_sitter_rust::LANGUAGE,
    highlight_query: tree_sitter_rust::HIGHLIGHTS_QUERY,
    new_scope_char: NewScopeChar::Brace,
    node_categorizer: |node| {
        use BlockType::*;

        match node.kind() {
            // scopes
            "struct_item" => Some(Object),
            "union_item" => Some(Object),
            "enum_item" => Some(Object),
            "impl_item" => Some(Object),
            "trait_item" => Some(Object),
            "function_item" => Some(FunctionDef),
            "type_item" => Some(Object),
            "block" => {
                if node.parent().map_or("", |s| s.kind()) == "function_item"
                    || node.parent().map_or("", |s| s.kind()) == "if_expression"
                    || node.parent().map_or("", |s| s.kind()) == "else_clause"
                    || node.parent().map_or("", |s| s.kind()) == "match_arm"
                    || node.parent().map_or("", |s| s.kind()) == "match_block"
                {
                    None
                } else {
                    Some(Object)
                }
            }
            "enum_variant" => Some(Generic),
            "field_declaration" => Some(Generic),
            "while_expression" => {
                if node.parent().map_or("", |s| s.kind()) == "expression_statement" {
                    None
                } else {
                    Some(Generic)
                }
            }
            "match_block" => Some(Switch),
            "binary_expression" => {
                if node.parent().map_or("", |s| s.kind()) == "block" {
                    None //some(generic) for this and idenitifer enabled multiline let, but caused other issues
                } else {
                    None
                }
            }
            "identifier" => {
                if node.parent().map_or("", |s| s.kind()) == "block" {
                    None
                } else {
                    None
                }
            }
            //Some(Switch),
            "match_arm" => Some(Generic),
            /*            "let_declaration" => {
                            if (node.named_child_count() == 2 || node.named_child_count() == 3)
                                && node
                                    .named_child(1)
                                    .map_or(false, |child| child.named_child_count() == 0)
                            {
                                Some(Generic)
                            } else {
                                Some(Object)
                            }
                        }
            */
            "let_declaration" => {
                if node.start_position().row == node.end_position().row {
                    Some(Generic)
                } else {
                    Some(Divider)
                }
            }
            "if_expression" => {
                if node.parent().map_or("", |s| s.kind()) == "else_clause" {
                    None
                } else {
                    Some(If)
                }
            }
            "for_expression" => {
                if node.parent().map_or("", |s| s.kind()) == "expression_statement" {
                    None
                } else {
                    Some(For)
                }
            }
            "struct_expression" => Some(Generic),
            "continue_expression" => Some(Generic),
            "call_expression" => {
                if let Some(parent) = node.parent() {
                    let value_contexts = [
                        "let_declaration",
                        "match_expression",
                        "binary_expression",
                        "argument_list",
                        "match_arm",
                        "if_expression",
                        "for_expression",
                        "while_expression",
                        "arguments",
                    ];
                    if value_contexts.contains(&parent.kind()) {
                        None
                    } else {
                        Some(Generic)
                    }
                } else {
                    None
                }
            }

            "macro_invocation" => {
                if let Some(parent) = node.parent() {
                    let value_contexts = [
                        "let_declaration",
                        "match_expression",
                        "binary_expression",
                        "argument_list",
                        "match_arm",
                        "if_expression",
                        "for_expression",
                        "while_expression",
                    ];
                    if value_contexts.contains(&parent.kind()) {
                        None
                    } else {
                        Some(Generic)
                    }
                } else {
                    Some(Generic)
                }
            }
            // normal expressions (incomplete)
            "use_item" => Some(Generic),
            "else_clause" => Some(Divider),
            //"break_expressionex" => Some(Generic),
            "return_expression" => Some(Generic),
            "assignment_expression" => Some(Generic),
            // comments
            "line_comment" => Some(Comment),
            "block_comment" => Some(Comment),

            // dividers to keep generics from merging

            // do not handle the rest
            _ => None,
        }
    },
    string_node_ids: StringNodeIDs {
        string: 360,
        string_bounds: &[162],
    },
    palettes : &[Palette::new(
            "General",
            &[
                Snippet::new(
                    "if",
                    "if condition {\n    // code\n} else if condition {\n    // code\n} else {\n    // code\n}",
                ),
                Snippet::new(
                    "loop",
                    "loop {\n    // code\n}",
                ),
                Snippet::new(
                    "while",
                    "while condition {\n    // code\n}",
                ),
                Snippet::new(
                    "fn",
                    "fn my_function() {\n    // code\n}",
                ),
                Snippet::new(
                    "match",
                    "match value {\n    Pattern1 => {None}\n    Pattern2 => {None}\n    _ => {}\n}",
                ),
                Snippet::new(
                    "result",
                    "fn divide(a: i32, b: i32) -> Result<i32, String> {\n    if b == 0 {\n        Err(String::from(\"Cannot divide by zero\"))\n    } else {\n        Ok(a / b)\n    }\n}",
                ),
            ])],
    highlight: STANDARD_HIGHLIGHT,
};

const VERILOG_LANGUAGE: LanguageConfig = LanguageConfig {
    name: "verilog",
    ts_lang: tree_sitter_verilog::LANGUAGE,
    highlight_query: concat!(
        "[\"module\" \"endmodule\" \"input\" \"output\" \"inout\" \"wire\" \"reg\"] @keyword\n",
        "[\"always\" \"initial\" \"begin\" \"end\" \"if\" \"else\" \"case\" \"endcase\"] @keyword\n",
        "[\"for\" \"while\" \"repeat\" \"forever\" \"task\" \"endtask\" \"function\" \"endfunction\"] @keyword\n",
        "[\"assign\" \"parameter\" \"localparam\" \"generate\" \"endgenerate\"] @keyword\n",
        "[\"integer\" \"real\" \"time\" \"realtime\" \"event\"] @type\n"
    ),
    new_scope_char: NewScopeChar::Begin,
    node_categorizer: |node| {
        use BlockType::*;

        match node.kind() {
            // modules and interfaces
            "module_declaration" => Some(Object),
            "interface_declaration" => Some(Object),
            "package_declaration" => Some(Object),
            
            // tasks and functions
            "task_declaration" => Some(FunctionDef),
            "function_declaration" => Some(FunctionDef),
            
            // control structures
            "if_statement" => Some(If),
            "case_statement" => Some(Switch),
            "for_statement" => Some(For),
            "while_statement" => Some(While),
            "repeat_statement" => Some(For),
            "forever_statement" => Some(While),
            
            // blocks
            "initial_construct" => Some(Generic),
            "always_construct" => Some(Generic),
            "final_construct" => Some(Generic),
            
            // declarations
            "data_declaration" => Some(Generic),
            "net_declaration" => Some(Generic),
            "parameter_declaration" => Some(Generic),
            "localparam_declaration" => Some(Generic),
            
            // instantiations
            "module_instantiation" => Some(Generic),
            "interface_instantiation" => Some(Generic),
            
            // assignments
            "continuous_assign" => Some(Generic),
            "procedural_continuous_assign" => Some(Generic),
            
            // comments
            "comment" => Some(Comment),
            
            // dividers
            "else_clause" => Some(Divider),
            "default_clause" => Some(Divider),
            
            _ => None,
        }
    },
    string_node_ids: StringNodeIDs {
        string: 200, // placeholder - needs to be determined from actual grammar
        string_bounds: &[34], // double quote
    },
    palettes: &[
        Palette::new(
            "Modules",
            &[
                Snippet::new("module", "module module_name(\n    // ports\n);\n    // module body\nendmodule\n"),
                Snippet::new("interface", "interface interface_name;\n    // interface body\nendinterface\n"),
                Snippet::new("task", "task task_name;\n    // task body\nendtask\n"),
                Snippet::new("function", "function return_type function_name;\n    // function body\nendfunction\n"),
            ],
        ),
        Palette::new(
            "Control",
            &[
                Snippet::new("if", "if (condition) begin\n    // statements\nend\n"),
                Snippet::new("if_else", "if (condition) begin\n    // if statements\nend else begin\n    // else statements\nend\n"),
                Snippet::new("case", "case (expression)\n    value1: begin\n        // statements\n    end\n    default: begin\n        // default statements\n    end\nendcase\n"),
                Snippet::new("for", "for (int i = 0; i < limit; i++) begin\n    // statements\nend\n"),
                Snippet::new("while", "while (condition) begin\n    // statements\nend\n"),
            ],
        ),
        Palette::new(
            "Blocks",
            &[
                Snippet::new("always", "always @(*) begin\n    // combinational logic\nend\n"),
                Snippet::new("always_ff", "always_ff @(posedge clk) begin\n    // sequential logic\nend\n"),
                Snippet::new("initial", "initial begin\n    // initialization\nend\n"),
                Snippet::new("final", "final begin\n    // finalization\nend\n"),
            ],
        ),
    ],
    highlight: STANDARD_HIGHLIGHT,
};

const SYSTEMVERILOG_LANGUAGE: LanguageConfig = LanguageConfig {
    name: "systemverilog",
    ts_lang: tree_sitter_systemverilog::LANGUAGE,
    highlight_query: concat!(
        "[\"module\" \"endmodule\" \"input\" \"output\" \"inout\" \"wire\" \"reg\" \"logic\"] @keyword\n",
        "[\"always\" \"always_ff\" \"always_comb\" \"always_latch\" \"initial\" \"begin\" \"end\"] @keyword\n",
        "[\"if\" \"else\" \"case\" \"endcase\" \"for\" \"while\" \"repeat\" \"forever\"] @keyword\n",
        "[\"task\" \"endtask\" \"function\" \"endfunction\" \"return\"] @keyword\n",
        "[\"class\" \"endclass\" \"interface\" \"endinterface\" \"package\" \"endpackage\"] @keyword\n",
        "[\"assign\" \"parameter\" \"localparam\" \"generate\" \"endgenerate\"] @keyword\n",
        "[\"bit\" \"byte\" \"int\" \"integer\" \"time\" \"real\" \"string\"] @type\n"
    ),
    new_scope_char: NewScopeChar::Begin,
    node_categorizer: |node| {
        use BlockType::*;

        match node.kind() {
            // modules and interfaces
            "module_declaration" => Some(Object),
            "interface_declaration" => Some(Object),
            "package_declaration" => Some(Object),
            "class_declaration" => Some(Object),
            "program_declaration" => Some(Object),
            
            // tasks and functions
            "task_declaration" => Some(FunctionDef),
            "function_declaration" => Some(FunctionDef),
            "method_declaration" => Some(FunctionDef),
            "constructor_declaration" => Some(FunctionDef),
            
            // control structures
            "if_statement" => Some(If),
            "case_statement" => Some(Switch),
            "casex_statement" => Some(Switch),
            "casez_statement" => Some(Switch),
            "unique_case_statement" => Some(Switch),
            "for_statement" => Some(For),
            "foreach_statement" => Some(For),
            "while_statement" => Some(While),
            "do_while_statement" => Some(While),
            "repeat_statement" => Some(For),
            "forever_statement" => Some(While),
            
            // try-catch for SystemVerilog
            "try_statement" => Some(Try),
            
            // blocks
            "initial_construct" => Some(Generic),
            "always_construct" => Some(Generic),
            "always_comb" => Some(Generic),
            "always_ff" => Some(Generic),
            "always_latch" => Some(Generic),
            "final_construct" => Some(Generic),
            
            // declarations
            "data_declaration" => Some(Generic),
            "net_declaration" => Some(Generic),
            "parameter_declaration" => Some(Generic),
            "localparam_declaration" => Some(Generic),
            "typedef_declaration" => Some(Generic),
            "property_declaration" => Some(Generic),
            "sequence_declaration" => Some(Generic),
            
            // instantiations
            "module_instantiation" => Some(Generic),
            "interface_instantiation" => Some(Generic),
            "class_instantiation" => Some(Generic),
            
            // assignments
            "continuous_assign" => Some(Generic),
            "procedural_continuous_assign" => Some(Generic),
            "blocking_assignment" => Some(Generic),
            "nonblocking_assignment" => Some(Generic),
            
            // assertions and coverage
            "assertion_statement" => Some(Generic),
            "assume_statement" => Some(Generic),
            "cover_statement" => Some(Generic),
            "expect_statement" => Some(Generic),
            
            // comments
            "comment" => Some(Comment),
            
            // dividers
            "else_clause" => Some(Divider),
            "default_clause" => Some(Divider),
            "catch_clause" => Some(Divider),
            
            _ => None,
        }
    },
    string_node_ids: StringNodeIDs {
        string: 300, // placeholder - needs to be determined from actual grammar
        string_bounds: &[34], // double quote
    },
    palettes: &[
        Palette::new(
            "Modules & Classes",
            &[
                Snippet::new("module", "module module_name(\n    // ports\n);\n    // module body\nendmodule\n"),
                Snippet::new("interface", "interface interface_name;\n    // interface body\nendinterface\n"),
                Snippet::new("class", "class class_name;\n    // class members\nendclass\n"),
                Snippet::new("package", "package package_name;\n    // package contents\nendpackage\n"),
                Snippet::new("program", "program program_name;\n    // program body\nendprogram\n"),
            ],
        ),
        Palette::new(
            "Control Flow",
            &[
                Snippet::new("if", "if (condition) begin\n    // statements\nend\n"),
                Snippet::new("if_else", "if (condition) begin\n    // if statements\nend else begin\n    // else statements\nend\n"),
                Snippet::new("case", "case (expression)\n    value1: begin\n        // statements\n    end\n    default: begin\n        // default statements\n    end\nendcase\n"),
                Snippet::new("for", "for (int i = 0; i < limit; i++) begin\n    // statements\nend\n"),
                Snippet::new("foreach", "foreach (array[i]) begin\n    // statements\nend\n"),
                Snippet::new("while", "while (condition) begin\n    // statements\nend\n"),
                Snippet::new("do_while", "do begin\n    // statements\nend while (condition);\n"),
            ],
        ),
        Palette::new(
            "Blocks & Processes",
            &[
                Snippet::new("always_comb", "always_comb begin\n    // combinational logic\nend\n"),
                Snippet::new("always_ff", "always_ff @(posedge clk) begin\n    // sequential logic\nend\n"),
                Snippet::new("always_latch", "always_latch begin\n    // latch logic\nend\n"),
                Snippet::new("initial", "initial begin\n    // initialization\nend\n"),
                Snippet::new("final", "final begin\n    // finalization\nend\n"),
            ],
        ),
        Palette::new(
            "Verification",
            &[
                Snippet::new("assert", "assert (condition) else $error(\"Assertion failed\");\n"),
                Snippet::new("assume", "assume (condition);\n"),
                Snippet::new("cover", "cover (condition);\n"),
                Snippet::new("expect", "expect (condition) else $error(\"Expectation failed\");\n"),
                Snippet::new("try_catch", "try begin\n    // risky operation\nend\ncatch begin\n    // error handling\nend\n"),
            ],
        ),
    ],
    highlight: STANDARD_HIGHLIGHT,
};

const STANDARD_HIGHLIGHT: &[(&str, Color32)] = &[
    ("function", FUNCTION),
    ("function.builtin", FUNCTION_BUILT_IN),
    ("keyword", KEYWORD),
    ("operator", OPERATOR),
    ("property", PROPERTY),
    ("punctuation.special", INTERPOLATION_SURROUNDING),
    ("string", STRING),
    ("type", TYPE),
    ("variable", VARIABLE),
    ("constructor", CONSTRUCTOR),
    ("constant", CONSTANT),
    ("constant.builtin", LITERAL),
    ("number", LITERAL),
    ("escape", ESCAPE_SEQUENCE),
    ("comment", COMMENT),
    ("embedded", DEFAULT), // treat inside of interpolation like top level
];

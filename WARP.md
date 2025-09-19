# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Project Overview

Lilypad is a text-based visual code editor architected to support multiple languages and platforms. It provides a unique "blocks-based" view of code where syntax elements are visualized as colored blocks, making code structure more apparent to developers.

## Key Architecture Components

### Multi-Platform Support
- **Native Desktop App**: Built with egui/eframe, runs as standalone application
- **VSCode Extension**: WebAssembly-based custom editor that integrates with VSCode's language services
- **Web Version**: Browser-based editor using WebAssembly

### Core Technical Stack
- **Language**: Rust (with WebAssembly compilation for web targets)
- **UI Framework**: egui for immediate-mode GUI
- **Syntax Parsing**: Tree-sitter for language parsing and syntax highlighting
- **Text Handling**: ropey crate for efficient text rope operations
- **Build System**: Uses `just` command runner for WebAssembly builds

### Language Support Architecture
The editor supports multiple programming languages through a unified configuration system:
- Python, Java, C#, C++, Rust, Verilog, and SystemVerilog are currently supported
- Each language has its own `LanguageConfig` with Tree-sitter grammar integration
- Block categorization maps syntax nodes to visual block types (Object, FunctionDef, If, While, For, Try, Switch, Generic, Comment, etc.)
- Syntax highlighting using Tree-sitter queries with custom color schemes
- Flexible scope handling supports different delimiter styles:
  - Colon-based scoping (Python): `if condition:`
  - Brace-based scoping (C-style): `if (condition) {`
  - Begin/end scoping (Verilog/SystemVerilog): `if (condition) begin ... end`

### Block Editor Architecture
The core visual editing system consists of:
- **BlockEditor**: Main coordinator managing source code, visual blocks, and UI interactions
- **TextEditor**: Handles text input, selection, cursor movement, and editing operations
- **BlockTrees**: Converts Tree-sitter syntax trees into visual block hierarchies
- **Source**: Manages the underlying text with undo/redo, language parsing, and text operations
- **Visual Block Types**: Different code constructs are rendered as different colored blocks

## Common Development Tasks

### Building the Project

**Native Desktop Application:**
```powershell
cargo run
```

**VSCode Extension (Development):**
```powershell
just wasm-vscode-dev
cd lilypad-vscode
npm install
# Open lilypad-vscode/ in VSCode Insiders and run via F5
```

**VSCode Extension (Release):**
```powershell
just wasm-vscode
cd lilypad-vscode
npm install
```

**Web Version (Development):**
```powershell
just wasm-web-dev
cd lilypad-web
http-server -p 8000
```

**Web Version (Release):**
```powershell
just wasm-web
cd lilypad-web
http-server -p 8000
```

### Testing and Linting
```powershell
cargo test --release
cargo clippy --release
cargo fmt --check
```

### Prerequisites for Development
- **Rust**: Install via rustup.rs (requires nightly toolchain)
- **wasm-pack**: For WebAssembly builds
- **LLVM**: Required for language grammars in WASM (llvm-ar must be in PATH)
- **Just**: Command runner for build scripts
- **Node.js & npm**: For VSCode extension and web builds
- **Default fonts**: SF Mono (macOS), Roboto Mono (Windows)

## File Structure Patterns

- `src/bin.rs` - Native application entry point
- `src/lib.rs` - WebAssembly library entry point  
- `src/block_editor/` - Core visual editing system
- `src/lang/` - Language support and Tree-sitter integration
- `src/theme/` - Visual themes and color schemes
- `lilypad-vscode/` - VSCode extension TypeScript code
- `lilypad-web/` - Web application JavaScript/HTML
- `justfile` - Build commands for WebAssembly targets

## Adding Language Support

To add a new programming language:

1. Add Tree-sitter dependency to `Cargo.toml`
2. Create language configuration in `src/lang/config.rs` with:
   - Tree-sitter language function
   - Highlight query from grammar
   - Block categorization function (maps syntax nodes to block types)
   - String node IDs for pseudo-selections
   - Code snippets for the palette
3. Add file extension mapping in `LanguageConfig::for_file()`
4. Update VSCode extension file associations in `lilypad-vscode/package.json`
5. For languages using begin/end scoping (like Verilog/SystemVerilog), use `NewScopeChar::Begin`

## WebAssembly Considerations

- Uses LLVM tools for C grammar compilation
- Environment variables set in `justfile` for cross-compilation
- Separate build targets for VSCode extension vs web application
- C shim module provides stdlib functions to C grammars in WASM environment

## Integration Points

- **LSP Integration**: VSCode extension bridges language server features (diagnostics, completions, hover info) to the visual editor
- **Debugging Support**: Breakpoint visualization and stack frame highlighting
- **Theme System**: Configurable color schemes for both syntax highlighting and block visualization
- **Font System**: Monospace font handling with character measurement for precise text layout

<citations>
<document>
<document_type>WARP_DOCUMENTATION</document_type>
<document_id>getting-started/quickstart-guide/coding-in-warp</document_id>
</document>
</citations>
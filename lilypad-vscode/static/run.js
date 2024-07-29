import init, { run_editor, set_text, apply_edit, copy_selection, cut_selection, insert_text, new_diagnostics, set_hover_info, set_quick_fixes, set_completions, undo, redo } from "./lilypad_web.js";

async function run() {
  await init();
  // fileName, fontFamily, and fontSize are set in another script tag
  run_editor(fileName, fontFamily, fontSize);
}
// web view -> extension messages
const vscode = acquireVsCodeApi();

export function started() {
  // send a resize event to the window to make sure the editor is sized correctly
  // run after a delay so things have the oppurtunity to appear
  setTimeout(() => {
    window.dispatchEvent(new UIEvent("resize"));
  }, 50);

  vscode.postMessage({
    type: "started",
  });
}

export function edited(newText, startLine, startCol, endLine, endCol) {
  const range = { startLine, startCol, endLine, endCol };
  vscode.postMessage({
    type: "edited",
    text: newText,
    range: range,
  });
}

export function setClipboard(text) {
  vscode.postMessage({
    type: "set_clipboard",
    text: text,
  });
}

export function requestQuickFixes(line, col) {
  vscode.postMessage({
    type: "get_quick_fixes",
    line: line,
    col: col,
  });
}

export function requestCompletions(line, col) {
  //Types in run.js are messages in lilypadEditor.ts switch statement
  vscode.postMessage({
    type: "get_completions",
    line: line,
    col: col,
  });
}

export function executeCommand(command, args) {
  vscode.postMessage({
    type: "execute_command",
    command: command,
    args: args,
  });
}

export function executeWorkspaceEdit(edit) {
  vscode.postMessage({
    type: "execute_workspace_edit",
    edit: edit
  });
}

export function telemetryEvent(cat, info) {
  vscode.postMessage({
    type: "telemetry_log",
    cat: cat,
    info: Object.fromEntries(info) 
  });
}

export function telemetryCrash(msg) {
  vscode.postMessage({
    type: "telemetry_crash",
    msg: msg,
  });
}

export function requestHoverInfo(line, col){
  vscode.postMessage({
    type: "hover_info",
    line: line,
    col: col,
  });
}



// extension -> web view messages
window.addEventListener("message", event => {
  const message = event.data;
  switch (message.type) {
    case "set_text":
      set_text(message.text);
      break;
    case "apply_edit":
      apply_edit(message.edit);
      break;
    case "new_diagnostics":
      new_diagnostics(message.diagnostics);
      break;
    case "return_quick_fixes":
      set_quick_fixes(message.actions);
      break;
    case "return_completions":
      set_completions(message.completions);
      break;
    case "undo":
      undo();
      break;
    case "redo":
      redo();
      break;
    case "return_documentation_info"://Call function when the rust code decides it is time
      set_hover_info(message.hover);
      break;
  }
});

// handle clipboard actions
document.addEventListener("copy", function(e) {
  copy_selection();
  e.preventDefault();
});

document.addEventListener("cut", function(e) {
  cut_selection();
  e.preventDefault();
});

document.addEventListener("paste", function(e) {
  let text = e.clipboardData.getData("text/plain");
  insert_text(text);
  e.preventDefault();
});

// start the editor
run();

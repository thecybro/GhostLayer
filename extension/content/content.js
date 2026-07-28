// This file will talk directly to background via:
// 
// await chrome.runtime.sendMessage({ })

let activeEditor = null;

// Check if user is focusing on any area that's content isContentEditable
// or is an input field and store it in our variable activeEditor so we can
// get the text in that editor and replace it afterwards
document.addEvenetListener("focusin", (event) => {
  const element = event.target;

  if (
    element instanceof HTMLTextAreaElement ||
    element instanceof HTMLInputElement ||
    element.isContentEditable
  ) {
    activeEditor = element;
  }
});

// Create a floating button called "Encrypt" that will get the text in the texteditor
// trigger encryption on that text and replace it with encrypted text
const encryptButton = document.createElement("button");
encryptButton.textContent = "Encrypt";

Object.assign(encryptButton.style, {
  position: "fixed",
  right: "20px",
  bottom: "20px",
  zIndex: "999999",
  padding: "10px 16px",
  cursor: "pointer"
});

document.body.appendChild(encryptButton);

// Extract text from the given editor and return it
function getTextFromEditor(editor) {
  if (
    editor instanceof HTMLTextAreaElement ||
    editor instanceof HTMLInputElement
  ) {
    return editor.value;
  }
  if (editor.isContentEditable) {
    return editor.textContent;
  }
  return "";
}

// take the encrypted(or newText) from background worker (current workflow)
// and replace the text of the editor with it
function replaceContentOfEditor(editor, newText) {
  if (
    editor instanceof HTMLTextAreaElement ||
    editor instanceof HTMLInputElement
  ) {
    editor.value = newText;
  } else if (editor.isContentEditable) {
    editor.textContent = newText;
  }

  // gotta tell the browser an input happened
  editor.dispatchEvent(
    new Event("input", {
      bubbles: true
    })
  );
}

encryptButton.addEventListener("click", async () => {
  if (!activeEditor) {
    console.error("No active editors found!");
    // notify("No active editors found", "error"); // notify() not built yet, uncomment when built
    return;
  };

  const plaintext = getTextFromEditor(activeEditor);

  if (!plaintext.trim()) {
    console.error("Textbox is empty!");
    // notify("Textbox is empty!", "error"); // not built yet
    return;
  }
  
  const result = await chrome.runtime.sendMessage({
    type: "ENCRYPT_MESSAGE",
    plaintext
  })

  // replace is the result was successful (result.success is the status check returned by background worker)
  // not a builtin feature
  if (result.success) {
    replaceContentOfEditor(activeEditor, result.encryptedText);
  } else {
    console.error("Couldn't replace the content of the editor!");
    // notify("Couldn't replace the content of the editor!", "error"); 
  }
})
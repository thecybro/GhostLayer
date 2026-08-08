// This file talks to the background worker using:
// await chrome.runtime.sendMessage({ ... })

const PREFIX = "ghl:msg"; // what a message key starts with once the code markers `` are stripped

let activeEditor = null;

// Prevent adding multiple Decrypt buttons to the same displayed message.
const seenMessages = new WeakSet();

function formatMessageKey(messageKey) {
  return `\`${messageKey}\``;
}

// Platforms that render code spans `` eat the backstricks, and the ones
// that don't leave in the text, so both shapes have to be accepted
function stripCodeMarkers(text) {
  return (text ?? "").trim().replace(/^`+/, "").replace(/`+$/, "").trim();
}

function isMessageKey(text) {
  return stripCodeMarkers(text).startsWith(PREFIX);
}

/*
 * Return the real editor associated with an element.
 *
 * Some websites focus a child inside a contenteditable editor rather than
 * the contenteditable element itself.
 */
function findEditor(element) {
  if (!(element instanceof Element)) {
    return null;
  }

  return element.closest(
    'textarea, input, [contenteditable]:not([contenteditable="false"]), [role="textbox"]'
  );
}


// We do not want encrypted text currently inside the composer to be detected
// as an incoming/displayed GhostLayer message.
function isInsideEditor(node) {
  const element =
    node instanceof Element ? node : node.parentElement;

  if (!element) {
    return false;
  }

  if (
    activeEditor &&
    (
      element === activeEditor ||
      activeEditor.contains(element)
    )
  ) {
    return true;
  }

  return findEditor(element) !== null;
}

// Ignore elements created by GhostLayer itself.
function isGhostLayerUi(node) {
  const element =
    node instanceof Element ? node : node.parentElement;

  return Boolean(
    element?.closest(
      ".ghostlayer-decrypt-button, .ghostlayer-friend-selector"
    )
  );
}

// Add a Decrypt button to one displayed encrypted message.
// Cuz otherwise it would automatically decrypt all messages
// which we dont want to happen
function addDecryptButton(messageElement, onDecrypt) {
  if (!(messageElement instanceof Element)) {
    return;
  }

  if (seenMessages.has(messageElement)) {
    return;
  }

  if (
    messageElement.querySelector(
      ":scope > .ghostlayer-decrypt-button"
    )
  ) {
    seenMessages.add(messageElement);
    return;
  }

  seenMessages.add(messageElement);

  const button = document.createElement("button");

  button.className = "ghostlayer-decrypt-button";
  button.textContent = "Decrypt";
  button.type = "button";

  Object.assign(button.style, {
    marginLeft: "8px",
    padding: "4px 8px",
    border: "1px solid #555",
    borderRadius: "6px",
    background: "#111820",
    color: "white",
    cursor: "pointer",
    fontSize: "12px",
    position: "relative",
    zIndex: "999999",
  });

  button.addEventListener("click", async (event) => {
    event.preventDefault();
    event.stopPropagation();

    button.disabled = true;
    button.textContent = "Decrypting...";

    try {
      const success = await onDecrypt();
      // console.log("Decryption success: ", success);
      
      if (success) {
        button.remove();
      } else {
        button.disabled = false;
        button.textContent = "Decrypt";
      }
    } catch (error) {
      console.log("GhostLayer decryption failed:", error);

      button.disabled = false;
      button.textContent = "Decrypt";
    }
  });

  messageElement.appendChild(button);
}

// Replace text inside one text node.
function replaceTextInNode(node, targetText, newText) {
  if (
    node.nodeType === Node.TEXT_NODE &&
    node.nodeValue?.includes(targetText)
  ) {
    node.nodeValue = node.nodeValue.replace(
      targetText,
      newText
    );

    return true;
  }

  return false;
}

// Replace matching text inside all text nodes under an element.
function replaceTextInChildNodes( rootNode, targetText, newText) {
  if (!rootNode) {
    return false;
  }

  if (rootNode.nodeType === Node.TEXT_NODE) {
    return replaceTextInNode(
      rootNode,
      targetText,
      newText
    );
  }

  const walker = document.createTreeWalker(
    rootNode,
    NodeFilter.SHOW_TEXT
  );

  let currentNode;
  let replaced = false;

  while ((currentNode = walker.nextNode())) {
    if (
      replaceTextInNode(
        currentNode,
        targetText,
        newText
      )
    ) {
      replaced = true;
    }
  }

  return replaced;
}

// Handle one text node containing a possible GhostLayer message.
function processEncryptedTextNode(textNode) {
  if (!(textNode instanceof Text)) {
    return;
  }

  if (isInsideEditor(textNode)) {
    return;
  }

  if (isGhostLayerUi(textNode)) {
    return;
  }

  const text = textNode.nodeValue?.trim();

  if (!isMessageKey(text)) {
    return;
  }

  const messageKey = stripCodeMarkers(text);

  const messageElement = textNode.parentElement;

  if (!messageElement) {
    return;
  }

  // console.log(
    // "Displayed message with GhostLayer prefix found:",
    // text
  // );

  addDecryptButton(messageElement, async () => {
    const decryptionResult =
      await chrome.runtime.sendMessage({
        type: "DECRYPT_MESSAGE",
        messageKey: messageKey,
      });

    // console.log("Decrypted Result: ", decryptionResult);
    
    if (!decryptionResult?.success) {
      console.log(
        `Couldn't decrypt the text: ${messageKey}`
      );

      console.error(
        decryptionResult?.error ??
        "Unknown decryption error."
      );

      return false;
    }

    const replaced = replaceTextInChildNodes(
      messageElement,
      text,
      decryptionResult.display
    );

    if (!replaced) {
      console.error(
        "Decryption succeeded, but the displayed ciphertext could not be replaced."
      );

      return false;
    }

    return true;
  });
}

// Scan an added node and all text nodes beneath it.
function scanNode(node) {
  if (node.nodeType === Node.TEXT_NODE) {
    processEncryptedTextNode(node);
    return;
  }

  if (!(node instanceof Element)) {
    return;
  }

  if (isInsideEditor(node) || isGhostLayerUi(node)) {
    return;
  }

  const walker = document.createTreeWalker(
    node,
    NodeFilter.SHOW_TEXT
  );

  let textNode;

  while ((textNode = walker.nextNode())) {
    processEncryptedTextNode(textNode);
  }
}

// Watch for newly displayed encrypted messages.
const observer = new MutationObserver((mutations) => {
  for (const mutation of mutations) {
    if (
      mutation.type === "characterData" &&
      mutation.target
    ) {
      processEncryptedTextNode(mutation.target);
      continue;
    }

    for (const node of mutation.addedNodes) {
      scanNode(node);
    }
  }
});

observer.observe(document.body, {
  childList: true,
  subtree: true,
  characterData: true,
});


// Also scan messages that were already on the page before this content
// script started.
scanNode(document.body);

/*
 * Remember the last editor the user focused.
 *
 * Clicking GhostLayer's Encrypt button removes focus from the editor, so the
 * editor must be stored beforehand.
 */
document.addEventListener("focusin", (event) => {
  const target = event.target;

  if (!(target instanceof Element)) {
    return;
  }

  const editor = findEditor(target);

  if (editor) {
    activeEditor = editor;
  }
});


// Create the temporary floating Encrypt button.
const encryptButton = document.createElement("button");

encryptButton.textContent = "Encrypt";
encryptButton.type = "button";
encryptButton.className = "ghostlayer-encrypt-button";

Object.assign(encryptButton.style, {
  position: "fixed",
  right: "20px",
  bottom: "20px",
  zIndex: "999999",
  padding: "10px 16px",
  cursor: "pointer",
});

document.body.appendChild(encryptButton);

// Read the current editor text.
function getTextFromEditor(editor) {
  if (
    editor instanceof HTMLTextAreaElement ||
    editor instanceof HTMLInputElement
  ) {
    return editor.value;
  }

  if (
    editor instanceof HTMLElement &&
    editor.isContentEditable
  ) {
    return editor.textContent ?? "";
  }

  return "";
}

function pasteIntoEditor(editor, newText) {
  const data = new DataTransfer();
  data.setData("text/plain", newText);

  return editor.dispatchEvent(
    new ClipboardEvent("paste", {
      bubbles: true,
      cancelable: true,
      clipboardData: data,
    })
  );
}

// Some editors ignore a synthetic paste but still read beforeinput and
// update their own model from it. This is the modern editing event, so it
// is worth one attempt before giving up.
function beforeInputIntoEditor(editor, newText) {
  const data = new DataTransfer();
  data.setData("text/plain", newText);

  return editor.dispatchEvent(
    new InputEvent("beforeinput", {
      bubbles: true,
      cancelable: true,
      composed: true,
      inputType: "insertFromPaste",
      dataTransfer: data,
    })
  );
}

// Replace editor text and notify the website that input occurred.
async function replaceContentOfEditor(editor, newText) {

  console.log("GhostLayer editor:", editor);
  console.log("  tag/role/ce:", editor.tagName,
  editor.getAttribute("role"), editor.getAttribute("contenteditable"));
  console.log("  is the editable itself:",
  editor.closest('[contenteditable="true"]') === editor);
  console.log("  nested editable:",
  editor.querySelector('[contenteditable="true"]'));
  console.log("  activeElement before focus:", document.activeElement);
  
  editor.focus();

  console.log("  activeElement after focus:", document.activeElement);
  
  // Slate throttles its selection sync at about 100ms, so a zero delay is not
  // enough for it to notice the selection we set. It reads edits off its own
  // model selection, and a stale one makes it ignore the edit entirely.
  function nextTick(delay = 150) {
    return new Promise((resolve) => setTimeout(resolve, delay));
  }
  
  if (editor instanceof HTMLTextAreaElement) {
    const setter = Object.getOwnPropertyDescriptor(
      HTMLTextAreaElement.prototype,
      "value"
    )?.set;

    if (!setter) {
      return false;
    }

    setter.call(editor, newText);

    editor.dispatchEvent(
      new InputEvent("input", {
        bubbles: true,
        composed: true,
        inputType: "insertText",
        data: newText,
      })
    );
    return true;
    
  } else if (editor instanceof HTMLInputElement) {
    const setter = Object.getOwnPropertyDescriptor(
      HTMLInputElement.prototype,
      "value"
    )?.set;

    if (!setter) {
      return false;
    }

    setter.call(editor, newText);

    editor.dispatchEvent(
      new InputEvent("input", {
        bubbles: true,
        composed: true,
        inputType: "insertText",
        data: newText,
      })
    );
    return true;
    
  } else if (
    editor instanceof HTMLElement &&
    editor.isContentEditable
  ) {
    const selection = window.getSelection();

    if (!selection) {
      return false;
    }

    // Each attempt below consumes the selection, so it has to be put back
    // before the next one runs.
    async function selectEverything() {
      const range = document.createRange();

      range.selectNodeContents(editor);

      selection.removeAllRanges();
      selection.addRange(range);

      // So we can let selectionchange reach the editor's own state before
      // commanding it
      await nextTick();
    }

    // Slate reports success from execCommand while quietly ignoring the edit
    // in its own model. That leaves the encrypted text on screen and the
    // original one in the message the site actually sends, so execCommand is
    // useless here no matter what it returns. Its paste handler does reach the
    // model, so Slate skips ahead to that.
    const isSlate = editor.hasAttribute("data-slate-editor");

    console.log("GhostLayer editor is slate: ", isSlate);

    await selectEverything();

    if (!isSlate) {
      const inserted = document.execCommand( // deprecated so have to replace it soon, but it works for nwo
        "insertText",
        false,
        newText
      );

      console.log("GhostLayer execCommand insertText: ", inserted);

      // Firing has already happened above in the respective
      // event handlers, so we dont have to do it again
      if (inserted) {
        return true;
      }

      // execCommand was refused, so the selection it consumed has to go back
      // before the next attempt.
      await selectEverything();
    }

    // The paste path makes the editor think the user pasted something, which
    // runs the text through the editor's own handler instead of past it.
    pasteIntoEditor(editor, newText);
    await nextTick();

    console.log("GhostLayer paste worked: ", getTextFromEditor(editor).includes(newText));

    if (getTextFromEditor(editor).includes(newText)) {
      return true;
    }

    // Last attempt, the editing event the paste path is built on top of.
    await selectEverything();
    beforeInputIntoEditor(editor, newText);
    await nextTick();

    console.log("GhostLayer beforeinput worked: ", getTextFromEditor(editor).includes(newText));

    if (getTextFromEditor(editor).includes(newText)) {
      return true;
    }

    // A failed paste can still eat the selection without inserting anything,
    // which would leave the box empty and lose what the user typed.
    console.log("GhostLayer editor text after all attempts: ", getTextFromEditor(editor));

    // Nothing reached the editor's own model. Writing the DOM directly here
    // would show the encrypted text while the site keeps sending the original
    // one, so the editor is left untouched and the caller is told it failed.
    return false;
  }
  return false;
}

encryptButton.addEventListener("mousedown", (event) => {
  event.preventDefault();
});
// Main encryption flow.
encryptButton.addEventListener("click", async () => {
  if (!activeEditor) {
    console.error("No active editor found!");
    return;
  }

  const plaintext = getTextFromEditor(activeEditor);

  if (!plaintext.trim()) {
    console.error("Textbox is empty!");
    return;
  }

  const friendsResult =
    await chrome.runtime.sendMessage({
      type: "GET_FRIENDS",
    });

  if (!friendsResult?.success) {
    console.error(
      friendsResult?.error ??
      "Could not load friends."
    );

    return;
  }

  const friends = friendsResult.friends ?? [];

  if (friends.length === 0) {
    console.error("No friends found.");
    return;
  }

  const selectedFriend =
    await showFriendsSelector(friends);

  if (!selectedFriend) {
    return;
  }

  const encryptionResult =
    await chrome.runtime.sendMessage({
      type: "ENCRYPT_MESSAGE",
      plaintext,
      publicKey: selectedFriend.public_key,
    });

  if (!encryptionResult?.success) {
    console.error(
      encryptionResult?.error ??
      encryptionResult?.display ??
      "Encryption failed."
    );

    return;
  }

  const replaced = await replaceContentOfEditor(
    activeEditor,
    formatMessageKey(encryptionResult.messageKey)
  );
  // to get something like `message_key_here`

  if (!replaced) {
    console.error(
      "GhostLayer: could not insert the encrypted text safely. " +
      "Do NOT press send as this site may still send your original message."
    );

    // A real Ctrl+V is a trusted paste, so every editor accepts it even when
    // none of the scripted attempts above got through. Putting the key on the
    // clipboard leaves the user a way to finish by hand.
    try {
      const formatedMessageKey = formatMessageKey(encryptionResult.messageKey);
      await navigator.clipboard.writeText(
        formatedMessageKey
      );

      console.error(
        "GhostLayer: copy this by hand:" +
        formatedMessageKey
      );
    } catch (error) {
      // The clipboard needs a recent user gesture and the friend selector can
      // outlast it, so the key is printed here rather than lost.
      console.error(
        "GhostLayer: could not reach the clipboard either.",
        error
      );
      console.error("GhostLayer: copy this by hand:", encryptionResult.messageKey);
      console.error("GhostLayer: your original text was:", plaintext);
    }
  }
});

// Display the friend-selection overlay.
function showFriendsSelector(friends) {
  return new Promise((resolve) => {
    const overlay = document.createElement("div");

    overlay.className = "ghostlayer-friend-selector";

    // mousedown is what moves focus, and preventDefault on it here covers
    // every child too, so the composer never loses its caret while the user
    // picks a friend.
    overlay.addEventListener("mousedown", (event) => {
      event.preventDefault();
    });

    Object.assign(overlay.style, {
      position: "fixed",
      inset: "0",
      zIndex: "1000000",
      background: "rgba(0, 0, 0, 0.65)",
      display: "flex",
      alignItems: "center",
      justifyContent: "center",
    });

    const selectorBox =
      document.createElement("div");

    Object.assign(selectorBox.style, {
      width: "320px",
      maxHeight: "400px",
      overflowY: "auto",
      padding: "18px",
      borderRadius: "12px",
      background: "#111820",
      color: "white",
      fontFamily: "sans-serif",
    });

    const title = document.createElement("div");

    title.textContent = "Encrypt for";
    title.style.marginBottom = "14px";
    title.style.fontWeight = "bold";

    selectorBox.appendChild(title);

    for (const friend of friends) {
      const friendButton =
        document.createElement("button");

      friendButton.type = "button";

      friendButton.textContent =
        friend.nickname ||
        friend.key_id ||
        "Unknown friend";

      Object.assign(friendButton.style, {
        display: "block",
        width: "100%",
        marginBottom: "8px",
        padding: "10px",
        border: "1px solid #33404d",
        borderRadius: "8px",
        background: "#1a242e",
        color: "white",
        textAlign: "left",
        cursor: "pointer",
      });

      friendButton.addEventListener("mousedown", (event) => {
        event.preventDefault();
      });
      
      friendButton.addEventListener(
        "click",
        (event) => {
          event.preventDefault();
          event.stopPropagation();

          overlay.remove();
          resolve(friend);
        }
      );

      selectorBox.appendChild(friendButton);
    }

    const cancelButton =
      document.createElement("button");

    cancelButton.type = "button";
    cancelButton.textContent = "Cancel";

    Object.assign(cancelButton.style, {
      width: "100%",
      marginTop: "8px",
      padding: "10px",
      border: "none",
      background: "transparent",
      color: "#aaa",
      cursor: "pointer",
    });

    cancelButton.addEventListener("mousedown", (event) => {
      event.preventDefault();
    });
    
    cancelButton.addEventListener(
      "click",
      (event) => {
        event.preventDefault();
        event.stopPropagation();

        overlay.remove();
        resolve(null);
      }
    );

    selectorBox.appendChild(cancelButton);
    overlay.appendChild(selectorBox);
    document.body.appendChild(overlay);

    overlay.addEventListener("click", (event) => {
      if (event.target === overlay) {
        overlay.remove();
        resolve(null);
      }
    });
  });
}

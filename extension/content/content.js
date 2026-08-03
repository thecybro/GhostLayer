// This file talks to the background worker using:
// await chrome.runtime.sendMessage({ ... })

const PREFIX = "ghl";

let activeEditor = null;

// Prevent adding multiple Decrypt buttons to the same displayed message.
const seenMessages = new WeakSet();

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

      if (success) {
        button.remove();
      } else {
        button.disabled = false;
        button.textContent = "Decrypt";
      }
    } catch (error) {
      console.error("GhostLayer decryption failed:", error);

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

  if (!text?.includes(PREFIX)) {
    return;
  }

  const messageElement = textNode.parentElement;

  if (!messageElement) {
    return;
  }

  console.log(
    "Displayed message with GhostLayer prefix found:",
    text
  );

  addDecryptButton(messageElement, async () => {
    const decryptionResult =
      await chrome.runtime.sendMessage({
        type: "DECRYPT_MESSAGE",
        messageKey: text,
      });

    if (!decryptionResult?.success) {
      console.log(
        `Couldn't decrypt the text: ${text}`
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

// Replace editor text and notify the website that input occurred.
function replaceContentOfEditor(editor, newText) {
  editor.focus();

  if (editor instanceof HTMLTextAreaElement) {
    const setter = Object.getOwnPropertyDescriptor(
      HTMLTextAreaElement.prototype,
      "value"
    )?.set;

    if (!setter) {
      return false;
    }

    setter.call(editor, newText);
  } else if (editor instanceof HTMLInputElement) {
    const setter = Object.getOwnPropertyDescriptor(
      HTMLInputElement.prototype,
      "value"
    )?.set;

    if (!setter) {
      return false;
    }

    setter.call(editor, newText);
  } else if (
    editor instanceof HTMLElement &&
    editor.isContentEditable
  ) {
    const selection = window.getSelection();

    if (!selection) {
      return false;
    }

    const range = document.createRange();

    range.selectNodeContents(editor);

    selection.removeAllRanges();
    selection.addRange(range);

    const inserted = document.execCommand( // deprecated so have to replace it soon, but it works for nwo
      "insertText",
      false,
      newText
    );

    if (!inserted) {
      editor.textContent = newText;
    }
  } else {
    return false;
  }

  editor.dispatchEvent(
    new InputEvent("input", {
      bubbles: true,
      composed: true,
      inputType: "insertText",
      data: newText,
    })
  );

  return true;
}

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

  const replaced = replaceContentOfEditor(
    activeEditor,
    encryptionResult.messageKey
  );

  if (!replaced) {
    console.error(
      "The message was encrypted, but GhostLayer could not replace the editor content."
    );
  }
});

// Display the friend-selection overlay.
function showFriendsSelector(friends) {
  return new Promise((resolve) => {
    const overlay = document.createElement("div");

    overlay.className = "ghostlayer-friend-selector";

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

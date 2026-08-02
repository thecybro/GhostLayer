// This file talks to the background worker using:
// await chrome.runtime.sendMessage({ ... })

let activeEditor = null;
const seenMessages = new WeakSet();

const observer = new MutatioObserver((mutations) => {
  for (const mutation of mutations) {
    for (const node of mutation.addedNodes) {
      if (!(node instanceof HTMLElement)) continue;

      // This is different for each platform, so have to find and replace with appropriate
      // selectors for each platform
      const messages = node.matches?.['data-message-id']
        ? [node]
        : node.querySelectorAll("['data-message-id']") ?? [];

      for (const message of messages) {
        if (seenMessages.has(message)) continue;
        seenMessages.add(message);

        const text = message.innerText?.trim();

        if (text) {
          console.log(`Incoming message ${text} found`);

          if (text.startsWith("ghl_message")) {
            // decoding logic will be added here
          }
          
          // TODO: Send the text to decrypt it
          // First show a small decrypt button alongside the
          // text, send only when it's clicked
          // 
          // const decryptedText = await chrome.runtime.SendMessage({
          //  type: "DECRYPT_MESSAGE"
          // })
          // 
          
        }
      }
    }
  }
});

observer.observe(document.body, {
  childList: true,
  subtree: true,
});

// Remember the last text editor the user focused.
// Clicking the Encrypt button removes focus from the editor,
// so we store the editor beforehand.
document.addEventListener("focusin", (event) => {
  const element = event.target;

  if (
    element instanceof HTMLTextAreaElement ||
    element instanceof HTMLInputElement ||
    element.isContentEditable
  ) {
    activeEditor = element;
  }
});


// Create a temporary floating Encrypt button.
// Later, this can be positioned beside the site's message editor.
const encryptButton = document.createElement("button");
encryptButton.textContent = "Encrypt";

Object.assign(encryptButton.style, {
  position: "fixed",
  right: "20px",
  bottom: "20px",
  zIndex: "999999",
  padding: "10px 16px",
  cursor: "pointer",
});

document.body.appendChild(encryptButton);


// Read the current text from an editor.
function getTextFromEditor(editor) {
  if (
    editor instanceof HTMLTextAreaElement ||
    editor instanceof HTMLInputElement
  ) {
    return editor.value;
  }

  if (editor.isContentEditable) {
    return editor.textContent ?? "";
  }

  return "";
}


// Replace the editor text and tell the website that an input occurred.
function replaceContentOfEditor(editor, newText) {
  if (
    editor instanceof HTMLTextAreaElement ||
    editor instanceof HTMLInputElement
  ) {
    editor.value = newText;
  } else if (editor.isContentEditable) {
    editor.textContent = newText;
  }

  editor.dispatchEvent(
    new Event("input", {
      bubbles: true,
    })
  );
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

  // Ask the background worker for the stored friends.
  const friendsResult = await chrome.runtime.sendMessage({
    type: "GET_FRIENDS",
  });

  if (!friendsResult.success) {
    console.error(friendsResult.error);
    return;
  }

  if (friendsResult.friendsLength === 0) {
    console.error("No friends found.");
    return;
  }

  // Wait until the user selects one friend.
  const selectedFriend = await showFriendsSelector(
    friendsResult.friends
  );

  // Cancel or clicking outside returns null.
  if (!selectedFriend) {
    return;
  }

  // console.log("From content.js:");
  // console.log({
  //   plaintext,
  //   selectedFriend,
  //   selectedFriend: selectedFriend.public_key
  // });
  
  // Send the plaintext and selected friend's public key
  // to the background worker.
  const encryptionResult = await chrome.runtime.sendMessage({
    type: "ENCRYPT_MESSAGE",
    plaintext,
    publicKey: selectedFriend.public_key,
  });

  if (!encryptionResult.success) {
    console.error(
      encryptionResult.error ?? encryptionResult.display
    );
    return;
  }

  
  // Use the complete outgoing GhostLayer message.
  // The message key ghl_message that we got
  replaceContentOfEditor(
    activeEditor,
    encryptionResult.messageKey
  );
});

// decryptionButton.addEventListener("click", async () => {
  
// }

function showFriendsSelector(friends) {
  return new Promise((resolve) => {
    const overlay = document.createElement("div");

    Object.assign(overlay.style, {
      position: "fixed",
      inset: "0",
      zIndex: "1000000",
      background: "rgba(0, 0, 0, 0.65)",
      display: "flex",
      alignItems: "center",
      justifyContent: "center",
    });

    const selectorBox = document.createElement("div");

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

    // Create one selectable button per friend.
    for (const friend of friends) {
      const friendButton = document.createElement("button");

      friendButton.textContent =
        friend.nickname || friend.key_id;

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

      friendButton.addEventListener("click", () => {
        overlay.remove();
        resolve(friend);
      });

      selectorBox.appendChild(friendButton);
    }

    const cancelButton = document.createElement("button");
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

    cancelButton.addEventListener("click", () => {
      overlay.remove();
      resolve(null);
    });

    selectorBox.appendChild(cancelButton);
    overlay.appendChild(selectorBox);

    // Append elements to body, not directly to document.
    document.body.appendChild(overlay);

    // Clicking outside the selector closes it.
    overlay.addEventListener("click", (event) => {
      if (event.target === overlay) {
        overlay.remove();
        resolve(null);
      }
    });
  });
}

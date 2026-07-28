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

  // need the friends list so that user can select for which friend to encrypt
  // the message
  const friendsResult = await chrome.runtime.sendMessage({
    type: "GET_FRIENDS"
  })

  if (!friendsResult.success) {
    console.error(friendsResult.error);
    return;
  }

  if (friendsResult.friends.length === 0) {
    console.error("No friends found");
    return;
  }

  const selectedFriend = await showFriendsSelector(
    friendsResult.friends
  );
  
  const encryptionResult = await chrome.runtime.sendMessage({
    type: "ENCRYPT_MESSAGE",
    plaintext,
    friendPublicKey: selectedFriend.public_key,
  })

  if (!encryptionResult.success) {
    console.error(encryptionResult.error ?? encryptionResult.display);
  }

    replaceContentOfEditor(activeEditor, result.encryptedText);
})

function showFriendsSelector(friends) {
  return new Promise((resolve) => {
    // Dark full-page layer behind the selector.
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
    title.textContent = "Encrypt for ";
    title.style.marginBottom = "14px";
    title.style.fontWeight = "bold";

    selectorBox.appendChild(title);

    for (const friend of friends) {
      const friendButton = document.createElement("div");

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
    };

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
    document.appendChild(overlay);

    // clicking the dark area outside of selectorBox also closes it
    overlay.addEventListener("click", (event) => {
      if (event.target === overlay) {
        overlay.remove();
        resolve(null);
      }
    });
  });
}


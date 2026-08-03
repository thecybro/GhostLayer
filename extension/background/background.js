import {
  encryptMessage,
  decryptMessage,
  loadDisplayData
} from "./index.js";

chrome.runtime.onMessage.addListener(
  (message, sender, sendResponse) => {
    if (message.type === "GET_FRIENDS") {
      handleGetFriends(sendResponse);
      return true;
    }

    if (message.type === "ENCRYPT_MESSAGE") {
      handleEncryptMessage(message, sendResponse);
      return true;
    }

    if (message.type === "DECRYPT_MESSAGE") {
      handleDecryptMessage(message, sendResponse);
    }
  }
);

async function handleGetFriends(sendResponse) {
  try {
    const result = await loadDisplayData();

    if (result.has_identity) {
      sendResponse({
        success: true,
        friends: result.friends,
        friendsLength: result.friend_index.length
      });
    } else {
      sendResponse({
        success: false,
        error: "No identity found!"
      });
    }
  } catch (error) {
    sendResponse({
      success: false,
      error: error instanceof Error
        ? error.message
        : String(error)
    });
  }
}

async function handleEncryptMessage(message, sendResponse) {
  try {
    // console.log("From background.js: ");
    // console.log(message);
    
    const result = await encryptMessage(
      message.publicKey,
      message.plaintext
    );

    // console.log("From  background.js after encryption:");
    // console.log(result);
    
    sendResponse(result);
  } catch (error) {
    sendResponse({
      success: false,
      status: "error",
      display: "Encryption failed!",
      error: error instanceof Error
        ? error.message
        : String(error),
      messageKey: null
    });
  }
}

async function handleDecryptMessage(message, sendResponse) {
  try {
    const result = decryptMessage(
      message.messageKey
    );

    if (result.success) {
      sendResponse({
        success: true,
        display: result.display
      })
      
    } else {
      sendResponse({
        success: false,
        error: result.error
      });
    }
  } catch (error) {
    sendResponse({
      success: false,
      error: error instanceof Error
        ? error.message
        : String(error)
    });
  }
}

// This file with talk to content worker via:
// 
// chrome.runtime.onMessage.addListener(
// (message, sender, sendResponse) => { 
// code here
// })
// 

import {
  encryptMessage,
  decryptMessage,

  loadDisplayData
} from "./index.js";

chrome.runtime.onMessage.addListener(
  (message, sender, sendResponse) => {
    if (message.type === "GET_FRIENDS") {
      loadDisplayData()
        .then((data) => {
          sendResponse({
            success: true,
            friends: data.friends,
          });
        })
        .catch((error) => {
          sendResponse({
            success: false,
            error: String(error),
          });
        });
      return true;
    }

    if (message.type === "ENCRYPT_MESSAGE") {
      encryptMessage(message.public_key, plaintext)
        .then((result) => {
          sendResponse(result)
        })
        .catch((error) => {
          sendResponse({
            success: false,
            status: "error",
            display: "Encryption failed!",
            error: error instanceof Error
              ? error.message
              : String(error),
            messageKey: null,
          });
        });
      return true;
    }

  }
)
import init, {
  create_identity,
  add_friend,
  load_display_data,
  copy_to_clipboard,

  encrypt,
  decrypt,
} from "../pkg/ghost.js";

import {
  saveToStorage,
  loadFromStorage
} from "../storage/storage.js";

export const wasmReady = init();

export function save(key, jsonString) {
  return saveToStorage(key, jsonString);
}

export function load(key = null) {
  return loadFromStorage(key);
}

export async function createIdentity(username) {
  await wasmReady;

  const result = JSON.parse(create_identity(username));

  if (result.success) {
    for (const write of result.write) {
      await saveToStorage(write.key, write.value);
    }
  }

  return {
    display: result.display,
    status: result.success ? "success" : "error"
  };
}

export async function addFriend(nickname, inviteKey) {
  await wasmReady;

  const stored = await loadFromStorage("friend_index");
  const currentIndexJson = stored.friend_index ?? "[]";

  const result = JSON.parse(
    add_friend(nickname, inviteKey, currentIndexJson)
  );

  if (result.success) {
    for (const write of result.write) {
      await saveToStorage(write.key, write.value);
    }
  }

  return {
    display: result.display,
    status: result.success ? "success" : "error"
  };
}

export async function copyToClipboard(item) {
  await wasmReady;

  const all = await loadFromStorage(null);
  const storageJson = JSON.stringify(all);

  try {
    const resultRaw = await copy_to_clipboard(storageJson, item);
    const result = JSON.parse(resultRaw);

    return {
      display: result.display,
      status: result.success ? "success" : "error"
    };
  } catch (err) {
    return {
      display: err instanceof Error ? err.message : String(err),
      status: "error"
    };
  }
}

// the public key it returns is our public public_key
// to ge the public key of friend, we do friend.public_key
export async function loadDisplayData() {
  await wasmReady;

  const all = await loadFromStorage(null);
  const storageJson = JSON.stringify(all);

  return JSON.parse(load_display_data(storageJson));
}

async function getIdentity() {
  const stored = await loadFromStorage("identity");

  if (!stored.identity) {
    return null;
  }
  try {
    return JSON.parse(stored.identity);
  } catch {
    throw new err("Stored identity contains invalid JSON");
   }
}

export async function encryptMessage(their_public_b64, message) {
  const identity = await getIdentity();

  if (!identity) {
    return {
      succcess: false,
      status: "error",
      display: "No identity found!"
    };
  }

  
  const my_private_b64 = identity.private_key;
  const my_public_b64 = identity.public_key;
  
  const result = JSON.parse(
    encrypt(my_public_b64, my_private_b64, their_public_b64, message)
  );

  return {
      display: result.display,
      success: result.success,
      status: result.success ? "success" : "error",
  
      // Keep the real Rust error available for debugging.
      error: result.error ?? null,
  
      // Assuming message_key contains the complete outgoing encrypted message.
      messageKey: result.message_key ?? null,
    };
}

export async function decryptMessage(their_public_b64, nonce, ciphertext) {
  const identity = await getIdentity();

  if (!identity) {
    return {
      success: false,
      status: "error",
      display: "No identity found!"
    };
  }
  
  const my_private_b64 = identity.private_key;

  const result = JSON.parse(
    decrypt(my_private_b64, their_public_b64, nonce, ciphertext)
  );

  return {
      display: result.display,
      success: result.success,
      status: result.success ? "success" : "error",
  
      error: result.error ?? null,
  
      messageKey: result.message_key ?? null,
    };
}

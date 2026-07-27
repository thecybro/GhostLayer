import init, {
  create_identity,
  add_friend,
  load_display_data,
  copy_to_clipboard
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

export async function loadDisplayData() {
  await wasmReady;

  const all = await loadFromStorage(null);
  const storageJson = JSON.stringify(all);

  return JSON.parse(load_display_data(storageJson));
}


export async function saveToStorage(key, jsonString) {
  await chrome.storage.local.set({ [key]: jsonString });
}
export async function loadFromStorage(key = null) {
  return await chrome.storage.local.get(key);
}
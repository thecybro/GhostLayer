
export async function saveToStorage(key, jsonString) {
  await chrome.storage.local.set({ [key]: jsonString });
}
export async function loadFromStorage(key) {
  const result = await chrome.storage.local.get(key);
  return result[key] ?? null;
}

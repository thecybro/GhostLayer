
export async function saveToStorage(key, jsonString) {
  await chrome.storage.local.set({ [key]: jsonString });
}
export async function loadFromStorage(key = null) {
    const result = await chrome.storage.local.get(key);

    if (key === null) {
        return result;
    }

    return result[key] ?? null;
}
import init, { create_identity, add_friend } from "./pkg/ghost.js";

function notify(text, type = "") {
  const el = document.getElementById("notification");
  el.textContent = text;
  el.className = "notification show" + (type ? " " + type : "");
  setTimeout(() => el.classList.remove("show"), 3000);
}

async function main() {
  await init();

  document.getElementById("create-identity-btn").addEventListener("click", async () => {
    const result = JSON.parse(create_identity());

    if (result.success) {
      for (const w of result.write) {
        await chrome.storage.local.set({ [w.key]: w.value });
      }
      notify(`Identity "${result.display}" has been created!` , "success");
    } else {
      notify(result.error, "error");
    }
  });

  document.getElementById("add-friend-btn").addEventListener("click", async () => {
    const nickname = document.getElementById('friend-nickname').value || null;
    const publicKey = document.getElementById('friend-pubkey').value;

    const stored = await chrome.storage.local.get("friend_index");
    const currentIndexJson = stored.friend_index ?? "[]";

    const result = JSON.parse(add_friend(nickname, publicKey, currentIndexJson));

    if (result.success) {
      for (const w of result.write) {
        await chrome.storage.local.set({ [w.key]: w.value });
      }
      notify(`Friend "${result.display}" has been added!`, "success");
    } else {
      notify(result.error, "error");
    }
  });
}

main();
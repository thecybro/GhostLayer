import init, { create_identity, add_friend, load_display_data } from "./pkg/ghost.js";

function notify(text, type = "") {
  const el = document.getElementById("notification");
  el.textContent = text;
  el.className = "notification show" + (type ? " " + type : "");
  setTimeout(() => el.classList.remove("show"), 3000);
}

async function loadUI() {
  const all = await chrome.storage.local.get(null);
  const storageJson = JSON.stringify(all);

  // console.log(storageJson);
  
  const result = JSON.parse(load_display_data(storageJson));

  if (result.has_identity) {
    document.getElementById("create-identity-btn").style.display = "none";
    document.getElementById("key-id").textContent = result.identity_key_id;
  }

  const friendsList = document.getElementById("friends-list");
  friendsList.innerHTML = "";

  if (result.friends.length === 0) {
    friendsList.innerHTML = '<div class="empty-state">No friends</div>';
  } else {
    let friendIndex = 1;
    for (const friend of result.friends) {
      const div = document.createElement("div");
      div.className = "friend-item";
      // nickname (optional), pubilc_key, and key_id are available.
      // public_key is too long to display, so
      // key_id is what we prepared to show if nickname isnot available
      div.innerHTML = `
        <span class="friend-number">${friendIndex}</span>
        <span class="friend-name">${friend.nickname}</span>
        <span class="friend-key">${friend.key_id}
        `;
      // div.textContent = friend.nickname || friend.key_id; 
      friendsList.appendChild(div);
      friendIndex += 1;
    }
  }
}

async function main() {
  await init();
  await loadUI();
  
  document.getElementById("create-identity-btn").addEventListener("click", async () => {
    const result = JSON.parse(create_identity());

    if (result.success) {
      for (const w of result.write) {
        await chrome.storage.local.set({ [w.key]: w.value });
        // console.log(w.value);
      }
      notify(`Identity "${result.display}" has been created!` , "success");
    } else {
      notify(result.error, "error");
    }
    loadUI();
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
    loadUI();
  });
}

main();
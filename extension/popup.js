import init, { create_identity, add_friend, load_display_data, copy_to_clipboard } from "./pkg/ghost.js";

function notify(text, type = "") {
  const el = document.getElementById("notification");
  el.textContent = text;
  el.className = "notification show" + (type ? " " + type : "");
  setTimeout(() => el.classList.remove("show"), 3000);
}

function isEmpty(value) {
  return value == null || (typeof value === "string" && value.trim() === "");
}   

async function loadUI() {
  const all = await chrome.storage.local.get(null);
  const storageJson = JSON.stringify(all);

  // console.log(storageJson);
  
  const result = JSON.parse(load_display_data(storageJson));

  if (result.has_identity) {
    const username = document.getElementById("username");
    const username_input = document.getElementById("username-input");
    
    document.getElementById("create-identity-btn").style.display = "none";
    document.getElementById("username").textContent = result.username;
    document.getElementById("username-input").style.display = "none";

    if (isEmpty(result.username)) {
      document.getElementById("key-label").textContent = "KeyID";
      document.getElementById("key-id").textContent = result.identity_key_id;
    } else {
      document.getElementById("key-label").textContent = "Name:";
      document.getElementById("key-id").textContent = result.username;
    }
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
    const username = document.getElementById("username-input").value;
    const result = JSON.parse(create_identity(username));
    // console.log(result.username);

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

  // Copies your public_key to clipboard so that you can send it to your friend
  document.getElementById("copy-invite-btn").addEventListener("click", async () => {
    const text = await copy_to_clipboard();
    notify(`Copied "${text}" to clipboard!`, "success");
  });
}

main();
import init, { create_identity, add_friend, load_display_data, copy_to_clipboard } from "./background/background.js";
import { saveToStorage, loadFromStorage } from "./storage/storage.js";

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
  const all = await loadFromStorage(null);
  const storageJson = JSON.stringify(all);
    
  const result = JSON.parse(load_display_data(storageJson));
  // console.log(`Has_identity: ${result.has_identity}`);

  if (result.has_identity) {
    // const username = document.getElementById("username");
    // const username_input = document.getElementById("username-input");
    
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
    // console.log(result.display);

    if (result.success) {
      for (const w of result.write) {
        // await chrome.storage.local.set({ [w.key]: w.value });
        await saveToStorage(w.key, w.value);
        // console.log(`Set value: ${w.key}, ${w.value}`)
        // console.log(w.value);
      }
      notify(`Identity "..(see console)" has been created!`, "success");
      console.log(`Identity "${result.display}" has been created!`);
    } else {
      notify(result.error, "error");
    }
    loadUI();
  });

  document.getElementById("add-friend-btn").addEventListener("click", async () => {
    const nickname = document.getElementById('friend-nickname').value || null;
    // const publicKey = document.getElementById('friend-pubkey').value;
    const inviteKey = document.getElementById('friend-pubkey').value;
    
    // const stored = await chrome.storage.local.get("friend_index");
    const stored = await loadFromStorage("friend_index"); // loadFromStorage is fixed, maybe?
    const currentIndexJson = stored.friend_index ?? "[]";

    const result = JSON.parse(add_friend(nickname, inviteKey, currentIndexJson));

    if (result.success) {
      for (const w of result.write) {
        // await chrome.storage.local.set({ [w.key]: w.value });
        await saveToStorage(w.key, w.value);
      }
      notify(`Friend "${result.display}" has been added!`, "success");
      // console.log(`Friend "${result.display}" has been added!`);
    } else {
      notify(result.error, "error");
    }
    loadUI();
  });

  // Copies your public_key to clipboard so that you can send it to your friend
  // copy_to_clipboard() also allows to copy username, and properties of friends,
  // but we haven't implemented that logic yet and probably wont until we see
  // real usecase for it.
  document.getElementById("copy-invite-btn").addEventListener("click", async () => {
    const all = await loadFromStorage(null);
    const storageJson = JSON.stringify(all);
    // console.log("storageJson: ", storageJson);
    try{
      let result_raw = await copy_to_clipboard(storageJson, "invite_key");
      const result = JSON.parse(result_raw);
      // console.log("Result: ", result);
      // console.log("result.success", result.success);
      if (result.success) {
        notify(`Copied invite key "${result.display.slice(0, 5)}.." to clipboard!`, "success");
        console.log(`Copied invite key "${result.display}" to clipboard!`);
        // console.log("In result.success block right now!");
        // console.log(result.display);
      } else {
        // console.log("in result.error block of else right now")
        notify(result.error, "error");
      }
    } catch (err) {
      // console.log("In err block of catch right now")
      notify(err, "error");
    }
  });

  document.getElementById("settings-link").addEventListener("click", () => {
    window.location.href = "tests/test.html";
  });
}

main();
import {
  wasmReady,
  loadDisplayData,
  createIdentity,
  addFriend,
  copyToClipboard
} from "./background/index.js";


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
  const result = await loadDisplayData();

  // console.log(result);
  
  if (result.has_identity) {
    document.getElementById("create-identity-btn").style.display = "none";
    document.getElementById("username").textContent = result.username ?? "";
    document.getElementById("username-input").style.display = "none";

    if (isEmpty(result.username)) {
      document.getElementById("key-label").textContent = "KeyID";
      document.getElementById("key-id").textContent =
        result.identity_key_id;
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

      div.innerHTML = `
        <span class="friend-number">${friendIndex}</span>
        <span class="friend-name">${friend.nickname || friend.key_id}</span>
        <span class="friend-key">${friend.key_id}</span>
      `;

      friendsList.appendChild(div);
      friendIndex += 1;
    }
  }
}

async function main() {
  await wasmReady;
  await loadUI();

  document.getElementById("create-identity-btn").addEventListener("click", async () => {
      const username = document.getElementById("username-input").value.trim() || null;

      const result = await createIdentity(username);

      notify(result.display, result.status);
      await loadUI();
    });

  document.getElementById("add-friend-btn").addEventListener("click", async () => {
      const nickname = document.getElementById("friend-nickname").value.trim() || null;

      const inviteKey = document.getElementById("friend-pubkey").value.trim();

      const result = await addFriend(nickname, inviteKey);

      notify(result.display, result.status);
      await loadUI();
    });

  document.getElementById("copy-invite-btn").addEventListener("click", async () => {
      const result = await copyToClipboard("invite_key");
      notify(result.display, result.status);
    });

  document.getElementById("settings-link").addEventListener("click", () => {
      window.location.href = "tests/test.html";
    });
}

main().catch(console.error);
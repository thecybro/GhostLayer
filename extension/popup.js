import init, { create_identity, add_friend } from "./pkg/ghost.js";

function notify(text, type = "") {
  const el = document.getElementById("notification");
  el.textContent = text;
  el.className = "notification show" + (type ? " " + type : "");
  setTimeout(() => el.classList.remove("show"), 3000);
}

async function main() {
    await init();
    document.getElementById("create-identity-btn").addEventListener("click", () => {
      notify(create_identity(), "success");
    });
    document.getElementById("add-friend-btn").addEventListener("click", () => {
      notify(add_friend(), "success");
    });
}
  // Chrome Storage:
  // This function stores
  // Private Key
  // Public Key
  // Friends
  // Settings
  
  // Now popup changes.
main();
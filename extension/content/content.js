async function encryptCurrentMessage() {
    const textarea = document.querySelector("textarea");

    if (!textarea || textarea.value.trim() === "") {
        return;
    }

    const response = await chrome.runtime.sendMessage({
        type: "encrypt",

        // Temporary.
        // Later these will come from storage/background.
        myPrivateKey: "...",
        theirPublicKey: "...",

        plaintext: textarea.value,
    });

    if (!response.success) {
        alert(response.error);
        return;
    }

    textarea.value = response.display;
}

const button = document.createElement("button");
button.textContent = "Encrypt";
button.addEventListener("click", encryptCurrentMessage);

document.body.appendChild(button);

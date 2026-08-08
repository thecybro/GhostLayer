# GhostLayer

End to end encryption for chat platforms that do not have it.

See ***[LIMITATIONS.md](LIMITATIONS.md)*** to know current limitations before you use it.

**GhostLayer** is a Chrome extension. You type a message, pick a friend, and it is
replaced with a ciphertext blob before it ever reaches the platform. Your friend
clicks **Decrypt** and reads it. Discord, Slack, and everyone in between see only
the blob.

Keys are generated on your machine and never leave it. There is no GhostLayer
account, no GhostLayer server, and nothing to sign up for.

The cryptography is X25519 key agreement with ChaCha20-Poly1305, compiled to
WebAssembly from Rust.

---

## Status

**Version 0.1.0. Alpha. Not audited.**

The end to end flow works and has been tested on four platforms. The
cryptography is built from well regarded primitives, but the way they are
composed has had no external review.

---

## What GhostLayer does not protect against

**Read [LIMITATIONS.md](LIMITATIONS.md) before you trust GhostLayer with
anything that matters.** It is the single source of truth for what this version
cannot do, and it explains why in each case. The short version:

1. A recipient can forge messages that look like they came from you
2. Long lived, unencrypted private keys, so no forward secrecy
3. Who is talking to whom is not hidden
4. You can lose access to your own sent messages
5. Nothing is configurable
6. The protocol and the platform support are both early

None of these are bugs. They are the shape of the current design, written down
so you can decide what GhostLayer is good enough for. Use it to keep ordinary
conversation away from platform side scanning. Do not use it where being wrong
gets someone hurt.

---

## Install

### From the Chrome Web Store

Get it from chrome web store if exists: [GhostLayer](#)

### From source

```bash
git clone https://github.com/thecybro/GhostLayer.git
cd GhostLayer
./build.sh
```

Then open `chrome://extensions`, turn on Developer mode, click **Load unpacked**,
and select the `extension/` directory.

Building requires a Rust toolchain and [`wasm-pack`](https://rustwasm.github.io/wasm-pack/).
`build.sh` compiles the `ghost` crate to WebAssembly and copies the output into
`extension/pkg/`.

---

## Usage

**1. Create an identity.** Open the extension popup, optionally enter a username,
and click Create Identity. This generates your key pair locally.

**2. Share your invite.** Click Copy Invite. You get a string like
`ghl:inv:1:<public key><username>`. Send it to a friend over any channel. It
contains your public key and nothing sensitive.

**3. Add a friend.** Paste their invite key into the popup, give them a nickname,
click Add Friend.

**4. Encrypt.** Type your message in any chat box, click the floating Encrypt
button, pick a friend. Your text is replaced with the message key. Send it
normally.

**5. Decrypt.** A Decrypt button appears next to incoming GhostLayer messages.
Click it. Decryption is never automatic, so you choose what to reveal and when.

You can decrypt your own sent messages too, as long as the recipient is still in
your friend list.

---

## Supported platforms

Tested and working:

- Discord
- Slack
- X
- Messenger

Everything else is untested rather than unsupported. GhostLayer works with plain
textareas and with rich-text editors that keep their own document model, so a
platform outside this list may work fine. It may also silently fail to insert
text, in which case the extension refuses to report success and drops the
ciphertext on your clipboard instead.

If a platform does not work, that is worth an issue.

---

## How it works

**Identity.** An X25519 key pair. The `key_id` shown in the UI is the first five
characters of the Base64 public key, used for display only.

**Key agreement.** The sender computes an X25519 shared secret from their private
key and the recipient's public key. The recipient computes the same value from
the other side of the pair. That secret is used directly as the
ChaCha20-Poly1305 key.

**Message format.** Every GhostLayer string is a framed payload:

```
ghl:<kind>:<version>:<payload>
```

`kind` is `inv` for invites and `msg` for messages. `version` selects the
protocol implementation, so old messages keep parsing after the format moves on.

A v1 message payload is three fixed-position fields concatenated:

```
<sender public key: 44 chars><nonce: 16 chars><ciphertext: rest>
```

all Base64. The 16-byte Poly1305 authentication tag is part of the ciphertext.

**Why the message carries the sender's key.** The recipient needs it to derive
the shared secret. It is also why the sender cannot derive that secret from the
message alone, and why reading your own sent messages requires your friend list.

**Code spans.** Message keys are wrapped in backticks before insertion. Chat
platforms substitute emoji inside plain text, and `:msg:` style tokens are
exactly what those substitutions target. A code span stops the platform from
rewriting the key on its way through.

The protocol is deliberately independent of this implementation. A Firefox
extension, a CLI, or a mobile client could speak GhostLayer v1 without sharing a
line of code with this repository.

---

## Repository layout

```
ghost/                  Rust crate, compiled to WebAssembly
  src/crypto.rs         X25519 agreement, ChaCha20-Poly1305, encrypt/decrypt
  src/protocol/         Frame parsing, versioned protocol implementations
  src/identity.rs       Key generation
  src/friends.rs        Friend records
  src/storage.rs        JSON <-> struct conversion

extension/
  manifest.json         MV3 manifest
  background/           Service worker, WASM bridge, storage access
  content/content.js    Editor detection, text insertion, message scanning
  popup.html / popup.js Identity and friend management UI
  pkg/                  Generated WASM output, do not edit by hand
```

---

## Contributing

Contributions are welcome. See [CONTRIBUTING.md](CONTRIBUTING.md).

Note that this project requires a Contributor License Agreement, because
GhostLayer is dual-licensed and the maintainer needs the right to relicense
contributions. The details are in the contributing guide.

One warning for anyone touching `content/content.js`: the split between
`execCommand` and the paste path, and the `data-slate-editor` branch that drives
it, is load-bearing. Some editors report that `execCommand` succeeded while
ignoring the edit in their own document model, which puts the ciphertext on
screen while the platform still sends your original plaintext. Simplifying that
branch reintroduces a bug that silently transmits the message you meant to hide.

---

## License

GNU Affero General Public License v3.0. See [LICENSE](LICENSE).

You may use, modify, and redistribute GhostLayer, including commercially, as
long as derivative works are released under the same license and their source is
made available.

The copyright holder retains the right to license this code under other terms.
If AGPL does not fit your use, open an issue.

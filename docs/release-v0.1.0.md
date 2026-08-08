# Release notes for v0.1.0

Draft body for the GitHub release.

---

First release of GhostLayer.

GhostLayer adds end-to-end encryption to chat platforms that do not have it. You
type a message, click Encrypt, pick a friend, and your text is replaced with a
ciphertext blob before it leaves the browser. Your friend clicks Decrypt to read
it. Keys are generated locally and never leave your device.

## What works

- Identity creation, invite keys, and friend management
- Encryption and decryption on Discord, Slack, X, and Messenger
- Reading your own sent messages, not just received ones
- Versioned wire format, so future protocol changes will not break old messages
- Honest failure: when text cannot be inserted into a site's message box, the
  extension refuses to report success and puts the ciphertext on your clipboard
  instead of leaving you to send plaintext believing it was encrypted

## Cryptography

X25519 key agreement, ChaCha20-Poly1305 authenticated encryption. Written in
Rust, compiled to WebAssembly.

## Read this before using it

Alpha software, not independently audited.

1. A recipient can forge messages that look like they came from you
2. Long lived, unencrypted private keys, so no forward secrecy
3. Who is talking to whom is not hidden
4. You can lose access to your own sent messages
5. Nothing is configurable
6. The protocol and the platform support are both early

Each one is explained in
[LIMITATIONS.md](https://github.com/thecybro/GhostLayer/blob/main/LIMITATIONS.md).
Read it before you use GhostLayer for anything that matters.

## Install

- Get it from chrome web store if exists: [GhostLayer](#)

- Or, for devs who want to keep it local:

  1. Download the zip from this release and unpack it, or clone and run
    `./ghost/build.sh` or `/build.sh`.
  2. Open `chrome://extensions` and enable Developer mode
  3. Load unpacked, select the `extension/` directory

## License

AGPL-3.0.

---

<!--## Steps to cut the release

Not part of the release body.

```bash
./package.sh

git add -A
git commit -m "release: v0.1.0"
git tag -a v0.1.0 -m "GhostLayer v0.1.0"
git push origin main --tags

gh release create v0.1.0 \
  dist/ghostlayer-0.1.0.zip \
  --title "GhostLayer v0.1.0" \
  --notes-file docs/release-v0.1.0.md
```

Trim the "Steps to cut the release" section out of the notes file first, or pass
`--notes` with the body inline instead.-->

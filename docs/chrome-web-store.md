# Chrome Web Store submission notes

Everything the listing form asks for, written out.

## Listing fields

**Name**

```
GhostLayer
```

**Short description** (132 char limit, currently 104)

Chrome prefills this from `description` in `manifest.json`, so keep the two in
step or the listing quietly drifts from the installed extension.

```
End to end encrypt messages in any web chat. Your keys never leave your device. No accounts, no servers.
```

**Category**: Privacy & Security

Communication is the obvious pick and is defensible, but Privacy & Security is
a smaller, better matched pool. The extension is not a chat client; it is a
thing you put on top of one. If a second category is offered, use Communication
there.

**Language**: English

**Mature content**: No

**Google Analytics ID**: leave blank. The extension makes no network requests
and the privacy policy says so, so adding analytics here would contradict it.

**Detailed description**

```
GhostLayer adds end-to-end encryption to chat platforms that do not have it.

Type a message, click Encrypt, pick a friend. Your text is replaced with a
ciphertext blob before it leaves your browser. Your friend clicks Decrypt and
reads it. The platform only ever sees the blob.

Keys are generated on your machine and never leave it. There is no GhostLayer
account, no GhostLayer server, and nothing to sign up for. The extension makes
no network requests at all.

HOW IT WORKS

Create an identity in the popup. Share your invite key with a friend over any
channel. Add their invite key to your friend list. From then on you can encrypt
to each other on any site with a message box.

Decryption is never automatic. A Decrypt button appears next to encrypted
messages and you choose what to reveal and when.

CRYPTOGRAPHY

X25519 key agreement and ChaCha20-Poly1305 authenticated encryption,
implemented in Rust and compiled to WebAssembly.

TESTED ON

Discord, Slack, X, and Messenger. Other sites with a message box may work.

IMPORTANT LIMITATIONS

GhostLayer is alpha software and has not been independently audited.

1. A recipient can forge messages that look like they came from you
2. Long lived, unencrypted private keys, so no forward secrecy
3. Who is talking to whom is not hidden
4. You can lose access to your own sent messages
5. Nothing is configurable
6. The protocol and the platform support are both early

Each one is explained here, and you should read it before using GhostLayer for
anything that matters:
https://github.com/thecybro/GhostLayer/blob/main/LIMITATIONS.md

OPEN SOURCE

AGPL-3.0. Source at https://github.com/thecybro/GhostLayer
```

## Privacy practices tab

**Single purpose**

```
GhostLayer lets a user encrypt a message before sending it through a web chat
platform, and decrypt messages sent to them by their contacts.
```

**Are you using remote code?**

```
No, I am not using remote code
```

Everything executes from inside the package. The WebAssembly module is bundled
at `pkg/ghost_bg.wasm`, not fetched. If a reviewer asks about
`wasm-unsafe-eval` in the manifest CSP, that flag only permits instantiating
the WASM that already ships in the package; it does not load anything remote.

**Data usage**: tick nothing. GhostLayer collects and transmits no user data.

Every checkbox on that list stays empty. Not personally identifiable
information, not health, not financial, not authentication, not personal
communications, not location, not web history, not user activity. The extension
reads page text in the browser to find message keys and discards it, which is
processing, not collection, and nothing leaves the machine.

**Privacy policy URL**

```
https://github.com/thecybro/GhostLayer/blob/main/PRIVACY.md
```

Certify: not being sold to third parties, not used for unrelated purposes, not
used to determine creditworthiness.

## Permission justifications

**`storage`**

```
Stores the user's own key pair and their friends' public keys in
chrome.storage.local. This is the only place identity data exists. Nothing is
written anywhere else and nothing is transmitted.
```

**`clipboardWrite`**

```
When the extension cannot insert the encrypted text into a site's message box,
it writes the encrypted text to the clipboard so the user can paste it in
manually. This is a fallback path, used only after automatic insertion fails.
```

**Host permission `<all_urls>`**

```
GhostLayer encrypts text the user types into any web message box. The user
chooses where to use it, so the extension cannot know in advance which sites
those are, and a fixed site list would silently fail everywhere else.

On each page the content script does two things: it locates the message box so
the Encrypt button can replace its contents, and it scans page text for strings
beginning with "ghl:msg" so it can offer a Decrypt button next to encrypted
messages.

No page content is stored, logged, or transmitted. The extension makes no
network requests. Page text is examined in the browser and discarded.
```

If the submission is rejected on this permission anyway, the fix is to move
`<all_urls>` into `optional_host_permissions` and request access per site on
first use. That is a functional change (the content script would no longer
auto-inject) and should be a deliberate decision, not a panic edit during
review.

## Distribution tab

**Visibility**: Public

Unlisted is worth considering for a first submission, since the item still goes
through the same review but is only reachable by direct link. If you want to
hand the link to a few people and watch for problems before it is searchable,
start Unlisted and switch to Public later. Switching does not need a new review.

**Distribution regions**: all regions.

**Pricing**: Free.

## Account level

Set once, applies to every item you publish.

**Publisher name**: shown on every listing. Whatever you set here is what users
see under the extension name, so it should match how the repo is signed.

**Publisher email**: must be verified before you can publish.

**Trader status**: non-trader. GhostLayer is free, has no in-app purchases, and
no contract is formed with anyone on the marketplace. Change this to trader
before shipping anything paid under this account, not after.

<!--## Before uploading

- [X] `./package.sh` and confirm the zip contains no `tests/` directory
- [X] Remove or hide the "Crypto Testing" button in `popup.html`. It links to
      `tests/test.html`, which is excluded from the package, so in a store build
      it leads to a broken page
- [X] `manifest.json` version matches the git tag
- [X] Screenshots: 1280x800 or 640x400. Needed at minimum are the popup with an
      identity created, and a chat window showing an encrypted message with the
      Decrypt button
- [X] Store icon: 128x128, already at `extension/icons/icon128.png`-->

<!--## Review expectations

First review of a new extension usually takes a few days. Extensions requesting
broad host permissions take longer and are more likely to come back with
questions. An extension that performs encryption is not a policy problem in
itself, but the combination of `<all_urls>` and reading page text means a human
will look at it.

Keep the GitHub repository public and current during review. A reviewer able to
read the source resolves questions faster than one who cannot.-->

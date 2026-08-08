# Contributing to GhostLayer

Bug reports, platform reports, and pull requests are all welcome.

## Contributor License Agreement

GhostLayer is released under the AGPL-3.0 and is also licensed commercially by
the copyright holder. That second part only stays possible if the copyright
holder can relicense the whole codebase, which means contributions cannot arrive
under terms that block it.

So, by opening a pull request you agree that:

1. You wrote the contribution, or you have the right to submit it.
2. You grant the copyright holder a perpetual, worldwide, irrevocable license to
   use, modify, sublicense, and relicense your contribution under any terms,
   including proprietary ones.
3. You keep your own copyright. This is a license grant, not an assignment. You
   can still use your contribution however you like.

If you are not comfortable with point 2, open an issue instead of a pull
request and we can discuss the change without you writing the code.

## Reporting a platform that does not work

This is genuinely useful and does not require writing any code. Chat platforms
each handle text insertion differently, and the only way to find out is to try.

Include:

- Which platform, and whether it is the web version
- What happened: no Encrypt button, text did not appear, text appeared but the
  original was sent, no Decrypt button on incoming messages
- The console output. Open DevTools with Ctrl+Shift+I. On Discord, turn on
  **Preserve log** in the console settings first, or the page will clear it
  before you can read anything.

## Reporting a security issue

Do not open a public issue for anything that affects the confidentiality of
messages. Open a GitHub security advisory on the repository instead.

Known limitations are listed in the README and are not security issues. In
particular: no forward secrecy, no sender authentication, and unencrypted key
storage are all documented design gaps rather than bugs.

## Development

```bash
./ghost/build.sh
```

This compiles the Rust crate with `wasm-pack` and copies the output into
`extension/pkg/`. Requires a Rust toolchain and `wasm-pack`.

Load `extension/` as an unpacked extension at `chrome://extensions`.

After any change, reload the extension **and then reload the tab you are testing
on**. Chrome does not inject an updated content script into tabs that were
already open, and testing against a stale content script will waste your
afternoon.

## Things to know before changing `content/content.js`

Rich-text editors on these platforms keep their own document model. The visible
DOM is rendered output, not the source of truth, and pressing Enter sends what
the model holds rather than what is on screen. Three consequences:

- **Never write to the DOM directly** to insert text. `element.textContent = x`
  updates the view while the model keeps the old value, so the platform sends
  the plaintext you were trying to hide. It also destroys the editor's internal
  node mapping, which breaks the cursor and text selection.
- **Do not trust `document.execCommand`'s return value.** Some editors return
  `true` while ignoring the edit. The `data-slate-editor` branch exists because
  of exactly this, and it routes those editors to the paste path instead.
- **Do not add a second synthetic `input` event after a successful
  `execCommand`.** `execCommand` already fires a real one, and editors that
  handle both will insert the text twice.

When insertion fails, the correct behavior is to leave the editor untouched and
report failure. Showing ciphertext that the platform will not actually send is
worse than not inserting anything.

## Things to know before changing the protocol

The frame format is `ghl:<kind>:<version>:<payload>`. Version dispatch lives in
`ghost/src/protocol/registry.rs`, and old versions stay in the `PROTOCOLS` list
so previously sent messages keep parsing. If you change a payload layout, add a
new version rather than editing v1.

Avoid single letters between colons in the frame. `:m:` is an emoji shortcode on
Discord and Slack, and those platforms will rewrite it inside the message. That
is why the kind tokens are `inv` and `msg` rather than `i` and `m`.

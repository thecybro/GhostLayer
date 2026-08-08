# GhostLayer Privacy Policy

Last updated: 8 August 2026

## Summary

GhostLayer collects nothing, transmits nothing, and has no servers.

## What GhostLayer stores

All of it stays in `chrome.storage.local` on your own machine:

- Your identity: an X25519 key pair and an optional username you chose
- Your friends: their public keys, and nicknames you chose

Nothing is uploaded. There is no GhostLayer account, no analytics, no telemetry,
no crash reporting, and no remote configuration. The extension makes no network
requests of any kind.

## What GhostLayer reads

The extension runs on pages you visit in order to find the message box and
detect GhostLayer messages already on the page. It reads page text for one
purpose: to find strings beginning with `ghl:msg`, so it can offer a Decrypt
button next to them.

Page content is never stored, never logged, and never sent anywhere. It is
examined in your browser and discarded.

## What leaves your browser

Only what you deliberately send: the encrypted message you paste into a chat,
and the invite key you choose to share. Both travel over the chat platform you
are already using, under that platform's own privacy policy, not GhostLayer's.

Your private key is never included in either one and never leaves your device.

## Uninstalling

Removing the extension deletes its local storage, including your identity. There
is no copy anywhere else, so if you have not backed up your private key it is
gone permanently, along with your ability to read messages sent to you.

## Limitations you should know about

GhostLayer encrypts message content. It does not hide that you sent a message,
when you sent it, or to whom. Your public key is included in every encrypted
message, so anyone reading the conversation can tell which messages came from
the same identity.

Your private key is stored unencrypted. Anything with access to your Chrome
profile can read it.

The full list of limitations is in [LIMITATIONS.md](LIMITATIONS.md).

## Contact

Open an issue at https://github.com/thecybro/GhostLayer/issues

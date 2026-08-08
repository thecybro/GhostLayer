# GhostLayer Limitations v0.1.0

GhostLayer v0.1.0 is alpha software and has not been independently audited. The
cryptography is built from well regarded primitives, but the way this project
puts them together has had no external review.

Read this list before you use it. Everything here is a known gap in the current
design, not a bug. They are written down so you can decide for yourself what
GhostLayer is good enough for.

---

## 1. A recipient can forge messages that look like they came from you

The key that protects a message is derived from your private key and your
friend's, which means both of you can compute the same key. Anyone who can read
a message can also produce a new one that decrypts correctly under that same
key.

GhostLayer gives you confidentiality, not authenticity. If a message decrypts,
you know it was written by someone holding one of the two keys. You do not know
which one.

This matters most in an argument. Your friend can produce a message that appears
to come from you, and nothing in GhostLayer can tell the difference.

## 2. Long lived, unencrypted private keys

Each identity has one key pair and it never rotates. Your private key sits in
`chrome.storage.local` as plain Base64, with no passphrase and no OS keychain.

If that key is ever compromised, every message you have exchanged with that
contact can be decrypted, including messages sent before the compromise and
messages sent after it. Modern messengers rotate keys per message or per session
so that a stolen key only exposes a narrow window. GhostLayer does not do this
yet.

Anything that can read your Chrome profile can read your identity.

## 3. Who is talking to whom is not hidden

Your public key travels in cleartext inside every encrypted message. Anyone
reading the conversation can tell which messages came from the same identity and
link them together, even though they cannot read the contents.

The platform also still sees that you sent a message, when you sent it, in which
channel, and roughly how long it was. GhostLayer hides the content of a message.
It does not hide the fact of it.

## 4. You can lose access to your own sent messages

To read a message you sent, GhostLayer tries each key in your friend list until
one of them works. The recipient's key is not stored in the message itself.

So if you remove a friend, or move your identity to another browser without also
importing your friends, the messages you sent that person become unreadable to
you. The ciphertext is intact. The key needed to open it is simply no longer on
your machine.

Your friend can still read them. You cannot.

## 5. Nothing is configurable

There is no settings screen in v0.1.0. Nothing you choose is remembered.

That means no auto decrypt, so every incoming message needs its own Decrypt
click. No default recipient, so every message needs you to pick a friend from
the list. No per site preferences, no way to disable GhostLayer on specific
sites, and no way to change any behaviour without editing the source.

## 6. The protocol and the platform support are both early

The message and invite formats are versioned, so they can change without
breaking messages that already exist. But they are early, and the format is
likely to change in ways that make v0.1.0 keys unreadable by later versions.

Platform support is empirical. GhostLayer has been tested on Discord, Slack, X,
and Messenger. Other sites may work or may fail. When GhostLayer cannot insert
text into a site's message box it refuses to claim success and puts the
ciphertext on your clipboard instead, so it should not silently send your
plaintext, but this depends on the site behaving in ways we have seen before.

## 7. No functionality to export storage and change keys

Currently, GhostLayer doesn't provide the functionality to allow you to export your
storage which you can later paste into another device or browser and continue as is.

And it also doesn't have the feature which lets you change your identity incase the
old one gets compromised.

---

## What this means in practice

Use GhostLayer to keep ordinary conversation away from platform side scanning
and casual snooping. That is what it is good at today.

Do not use it where being wrong has real consequences: not for anything that
would put someone in danger, not where you need to prove who wrote a message,
and not where a leaked key later would be a serious problem.

## No warranty

GhostLayer is provided as is, without warranty of any kind. You use it at your
own risk, and the author is not liable for any loss or harm arising from its
use, including the limitations described above.

This is not a legal escape hatch bolted on at the end. The gaps are listed above
in plain language precisely so that you can make an informed choice before you
trust it with anything.

See the GNU Affero General Public License v3.0, sections 15 and 16, in
[LICENSE](LICENSE) for the formal terms.

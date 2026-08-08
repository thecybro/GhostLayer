#!/usr/bin/env python3
"""
Turns the raw 1366x768 captures in previews/final_shots into Chrome Web Store
screenshots: 1280x800, 24-bit RGB, no alpha, nothing personal left in frame.

Rerun after retaking any shot:  python3 store-assets/build-screenshots.py
"""
from PIL import Image, ImageFilter
import pathlib

SRC = pathlib.Path("previews/final_shots")
OUT = pathlib.Path("store-assets/screenshots")
TARGET = (1280, 800)

# The crop alone removes the OS menu bar, the browser tab strip, the URL bar
# and its channel id, the Discord server rail, the whole DM list, and the
# account panel in the bottom left. Exactly 1.6:1, so nothing is letterboxed.
CROP = (380, 152, 1366, 768)          # 986 x 616

SURFACE  = (32, 32, 35)               # Discord's panel colour
COMPOSER = (39, 40, 43)               # inside the message box
DIM      = (39, 40, 43)               # behind the extension popup

# The Encrypt button is position:fixed, so it lands on the same pixels in every
# shot, and it sits on top of a region that gets filled. Lift, fill, put back.
ENCRYPT = (1264, 706, 1350, 752)

# Flat fills read as collapsed or empty UI. Blur reads as "something was hidden
# here", so it is used only where an element genuinely belongs in the frame.
COMMON_BLUR = [
    (386, 156, 476, 198),             # contact avatar and display name
    (390, 198, 438, 706),             # personal profile photos beside messages
]
PLACEHOLDER = (440, 714, 620, 750)    # "Message @<contact>" names the contact

# Only these shots have an empty composer showing that placeholder.
NEEDS_PLACEHOLDER_FILL = {
    "4-decryption_button.png", "5-decrypted_text.png", "popup.png",
}


def regions(name):
    if name == "popup.png":
        # the popup overlaps the profile card, so only the strip beside it is
        # filled, plus the profile button peeking out under the popup
        fill = [((1192, 152, 1366, 768), SURFACE),
                ((1050, 700, 1366, 768), DIM)]
    else:
        fill = [((1057, 196, 1366, 768), SURFACE),   # profile card
                ((1096, 156, 1366, 198), SURFACE)]   # search box holds the tag
    if name in NEEDS_PLACEHOLDER_FILL:
        fill.append((PLACEHOLDER, COMPOSER))
    return fill


def main():
    OUT.mkdir(parents=True, exist_ok=True)

    for src in sorted(SRC.iterdir()):
        if src.suffix.lower() != ".png":
            continue

        im = Image.open(src)
        if im.mode in ("RGBA", "LA"):
            flat = Image.new("RGB", im.size, (0, 0, 0))
            flat.paste(im, mask=im.split()[-1])
            im = flat
        else:
            im = im.convert("RGB")

        keep = im.crop(ENCRYPT)
        for box, colour in regions(src.name):
            im.paste(Image.new("RGB", (box[2] - box[0], box[3] - box[1]), colour), box)
        im.paste(keep, ENCRYPT[:2])

        for box in COMMON_BLUR:
            r = im.crop(box)
            im.paste(r.filter(ImageFilter.GaussianBlur(max(9, min(r.size) // 3))), box)

        out = OUT / src.name
        im.crop(CROP).resize(TARGET, Image.LANCZOS).save(out, "PNG")
        print(f"{src.name:<28} -> {out}")


if __name__ == "__main__":
    main()

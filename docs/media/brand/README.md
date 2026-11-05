# NIROVA brand assets

The approved NIROVA mark is the gradient ribbon N on a near-black background, based on the supplied project artwork. Do not use Horizon marks or candidate logos in product-facing surfaces.

## Masters

- `icon.svg`: scalable application icon.
- `icon-macos.svg`: macOS application tile.
- `avatar.svg`: full-bleed brand avatar.
- `mark.svg` and `mark-dark.svg`: standalone gradient mark.
- `social/card.html`: social preview source.

## Generated assets

Run `cargo run -p xtask -- icons` from the repository root. It renders NIROVA icons for Windows, Linux, macOS, the game window, and website favicons. The installer and desktop entry use the same generated icon family.

The public community links are configured only in `launcher/links.json`. Empty values mean no Discord server, sponsor, website, or official game server is advertised; do not substitute upstream Horizon links.

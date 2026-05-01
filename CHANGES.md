# CHANGES

This file records modifications made to files inherited from upstream Servo
(https://github.com/servo/servo), as required by MPL-2.0 §3.3.

Modifications to MPL-2.0 files remain MPL-2.0. New files authored by TreeCloud
AI that are not derivatives are licensed under [LICENSE-SAURON](./LICENSE-SAURON).

Format: each entry lists the upstream commit the fork diverged at, then the
high-level changes per release/branch. For file-level history, use `git log`.

---

## Fork point

- Forked from `servo/servo` at commit `f73f72afc61` on 2026-04-30.
- Upstream remote available locally as `upstream`
  (`git remote add upstream https://github.com/servo/servo.git`).

## Unreleased

### Modified MPL-2.0 files

- `ports/servoshell/platform/macos/Info.plist` — rebranded display
  strings (`CFBundleName`, `CFBundleGetInfoString`, `CFBundleIdentifier`,
  Bluetooth usage description) to "Sauron" / `com.treecloud.sauron`.
  The `CFBundleExecutable` value is intentionally left as `servoshell`
  to match the unchanged binary name.
- `ports/servoshell/platform/windows/servoshell.exe.manifest` —
  `assemblyIdentity name` changed from `servo.ServoShell` to
  `treecloud.Sauron`.
- `ports/servoshell/desktop/headed_window.rs` — Linux WM_CLASS
  changed from `("org.servo.Servo", "Servo")` to
  `("com.treecloud.sauron", "Sauron")` so the window can be pinned
  to the taskbar under the Sauron identity.
- `ports/servoshell/build.rs` — added `winresource` overrides so the
  Windows .exe Properties dialog reports `ProductName=Sauron`,
  `FileDescription=Sauron AI Browser`, `CompanyName=TreeCloud AI`.
  Without these, the defaults derived from the `servoshell` Cargo
  package leak through to end users.
- `README.md` — rewritten to describe Sauron AI; build instructions
  retained, attribution to upstream Servo preserved and expanded.

### Modified MPL-2.0 files (continued, new tab branding)

- `resources/resource_protocol/newtab.html` — `<title>` rebranded to
  "Sauron - New Tab", logo swapped to `sauron.svg`, "Home" link and
  logo target now point to https://tree-cloud.com.
- `resources/resource_protocol/newtab.css` — image width tightened
  and dark-mode `filter: invert(1)` added so the monochrome Sauron
  SVG is visible on the dark theme.
- `ports/servoshell/prefs.rs` — default homepage and CLI URL fallback
  changed from `https://(www.)servo.org` to `resource:///newtab.html`
  so a fresh launch lands on the Sauron new-tab page instead of the
  upstream Servo project site.

### Renamed

- `resources/org.servo.Servo.desktop` → `resources/com.treecloud.sauron.desktop`
  with display strings rebranded. The `Exec=` path still points at the
  unchanged `servoshell` binary.

### Added (non-MPL, Apache-2.0)

- `NOTICE.md` — fork attribution and licensing boundary.
- `LICENSE-SAURON` — Apache-2.0 for new TreeCloud AI components.
- `resources/sauron.svg` — Sauron logo (vector). PNG/ICO/ICNS variants
  pending — until they exist, the macOS bundle and Windows .exe still
  embed Servo's icons.

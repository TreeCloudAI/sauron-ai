# Sauron AI

Sauron is an AI-driven web browser built by [TreeCloud AI](https://tree-cloud.com).
It pairs a modern, parallel browser engine with an agentic layer, a vision
pipeline, and Palantír — services that let an agent perceive, reason about, and
act on the live web on your behalf.

Sauron is a fork of [Servo](https://servo.org), the parallel browser engine in
Rust originally developed by Mozilla and currently stewarded by the Linux
Foundation. We gratefully build on Servo's work; see [NOTICE.md](./NOTICE.md)
for attribution and licensing details, and [CHANGES.md](./CHANGES.md) for the
record of modifications to inherited files.

- Engine code inherited from Servo: [MPL-2.0](./LICENSE)
- New components authored by TreeCloud AI: [Apache-2.0](./LICENSE-SAURON)

> **Note:** the on-disk crate and binary are still named `servoshell`. This is
> intentional — keeping the internal name aligned with upstream Servo minimizes
> merge friction while we absorb upstream improvements. The user-facing brand
> is "Sauron" (bundle IDs, app names, icons). A full rename is on the roadmap.

## Getting started

Sauron currently builds on 64-bit macOS, 64-bit Linux, 64-bit Windows, 64-bit
OpenHarmony, and Android. For more detailed build instructions, see the Servo
Book under [Getting the Code] and [Building Servo] — the build system (`mach`)
is unchanged from upstream.

[Getting the Code]: https://book.servo.org/building/getting-the-code.html
[Building Servo]: https://book.servo.org/building/building.html

### macOS

- Download and install [Xcode](https://developer.apple.com/xcode/) and [`brew`](https://brew.sh/).
- Install `uv`: `curl -LsSf https://astral.sh/uv/install.sh | sh`
- Install `rustup`: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Restart your shell to make sure `cargo` is available
- Install the other dependencies: `./mach bootstrap`
- Build: `./mach build` (produces the `servoshell` binary, branded as Sauron)

### Linux

- Install `curl`:
  - Arch: `sudo pacman -S --needed curl`
  - Debian, Ubuntu: `sudo apt install curl`
  - Fedora: `sudo dnf install curl`
  - Gentoo: `sudo emerge net-misc/curl`
- Install `uv`: `curl -LsSf https://astral.sh/uv/install.sh | sh`
- Install `rustup`: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Restart your shell to make sure `cargo` is available
- Install the other dependencies: `./mach bootstrap`
- Build: `./mach build`

### Windows

- Download [`uv`](https://docs.astral.sh/uv/getting-started/installation/#standalone-installer), [`choco`](https://chocolatey.org/install#individual), and [`rustup`](https://win.rustup.rs/)
  - Be sure to select *Quick install via the Visual Studio Community installer*
- In the Visual Studio Installer, ensure the following components are installed:
  - **Windows 10/11 SDK (anything >= 10.0.19041.0)** (`Microsoft.VisualStudio.Component.Windows{10, 11}SDK.{>=19041}`)
  - **MSVC v143 - VS 2022 C++ x64/x86 build tools (Latest)** (`Microsoft.VisualStudio.Component.VC.Tools.x86.x64`)
  - **C++ ATL for latest v143 build tools (x86 & x64)** (`Microsoft.VisualStudio.Component.VC.ATL`)
- Restart your shell to make sure `cargo` is available
- Install the other dependencies: `.\mach bootstrap`
- Build: `.\mach build`

### Android

- Ensure that the following environment variables are set:
  - `ANDROID_SDK_ROOT`
  - `ANDROID_NDK_ROOT`: `$ANDROID_SDK_ROOT/ndk/28.2.13676358/`
 `ANDROID_SDK_ROOT` can be any directory (such as `~/android-sdk`).
  All of the Android build dependencies will be installed there.
- Install the latest version of the [Android command-line
  tools](https://developer.android.com/studio#command-tools) to
  `$ANDROID_SDK_ROOT/cmdline-tools/latest`.
- Run the following command to install the necessary components:
  ```shell
  sudo $ANDROID_SDK_ROOT/cmdline-tools/latest/bin/sdkmanager --install \
   "build-tools;34.0.0" \
   "emulator" \
   "ndk;28.2.13676358" \
   "platform-tools" \
   "platforms;android-33" \
   "system-images;android-33;google_apis;x86_64"
  ```
- Follow the instructions above for the platform you are building on

### OpenHarmony

- Follow the instructions above for the platform you are building on to prepare the environment.
- Depending on the target distribution (e.g. `HarmonyOS NEXT` vs pure `OpenHarmony`) the build configuration will differ slightly.
- Ensure that the following environment variables are set
  - `DEVECO_SDK_HOME` (Required when targeting `HarmonyOS NEXT`)
  - `OHOS_BASE_SDK_HOME` (Required when targeting `OpenHarmony`)
  - `OHOS_SDK_NATIVE` (e.g. `${DEVECO_SDK_HOME}/default/openharmony/native` or `${OHOS_BASE_SDK_HOME}/${API_VERSION}/native`)
  - `SERVO_OHOS_SIGNING_CONFIG`: Path to json file containing a valid signing configuration for the demo app.
- Review the detailed instructions at [Building for OpenHarmony].
- The target distribution can be modified by passing `--flavor=<default|harmonyos>` to `mach <build|package|install>`.

[Building for OpenHarmony]: https://book.servo.org/hacking/building-for-openharmony.html

## Upstream sync

This fork tracks `servo/servo` as the `upstream` remote:

```sh
git fetch upstream
git merge upstream/main   # or rebase, depending on branch policy
```

See [CHANGES.md](./CHANGES.md) for what has been modified since the fork point.

## License

- Files inherited from Servo: MPL-2.0 — see [LICENSE](./LICENSE)
- New TreeCloud AI components: Apache-2.0 — see [LICENSE-SAURON](./LICENSE-SAURON)
- WHATWG specs: see [LICENSE_WHATWG_SPECS](./LICENSE_WHATWG_SPECS)

## Trademarks

"Servo" is a trademark of the Servo project. This project is not endorsed by,
affiliated with, or sponsored by the Servo project or the Linux Foundation.
"Sauron" and the Sauron branding belong to TreeCloud AI.

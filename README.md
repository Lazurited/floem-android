<div align="center">
<img width=120 height=120 src="https://lap.dev/images/floem.svg"></img>

# floem-android

A fork of [Floem](https://github.com/lapce/floem) with an Android backend.

> note: the discord server isn't mine

[![Discord](https://img.shields.io/discord/946858761413328946?color=%237289DA&label=discord)](https://discord.gg/RB6cRYerXX)

_floem-android is a fork. Upstream Floem lives at [lapce/floem](https://github.com/lapce/floem). This fork adds an Android backend for use by [Lazurite](https://github.com/Lazurited/lazurite)._

</div>

## About

Floem is a native Rust UI library with fine-grained reactivity and great ergonomics. This fork tracks upstream Floem and adds:

- Android platform support via `floem-winit-android`
- Vulkan and GLES 3.x rendering through `wgpu`
- Android-specific input handling (hardware keyboard, IME, trackpad)

Upstream attribution is preserved. See [NOTICE](NOTICE) and [LICENSE](LICENSE).

## Status

Pre-alpha. The Android target does not yet compile end-to-end.

| Component | Status |
| :--- | :--- |
| `winit-android` backend | Provided by [lapce/winit](https://github.com/lapce/winit) |
| `rfd` (file dialogs) gated for Android | In progress |
| `softbuffer` gated for Android | Not started |
| `accesskit` gated for Android | Not started |
| Full `cargo check` for `aarch64-linux-android` | Not yet |

## Quickstart

The example below is from upstream Floem and works on desktop targets.

```rust
use floem::prelude::*;

fn main() {
    floem::launch(counter_view);
}

fn counter_view() -> impl IntoView {
    let mut counter = RwSignal::new(0);

    Stack::horizontal((
        Button::new("Increment").action(move || counter += 1),
        Label::derived(move || format!("Value: {counter}")),
        Button::new("Decrement").action(move || counter -= 1),
    ))
    .style(|s| s.size_full().items_center().justify_center().gap(10))
}
```

## Upstream Features

Inherited from Floem:

- ***Cross-platform***: Windows, macOS, and Linux via [wgpu](https://github.com/gfx-rs/wgpu) with [vger](https://github.com/audulus/vger-rs), [vello](https://github.com/linebender/xilem), or [Skia](https://github.com/RazrFalcon/tiny-skia). CPU fallback via [tiny-skia](https://github.com/linebender/tiny-skia).
- ***Fine-grained reactivity***: Reactive primitives inspired by [leptos_reactive](https://crates.io/crates/leptos_reactive).
- ***Performance***: View tree constructed once, virtual lists available.
- ***Flexbox & Grid layout***: Via [Taffy](https://crates.io/crates/taffy).
- ***Customizable widgets***: Styling API with themes and classes.
- ***Transitions and animations***: CSS-like transitions plus keyframe animations with spring easing.
- ***Element inspector***: Browser-devtools-style diagnostic tool.
- ***Global themes***: Light and dark built in.
- ***Localization***: Via [Fluent](https://github.com/projectfluent/fluent-rs).

## Android Additions

Added by this fork:

- Android backend wired through `floem-winit-android`
- Target-gated desktop-only crates (`rfd`, `softbuffer`, `accesskit`)
- Android-specific `#[cfg]` branches in `src/platform/`

## Community

This fork does not have its own chat server or forum.

- Bug reports, feature requests, and patches for `floem-android` go to
  this repo's Issues and
  Pull Requests.
- Upstream Floem discussion happens on the Floem Discord
  ([invite](https://discord.gg/RB6cRYerXX), `#floem` channel). That server is
  run by the Floem project, not by Lazurite. Use it for general Floem
  questions, not for `floem-android` bug reports.

## Syncing Upstream

This fork tracks [lapce/floem](https://github.com/lapce/floem.com).

```bash
git remote add upstream https://github.com/lapce/floem.git
git fetch upstream
git merge upstream/master
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). Android-specific patches are welcome
via Issues and Pull Requests on this repo.

## License

MIT, inherited from Floem. See [LICENSE](LICENSE).

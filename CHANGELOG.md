# Changelog

All notable changes to this project will be documented in this file.

## [4.2.3] - 2026-07-04

### Bug Fixes

- Switch to `AdwHeaderBar`
- Move repository

### Documentation

- Add packaging status
- Add `Contributing`
- Update ci badge

### Operations

- Bump actions/cache from 5.0.3 to 5.0.4
- Bump taiki-e/setup-cross-toolchain-action
- Bump taiki-e/create-gh-release-action from 1.9.2 to 1.9.3
- Bump codecov/codecov-action from 5.5.2 to 6.0.0
- Bump taiki-e/upload-rust-binary-action from 1.28.0 to 1.29.1
- Bump actions/cache from 5.0.4 to 5.0.5
- Bump taiki-e/upload-rust-binary-action from 1.29.1 to 1.30.2
- Bump taiki-e/create-gh-release-action from 1.9.3 to 1.11.0
- Bump actions/labeler from 6.0.1 to 6.1.0
- Bump codecov/codecov-action from 6.0.0 to 6.0.1
- Bump taiki-e/setup-cross-toolchain-action
- Bump codecov/codecov-action from 6.0.1 to 7.0.0
- Bump actions/checkout from 6.0.2 to 6.0.3
- Update actions
- Add coverage flags
- Switch to `codecov-cli`
- Pull codecov directly
- Disable release notes assistant
- Extract changelog

### Build

- Bump rand from 0.10.0 to 0.10.1
- Edit features

## [4.2.2] - 2026-03-01

### Bug Fixes

- Drop `rand_chacha`
- Set `input-purpose`

### Operations

- Bump taiki-e/upload-rust-binary-action from 1.27.0 to 1.28.0

### Testing

- Update checksums

### Build

- Update gtk

## [4.2.1] - 2026-02-01

### Operations

- Bump actions/cache from 5.0.1 to 5.0.3
- Bump actions/checkout from 6.0.1 to 6.0.2
- Bump taiki-e/create-gh-release-action from 1.9.1 to 1.9.2
- Bump taiki-e/setup-cross-toolchain-action

### Build

- Bump `fast_image_resize` to `6.0`

## [4.2.0] - 2026-01-01

### Bug Fixes

- Change tooltip
- Update `read_image()`

### Features

- Add `version()`

### Operations

- Bump actions/checkout from 4 to 5
- Pin actions
- Pin `labeler`
- Bump codecov/codecov-action from 5.5.0 to 5.5.1
- Bump actions/labeler from 5.0.0 to 6.0.1
- Bump actions/cache from 4.2.4 to 4.3.0
- Bump taiki-e/setup-cross-toolchain-action
- Bump actions/checkout from 5.0.0 to 6.0.0
- Bump codecov/codecov-action from 5.5.1 to 5.5.2
- Bump taiki-e/setup-cross-toolchain-action
- Bump actions/checkout from 6.0.0 to 6.0.1
- Bump actions/cache from 4.3.0 to 5.0.1
- Fix action version

### Testing

- Update integration/unit cases
- Switch to `cargo_bin!`

### Build

- Bump slab from 0.4.10 to 0.4.11
- Bump `png` to `0.18`

## [4.1.4] - 2025-07-29

### Bug Fixes

- Update `window` wrapper

### Build

- Update GTK dependencies

## [4.1.3] - 2025-07-06

### Operations

- Drop `reviewers`

### Build

- Exclude desktop files
- Bump `indicatif` to 0.18

## [4.1.2] - 2025-05-14

### Bug Fixes

- Send errors
- Unlock post-error window
- Change error timeout

### Miscellaneous tasks

- Add desktop files

## [4.1.1] - 2025-05-11

### Miscellaneous tasks

- Add errors todo

### Styling

- Remove extra semicolons

### Build

- Add lto profile

## [4.1.0] - 2025-03-23

### Bug Fixes

- Adjust spinner
- Fix button states
- Fix `Window::load_file()`
- Improve seed entry
- Unblock `load_file()`
- Disable main buttons during processing
- Fix seed mode
- Improve file load
- Fix ansi button
- Filter seed entries

### Features

- Add spinner

### Operations

- Exclude beta/nightly from release

### Refactor

- `Image` -> `Base`
- Improve `Window::busy()`
- Drop `Base::is_present`
- Fix redundant clones

### Build

- Make `async-channel` optional

## [4.0.1] - 2025-03-16

### Bug Fixes

- Handle invalid chunk rates
- Handle missing palette
- Mark `must_use` functions
- Explicit `input` clone
- Start with `ColorScheme::ForceDark`
- Update core options

### Documentation

- Update dependency usage
- Use `cargo add`
- Fix usage code block
- Comment `generate_ansi_data()`

### Miscellaneous tasks

- Add ansi example
- Update `exclude`
- Ignore `debug/`
- Ignore backups
- Move examples
- Exclude examples

### Operations

- Add changelog

### Performance

- Avoid palette allocation

### Refactor

- Move vars into `format`

### Styling

- Add missing semi-columns

### Build

- Use gtk4 v4_16
- Use libadwaita v1_6

## [4.0.0] - 2025-03-14

### Bug Fixes

- Mark structs non-exhaustive
- Update `save_file()`
- Improve grayscale support
- Update `generate_texture()`
- Improve `get_ansi_color()` i32 casts
- Update texture generator
- Improve u8 casting
- Update argument descriptions
- Use `Uniform::new()`

### Documentation

- Update `Options`

### Features

- Add ANSI color set support
- Add `ansi` option
- Add `ansi` button
- Add `MoshData::palette`
- [**breaking**] Support all color types
- Add `MoshError::InvalidPalette`
- Implement `TryFromIntError`
- Implement `uniform::Error`

### Miscellaneous tasks

- Update LICENSE

### Operations

- Change codecov threshold

### Refactor

- Rename color type string function
- Collapse `options.ansi` block
- Merge `mosh` arm patterns
- Drop redundant match arm
- Remove deprecated functions
- Update `win.save-file`
- Make `get_ansi_color()` private

### Styling

- Fix project formatting

### Testing

- Move assets
- Add `ansi_alpha`
- Update assertions
- Add `invalid_range`

### Build

- Add gui block
- Bump rust edition to 2024
- Update dependencies

## [3.6.1] - 2025-01-07

### Operations

- Update dependabot config
- Bump codecov/codecov-action from 4 to 5

### Build

- Bump clap from 4.5.9 to 4.5.13
- Bump predicates from 3.1.0 to 3.1.2
- Bump fast_image_resize from 4.2.0 to 4.2.1
- Bump assert_cmd from 2.0.14 to 2.0.15
- Bump `fast_image_resize` to 5.1

## [3.6.0] - 2024-07-22

### Bug Fixes

- Use `clone_from()`
- Update `pixelation()`

### Features

- Migrate to `AboutDialog`

### Operations

- Configure dependabot
- Configure labeler
- Add pull request template
- Add `lib` label

### Build

- Update dependencies
- Bump clap from 4.5.4 to 4.5.9
- Bump fast_image_resize from 3.0.4 to 4.2.0

## [3.5.3] - 2024-04-17

### Bug Fixes

- Update `load_css()`

### Operations

- Bump `codecov-action` to v4
- Add codecov token

### Build

- Bump gtk features
- Update `fast_image_resize`

## [3.5.2] - 2024-02-22

### Bug Fixes

- Write image buffer
- Update `FileFilter`

### Miscellaneous tasks

- Update GTK dependencies
- Update LICENSE

### Operations

- Bump `checkout` to v4
- Bump `cache` to v4
- Drop `actions-rs/toolchain`

### Build

- Fix glib-build-tools

## [3.5.1] - 2023-10-20

### Miscellaneous tasks

- Bump rustix from 0.37.19 to 0.37.25

### Refactor

- Drop redundant pattern matching

## [3.5.0] - 2023-06-08

### Bug Fixes

- Drop vertical separator
- Switch to `adw::AboutWindow`

### Features

- Indicate debug builds

### Miscellaneous tasks

- Update indicatif

### Refactor

- Update debug functions

## [3.4.0] - 2023-06-01

### Bug Fixes

- Edit `win.mosh-file`
- Fix `website` string
- Update `default_width`
- Add missing tooltips
- Edit menu tooltip
- Improve spacing
- Edit `color_type` label
- Fix error message
- Show filename only
- Handle filename errors
- Improve error output
- Reactivate batch spinner
- Dim separators
- Do not use `gtk::AlertDialog`

### Features

- Add `win.about`
- Improve dialogs
- Set app logo
- Add color type label
- Improve layout
- Improve `seed`
- Implement `Clone`
- Add `win.mosh-rewind`
- Add color mode indicator
- Show current filename
- Add `ToastOverlay`

### Miscellaneous tasks

- Move`win.mosh-file`
- Handle numerical errors only
- Update `Window`
- Update dependencies
- Improve `args()`
- Add desktop entry

### Refactor

- Move `mode`

### Styling

- Fix xml formatting

## [3.3.0] - 2023-05-25

### Bug Fixes

- Don't filter files on macos
- Edit style manager button
- Add missing shortcuts
- Use universal shortcut descriptor

### Features

- Add color scheme button
- Add custom seed support

### Miscellaneous tasks

- Drop deprecated structs
- Update dependencies
- Set `gtk` to v4_10

### Operations

- Switch `lint` to macos

## [3.2.1] - 2023-05-22

### Bug Fixes

- Drop picture shadow

### Miscellaneous tasks

- Update dependencies

## [3.2.0] - 2023-05-21

### Bug Fixes

- Fix `BANNER`
- Drop `StyleContext::add_provider_for_display`
- Improve image status
- Improve error handing

### Documentation

- Update README.md
- Update `Compilation`
- Fix `MoshData` note
- Update README.md

### Features

- Add shortcuts
- Add file ops shortcuts
- Add shortcuts button
- Remap shortcuts

### Miscellaneous tasks

- Update dependencies
- Improve `MoshError`
- Update dependencies
- Update dependencies
- Update clap
- Update gresource prefix
- Fix error formatting

### Testing

- Update checksums

## [3.0.0] - 2023-01-13

### Testing

- Add `encoding`
- Add `pixelation`
- Update `pixelation`
- Update `encoding`
- Fix `encoding`

### Cli

- Update help message

## [2.0.0] - 2022-09-09

### Bench

- Move images

## [1.1.0] - 2022-08-04

### Testing

- Add `grayscale_alpha`

## [1.0.0] - 2022-07-23



# Changelog

All notable changes to this project will be documented in this file.

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



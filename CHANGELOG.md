# Changelog

All notable changes to this project will be documented in this file.

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

### CI/CD

- Switch `lint` to macos

### Features

- Add color scheme button
- Add custom seed support

### Miscellaneous tasks

- Drop deprecated structs
- Update dependencies
- Set `gtk` to v4_10

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


# Gamedirs

Opinionated storage basedirs for applications, mainly for games.

## Settings / Configs

Settings should be local and bound to the device.

### Windows

`%LOCALAPPDATA%\<company>\<app>\Config\`

### macOS

`~/Library/Application Support/<bundle-id>/`

### Linux

`$XDG_CONFIG_HOME/<company>/<app>/` falls back to `~/.config/<company>/<app>/` 

## Save Games

These are things that should sync to the cloud, and be available to the user's devices.

### Saves - Windows

`%USERPROFILE%\Saved Games\<company>\<app>\` (use SHGetKnownFolderPath(FOLDERID_SavedGames)?)

### Saves - macOS

`~/Library/Application Support/<bundle-id>` (some use Company/Game).

### Saves - Linux

`$XDG_DATA_HOME/<company>/<app>/` (fallback `~/.local/share/<company>/<app>/`)
(basedir-spec) [https://specifications.freedesktop.org/basedir-spec/latest/]

## Logs and Debug output

Things that are inherently tied to a device, should not be synced.

### Logs - Windows

`%LOCALAPPDATA%\<company>\<game>\logs\` (local, non-roaming, the logs can be quite large).

### Logs - macOS

`~/Library/Logs/<bundle-id>/` (user logs)

### Logs - Linux

Use `$XDG_STATE_HOME` (default `~/.local/state`), e.g.:
`$XDG_STATE_HOME/<company>/<app>/logs/` → fallback `~/.local/state/<company>/<app>/logs/`.

## Temp storage (short-lived, safe to delete)

Should not be available after reboot. For temporary files that are discarded or moved to a permanent location.

### Temp - Windows

`GetTempPath` (respects `%TEMP%`, `%TMP%`, etc.)

### Temp - macOS

Use `FileManager.temporaryDirectory` or `NSTemporaryDirectory()` for per-user temp space.

### Temp - Linux

Use `$TMPDIR` if set, otherwise `/tmp` 


# Other

bundle_id: `com.companyname.appname`

---

_Copyright (c) Peter Bjorklund. All rights reserved._

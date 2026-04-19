# tuistctl

A terminal UI for running `tuist generate` on specific targets without memorizing flags or env vars

## Install

```sh
cargo install --git https://github.com/danilpapa/tuistctl
```

## Update

Same command reinstalls the latest version from the repo:

```sh
cargo install --git https://github.com/danilpapa/tuistctl
```

## Requirements

- Rust toolchain (`cargo`)
- `xcodebuild` in PATH (comes with Xcode)
- `tuist` in PATH
- An `.xcworkspace` file reachable from the directory where you run `tuistctl`

## Usage

Run from anywhere inside your project (tuistctl walks up to find the `.xcworkspace`):

```sh
tuistctl
```

1. **Target selection** — pick one or more Xcode schemes with `Space`, confirm with `Enter`
2. **Option selection** — pick optional tuist flags/env vars, confirm with `Enter`  
3. **Generation** — the command runs while a matrix animation plays; exits when done

Navigation: `↑ ↓` move cursor · `Space` toggle · `Enter` confirm · `q / Esc` go back

## Options file

`tuistctl` reads an `options.txt` file next to the workspace to populate the options screen. Each line is one option:

```
// comment line, skipped
TUIST_SOME_FLAG          // shown as env var: TUIST_SOME_FLAG=1
cache = tuist cache      // shown as exec command, runs: tuist cache
```

- Lines starting with `//` are skipped  
- `name = command` → executes `command` when selected  
- `name` alone → appended as `name=1` env var  
- Inline `// tip` after a definition is shown as a hint in the UI

## How command generation works

tuistctl stubs out the `tuist generate` invocation so you never type it manually:

```
tuist generate <target1> <target2>
```

If options are selected, it appends them:

```
tuist generate <targets> && tuist cache SOME_FLAG=1
```

The command executes in the directory containing the `.xcworkspace` file, exactly as if you had `cd`-ed there and typed it yourself.

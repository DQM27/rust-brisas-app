---
description: Load development environment and run commands
---

To interact with the project (build, test, run), you MUST load the `dev-env.ps1` script first.

## Usage

```powershell
. .\dev-env.ps1
```

## Available Commands

- `dev`: Run the app (cargo run)
- `dev -Check`: Verify code (cargo check)
- `dev -Build`: Build the app (cargo build)
- `tauri dev`: Run tauri dev server

## Example: Verify Code

```powershell
. .\dev-env.ps1; dev -Check
```

## Git Best Practices

// turbo-all
1. Make **atomic commits** after each significant change
2. Use descriptive commit messages in Spanish
3. Before starting work, check current branch with `git status`
4. To undo changes, use `git checkout <commit>` to test a version without altering history
5. Push changes periodically to backup work

```powershell
# Commit after significant changes
git add -A
git commit -m "descripción del cambio"
```


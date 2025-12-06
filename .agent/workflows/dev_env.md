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

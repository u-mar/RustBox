# RustBox 🦀📦
*A BusyBox-style reimplementation of GNU coreutils in Rust.*

RustBox provides a collection of small, fast, and reliable command-line utilities written in Rust.  
Each utility is installed as its own binary (`fastcat`, `fastgrep`, …), similar to GNU coreutils but built the Rust way.

---

## 🚀 Goals
- Provide fast, memory-safe replacements for common Unix utilities.
- Learn and practice Rust systems programming.
- Keep utilities small and composable.
- Mimic familiar GNU coreutils behavior where possible.

---

## 📦 Utilities

| Command    | Status | Description |
|------------|--------|-------------|
| `fastcat`  | ✅ Done | Concatenate and print files (`cat`) |
| `fastgrep` | ✅ Done | Print lines matching a pattern (`grep`) |
| `fastwc`   | ⏳ WIP | Print newline, word, and byte counts (`wc`) |
| `fastls`   | ❌ Todo | List directory contents (`ls`) |
| `fasthead` | ❌ Todo | Output the first lines of a file (`head`) |
| `fasttail` | ❌ Todo | Output the last lines of a file (`tail`) |

---

## 🔧 Installation

Clone and install all binaries system-wide:

```bash
git clone https://github.com/yourusername/rustbox.git
cd rustbox
cargo install --path . --force

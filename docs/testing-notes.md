# neocmakelsp Integration Testing Notes

Test Date: 2026-05-20

## Test Steps

### 1. Build Extension
```bash
cargo build
```
Result: ✅ Passed (0 errors, 18 warnings)

### 2. Testing in Zed

#### Manual Testing Checklist:

1. **CMake LSP Activation**
   - Open a CMake project (containing CMakeLists.txt)
   - Open CMakeLists.txt file
   - Check LSP logs: `dev: open language server logs`
   - Expected: See "msvc-cmake-neocmake" LSP start

2. **PATH Lookup**
   - If neocmakelsp is in PATH
   - Expected: Log shows "found neocmakelsp in PATH: ..."

3. **GitHub Download Fallback**
   - If neocmakelsp is not in PATH
   - Expected: Log shows "neocmakelsp not found in PATH, attempting download"
   - Expected: Downloads matching platform asset from `neocmakelsp/neocmakelsp` latest release
   - Windows expected asset: `neocmakelsp-x86_64-pc-windows-msvc.zip`

4. **.neocmake.toml Configuration**
   - Create `.neocmake.toml` in project root:
     ```toml
     [format]
     enable = false

     [lint]
     enable = true
     ```
   - Expected: neocmakelsp reads this file itself
   - Expected: Extension does not parse or merge `.neocmake.toml`

5. **settings.json Override**
   - Create `.zed/settings.json`:
     ```json
     {
       "lsp": {
         "msvc-cmake-neocmake": {
           "format": {
             "enable": true
           }
         }
       }
     }
     ```
   - Expected: Log shows "settings.json override: format.enable = true"

6. **clangd Still Works**
   - Open C/C++ file
   - Expected: clangd LSP works normally

## Test Results

| Test Item | Status |
|-----------|--------|
| Extension Build | ✅ Passed |
| CMake LSP Activation | ⏳ Awaiting user testing in Zed |
| PATH Lookup | ⏳ Awaiting user testing in Zed |
| GitHub Download | ⏳ Awaiting user testing in Zed |
| .neocmake.toml Configuration | ⏳ Awaiting user testing in Zed |
| settings.json Override | ⏳ Awaiting user testing in Zed |
| clangd Compatibility | ⏳ Awaiting user testing in Zed |
| **CMake Language Recognition** | ⏳ Awaiting user testing in Zed |

## V0.5.0 Update (2026-05-20)

### Fixed Content
1. **Added CMake Language Definition**: Created `languages/cmake/` directory and config files
   - `config.toml` - Language configuration (matches CMakeLists.txt files)
   - `highlights.scm` - Syntax highlighting rules
   - `indents.scm` - Indentation rules
   - `injections.scm` - Injection rules
   - `textobjects.scm` - Text object rules

2. **Modified extension.toml**:
   - Changed `languages = ["cmake"]` to `language = "CMake"`
   - Added `[grammars.cmake]` entry for Tree-sitter grammar

### Test Steps
1. Rebuild extension: `cargo build --target wasm32-unknown-unknown --release`
2. Install to Zed extension directory
3. Open CMakeLists.txt file
4. **Expected Result**: File should be recognized as CMake language (no longer "plain text")
5. Check LSP logs: `dev: open language server logs`

## Notes

- Extension compiles to `target/debug/zed-msvc-toolkit.dll`
- Need to copy this DLL to Zed extension directory for testing
- Zed extension directory typically located at `%ZED_USER_EXTENSIONS_DIR%`
- neocmakelsp prioritizes binary in PATH; automatically downloads GitHub Release when not in PATH.

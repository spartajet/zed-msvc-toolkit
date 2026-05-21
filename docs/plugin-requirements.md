This is a Software Requirements Specification (SRS) for a **MSVC C++ Support Plugin** developed for the Zed editor. This specification focuses on Windows-based C++ development and debugging workflows using Visual Studio 2022+ and CMake.
---
# Software Requirements Specification (SRS): Zed MSVC C++ Assistant

## 1. Introduction

### 1.1 Purpose

This document aims to clarify the functional requirements, non-functional requirements, and constraints of the Zed editor extension `Zed MSVC C++ Assistant`. The plugin aims to address Zed's lack of native support for MSVC toolchain and CMake on Windows, providing out-of-the-box C++ intelligent editing, building, and debugging experience.

### 1.2 Scope

This plugin applies to the following technology stack:
- **Operating System**: Windows 10/11
- **Compiler Toolchain**: MSVC (installed with Visual Studio 2022 v143+ or higher)
- **Build System**: CMake (3.15+)
- **Editor**: Zed (latest stable version)
- **Language**: C/C++

**Included Features**:
- Automatically detect VS2022+ installation path and Windows SDK.
- Automatically generate and maintain `.clangd` configuration, ensuring `clangd` works accurately in MSVC environment.
- Assist with CMake configuration and `compile_commands.json` generation/integration.
- Integrate MSVC debugger (`vsdbg`) based on DAP, supporting PDB breakpoint debugging.

**Excluded Features**:
- Active support for older Visual Studio (2019 and below) (may be compatible but not guaranteed).
- IntelliSense support for non-CMake projects (e.g., pure `.sln`/`.vcxproj`).
- GUI-based CMake project configuration panel (like VSCode's CMake Tools sidebar, not implemented in V1).

---

## 2. Overall Description

### 2.1 User Characteristics and Scenarios

Target users are engineers using Zed for C++ development on Windows. They are familiar with CMake and MSVC, but are frustrated with Zed's default environment where header files show red, cannot recognize MSVC-specific macros (like `__declspec`), and cannot debug PDB programs.

### 2.2 Operating Environment and Constraints

- **Sandbox Constraints**: Plugin runs in Zed's Wasm sandbox, cannot directly execute `vcvarsall.bat` to set environment variables. Must parse file system or call external executables to obtain paths, and write results to configuration files or pass as command-line arguments.
- **Dependency Requirements**: User must have Visual Studio 2022 installed with "Desktop development with C++" workload, and `cmake.exe` must be in system PATH.

---

## 3. Functional Requirements

### 3.1 Environment Detection Module

**FR1.1: Visual Studio Instance Detection**
- Plugin must obtain VS2022+ installation path by executing `vswhere.exe` (hardcoded path: `C:\Program Files (x86)\Microsoft Visual Studio\Installer\vswhere.exe`).
- Must extract the highest version MSVC compiler path (e.g., `MSVC/14.xx.xxxxx`) and corresponding `include` and `lib` directories.

**FR1.2: Windows SDK Detection**
- Parse VS installation directory or registry to obtain installed Windows SDK version and `um`, `shared`, `ucrt` header file paths.

**FR1.3: CMake Detection**
- Verify if `cmake.exe` is available; if not, show error prompt in Zed.

---

### 3.2 LSP IntelliSense Support

**FR2.1: Dynamic `.clangd` Configuration Generation**
- When `.clangd` file does not exist in workspace, plugin automatically generates `.clangd` file in project root directory.
- Configuration content must include:
  - `CompileFlags.Compiler: clang-cl`: Instruct clangd to use clang-cl/MSVC-style argument interpretation.
  - `CompileFlags.Add`: Automatically inject detected MSVC and Windows SDK `/I` (include directory) arguments.

**FR2.2: CMake Compile Database Integration**
- **FR2.2.1**: If `compile_commands.json` exists in workspace root or `build/` directory, `.clangd` configuration must add `CompilationDatabase: <path>` pointing to that file.
- **FR2.2.2**: When CMake uses Visual Studio generator (e.g., `-G "Visual Studio 17 2022"`), it does not generate `compile_commands.json` in source root by default. Plugin must detect `build/.cmake/api/v1/reply` or prompt user to use Ninja generator (`-G "Ninja"`) to generate database.

**FR2.3: Language Server Startup**
- Use `clangd` from system PATH (if not found, prompt user to install).
- Startup arguments must include `--header-insertion=never` (avoid clangd auto-inserting headers conflicting with MSVC macros).

---

### 3.3 Build Assistance Module

**FR3.1: CMake Configure Command**
- Register Zed command `msvc-cpp: cmake configure`.
- On execution, automatically build command `cmake -B build -G "Ninja" -DCMAKE_BUILD_TYPE=Debug` (prioritize Ninja to ensure compile database generation; if system has no Ninja, fallback to VS generator).
- Output results in Zed bottom terminal panel.

**FR3.2: CMake Build Command**
- Register Zed command `msvc-cpp: cmake build`.
- Execute `cmake --build build --config Debug`.

---

### 3.4 Debug Support Module

**FR4.1: Debug Adapter Management**
- Plugin must embed or automatically download Microsoft's official `vsdbg.exe` (VS Code's DAP debug core).
- Download target path: `<Zed Extension Data Dir>/msvc-cpp/vsdbg.exe`.

**FR4.2: Debug Configuration Launch**
- Register DAP type `msvc-cpp`.
- When user presses F5 or starts via debug panel, plugin automatically generates `launch.json` or builds DAP launch parameters in memory based on currently open file or project configuration:
  - `program`: Points to `build/Debug/<target>.exe` (must combine CMake's `CMAKE_RUNTIME_OUTPUT_DIRECTORY` or default path inference).
  - `symbolPath`: Points to corresponding `.pdb` file.
  - `cwd`: Workspace path.

**FR4.3: Breakpoint and Variable Viewing**
- Relies on Zed's native DAP client capabilities, implementing source-level breakpoints, single-step execution, local variables and watch viewing through `vsdbg.exe`.

---

## 4. Non-Functional Requirements

### 4.1 Performance Requirements

- **NFR1.1**: Environment detection (`vswhere` etc.) total duration must not exceed 2 seconds, avoiding blocking editor startup.
- **NFR1.2**: Auto-generated `.clangd` file size should be controlled within reasonable range, avoiding injecting too many invalid paths causing slow `clangd` startup.

### 4.2 Usability Requirements

- **NFR2.1 (Zero-Config Philosophy)**: After user installs plugin and opens directory containing `CMakeLists.txt`, code highlighting and navigation should work immediately without manual configuration.
- **NFR2.2**: When required external dependencies (like Ninja, clangd) are missing, must provide clear one-click installation guidance or error messages through Zed's notification system, not silent failure.

### 4.3 Security Requirements

- **NFR3.1**: `vsdbg.exe` download must use HTTPS and verify Microsoft's official SHA256 hash, preventing supply chain attacks.

### 4.4 Compatibility Requirements

- **NFR4.1**: Plugin-generated `.clangd` file should not overwrite user's manually modified custom configuration; if `.clangd` already exists, should use merge strategy or prompt user before appending content.

---

## 5. Data and Interface Design

### 5.1 External Interface Calls

| Interface Name | Call Method | Purpose |
| :--- | :--- | :--- |
| `vswhere.exe` | Synchronous Shell execution | Obtain VS and MSVC installation absolute paths |
| `cmake.exe` | Zed async terminal task | Execute project Configure/Build |
| `clangd.exe` | Zed Language Server interface | Provide LSP service |
| `vsdbg.exe` | Zed Debug Adapter interface | Provide DAP debugging service |

### 5.2 Generated File Format Example (`.clangd`)

```yaml
# Auto-generated by Zed MSVC C++ Assistant
CompileFlags:
  Compiler: clang-cl
  Add:
    # MSVC standard library
    - /IC:/Program Files/Microsoft Visual Studio/2022/Community/VC/Tools/MSVC/14.38.33130/include
    # Windows SDK
    - /IC:/Program Files (x86)/Windows Kits/10/Include/10.0.22621.0/ucrt
    - /IC:/Program Files (x86)/Windows Kits/10/Include/10.0.22621.0/um
    - /IC:/Program Files (x86)/Windows Kits/10/Include/10.0.22621.0/shared
Diagnostics:
  Suppress: ['pp_file_not_found'] # Optimization: Suppress some false positives from deep SDK dependencies not found
```

---

## 6. Milestone Planning (Suggested)

- **V0.1 (MVP)**: Implement `vswhere` detection and `.clangd` auto-generation, enabling `clangd` IntelliSense in MSVC environment.
- **V0.2**: Integrate `vsdbg`, implementing basic PDB debugging (manually configure `launch.json`).
- **V1.0 (Release)**: Add CMake Configure/Build Zed command bindings, implement automatic inference and debug startup based on CMake output directory (zero-config debugging). Improve error handling and user prompts.

---

## Implementation Status

### V0.2 (Completed)

V0.2 adds on top of V0.1:
- `compile_commands.json` detection (root or `build/` subdirectory)
- `CompilationDatabase` configuration generation
- CMake project detection (`CMakeLists.txt` existence)

Detailed documentation: `docs/v0.1-usage.md`

### V0.1 (Completed)

V0.1 design and implementation plan has been split into:

- `docs/superpowers/specs/2026-05-20-zed-msvc-toolkit-design.md`
- `docs/superpowers/plans/2026-05-20-v0-1-msvc-clangd.md`
- `docs/v0.1-usage.md`

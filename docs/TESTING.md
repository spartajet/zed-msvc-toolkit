# Zed MSVC C++ Assistant - Testing Guide

## Running Unit Tests

```bash
# Run all tests
cargo test

# Run specific module tests
cargo test --lib cmake::tasks
cargo test --lib environment::msvc
cargo test --lib lsp::clangd_config

# Show test output
cargo test -- --nocapture

# Run specific test
cargo test test_ninja_generator_produces_correct_args
```

## Test Coverage

### CMake Module Tests

**File**: `src/cmake/tools.rs`

- `ninja_generator_produces_correct_args` - Ninja generator arguments
- `visual_studio_generator_produces_correct_args` - VS generator arguments
- `build_type_produces_correct_cmake_var` - Build type variable
- `build_type_produces_correct_build_arg` - Build argument
- `configure_command_for_ninja` - Ninja configure command
- `configure_command_for_visual_studio` - VS configure command
- `build_command_includes_config` - build command format
- `configure_command_arguments_are_separate` - Argument separation verification
- `source_dir_with_spaces_is_separate_argument` - Path space handling

**File**: `src/cmake/tasks.rs`

- `generate_tasks_json_creates_valid_json` - JSON format verification
- `tasks_include_configure_and_build` - Task completeness
- `tasks_use_workspace_root_variable` - Variable usage
- `custom_build_dir_and_type` - Custom configuration

**File**: `src/cmake/compile_db.rs`

- `find_compile_commands_in_root` - Root directory detection
- `find_compile_commands_in_build_subdir` - build subdirectory detection
- `returns_none_when_not_found` - File not found handling
- `parent_directory_is_root` - Return parent directory
- `parent_directory_is_build_subdir` - Return build subdirectory

### Environment Module Tests

**File**: `src/environment/msvc.rs`

- `select_latest_toolset_version` - Toolset version selection
- `select_toolset_from_directories` - Directory selection
- `empty_directory_list_returns_none` - Empty list handling
- `single_directory_is_selected` - Single directory handling
- `non_numeric_directories_are_ignored` - Non-numeric directory filtering

**File**: `src/environment/windows_sdk.rs`

- `sdk_paths_with_all_components` - Complete SDK paths
- `sdk_paths_with_missing_shared` - Missing shared component
- `empty_sdk_directories_returns_none` - Empty SDK handling
- `sdk_version_sorting` - Version sorting

### LSP Module Tests

**File**: `src/lsp/clangd_config.rs`

- `generates_clangd_config_with_msvc_paths` - MSVC path configuration
- `generates_fallback_config_without_sdk` - No SDK fallback
- `clangd_config_without_compile_db` - No compile database
- `clangd_config_with_compile_db` - With compile database
- `paths_with_spaces_are_quoted` - Path quoting handling
- `paths_without_spaces_are_not_quoted` - No-space paths

## Integration Testing

### Preparing Test Environment

1. Install required tools:
   ```bash
   # Check Visual Studio
   "C:\Program Files (x86)\Microsoft Visual Studio\Installer\vswhere.exe"

   # Check clangd
   clangd --version

   # Check CMake
   cmake --version
   ```

2. Create test CMake project:
   ```bash
   mkdir test-cmake-project
   cd test-cmake-project
   cat > CMakeLists.txt << 'EOF'
   cmake_minimum_required(VERSION 3.15)
   project(TestProject)

   add_executable(main main.cpp)
   EOF

   cat > main.cpp << 'EOF'
   #include <iostream>

   int main() {
       std::cout << "Hello, MSVC!" << std::endl;
       return 0;
   }
   EOF
   ```

3. Open test project in Zed

### Test Steps

#### 1. Language Server Startup

1. Open any `.c` or `.cpp` file
2. Open Zed's "Outline" panel
3. Verify clangd is running (should show symbol indexing)

#### 2. CMake Task Execution

1. Copy task file:
   ```bash
   cp docs/zed-tasks-example.json .zed/tasks.json
   ```

2. Open task panel: `Ctrl+Shift+T`

3. Run "CMake: Configure (Debug)"

4. Verify `build/` directory is generated

5. Run "CMake: Build (Debug)"

6. Verify executable is generated

#### 3. Compile Database Testing

1. Configure project (if not already configured):
   ```bash
   cmake -B build -DCMAKE_EXPORT_COMPILE_COMMANDS=ON
   ```

2. Verify `build/compile_commands.json` exists

3. Open C++ file in Zed

4. Verify code navigation and autocomplete work normally

## Manual Verification Checklist

- [ ] clangd starts automatically when opening C/C++ files
- [ ] Header file navigation (F12) works normally
- [ ] Code completion shows MSVC standard library symbols
- [ ] Task panel shows CMake tasks
- [ ] CMake Configure successfully generates build files
- [ ] CMake Build successfully generates executable
- [ ] `compile_commands.json` is automatically detected
- [ ] clangd uses compile database for code analysis

## Debugging Test Failures

### WASM Test Limitations

Unit tests cannot run directly on WASM target:
```bash
# This will fail
cargo test --target wasm32-unknown-unknown
```

Use host target to run:
```bash
# This will work
cargo test
```

### View Detailed Output

```bash
# Show test output
cargo test -- --nocapture

# Show detailed test information
cargo test -- --show-output

# Run but ignore errors (see all results)
cargo test -- --no-fail-fast
```

## Performance Testing

### Measure Extension Startup Time

1. Open Zed logs: `Ctrl+Shift+P` → "Zed: Open Logs"
2. Search for "zed-msvc-toolkit" related messages
3. Check language server startup duration

### Measure clangd Indexing Time

1. Open large C++ project
2. Watch clangd logs for indexing progress
3. Record complete indexing duration

## Continuous Integration

Local CI testing workflow:
```bash
# Format check
cargo fmt -- --check

# Clippy check
cargo clippy -- -D warnings

# Unit tests
cargo test

# WASM compile check
cargo build --target wasm32-unknown-unknown --release
```

## Reporting Issues

When tests fail, please include:
1. Zed version
2. Windows version
3. Visual Studio version
4. Error messages or logs
5. Reproduction steps
6. Minimal test project (if applicable)

# CMake 3.15+ selects the MSVC runtime per target under CMP0091. Raylib's
# CMake project enables that policy, so compiler flags alone can be overridden
# by its default DLL runtime. Load this file via CMAKE_TOOLCHAIN_FILE before
# project() creates Raylib and GLFW targets.
set(CMAKE_POLICY_DEFAULT_CMP0091 NEW)

# Rust's +crt-static and cc's static_crt use /MT in both Cargo profiles.
# /MTd would introduce a separate debug CRT and incompatible allocations.
set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreaded" CACHE STRING
    "Match Rust's statically linked MSVC runtime in every profile" FORCE)

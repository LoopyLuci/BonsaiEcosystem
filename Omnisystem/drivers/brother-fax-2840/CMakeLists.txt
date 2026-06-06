cmake_minimum_required(VERSION 3.24)
project(BrotherFAXDriver
    VERSION 1.0.0
    DESCRIPTION "macOS DriverKit driver for Brother IntelliFAX 2840"
    LANGUAGES CXX
)

# ============================================================================
# Build Configuration
# ============================================================================

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_VISIBILITY_PRESET hidden)
set(CMAKE_VISIBILITY_INLINES_HIDDEN ON)

# Apple-specific settings
set(CMAKE_OSX_DEPLOYMENT_TARGET "11.0" CACHE STRING "Minimum macOS version")
set(CMAKE_OSX_ARCHITECTURES "arm64;x86_64" CACHE STRING "Supported architectures")

# ============================================================================
# DriverKit Extension Target
# ============================================================================

add_library(BrotherFAXDriver MODULE
    BrotherFAXDriver.cpp
    BrotherFAXDriver.hpp
)

# DriverKit framework location (typically in Xcode)
set(DRIVERKIT_FRAMEWORK_PATH
    "/Applications/Xcode.app/Contents/Developer/Platforms/DriverKit.platform/Developer/SDKs/DriverKit.sdk"
    CACHE PATH "Path to DriverKit SDK"
)

target_include_directories(BrotherFAXDriver PRIVATE
    ${DRIVERKIT_FRAMEWORK_PATH}/usr/include
)

target_link_directories(BrotherFAXDriver PRIVATE
    ${DRIVERKIT_FRAMEWORK_PATH}/usr/lib
)

target_link_libraries(BrotherFAXDriver PRIVATE
    DriverKit
)

# Compiler flags for DriverKit
target_compile_options(BrotherFAXDriver PRIVATE
    -fno-common
    -fno-strict-aliasing
    -fno-builtin
    -fPIC
    -mkernel
    -static
    -nostdinc
    -Werror=deprecated-declarations
    -Wall
    -Wextra
)

# Set bundle properties
set_target_properties(BrotherFAXDriver PROPERTIES
    BUNDLE TRUE
    BUNDLE_EXTENSION "dext"
    MACOSX_BUNDLE_INFO_PLIST "${CMAKE_CURRENT_SOURCE_DIR}/Info.plist"
    MACOSX_BUNDLE_BUNDLE_NAME "BrotherFAXDriver"
    MACOSX_BUNDLE_BUNDLE_VERSION "1.0.0"
    MACOSX_BUNDLE_SHORT_VERSION_STRING "1.0"
    MACOSX_BUNDLE_COPYRIGHT "Copyright © 2026 Bonsai Ecosystem"
)

# ============================================================================
# Installation
# ============================================================================

install(
    TARGETS BrotherFAXDriver
    BUNDLE DESTINATION "Library/SystemExtensions"
)

# ============================================================================
# Code Signing
# ============================================================================

# For development, sign with ad-hoc identity
# For production, use proper code signing certificate
if(APPLE)
    set(CODESIGN_IDENTITY "-" CACHE STRING "Code signing identity")

    add_custom_command(TARGET BrotherFAXDriver POST_BUILD
        COMMAND /usr/bin/codesign
            -s "${CODESIGN_IDENTITY}"
            -f
            --entitlements "${CMAKE_CURRENT_SOURCE_DIR}/Entitlements.plist"
            "$<TARGET_BUNDLE_DIR:BrotherFAXDriver>"
        COMMENT "Signing DriverKit extension"
        VERBATIM
    )
endif()

# ============================================================================
# Documentation
# ============================================================================

message(STATUS "BrotherFAXDriver v${PROJECT_VERSION}")
message(STATUS "  macOS Deployment Target: ${CMAKE_OSX_DEPLOYMENT_TARGET}")
message(STATUS "  Architectures: ${CMAKE_OSX_ARCHITECTURES}")
message(STATUS "  Build Type: ${CMAKE_BUILD_TYPE}")

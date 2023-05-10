set(HttpSyncVersion "http_sync-v0.0.1") # generated; do not edit

# TODO Remove this workaround once Flutter supports Windows ARM.
# https://github.com/flutter/flutter/issues/116196
set(FLUTTER_TARGET_PLATFORM windows-x64)

# The Flutter tooling requires that developers have a version of Visual Studio
# installed that includes CMake 3.14 or later. You should not increase this
# version, as doing so will cause the plugin to fail to compile for some
# customers of the plugin.
cmake_minimum_required(VERSION 3.14)

# Project-level configuration.
set(PROJECT_NAME "flutter_http_sync")
project(${PROJECT_NAME} LANGUAGES CXX)

# Download the binaries if they are not already present.
set(HttpSyncLibRoot "${CMAKE_CURRENT_SOURCE_DIR}/${HttpSyncVersion}")
set(HttpSyncArchivePath "${HttpSyncLibRoot}.tar.gz")
if(NOT EXISTS ${HttpSyncArchivePath})
  file(DOWNLOAD
    "https://github.com/chinabrant/http_sync/releases/download/${HttpSyncVersion}/EmbeddedHttpSync.tar.gz"
    ${HttpSyncArchivePath}
    TLS_VERIFY ON
  )
endif()

# Extract the binaries, overriding any already present.
file(REMOVE_RECURSE ${HttpSyncLibRoot})
file(MAKE_DIRECTORY ${HttpSyncLibRoot})
execute_process(
  COMMAND ${CMAKE_COMMAND} -E tar xzf ${HttpSyncArchivePath}
  WORKING_DIRECTORY ${HttpSyncLibRoot}
)

# List of absolute paths to libraries that should be bundled with the plugin.
# This list could contain prebuilt libraries, or libraries created by an
# external build triggered from this build file.
set(flutter_http_sync_bundled_libraries
  "${HttpSyncLibRoot}/${FLUTTER_TARGET_PLATFORM}/embedded_http_sync.dll"
  PARENT_SCOPE
)

#!/bin/bash

CURR_VERSION=httpsync-v`awk '/^version: /{print $2}' packages/http_sync/pubspec.yaml`

# iOS & macOS
APPLE_HEADER="release_tag_name = '$CURR_VERSION' # generated; do not edit"
sed -i.bak "1 s/.*/$APPLE_HEADER/" packages/flutter_http_sync/ios/flutter_http_sync.podspec
sed -i.bak "1 s/.*/$APPLE_HEADER/" packages/flutter_http_sync/macos/flutter_http_sync.podspec
rm packages/flutter_http_sync/macos/*.bak packages/flutter_http_sync/ios/*.bak

# CMake platforms (Linux, Windows, and Android)
CMAKE_HEADER="set(HttpSyncVersion \"$CURR_VERSION\") # generated; do not edit"
for CMAKE_PLATFORM in android linux windows
do
    sed -i.bak "1 s/.*/$CMAKE_HEADER/" packages/flutter_http_sync/$CMAKE_PLATFORM/CMakeLists.txt
    rm packages/flutter_http_sync/$CMAKE_PLATFORM/*.bak
done

git add packages/flutter_http_sync/

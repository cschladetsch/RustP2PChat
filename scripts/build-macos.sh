#!/bin/bash

# Build script for macOS installer
set -e

APP_NAME="RustP2PChat"
VERSION="0.1.0"
BUNDLE_ID="com.rustchat.p2pchat"

echo "Building macOS installer for $APP_NAME v$VERSION..."

# Add macOS targets if not already added
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin

# Build for both architectures
echo "Building for x86_64-apple-darwin..."
cargo build --release --target x86_64-apple-darwin

echo "Building for aarch64-apple-darwin..."
cargo build --release --target aarch64-apple-darwin

# Create universal binary
echo "Creating universal binary..."
mkdir -p target/universal-apple-darwin/release
lipo -create \
    target/x86_64-apple-darwin/release/rust-p2p-chat \
    target/aarch64-apple-darwin/release/rust-p2p-chat \
    -output target/universal-apple-darwin/release/rust-p2p-chat

# Create app bundle structure
APP_BUNDLE="$APP_NAME.app"
echo "Creating app bundle: $APP_BUNDLE"

rm -rf "$APP_BUNDLE"
mkdir -p "$APP_BUNDLE/Contents/MacOS"
mkdir -p "$APP_BUNDLE/Contents/Resources"

# Copy binary
cp target/universal-apple-darwin/release/rust-p2p-chat "$APP_BUNDLE/Contents/MacOS/$APP_NAME"
chmod +x "$APP_BUNDLE/Contents/MacOS/$APP_NAME"

# Create Info.plist
cat > "$APP_BUNDLE/Contents/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>$APP_NAME</string>
    <key>CFBundleIdentifier</key>
    <string>$BUNDLE_ID</string>
    <key>CFBundleName</key>
    <string>$APP_NAME</string>
    <key>CFBundleVersion</key>
    <string>$VERSION</string>
    <key>CFBundleShortVersionString</key>
    <string>$VERSION</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleSignature</key>
    <string>????</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.15</string>
    <key>LSApplicationCategoryType</key>
    <string>public.app-category.utilities</string>
    <key>NSHighResolutionCapable</key>
    <true/>
</dict>
</plist>
EOF

echo "App bundle created: $APP_BUNDLE"

# Create DMG
DMG_NAME="$APP_NAME-$VERSION.dmg"
echo "Creating DMG: $DMG_NAME"

# Create temporary directory for DMG contents
DMG_TEMP="dmg_temp"
rm -rf "$DMG_TEMP"
mkdir "$DMG_TEMP"

# Copy app bundle to temp directory
cp -R "$APP_BUNDLE" "$DMG_TEMP/"

# Create Applications symlink
ln -s /Applications "$DMG_TEMP/Applications"

# Create DMG
hdiutil create -volname "$APP_NAME" -srcfolder "$DMG_TEMP" -ov -format UDZO "$DMG_NAME"

# Clean up
rm -rf "$DMG_TEMP"

echo "macOS installer created: $DMG_NAME"
echo "To install: Mount the DMG and drag $APP_NAME.app to Applications folder"
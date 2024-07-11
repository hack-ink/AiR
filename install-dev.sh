cargo bundle --release
codesign --deep --force --verify --verbose --sign "Xavier Lau" target/release/bundle/osx/AiR.app
rm -rf /Applications/AiR.app && cp -r target/release/bundle/osx/AiR.app /Applications/AiR.app

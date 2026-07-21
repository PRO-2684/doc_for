#!/bin/bash

# If no arguments are passed, print the version
if [ $# -eq 0 ]; then
    # Exit with git's exit code
    git describe --tags --abbrev=0
    exit $?
fi

# If more than one argument is passed, print an error message
if [ $# -gt 1 ]; then
    echo "Error: Too many arguments"
    exit 1
fi

# Otherwise, set the version
version=$1
# Check if the version is valid
if [[ ! $version =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo "Error: Invalid version format. Use X.Y.Z"
    exit 1
fi
# Check if the version already exists
if git tag | grep -q "v$version"; then
    echo "Error: Version $version already exists"
    exit 1
fi
# Set version in Cargo.toml
sed -i "s/^version = \".*\"/version = \"$version\"/" Cargo.toml
# Also updating the line `doc_for_derive = { path = "doc_for_derive", version = "0.1.3" }`
sed -i "s/^doc_for_derive = { path = \"doc_for_derive\", version = \".*\" }/doc_for_derive = { path = \"doc_for_derive\", version = \"$version\" }/" Cargo.toml
# Set version in doc_for_derive/Cargo.toml
sed -i "s/^version = \".*\"/version = \"$version\"/" doc_for_derive/Cargo.toml
cargo generate-lockfile && cd doc_for_derive && cargo generate-lockfile && cd ..
# Commit the changes
git add Cargo.toml doc_for_derive/Cargo.toml Cargo.lock doc_for_derive/Cargo.lock
git commit -S -m "Bump version to v$version"
# Create a new tag
git tag -s v$version -m "Version $version"

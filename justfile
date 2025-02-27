alias p := publish
alias t := test
alias v := version

# Publish
[doc("\u{001b}[4mP\u{001b}[24mublish")]
publish:
    cd doc_for_derive && cargo publish && cd .. && cargo publish

# Run tests
[doc("Run \u{001b}[4mt\u{001b}[24mests")]
test *args:
    cargo test {{args}}

# Set version
[doc("Set \u{001b}[4mv\u{001b}[24mersion")]
version VERSION:
    # Set version in Cargo.toml
    sed -i "s/version = \".*\"/version = \"{{VERSION}}\"/" Cargo.toml
    # Set version in doc_for_derive/Cargo.toml
    sed -i "s/version = \".*\"/version = \"{{VERSION}}\"/" doc_for_derive/Cargo.toml
    cargo generate-lockfile
    # Add changes to git
    git add Cargo.toml doc_for_derive/Cargo.toml Cargo.lock
    # Commit changes
    git commit -S -m "Bump version to v{{VERSION}}"
    # Tag version
    git tag -s v{{VERSION}} -m "Version v{{VERSION}}"

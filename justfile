alias p := publish
alias t := test
alias v := version

default:
  just --list

# Publish
[doc("\u{001b}[4mP\u{001b}[24mublish")]
publish:
    cd doc_for_derive && cargo publish && cd .. && cargo publish

# Run tests
[doc("Run \u{001b}[4mt\u{001b}[24mests")]
test *args:
    cargo test {{args}}

# Set or get version
[doc("Set or get \u{001b}[4mv\u{001b}[24mersion")]
version VERSION="none":
    @if [ "{{VERSION}}" = "none" ]; then \
        git describe --tags --abbrev=0; \
    else \
        just set-version {{VERSION}}; \
    fi

# Set version
[doc("Set version")]
set-version VERSION:
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

alias p := publish
alias t := test

# Publish
[doc("\u{001b}[4mP\u{001b}[24mublish")]
publish:
    cd doc_for_derive && cargo publish && cd .. && cargo publish

# Tests
[doc("\u{001b}[4mT\u{001b}[24mests")]
test *args:
    cargo test {{args}}

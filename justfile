set shell := ["cmd.exe", "/c"]

dev:
    cargo watch -q -w src/ -x run

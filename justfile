set shell := ["powershell.exe", "-c"]

fmt:
    cargo fmt; cd vscode-salt; yarn format
clippy:
    cargo clippy --all-targets --all-features
run:
    cargo run
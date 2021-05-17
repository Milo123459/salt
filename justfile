set shell := ["powershell.exe", "-c"]

fmt:
    cargo fmt
clippy:
    cargo clippy --all-targets --all-features
run:
    cargo run
extension-dev:
    cd vscode-salt; yarn watch
install: 
    cargo build; cd vscode-salt; yarn
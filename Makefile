build:
	cargo build --release

run:
	cargo run --release

clean:
	cargo clean

rayon:
	cargo build --release --features rayon

linux:
	cargo build --release --target=x86_64-unknown-linux-musl

linux-rayon:
	cargo build --release --features rayon --target=x86_64-unknown-linux-musl

windows:
	cargo build --release --target=x86_64-pc-windows-gnu

windows-rayon:
	cargo build --release --features rayon --target=x86_64-pc-windows-gnu

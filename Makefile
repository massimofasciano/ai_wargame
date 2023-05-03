build:
	cargo build --release

run:
	cargo run --release -- auto

clean:
	cargo clean
	docker rmi joseluisq/rust-linux-darwin-builder

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

wasi:
	cargo wasi build --release

wasi-run:
	cargo wasi run --release -- auto

mac-intel:
	# cargo build --release --target x86_64-apple-darwin --features rayon
	docker run --rm \
    --volume "${PWD}":/root/src \
    --workdir /root/src \
      joseluisq/rust-linux-darwin-builder:1.69.0 \
        sh -c "cargo build --release --target x86_64-apple-darwin --features rayon"
	# docker rmi joseluisq/rust-linux-darwin-builder

mac-arm:
	# cargo build --release --target aarch64-apple-darwin --features rayon
	docker run --rm \
    --volume "${PWD}":/root/src \
    --workdir /root/src \
      joseluisq/rust-linux-darwin-builder:1.69.0 \
        sh -c "cargo build --release --target aarch64-apple-darwin --features rayon"
	# docker rmi joseluisq/rust-linux-darwin-builder

dist-windows: windows-rayon
	rm -f dist/ai_wargame_win.zip
	cp target/x86_64-pc-windows-gnu/release/ai_wargame.exe dist
	cd dist ; zip -9 ai_wargame_win.zip ai_wargame.exe
	rm -f dist/ai_wargame.exe

dist-linux: linux-rayon
	rm -f dist/ai_wargame_linux.tar.gz
	cp target/x86_64-unknown-linux-musl/release/ai_wargame dist
	strip -s dist/ai_wargame
	cd dist ; tar -zcvf ai_wargame_linux.tar.gz ai_wargame
	rm -f dist/ai_wargame

dist-mac-intel: mac-intel
	rm -f dist/ai_wargame_mac_intel.tar.gz
	cp target/x86_64-apple-darwin/release/ai_wargame dist
	cd dist ; tar -zcvf ai_wargame_mac_intel.tar.gz ai_wargame
	rm -f dist/ai_wargame

dist-mac-arm: mac-arm
	rm -f dist/ai_wargame_mac_arm.tar.gz
	cp target/aarch64-apple-darwin/release/ai_wargame dist
	cd dist ; tar -zcvf ai_wargame_mac_arm.tar.gz ai_wargame
	rm -f dist/ai_wargame

dist-wasi: wasi
	rm -f dist/ai_wargame_wasm.tar.gz
	cp target/wasm32-wasi/release/ai_wargame.wasm dist
	cd dist ; zip -9 ai_wargame_wasm.zip ai_wargame.wasm
	rm -f dist/ai_wargame.wasm

dist: dist-linux dist-windows dist-wasi

dist-docker: dist-mac-intel dist-mac-arm

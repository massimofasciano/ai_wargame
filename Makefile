console = --no-default-features --features "console"
openssl-vendored = --features "openssl-vendored"
wasi = --no-default-features --features "wasi"

broker = --broker http://localhost:8001/test

build:
	cargo build $(console) --release

run:
	cargo run $(console) --release -- -p auto

broker-attacker:
	cargo run $(console) --release -- -p attacker $(broker)

broker-defender:
	cargo run $(console) --release -- -p defender $(broker)

clean:
	cargo clean
	@echo "after docker builds, might need to run: sudo rm -rf target/*-apple-darwin"

linux:
	cargo build $(console) $(openssl-vendored) --release --target=x86_64-unknown-linux-musl

windows:
	cargo build $(console) --release --target=x86_64-pc-windows-gnu

wasi:
	cargo wasi build $(wasi) --release

wasi-run:
	cargo wasi run $(wasi) --release -- -p auto

mac-intel:
	# cargo build --release --target x86_64-apple-darwin --features rayon
	docker run --rm \
    --volume "${PWD}":/root/src \
    --workdir /root/src \
      joseluisq/rust-linux-darwin-builder:1.69.0 \
        sh -c "cargo build $(console) --release --target x86_64-apple-darwin"

mac-arm:
	# cargo build --release --target aarch64-apple-darwin --features rayon
	docker run --rm \
    --volume "${PWD}":/root/src \
    --workdir /root/src \
      joseluisq/rust-linux-darwin-builder:1.69.0 \
        sh -c "cargo build $(console) --release --target aarch64-apple-darwin"

dist-windows: windows
	rm -f dist/ai_wargame_win.zip
	cp target/x86_64-pc-windows-gnu/release/ai_wargame.exe dist
	cd dist ; zip -9 ai_wargame_win.zip ai_wargame.exe
	rm -f dist/ai_wargame.exe

dist-linux: linux
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

.PHONY : dist
dist: dist-linux dist-windows dist-wasi

dist-docker: dist-mac-intel dist-mac-arm

clean-docker:
	# check the image id (joseluisq/rust-linux-darwin-builder:1.69.0)
	docker rmi 30eb5e48dfa2


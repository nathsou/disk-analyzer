
build: clean build_api build_front
	cp api/target/release/disk-analyzer ./disk-analyzer
	cp -rf front/dist ./disk-analyzer/front

build_api:
	(cd api && cargo build --release)

build_front:
	(cd front && npm install && npm run build)

clean:
	rm -rf disk-analyzer
	mkdir -p disk-analyzer

cross: clean build_front
	cargo install cross

	(cd api && cross build --release --target=x86_64-apple-darwin)
	(cd api && cross build --release --target=aarch64-apple-darwin)
	(cd api && cross build --release --target=x86_64-unknown-linux-gnu)
	(cd api && cross build --release --target=x86_64-pc-windows-gnu)

	cp api/target/x86_64-apple-darwin/release/disk-analyzer ./disk-analyzer/x86_64-apple-darwin/
	cp api/target/aarch64-apple-darwin/release/disk-analyzer ./disk-analyzer/aarch64-apple-darwin
	cp api/target/x86_64-unknown-linux-gnu/release/disk-analyzer ./disk-analyzer/x86_64-unknown-linux-gnu
	cp api/target/x86_64-pc-windows-gnu/release/disk-analyzer ./disk-analyzer/x86_64-pc-windows-gnu

	cp -rf front/dist ./disk-analyzer/x86_64-apple-darwin/front
	cp -rf front/dist ./disk-analyzer/aarch64-apple-darwin/front
	cp -rf front/dist ./disk-analyzer/x86_64-unknown-linux-gnu/front
	cp -rf front/dist ./disk-analyzer/x86_64-pc-windows-gnu/front



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
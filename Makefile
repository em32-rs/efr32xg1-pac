all: src

lib.rs build.rs device.x: EFR32MG1P133F256GM48.svd
	svd2rust -i $<

src: lib.rs
	form -i lib.rs -o src
	rm lib.rs
	cargo fmt

clean:
	rm -rf src lib.rs build.rs device.x

.PHONY: all clean

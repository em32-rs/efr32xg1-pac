all: src

EFR32MG1P133F256GM48.svd:
	echo "Please extract this file from https://www.silabs.com/documents/public/cmsis-packs/SiliconLabs.EFR32MG1P_DFP.5.5.0.pack or a more recent source"
	exit 1

lib.rs build.rs device.x: EFR32MG1P133F256GM48.svd
	svd2rust -i $<
	rm lib.rs

src: lib.rs
	form -i lib.rs -o src
	cargo fmt

clean:
	rm -rf src lib.rs build.rs device.x

.PHONY: all clean

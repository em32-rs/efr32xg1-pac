all: src

%.normalized-svd: %.svd
	# If the absence of sramSize and flashSize starts upsetting svd2rust, the whole approach must be changed
	sed $< \
		-e 's/<name>EFR32.G1[A-Z].*<\/name>/<name>EFR32xG1<\/name>/' \
		-e 's/<description>Silicon Labs EFR32.G1[A-Z].* Cortex-M MCU<\/description>/<description>Silicon Labs EFR32xG1 Cortex-M MCU<\/description>/' \
		-e '/<flashSize>0x........<\/flashSize>/d' \
		-e '/<sramSize>0x........<\/sramSize>/d' \
		> $@

check: $(patsubst %.svd,%.normalized-svd,$(wildcard *.svd))
	for x in $^; do diff $< $$x; done

lib.rs build.rs device.x: EFR32MG1P133F256GM48.normalized-svd
	svd2rust -i $<

src: check lib.rs
	form -i lib.rs -o src
	rm lib.rs
	cargo fmt

clean:
	rm -rf src lib.rs build.rs device.x
	rm -rf *.normalized-svd

.PHONY: all clean check

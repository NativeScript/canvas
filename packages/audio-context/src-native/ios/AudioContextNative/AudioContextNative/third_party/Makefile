PREFIX := $(CURDIR)/build/usr
JOBS := $(shell getconf _NPROCESSORS_ONLN 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo 1)

.PHONY: all libogg opus opusfile clean

all: libogg opus opusfile

libogg:
	@if [ -d libogg ]; then \
		cd libogg && ./configure --disable-shared --enable-static --prefix=$(PREFIX) && make -j$(JOBS) && make install; \
	else \
		echo "libogg source not found; run fetch_opus_deps.sh"; exit 1; \
	fi

opus:
	@if [ -d opus ]; then \
		cd opus && ./configure --disable-shared --enable-static --prefix=$(PREFIX) && make -j$(JOBS) && make install; \
	else \
		echo "opus source not found; run fetch_opus_deps.sh"; exit 1; \
	fi

opusfile:
	@if [ -d opusfile ]; then \
		cd opusfile && ./configure --disable-shared --enable-static --with-ogg=$(PREFIX) --with-opus=$(PREFIX) --prefix=$(PREFIX) && make -j$(JOBS) && make install; \
	else \
		echo "opusfile source not found; run fetch_opus_deps.sh"; exit 1; \
	fi

clean:
	rm -rf build

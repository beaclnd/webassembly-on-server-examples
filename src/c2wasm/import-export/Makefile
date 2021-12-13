CC = clang

export: export.c 
	${CC} --target=wasm32-wasi \
		--sysroot ~/Workspace/applications/wasi-sdk-14.0/share/wasi-sysroot \
		-nostartfiles \
		-O3 \
		-flto \
		-Wl,--no-entry,--export-dynamic,--lto-O3 \
		-o export.wasm export.c

import: import.c
	${CC} --target=wasm32-wasi \
		--sysroot ~/Workspace/applications/wasi-sdk-14.0/share/wasi-sysroot \
		-nostartfiles \
		-O3 \
		-flto \
		-Wl,--no-entry,--export-dynamic,--import-undefined,--import-memory,--lto-O3 \
		-o import.wasm import.c
	wasm2wat import.wasm > import.wat
	sed -i 's/(export.*placeHolderForMemoryExport.*)/(export \"memory\" (memory 0))/' import.wat
	wat2wasm import.wat > import.wasm

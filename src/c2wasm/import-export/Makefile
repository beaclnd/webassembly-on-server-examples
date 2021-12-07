CC = clang

default: export.c 
	${CC} --target=wasm32-wasi \
		--sysroot ~/Workspace/applications/wasi-sdk-14.0/share/wasi-sysroot \
		-nostartfiles \
		-Wl,--no-entry,--export-dynamic \
		-O3 \
		-o export.wasm export.c
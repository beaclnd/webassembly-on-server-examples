CC = clang

default: hello-world.c
	${CC} --target=wasm32-wasi \
		--sysroot ~/Workspace/applications/wasi-sdk-14.0/share/wasi-sysroot \
		-O3 \
		-o hello-world.wasm hello-world.c  
ENTRY_POINT = blogee
TEMP_ENTRY_POINT = tmp
APP_ENTRY_POINT = src/${ENTRY_POINT}.cr
APP_TEMP_FOR_COMPILE = src/${TEMP_ENTRY_POINT}.cr
TARGET = ./build/release/
PWD = $(shell pwd)

.ONESHELL:
release:
	@cat ${APP_ENTRY_POINT} >> ${APP_TEMP_FOR_COMPILE} <<EOF
	require "llvm/lib_llvm"
	require "llvm/enums"
	EOF

	docker run --rm -it -v ${PWD}:/app -w /app jrei/crystal-alpine crystal build --static --release ${APP_TEMP_FOR_COMPILE}
	rm ${APP_TEMP_FOR_COMPILE}
	mkdir -p ${TARGET}
	@mv -f ${TEMP_ENTRY_POINT} ${TARGET}/${ENTRY_POINT}

clean:
	@rm -rf ./build/

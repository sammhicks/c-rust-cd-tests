
SRCS := src.c

BIN := bin

.PHONY: all
all: ${BIN}

${BIN}: ${SRCS}
	$(CC) -Wall -Wpedantic -o $@ $^

.PHONY: clean
clean:
	${RM} ${BIN}

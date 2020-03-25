CC := gcc
CFLAGS = -std=c99 -O0
TARGET = compiler_2

all: $(TARGET)

$(TARGET): lexer.c compiler_2.c main.c
	$(CC) $(CFLAGS) $^ -o $@

clean:
	rm -f *.o *.exe

#include <stdio.h>
#include <fcntl.h>
#include <errno.h>
#include <stdlib.h>
#include <stdint.h>
#include <unistd.h>
#include <string.h>

int stub_function(int input) {
    printf("Message from C stub : %d\n", input);
    return input * 3;
}
void print_function(char *str) {
    printf("%s", str);
}

uint8_t* read_10_mb(char *filename) {
	int f;
	uint8_t *ptr;

	ptr = (uint8_t *) malloc(10000000);
	if (ptr == NULL) {
		fprintf(stderr, "Error allocating 10MB of memory\n");
		return (void *) 0;
	}
	memset(ptr, 0, 10000000);

	f = open(filename, O_RDONLY);
	if (f == -1) {
		fprintf(stderr, "Error opening file: %s\n", filename);
		return (void *) 0;
	}

	close(f);
	
	return ptr;
}

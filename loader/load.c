#include <stdio.h>
#include <stdlib.h>
#include <dlfcn.h>
 
int main() {
  	void *solib;
  	int (*respond_promptly)(int v);
	char *error;
 
  	solib = dlopen("../target/release/liboxidizer.so", RTLD_LAZY);
  	if ( !solib ) {
		fprintf(stderr, "%s\n", dlerror());
		exit(1);
	}
	respond_promptly = dlsym(solib, "respond_promptly");
	if ((error = dlerror()) != NULL) {
		fprintf(stderr, "%s\n", error);
		exit(2);
	}

	fprintf(stdout, "Answer for %d is: %d\n", 1, respond_promptly(1));
	fprintf(stdout, "Answer for %d is: %d\n", 3, respond_promptly(3));

	if (solib != NULL ) dlclose(solib);
	return EXIT_SUCCESS;
}

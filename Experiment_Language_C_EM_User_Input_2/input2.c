#include <stdio.h>
#include <emscripten.h>

// return the nth value of the fibonacci sequence
int EMSCRIPTEN_KEEPALIVE fib (int n) {
	if (n == 0 || n == 1)
		return 1;
	else
		return fib(n - 1) + fib(n - 2);
}

// return testing... and the parameter to the function
const char * EMSCRIPTEN_KEEPALIVE hello (char *word) {
	puts("testing ...");
	return word;
}

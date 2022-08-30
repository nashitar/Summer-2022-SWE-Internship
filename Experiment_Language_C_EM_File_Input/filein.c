#include <stdio.h>
int print (char* path){ // return type: int, paratemers: file path
	FILE *file = fopen (path, "r"); // open file for reading
	if (!file) { // if file var is NULL, the program was unable to open the file and should abort
		printf("could not open file\n");
		return 1;
	}
	while (!feof(file)) { 
		// until the end of the file has been reached, get the current character and write it to standard output
		char c = fgetc(file);
		if (c != EOF) { // excluding the end of file character
			putchar (c);
		}
	}
	fclose(file); // close the file to prevent memory errors/warnings
	return 0;
}


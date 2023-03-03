#include <stdio.h>
#include <stdlib.h>

const int VERY_LONG = 1000000000;
const int QUITE_SHORT = 10;

int main() {
  int i = 0;
  char *str;

  while (i < VERY_LONG) {
    printf("run %d\n", i);
    str = malloc(4096 * sizeof(char));
    free(str);
    free(str);
    i++;
  }

  return 0;
}


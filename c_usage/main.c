#include "dict.h"
#include <stdio.h>

int main(void) {
  printf("\nC program:\n\n");
  Dictionary *d = dict_new();

  char one[] = "one";
  dict_insert(d, 1, (uint8_t *)one, 3);

  char two[] = "two";
  dict_insert(d, 2, (uint8_t *)two, 3);

  char three[] = "three";
  dict_insert(d, 3, (uint8_t *)three, 5);

  if (!dict_contains(d, 1)) {
    printf("no 1\n");
  }

  if (dict_contains(d, 2)) {
    printf("has 2\n");
  }

  size_t len;
  uint8_t *ret = dict_get(d, 3, &len);
  for (uint8_t i = 0; i < len; i++) {
    printf("%c", ret[i]);
  }
  printf("\n");

  dict_remove(d, 1);

  if (!dict_contains(d, 2)) {
    printf("no 1\n");
  }

  dict_free(d);

  return 0;
}

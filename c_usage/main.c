#include "dict.h"
#include <stdio.h>

int main(void) {
  Dictionary *d = dict_new();

  const char *s = "hello from C";
  dict_insert(d, 1, (const uint8_t *)s, 12);

  if (dict_contains(d, 1)) {
    size_t len = 0;
    const uint8_t *p = dict_get(d, 1, &len);
    printf("key=1 len=%zu val=%.*s\n", len, (int)len, (const char *)p);
  }

  dict_free(d);
  return 0;
}

#pragma once
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

typedef struct Dictionary Dictionary;

Dictionary *dict_new(void);

void dict_insert(Dictionary *dict, uint64_t key, uint8_t *val, size_t len);
bool dict_contains(Dictionary *dict, uint64_t key);

uint8_t *dict_get(Dictionary *dict, uint64_t key, size_t *out_len);

void dict_remove(Dictionary *dict, uint64_t key);

void dict_free(Dictionary *dict);

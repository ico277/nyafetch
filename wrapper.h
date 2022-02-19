#pragma once
#include <pci/types.h>

char **get_gpu();
int get_gpu_count();
char *parse_vendor(u16 vendor_id);

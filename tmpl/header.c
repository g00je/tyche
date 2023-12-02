
#include "common.h"

${struct}

typedef struct ${name}Object {
    PyObject_HEAD
    ${name} raw;
    uint32_t idx;
} ${name}Object;

// end of header


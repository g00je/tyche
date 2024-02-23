
#define LEN(array) sizeof(array) / sizeof(array[0])

#define GENE_GETSET(prefix, offset)\
PyObject *prefix##_get(uint8_t *self, void *Py_UNUSED(C)) {\
    char str[17];\
    gene_t *gene = (gene_t *)&self[offset];\
    if (!gene->raw) Py_RETURN_NONE;\
    snprintf(str, 17, "%016lx", gene->raw);\
    return PyUnicode_FromString(str);\
}\
int prefix##_set(uint8_t *self, PyObject *value, void *Py_UNUSED(C)) {\
    gene_t *gene = (gene_t *)&self[offset];\
    if (Py_IsNone(value)) {\
        gene->raw = 0;\
        return 0;\
    }\
    if (check_str(value)) return -1;\
    Py_ssize_t len = 0;\
    const char *data = PyUnicode_AsUTF8AndSize(value, &len);\
    if (len != 16) {\
        PyErr_SetString(\
            PyExc_ValueError,\
            "gene length must be 16."\
        );\
        return -1;\
    }\
    gene->raw = strtoul(data, NULL, 16);\
    return 0;\
}


#define FLAGBOOL_GETSET(prefix, offset, flag)\
PyObject *prefix##_get(uint8_t *self, void *Py_UNUSED(C)) {\
    return PyBool_FromLong(*(uint64_t *)&self[offset] & flag);\
}\
int prefix##_set(uint8_t *self, PyObject *value, void *Py_UNUSED(C)) {\
    if (value == NULL) {\
        PyErr_SetString(\
            PyExc_TypeError,\
            "Cannot delete the attribute"\
        );\
        return -1;\
    }\
    if (!PyBool_Check(value)) {\
        PyErr_SetString(\
            PyExc_TypeError,\
            "The attribute value must be a bool"\
        );\
        return -1;\
    }\
    if (Py_IsTrue(value)) {\
        *(uint64_t *)&self[offset] |= flag;\
    } else {\
        *(uint64_t *)&self[offset] &= ~flag;\
    }\
    return 0;\
}


#define STR_GETSET(prefix, offset, size)\
PyObject *prefix##_get(uint8_t *self, void *Py_UNUSED(C)) {\
    Py_ssize_t len = size;\
    char *str = (char *)&self[offset];\
    if (!str[len-1]) len = strlen(str);\
    return PyUnicode_Decode(str, len, "utf-8", "ignore");\
}\
int prefix##_set(uint8_t *self, PyObject *value, void *Py_UNUSED(C)) {\
    if (check_str(value)) return -1;\
    Py_ssize_t len = 0;\
    const char *data = PyUnicode_AsUTF8AndSize(value, &len);\
    set_str((char *)&self[offset], data, (size_t)len, size);\
    return 0;\
}

#define FIELD(name, prefix)\
{\
    name,\
    (getter) prefix##_get,\
    (setter) prefix##_set,\
    NULL, NULL\
}



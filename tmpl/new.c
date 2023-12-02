
// __new__ {

PyObject *${name}_new(PyTypeObject *type, PyObject *args, PyObject *kwds) {

    uint32_t base_idx = 0;
    Py_buffer buffer;
    memset(&buffer, 0, sizeof(Py_buffer));

    static char *pars_kwlist[] = {"raw", "idx", NULL};

    if (!PyArg_ParseTupleAndKeywords(
        args, kwds, "|y*I", pars_kwlist,
        &buffer, &base_idx
    )) {
        PyBuffer_Release(&buffer);
        return NULL;
    }

    ${name}Object *self = (${name}Object *) type->tp_alloc(type, 0);
    if (self == NULL) {
        PyBuffer_Release(&buffer);
        return NULL;
    }

    self->idx = base_idx;
    
    // if there was not any data to unpack we just
    // return the default empty item
    if (buffer.obj == NULL) {
        memset(&self->raw, 0, sizeof(${name}));
        PyBuffer_Release(&buffer);
        return (PyObject *)self;
    }

    // check if item data is valid or not
    // if not return an error
    if (buffer.len < (ssize_t)sizeof(${name})) {
        PyErr_SetString(PyExc_ValueError, "invalid ${name} raw length");
        PyBuffer_Release(&buffer);
        return NULL;
    }

    // check if we need to return a list of items 
    // or just one item 
    Py_ssize_t length = buffer.len / sizeof(${name});
    ${name} *raw = buffer.buf;

    memcpy(&self->raw, raw, sizeof(${name}));

    if (length > 1) {
        PyObject *list = PyList_New(length);
        if (list == NULL) {
            PyBuffer_Release(&buffer);
            return NULL;
        }

        type->tp_init((PyObject *)self, args, kwds);
        PyList_SetItem(list, 0, (PyObject *)self);

        for (Py_ssize_t i = 1; i < length; i++) {
            self = (${name}Object *)type->tp_alloc(type, 0);
            if (self == NULL) {
                PyBuffer_Release(&buffer);
                return NULL;
            }

            self->idx = base_idx + i;
            memcpy(&self->raw, &raw[i], sizeof(${name}));

            type->tp_init((PyObject *)self, args, kwds);
            PyList_SetItem(list, i, (PyObject *)self);
        }

        PyBuffer_Release(&buffer);
        return list;
    }
    
    PyBuffer_Release(&buffer);
    return (PyObject *)self;
}

// } __new__


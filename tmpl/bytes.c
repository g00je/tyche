
// ${name}.${attr} {

PyObject *${name}_get_${attr}(${name}Object *self, void *Py_UNUSED(C)) {
    return PyBytes_FromStringAndSize(
        (char *)self->raw.${attr}, sizeof(self->raw.${attr})
    );
}

int ${name}_set_${attr}(${name}Object *self, PyObject *value, void *Py_UNUSED(C)) {
    if (value == NULL) {
        PyErr_SetString(
            PyExc_TypeError,
            "${attr} connot be deleted!"
        );
        return -1;
    }

    if (!PyBytes_Check(value)) {
        PyErr_SetString(
            PyExc_TypeError,
            "invalid ${attr} type. only bytes are valid"
        );
        return -1;
    }

    if (PyBytes_Size(value) != sizeof(self->raw.${attr})) {
        PyErr_SetString(
            PyExc_TypeError,
            "invalid ${attr} size"
        );
        return -1;
    }

    memcpy(
        self->raw.${attr},
        PyBytes_AsString(value),
        sizeof(self->raw.${attr})
    );

    return 0;
}

// } ${name}.${attr}



#include "./common.h"

// {% include 'new.c' %}


int Session_init(
    SessionObject *Py_UNUSED(self),
    PyObject *Py_UNUSED(args),
    PyObject *Py_UNUSED(kwds)
) {
    return 0;
}


/* ---------- Session.ip ---------- */

PyObject *Session_get_ip(SessionObject *self, void *Py_UNUSED(C)) {
    return PyUnicode_FromFormat(
        "%d.%d.%d.%d",
        self->data.ip[0],
        self->data.ip[1],
        self->data.ip[2],
        self->data.ip[3]
    );
    // return PyBytes_FromStringAndSize(
    //     (char *)self->data.ip, sizeof(self->data.ip)
    // );
}

int Session_set_ip(SessionObject *self, PyObject *value, void *Py_UNUSED(C)) {
    if (value == NULL) {
        PyErr_SetString(PyExc_TypeError, "Cannot delete the ip attribute");
        return -1;
    }

    if (!PyBytes_Check(value)) {
        PyErr_SetString(
            PyExc_TypeError,
            "The ip attribute value must be a bytes"
        );
        return -1;
    }

    if (PyBytes_Size(value) != sizeof(self->data.ip)) {
        PyErr_SetString(
            PyExc_TypeError,
            "The ip attribute value length must be 4"
        );
        return -1;
    }

    memcpy(
        self->data.ip,
        PyBytes_AsString(value),
        sizeof(self->data.ip)
    );

    return 0;
}



/* ---------- Session.token ---------- */
PyObject *Session_get_token(SessionObject *self, void *Py_UNUSED(C)) {
    return PyBytes_FromStringAndSize(
        (char *)self->data.token, sizeof(self->data.token)
    );
}

int Session_set_token(SessionObject *self, PyObject *value, void *Py_UNUSED(C)) {
    if (value == NULL) {
        PyErr_SetString(PyExc_TypeError, "Cannot delete the token attribute");
        return -1;
    }

    if (!PyBytes_Check(value)) {
        PyErr_SetString(
            PyExc_TypeError,
            "The token attribute value must be a bytes"
        );
        return -1;
    }

    if (PyBytes_Size(value) != sizeof(self->data.token)) {
        PyErr_SetString(
            PyExc_TypeError,
            "The token attribute value length must be 64"
        );
        return -1;
    }

    memcpy(
        self->data.token,
        PyBytes_AsString(value),
        sizeof(self->data.token)
    );

    return 0;
}

STR_GETSET(
    Session_name,
    offsetof(SessionObject, data.name),
    sizeof(((Session *)0)->name)
)

PyGetSetDef Session_getset[] = {
    FIELD("name", Session_name),
    {
        "ip", 
        (getter) Session_get_ip,
        (setter) Session_set_ip,
        PyDoc_STR("session's ip. fixed 4 bytes."),
        NULL
    },
    {
        "token", 
        (getter) Session_get_token,
        (setter) Session_set_token,
        PyDoc_STR("session's token. fixed 64 bytes."),
        NULL
    },
    { NULL }
};


/* -------------------- METHODS -------------------- */

PyObject *Session_bytes(SessionObject *self, PyObject *Py_UNUSED(ignored)) {
    return PyBytes_FromStringAndSize(
        (char *)&self->data,
        sizeof(Session)
    );
}


PyMethodDef Session_methods[] = {
    {
        "__bytes__", (PyCFunction) Session_bytes,
        METH_NOARGS, "Return the bytes"
    },
    { NULL }
};


PyMemberDef Session_members[] = {
    {
        "timestamp", T_ULONGLONG, 
        offsetof(SessionObject, data.timestamp), 0, 
        PyDoc_STR("session's timestamp.")
    },
    {
        "session_id", T_ULONGLONG, 
        offsetof(SessionObject, session_id), READONLY, 
        PyDoc_STR("session's index/id.")
    },
    { NULL } 
};


PyTypeObject SessionType = {
    PyVarObject_HEAD_INIT(NULL, 0)
    .tp_name = "tyche.Session",
    .tp_doc = PyDoc_STR("Plutus Session struct"),
    .tp_basicsize = sizeof(SessionObject),
    .tp_itemsize = 0,
    .tp_flags = Py_TPFLAGS_DEFAULT | Py_TPFLAGS_BASETYPE,
    .tp_init = (initproc)Session_init,
    .tp_new = Session_new,
    .tp_members = Session_members,
    .tp_methods = Session_methods,
    .tp_getset = Session_getset,
};


uint8_t setup_session(PyObject *module) {
    if (PyType_Ready(&SessionType) < 0)
        return -1;

    if (PyModule_AddType(module, &SessionType) < 0) {
        return -1;
    }

    if (PyDict_SetItemString(
        SessionType.tp_dict, "__size__",
        PyLong_FromSize_t(sizeof(Session))
    ))
        return -1;

    return 0;
}

// {{cool}}

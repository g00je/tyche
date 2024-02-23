
#ifndef __TYCHE_COMMON_H__
#define __TYCHE_COMMON_H__

#define PY_SSIZE_T_CLEAN
#include <Python.h>
#include <inttypes.h>
#include <structmember.h>

#include "types.h"
#include "macro.h"


/* ---------- MODELS ---------- */
typedef struct SessionObject {
    PyObject_HEAD
    Session data;
    uint64_t session_id;
} SessionObject;

typedef struct UserObject {
    PyObject_HEAD
    User data;
    PyObject *picture;
    PyObject *sessions;
} UserObject;

typedef struct AgentObject {
    PyObject_HEAD
    Agent data;
} AgentObject;

typedef struct EateryObject {
    PyObject_HEAD
    Eatery data;
    PyObject *opening_hours;
    PyObject *pictures;
} EateryObject;

typedef struct PictureObject {
    PyObject_HEAD
    uint64_t picture_id;
    Picture data;
    PyObject *salt;
} PictureObject;

typedef struct DurationObject {
    PyObject_HEAD
    Duration data;
} DurationObject;

typedef struct DishObject {
    PyObject_HEAD
    uint64_t dish_id;
    gene_t block;
    Dish data;
    PyObject *pictures;
} DishObject;

typedef struct MenuBlockObject {
    PyObject_HEAD
    MenuBlock data;
    PyObject *menu;
} MenuBlockObject;

typedef struct ReviewObject {
    PyObject_HEAD
    uint64_t review_id;
    gene_t block;
    Review data;
} ReviewObject;

typedef struct ReviewBlockObject {
    PyObject_HEAD
    ReviewBlock data;
    PyObject *reviews;
} ReviewBlockObject;


/* ---------- API ---------- */
typedef struct ResponseHeadObject {
    PyObject_HEAD
    ResponseHead data;
} ResponseHeadObject;


uint8_t setup_user(PyObject *module);
uint8_t setup_agent(PyObject *module);
uint8_t setup_session(PyObject *module);
uint8_t setup_eatery(PyObject *module);
uint8_t setup_picture(PyObject *module);
uint8_t setup_duration(PyObject *module);
uint8_t setup_dish(PyObject *module);
uint8_t setup_menu_block(PyObject *module);
uint8_t setup_review(PyObject *module);
uint8_t setup_review_block(PyObject *module);

/* --- API --- */
uint8_t setup_head(PyObject *module);

extern PyTypeObject SessionType;
extern PyTypeObject UserType;
extern PyTypeObject AgentType;
extern PyTypeObject PictureType;
extern PyTypeObject EateryType;
extern PyTypeObject DurationType;
extern PyTypeObject DishType;
extern PyTypeObject ReviewType;
extern PyTypeObject ResponseHeadType;

extern PyObject *login_args(PyObject *self, PyObject *args);
extern PyObject *gene(PyObject *self, PyObject *args);


int8_t check_str(PyObject *value);
void set_str(char *dest, const char *data, size_t len, size_t size);

#endif // __TYCHE_COMMON_H__


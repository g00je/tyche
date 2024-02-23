
#ifndef __PNC_TYPES_H__
#define __PNC_TYPES_H__

#include <stdbool.h>
#include <stdint.h>

#define PAGE_SIZE 25

#define FLAG_ALIVE             (1 << 0)
#define FLAG_DETAIL_EDITED     (1 << 1)
#define FLAG_DISH_AVAILABLE    (1 << 2)
#define FLAG_EATERY_CLOSED     (1 << 2)

typedef unsigned char byte;
typedef uint8_t  admin_perms_t[64];
typedef uint64_t position_t;

typedef uint32_t geneid_t;
typedef union gene_t {
    struct {
        geneid_t id;
        uint16_t pepper;
        uint16_t server;
    };
    uint64_t raw;
} gene_t;
_Static_assert(sizeof(gene_t) == 8, "invalid gene size");

typedef struct EntityHead {
    uint64_t flag;
    gene_t gene;
} EntityHead;

typedef struct ResponseHead {
    uint32_t status;    // status codes
    uint32_t size;      // size of the response
    double elapsed;     // time took to return the response
} ResponseHead;
_Static_assert(sizeof(ResponseHead) == 16, "ResponseHead Size is invalid.");

typedef struct Picture {
    uint32_t server;
    uint8_t ext;
    byte salt[3];
} Picture;
_Static_assert(sizeof(Picture) == 8, "Picture Size is invalid.");

/* --------------- User --------------- */

typedef struct Session {
    uint8_t ip[4];
    char name[36];
    // if timestamp is 0, Session is Dead;
    uint64_t timestamp;
    byte token[64];
} Session;
_Static_assert(sizeof(Session) == 112, "Session Size is invalid.");

typedef struct User {
    uint64_t flag;
    gene_t gene;
    gene_t agent;
    gene_t reviews;
    Picture picture;
    char phone[12];
    uint16_t cc;
    char name[50];
    Session sessions[3];
} User;
_Static_assert(sizeof(User) == 440 , "User Size is invalid");

typedef struct Agent {
    uint64_t flag;
    gene_t gene;
    gene_t user;
    admin_perms_t admin_perms;
} Agent;
_Static_assert(sizeof(Agent) == 88, "Agent Size is invalid");


typedef struct UserLoginArgs {
    uint16_t cc;
    char phone[12];
    Session session;
} UserLoginArgs;


/* --------------- Entity --------------- */

typedef struct Duration {
    uint8_t open;
    uint8_t close;
} Duration;

typedef struct Eatery {
    uint64_t flag;
    gene_t gene;

    uint8_t category;
    uint8_t theme;
    int16_t tables;
    uint16_t menu_count;
    uint16_t review_count;

    double latitude;
    double longitude;
    gene_t menu;
    gene_t review;
    gene_t detail;
    Picture pictures[7];

    uint16_t cc;
    char phone[12];
    Duration opening_hours[7][4];
    char name[62];
    uint32_t star_sum;
} Eatery;
_Static_assert(sizeof(Eatery) == 256, "Eatery Size is invalid");

typedef struct Dish {
    uint64_t flag;
    uint8_t type;
    char name[53];
    uint16_t currency;
    Picture pictures[4];
    int64_t price;
} Dish;
_Static_assert(sizeof(Dish) == 104, "Dish Size is invalid %lu");

typedef struct Review {
    uint64_t flag;
    gene_t user;
    gene_t detail;
    uint64_t timestamp;
    uint8_t star;
    char summary[223];
} Review;
_Static_assert(sizeof(Review) == 256, "Review Size is invalid");


typedef struct BlockHeader {
    uint64_t flag;
    gene_t gene;

    gene_t eatery;
    gene_t past;
    gene_t next;
    uint8_t live;
    byte reserved[7];
} BlockHeader;

typedef struct ReviewBlock {
    BlockHeader header;
    Review reviews[PAGE_SIZE];
} ReviewBlock;

typedef struct MenuBlock {
    BlockHeader header;
    Dish menu[PAGE_SIZE];
} MenuBlock;

#endif // __PNC_TYPES_H__


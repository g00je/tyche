use std::fmt::Debug;

use plutus_macros::model;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

static PAGE_SIZE: u64 = 32;
static REQUEST_SIZE: u64 = 60000; // 60K
static DETAIL_ADDED_SIZE: u64 = 256;
static DETAIL_MAX_LENGTH: u64 = 20000; // 20K
static SNAKE_MAX_LENGTH: u64 = 32768; // 4096 * 8

static FLAG_ALIVE: u64 = 1 << 0;
static FLAG_EDITED: u64 = 1 << 1;
static FLAG_PRIVATE: u64 = 1 << 2;

static FLAG_DISH_AVAILABLE: u64 = 1 << 16;
static FLAG_EATERY_CLOSED: u64 = 1 << 16;

mod macros;

fn phone_validator(value: String) -> PyResult<String> {
    let result = value.chars().enumerate().find_map(|(i, c)| {
        if c.is_ascii_digit() {
            return None;
        }

        Some(PyValueError::new_err(format!(
            "invalid phone number char: '{c}' at {i}"
        )))
    });

    match result {
        Some(err) => Err(err),
        None => Ok(value),
    }
}

#[model]
struct ResponseHead {
    status: u32,
    size: u32,
    elapsed: f64,
}

#[model(hex)]
struct Gene {
    id: u32,
    pepper: u32,
    server: u32,
    version: u8,
    idx: u8,
    iter: u8,
    _reserved: u8,
}

impl CGene {
    fn is_none(&self) -> bool {
        self.id == 0
    }
}

#[model]
struct PondIndex {
    flag: u64,
    gene: Gene,
    owner: Gene,
    block_count: u64,
    item_count: u64,
    first: Gene,
    last: Gene,
}

#[model]
struct Detail {
    flag: u64,
    gene: Gene,
    size: u64,
    position: u64,
    // end of head
    length: u64,
}

#[model]
struct Record {
    flag: u64,
    gene: Gene,
    checksum: [u8; 16],
    server: u32,
    width: u32,
    height: u32,
    size: u32,
    ext: u8,
    _reserved: [u8; 3],
    duration: f32,
}

#[model]
struct Agent {
    flag: u64,
    gene: Gene,
    user: Gene,
    #[bigint]
    admin_perms: [u8; 64],
}

#[model]
struct Duration {
    #[int(max = 97)]
    open: u8,
    #[int(max = 98)]
    close: u8,
}

#[model]
struct Eatery {
    flag: u64,
    gene: Gene,
    latitude: f64,
    longitude: f64,
    menu: Option<Gene>,
    review: Option<Gene>,
    detail: Option<Gene>,
    extra: Option<Gene>,
    photos: [Option<Gene>; 7],
    stars: [u64; 5],
    theme: u32,
    #[int(max = 999)]
    cc: u16,
    tables: i16,
    category: u8,
    #[int(max = 22)]
    zoom: u8,
    #[str]
    phone: [u8; 12],
    opening_hours: [[Duration; 4]; 7],
    #[str]
    name: [u8; 58],
    #[flag]
    closed: FLAG_EATERY_CLOSED,
}

#[model]
struct Dish {
    flag: u64,
    gene: Gene,
    ty: u8,
    #[str]
    name: [u8; 128],
    #[str]
    note: [u8; 127],
    photos: [Option<Gene>; 4],
    price: i64,
    #[flag]
    available: FLAG_DISH_AVAILABLE,
}

#[model]
struct Review {
    flag: u64,
    gene: Gene,   // block gene of self
    target: Gene, // eatery OR user
    cousin: Gene, // eatery review OR user review. its not there own block
    detail: Option<Gene>,
    timestamp: u64,
    #[int(max = 6)]
    star: u8,
    state: u8,
    #[str]
    summary: [u8; 222],
}

#[model]
struct BlockHeader {
    flag: u64,
    gene: Gene,
    index: Gene,
    past: Option<Gene>,
    next: Option<Gene>,
    live: u8,
    _reserved: [u8; 7],
}

#[model]
struct ReviewBlock {
    header: BlockHeader,
    reviews: [Review; 32],
}

#[model]
struct MenuBlock {
    header: BlockHeader,
    menu: [Dish; 32],
}

#[model]
struct SessionInfo {
    client: u8,
    os: u8,
    browser: u8,
    device: u8,
    client_version: u16,
    os_version: u16,
    browser_version: u16,
    _reserved: u16,
}

#[model]
struct Session {
    #[ipv4]
    ip: [u8; 4],
    info: SessionInfo,
    // if timestamp is 0, Session is Dead
    timestamp: u64,
    token: [u8; 64],
}

#[model]
struct User {
    flag: u64,
    gene: Gene,
    agent: Option<Gene>,
    review: Option<Gene>,
    photo: Option<Gene>,
    reviews: [u64; 3],
    #[str(validator = phone_validator)]
    phone: [u8; 12],
    #[int(max = 999)]
    cc: u16,
    #[str]
    name: [u8; 50],
    sessions: [Session; 3],
}

#[model]
struct UserLoginArgs {
    #[int(max = 999)]
    cc: u16,
    #[str(validator = phone_validator)]
    phone: [u8; 12],
    _pad: u16,
    session: Session,
}

#[pymodule]
fn plutus_internal(_py: Python, m: &PyModule) -> PyResult<()> {
    macros::act_on_models! {
        ($X:ident) => { m.add_class::<$X>()?; }
    }

    m.add("PAGE_SIZE", PAGE_SIZE)?;
    m.add("REQUEST_SIZE", REQUEST_SIZE)?;
    m.add("SNAKE_MAX_LENGTH", SNAKE_MAX_LENGTH)?;
    m.add("DETAIL_ADDED_SIZE", DETAIL_ADDED_SIZE)?;
    m.add("DETAIL_MAX_LENGTH", DETAIL_MAX_LENGTH)?;
    m.add("FLAG_ALIVE", FLAG_ALIVE)?;
    m.add("FLAG_EDITED", FLAG_EDITED)?;
    m.add("FLAG_PRIVATE", FLAG_PRIVATE)?;
    m.add("FLAG_DISH_AVAILABLE", FLAG_DISH_AVAILABLE)?;
    m.add("FLAG_EATERY_CLOSED", FLAG_EATERY_CLOSED)?;

    Ok(())
}

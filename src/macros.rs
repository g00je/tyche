#[macro_export]
macro_rules! act_on_list {
    ([$($m:ident),*], $g:ident) => { $( $g! ( $m ); )* }
}

#[macro_export]
macro_rules! act_on_models {
    ($($mac:tt)*) => {
        macro_rules! act { $($mac)* }

        $crate::macros::act_on_list! {
            [ResponseHead, Gene, Detail, Record, Agent, Duration,
             Star, Eatery, Dish, Review, BlockHeader, ReviewBlock, PondIndex,
             MenuBlock, SessionInfo, Session, User, UserLoginArgs],
            act
        }
    }
}

pub(crate) use act_on_list;
pub(crate) use act_on_models;

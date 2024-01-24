use crate::{
    binary::WWABinary,
    coord::WWACoord,
    object::{normal::Normal, Player, Status},
};

pub trait Parser<T> {
    fn parse(input: &WWABinary) -> T;
}

use libwwamap::{binary::Binary, string::list::Meta, tiled, wwamap::WWAMap
};
use std::{
    fs,
    path::Path
};

#[test]
fn meta_check() {
    let path = Path::new("tests/wwamk310/mapdata/caves01.dat");
    let buf: Vec<u8> = fs::read(path).unwrap();

    let bin: Binary = Binary::decode(&buf, None).unwrap();
    let meta: WWAMap = WWAMap::parse(&bin).unwrap();
    assert_eq!(meta.version, 31);
    assert_eq!(meta.player.position, tiled::Map{x: 5, y: 69});
    assert_eq!(meta.player.gameover_position, tiled::Map{x: 25, y: 17});
    assert_eq!(meta.player.energy_max, 0);
    assert_eq!(meta.player.status.energy, 1000);
    assert_eq!(meta.player.status.strength, 40);
    assert_eq!(meta.player.status.defence, 20);
    assert_eq!(meta.player.status.gold, 0);
    assert_eq!(meta.string.password, "");
    assert_eq!(meta.string.title, "Cave Dungeon Level 1");
    assert_eq!(meta.string.img_file, "caves01.gif");
    assert_eq!(meta.string.message[Meta::ConfirmLink as usize], "");
    assert_eq!(meta.string.message[Meta::InsufficientFunds as usize], "");
    assert_eq!(meta.string.message[Meta::NoItem as usize], "");
    assert_eq!(meta.string.message[Meta::UseItem as usize], "");
    assert_eq!(meta.string.message[Meta::GetItem as usize], "");
    assert_eq!(meta.string.message[Meta::FullItem as usize], "");
    assert_eq!(meta.string.message[Meta::SoundConfirm as usize], "");
}

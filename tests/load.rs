use libwwamap::{
    binary::Binary, entity::object::ObjectType, string::array::Meta, tiled, wwamap::WWAMap,
};
use std::{fs, path::Path};

#[test]
fn meta_check() {
    libwwamap::apply_log();
    let path = Path::new("tests/wwamk310/mapdata/caves01.dat");
    let buf: Vec<u8> = fs::read(path).unwrap();

    let bin: Binary = Binary::decode(&buf, None).unwrap();
    let wwamap: WWAMap = bin.try_into().unwrap();
    assert_eq!(wwamap.version, 31);
    assert_eq!(wwamap.player.position, tiled::Map { x: 5, y: 69 });
    assert_eq!(wwamap.player.gameover_position, tiled::Map { x: 25, y: 17 });
    assert_eq!(wwamap.player.energy_max, 0);
    assert_eq!(wwamap.player.status.energy, 1000);
    assert_eq!(wwamap.player.status.strength, 40);
    assert_eq!(wwamap.player.status.defence, 20);
    assert_eq!(wwamap.player.status.gold, 0);
    assert_eq!(wwamap.string[Meta::Password], "");
    assert_eq!(wwamap.string[Meta::Title], "Cave Dungeon Level 1");
    assert_eq!(wwamap.string[Meta::ImgFile], "caves01.gif");
    assert_eq!(wwamap.string[Meta::ConfirmLink], "");
    assert_eq!(wwamap.string[Meta::InsufficientFunds], "");
    assert_eq!(wwamap.string[Meta::NoItem], "");
    assert_eq!(wwamap.string[Meta::UseItem], "");
    assert_eq!(wwamap.string[Meta::GetItem], "");
    assert_eq!(wwamap.string[Meta::FullItem], "");
    assert_eq!(wwamap.string[Meta::SoundConfirm], "");
    dbg!(&wwamap.object);
    assert_eq!(wwamap.object[0], ObjectType::Normal);
    assert_eq!(wwamap.object[1], ObjectType::Message);
    assert_eq!(wwamap.object[2], ObjectType::Message);
    assert_eq!(wwamap.object[3], ObjectType::Message);
    assert_eq!(wwamap.object[4], ObjectType::Message);
    assert_eq!(wwamap.object[5], ObjectType::Message);
    assert_eq!(wwamap.object[6], ObjectType::Message);
    assert_eq!(wwamap.object[7], ObjectType::Message);
}

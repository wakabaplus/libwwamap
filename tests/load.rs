use libwwamap::{binary, meta, tiled};
use std::{fs, path::Path};

#[test]
fn meta_check() {
    let path = Path::new("tests/wwamk310/mapdata/caves01.dat");
    let buf: Vec<u8> = fs::read(path).unwrap();

    let bin = binary::Binary::decode(&buf, None).unwrap();
    let meta = meta::Meta::parse(&bin).unwrap();
    assert_eq!(meta.version, 31);
    assert_eq!(meta.player.position, tiled::Map{x: 5, y: 69});
    assert_eq!(meta.player.gameover_position, tiled::Map{x: 25, y: 17});
    assert_eq!(meta.player.energy_max, 0);
    assert_eq!(meta.player.status.energy, 1000);
    assert_eq!(meta.player.status.strength, 40);
    assert_eq!(meta.player.status.defence, 20);
    assert_eq!(meta.player.status.gold, 0);
}

use libwwamap::wwa;
use std::{fs, path::Path};

#[test]
fn meta_check() {
    let path = Path::new("tests/wwamk310/mapdata/caves01.dat");
    let buf: Vec<u8> = fs::read(path).unwrap();

    let bin = wwa::Binary::decode(&buf).unwrap();
    let meta = wwa::Meta::parse(&bin).unwrap();
    assert_eq!(meta.version, 31);
}

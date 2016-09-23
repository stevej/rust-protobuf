use super::test_map_pb::*;

use test::*;

#[test]
fn test_map() {
    let mut map = TestMap::new();

    test_serialize_deserialize("", &map);

    map.mut_m().insert("two".to_owned(), 2);
    test_serialize_deserialize("0a 07 0a 03 74 77 6f 10 02", &map);

    map.mut_m().insert("sixty six".to_owned(), 66);
    // cannot (easily) test hex, because order is not specified
    test_serialize_deserialize_no_hex(&map);
}
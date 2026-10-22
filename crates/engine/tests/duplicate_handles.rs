//! A handle repeated in `handles` is transformed once, not once per occurrence.

use cadcraft_engine::Session;
use cadcraft_engine::doc::{Common, Entity, EntityKind, Line};
use cadcraft_engine::geom::Vec3;
use serde_json::json;

#[test]
fn repeated_handle_moves_once() {
    let mut s = Session::new();
    let space = s.space();
    let d = s.doc_mut().unwrap();
    let handle = d.new_handle();
    let kind = EntityKind::Line(Line { a: Vec3::new(0.0, 0.0, 0.0), b: Vec3::new(1.0, 0.0, 0.0) });
    d.space_mut(&space).unwrap().push(Entity { handle, common: Common::default(), kind });
    let h = handle.hex();
    s.execute("move", &json!({"handles": [h, h], "from": [0, 0], "to": [10, 0]})).unwrap();
    let EntityKind::Line(l) = &s.doc().unwrap().entity(handle).unwrap().kind else { panic!("not a line") };
    assert_eq!((l.a.x, l.b.x), (10.0, 11.0));
}

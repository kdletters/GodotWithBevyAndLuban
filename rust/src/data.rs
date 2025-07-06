use cfg::prelude::*;
use godot::classes::file_access::ModeFlags;
use godot::prelude::GFile;
use lazy_static::lazy_static;
use luban_lib::ByteBuf;
use std::io::Read;

lazy_static! {
    pub static ref TABLES: Tables = {
        let tables = Tables::new(|name| {
            let mut file =
                GFile::open(&format!("res://data/bytes/{}.bytes", name), ModeFlags::READ)
                    .expect("load bytes failed.");
            let mut buf = vec![];
            let _ = file.read_to_end(&mut buf);
            Ok(ByteBuf::new(buf))
        });
        tables.expect("REASON")
    };
}

use rlua::Table;

pub fn populate_lua_tbl(tbl: &mut Table, funcs: Vec<&str>) {
    for (ix, name) in funcs.into_iter().enumerate() {
        tbl.set(ix + 1, name).unwrap();
    }
}
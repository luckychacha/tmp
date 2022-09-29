use anyhow::Result;
use obj::{load_obj, Obj};
use std::fs::File;
use std::io::BufReader;

fn main() -> Result<()> {
    let obj_file = File::open("../model-resource/barrel.obj")?;
    let tmp = BufReader::new(obj_file);
    let obj: Obj = load_obj(tmp)?;

    // Do whatever you want
    obj.vertices;
    obj.indices;

    Ok(())
}

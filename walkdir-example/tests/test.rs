use walkdir::{self, WalkDir};

#[test]
fn test() {
    for entry in WalkDir::new("D:\\Documents\\ds_idts") {
        let entry = entry.unwrap();
        println!("{:?}", entry.path().display());
    }
}

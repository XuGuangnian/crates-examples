use std::path::Path;

#[test]
fn path_parent() {
    let remote_path = Path::new("/remote/xx.zip");
    let parent_path = remote_path.parent().unwrap();
    println!("{:?}", parent_path);
}

#[test]
fn path_metadata() {
    let local_path = Path::new("D:\\Documents\\ds_idts\\阿语译文-提交-7份.zip");
    println!("{:?}", local_path.metadata().unwrap().len());
    let local_path = Path::new("D:\\Documents\\ds_idts");
    println!("{:?}", local_path.metadata().unwrap().len());
}

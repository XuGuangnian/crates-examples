use std::path::Path;



pub(crate) fn example() {
    let remote_path = Path::new("xx.zip");
    let parent_path = remote_path.parent().unwrap();
    println!("{:?}", parent_path);

    let local_path = Path::new("D:\\Documents\\ds_idts\\阿语译文-提交-7份.zip");
    println!("{:?}", local_path.metadata().unwrap().len());
}
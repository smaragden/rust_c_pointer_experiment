use gp_safe::CameraFile;

fn main() {
    println!("Running gp_safe");
    let file = CameraFile::new();
    println!("mimetype: {}", file.mimetype());
}

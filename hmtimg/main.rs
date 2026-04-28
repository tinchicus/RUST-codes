use std::env;
use std::path::Path;

fn main() {
    let path_imagen = env::args().skip(1).next().unwrap();
    let path = Path::new(&path_imagen);
    let img = image::open(path).unwrap();
    let rotado = img.rotate90();
    rotado.save(path).unwrap();
}

use anyhow::Result;
use exif::{In, Tag};
use std::{fs, io::Cursor};

fn main() -> Result<()> {
    let dir = fs::read_dir("./")?;

    for entry in dir {
        let e = match entry {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Couldn't read entry: {e}");
                continue;
            }
        };

        let meta = match e.metadata() {
            Ok(m) => m,
            Err(e) =>  {
                eprintln!("Couldn't read metadata: {e}");
                continue;
            }
        };

        if meta.is_file() {
            let file =  match fs::read(e.path()) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("Error reading file: {e}");
                    continue;
                }
            };

            let exif_reader = exif::Reader::new();

            // need to create something that implements `Read`
            let mut file = Cursor::new(file);

            let exif_data = match exif_reader.read_from_container(&mut file){
                Ok(exif) => exif,
                Err(e) => {
                    eprintln!("Couldn't read exif data: {e}");
                    continue;
                }
            };

            let res_x = exif_data.get_field(Tag::PixelXDimension, In::PRIMARY);
            let res_y = exif_data.get_field(Tag::PixelYDimension, In::PRIMARY);

            let (x, y) = match (res_x, res_y) {
                (Some(x), Some(y)) => {
                    let x = x.value.get_uint(0).unwrap_or(0);
                    let y = y.value.get_uint(0).unwrap_or(0);
                    (x, y)
                },
                (_, _) => (0,0),
            };

            if x == 0 || y == 0 {
                println!("{}\t Size: 0 MP", e.file_name().to_string_lossy());
            } else {
                let res = x as f64 * y as f64 / 1_000_000.0;
                println!("{}\t Size: {res:.02} MP", e.file_name().to_string_lossy());
            }
        }
    }

    Ok(())
}

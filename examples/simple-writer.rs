use ffav::easy::{AudioDesc, SimpleWriter, VideoDesc};
use std::convert::TryInto;
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a_desc = AudioDesc::new();
    let v_desc = VideoDesc::with_h264(352, 288, 4000, 1000000);
    let example_bytes = include_bytes!("envivio-352x288.264.framed");
    for n in 0..100000 {
        let start = Instant::now();
        let mut mp4_writer =
            SimpleWriter::new("/tmp/envivio-352x288.264.mp4", &[&a_desc, &v_desc], None)?;
        let mut ts_writer = SimpleWriter::new(
            "/tmp/envivio-352x288.264.ts",
            &[&a_desc, &v_desc],
            Some("mpegts"),
        )?;
        let mut offset: usize = 0;
        let mut pts = 0;
        while offset + 4 < example_bytes.len() {
            let size_bytes = &example_bytes[offset..offset + 4];
            let frame_size = i32::from_be_bytes(size_bytes.try_into().unwrap()) as usize;
            offset += 4;
            let frame_bytes = &example_bytes[offset..offset + frame_size];
            offset += frame_size;
            mp4_writer.write_bytes(frame_bytes, pts, 40000, 0)?;
            ts_writer.write_bytes(frame_bytes, pts, 40000, 0)?;
            pts += 40000;
        }
        let duration = start.elapsed();
        println!(
            "#{:6} Time elapsed {:?} to processing {:?} bytes!",
            n,
            duration,
            example_bytes.len()
        );
    }

    Ok(())
}

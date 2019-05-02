use structopt::*;

use std::fs::File;
use std::path::PathBuf;

#[derive(StructOpt, Debug)]
struct Opt {
    /// Files to process
    #[structopt(name = "A", parse(from_os_str))]
    a: PathBuf,
    #[structopt(name = "B", parse(from_os_str))]
    b: PathBuf,
}

fn diff_plane(a: &[u8], b: &[u8]) -> bool {
    for (a, b) in a.iter().zip(b.iter()) {
        if a != b {
            return true;
        }
    }

    false
}

fn diff_frame(a: &y4m::Frame, b: &y4m::Frame) -> bool {
    let ya = a.get_y_plane();
    let yb = b.get_y_plane();

    if diff_plane(ya, yb) {
        return true
    }

    let ua = a.get_u_plane();
    let ub = b.get_u_plane();

    if diff_plane(ua, ub) {
        return true
    }

    let va = a.get_v_plane();
    let vb = b.get_v_plane();

    if diff_plane(va, vb) {
        return true
    }

    false
}

fn main() {
    pretty_env_logger::init();
    let opt = Opt::from_args();

    println!("{:?}", opt);

    let mut a = File::open(opt.a).expect("File not found");
    let mut b = File::open(opt.b).expect("File not found");

    let mut dec_a = y4m::decode(&mut a).unwrap();
    let mut dec_b = y4m::decode(&mut b).unwrap();

    if dec_a.get_width() != dec_b.get_width() ||
        dec_a.get_height() != dec_b.get_height() ||
            dec_a.get_colorspace() as usize != dec_b.get_colorspace() as usize {
        panic!("Incompatible files");
    }

    let mut count = 0;

    while let (Ok(a), Ok(b)) = (dec_a.read_frame(), dec_b.read_frame()) {
        if diff_frame(&a, &b) {
            println!("Frame {} different", count);
        }

        count += 1;
    }
}

pub fn hexdump_w(vec: Vec<u8>, width: usize) {
    let mut offset = 0;
    for chunk in vec.chunks(width) {
        print!("{:5x} |", offset);

        for v in chunk.iter() {
            print!(" {:02x}", v);
        }
        print!("{} | ", "   ".repeat(width-chunk.len()));

        for v in chunk.iter() {
            if *v >= 0x20 && *v <= 0x7E {
                print!("{}", *v as char);
            } else {
                print!(".");
            }
        }
        println!("{} |", " ".repeat(width-chunk.len()));


        offset += width;
    }
}

pub fn hexdump(vec: Vec<u8>) {
    hexdump_w(vec, 16)
}
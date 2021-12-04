fn print_bits(w: u64) {
    for i in 0..64 {
        if (w >> (63-i)) & 1 == 1 {
            print!("#");
        }
        else {
            print!(" ");
        }
    }
    println!("");
}

fn rule110(w: u64) -> u64 {
    let a=w>>1;
    let b=w;
    let c=w<<1;
    (!((a&!(b^c)) | (!a&!b&!c)))
}

fn main() {
    let mut w = 1<<12;
    for i in 0..100 {
        print_bits(w);
        w = rule110(w);
    }
}

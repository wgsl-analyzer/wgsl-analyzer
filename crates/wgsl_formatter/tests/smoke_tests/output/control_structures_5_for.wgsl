fn main() {
    for (var i: u32 = 0; i < 10u; i += 7) {
        a += i;
    }
}

fn vertical() {
    for (var i: u32 = 0; i < 10u; i += 7) {
        a += i;
    }
}

fn horizontal() {for (var i: u32 = 0; i < 10u; i += 7) {a += i;}}


fn mixed() {
    for (var i: u32 = 0; i < 10u; i += 7) {
        a += i;
        if a > 42u {break;}
    }
}

fn nested() {
    for (var i: u32 = 0; i < 10u; i += 7) {
        for (var j: u32 = 0; i < 10u; i += 7) {
            a += i;
            a /= j;
            if a > 42u {break;}
        }
    }
}

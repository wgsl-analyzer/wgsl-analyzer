import simple_wesl::lowbias32;

fn main() {
    const a = lowbias32(3);
    const b = simple_wesl::lowbias32(9);
    const c = nested::A(a, b);
}
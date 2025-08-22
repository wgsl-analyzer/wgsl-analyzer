fn main() {
    do_thing();
}

fn nested() {
    do_thing(get_thing());
}

fn args_1() {
    do_thing(1);
}

fn args_2() {
    do_thing(1, false);
}

fn args_3() {
    do_thing(1, false, 0.3);
}

fn args_4() {
    do_thing(1, false, 0.3, 2821u);
}

fn args_4() {
    do_thing(1, false, 0.3, 2821u);
}

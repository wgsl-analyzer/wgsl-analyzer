use crate::tests::util::{check, check_tabs};
use expect_test::expect;

#[test]
fn format_for() {
    check(
        "fn main() {
    for( var i = 0;i < 100;   i = i + 1  ){}
}",
        expect![["
                fn main() {
                    for (var i = 0; i < 100; i = i + 1) {}
                }"]],
    );
}

#[test]
fn format_if() {
    check(
        "fn main() {
    if(x < 1){}
    if  (  x < 1   )  {}
}",
        expect![["
            fn main() {
                if x < 1 {}
                if x < 1 {}
            }"]],
    );
}

#[test]
fn format_if_2() {
    check(
        "fn main() {
    if(x < 1){}
    else{
        let a = 3;
    }else     if(  x > 2 ){}
}",
        expect![["
            fn main() {
                if x < 1 {} else {
                    let a = 3;
                } else if x > 2 {}
            }"]],
    );
}

#[test]
fn format_while() {
    check(
        "fn main() {
        while(x < 1){}
        while  (  x < 1   )  {}
    }",
        expect![["
            fn main() {
                while x < 1 {}
                while x < 1 {}
            }"]],
    );
}

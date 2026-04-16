use crate::test_util::assert_out_of_scope;

#[test]
pub fn format_continuing_statement_without_loop() {
    assert_out_of_scope(
        "fn main() {
        continuing {

        }


        }",
        "Wgsl disallows continuing statements outside of loop statements",
    );
}

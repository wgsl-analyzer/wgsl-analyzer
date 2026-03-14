#![expect(clippy::too_many_lines, reason = "snapshot test data")]

use expect_test::expect;
use syntax::ExtensionsConfig;

use crate::tests::check_infer;

#[test]
fn abs() {
    check_infer(
        ExtensionsConfig {
            f16: true,
            ..Default::default()
        },
        "
enable f16;
fn foo() {
    let abstract_int = abs(1);
    let abstract_float = abs(1.0);
    let signed_integer_32 = abs(1i);
    let unsigned_integer_32 = abs(1u);
    let float_32 = abs(1.0f);
    let float_16 = abs(1.0h);

    let abstract_int_vec = abs(vec2(1));
    let abstract_float_vec = abs(vec2(1.0));
    let signed_integer_32_vec = abs(vec2(1i));
    let unsigned_integer_32_vec = abs(vec2(1u));
    let float_32_vec = abs(vec2(1.0f));
    let float_16_vec = abs(vec2(1.0h));
}
",
        expect![[r#"
            31..43 'abstract_int': i32
            46..52 'abs(1)': integer
            50..51 '1': integer
            62..76 'abstract_float': f32
            79..87 'abs(1.0)': float
            83..86 '1.0': float
            97..114 'signed...ger_32': i32
            117..124 'abs(1i)': i32
            121..123 '1i': i32
            134..153 'unsign...ger_32': u32
            156..163 'abs(1u)': u32
            160..162 '1u': u32
            173..181 'float_32': f32
            184..193 'abs(1.0f)': f32
            188..192 '1.0f': f32
            203..211 'float_16': f16
            214..223 'abs(1.0h)': f16
            218..222 '1.0h': f16
            234..250 'abstra...nt_vec': vec2<i32>
            253..265 'abs(vec2(1))': vec2<integer>
            257..264 'vec2(1)': vec2<integer>
            262..263 '1': integer
            275..293 'abstra...at_vec': vec2<f32>
            296..310 'abs(vec2(1.0))': vec2<float>
            300..309 'vec2(1.0)': vec2<float>
            305..308 '1.0': float
            320..341 'signed...32_vec': vec2<i32>
            344..357 'abs(vec2(1i))': vec2<i32>
            348..356 'vec2(1i)': vec2<i32>
            353..355 '1i': i32
            367..390 'unsign...32_vec': vec2<u32>
            393..406 'abs(vec2(1u))': vec2<u32>
            397..405 'vec2(1u)': vec2<u32>
            402..404 '1u': u32
            416..428 'float_32_vec': vec2<f32>
            431..446 'abs(vec2(1.0f))': vec2<f32>
            435..445 'vec2(1.0f)': vec2<f32>
            440..444 '1.0f': f32
            456..468 'float_16_vec': vec2<f16>
            471..486 'abs(vec2(1.0h))': vec2<f16>
            475..485 'vec2(1.0h)': vec2<f16>
            480..484 '1.0h': f16
        "#]],
    );
}

#[test]
fn acos() {
    check_infer(
        ExtensionsConfig {
            f16: true,
            ..Default::default()
        },
        "
enable f16;
fn foo() {
    let abstract_float = acos(1.0);
    let float_32 = acos(1.0f);
    let float_16 = acos(1.0h);

    let abstract_float_vec = acos(vec2(1.0));
    let float_32_vec = acos(vec2(1.0f));
    let float_16_vec = acos(vec2(1.0h));
}
",
        expect![[r#"
            31..45 'abstract_float': f32
            48..57 'acos(1.0)': float
            53..56 '1.0': float
            67..75 'float_32': f32
            78..88 'acos(1.0f)': f32
            83..87 '1.0f': f32
            98..106 'float_16': f16
            109..119 'acos(1.0h)': f16
            114..118 '1.0h': f16
            130..148 'abstra...at_vec': vec2<f32>
            151..166 'acos(vec2(1.0))': vec2<float>
            156..165 'vec2(1.0)': vec2<float>
            161..164 '1.0': float
            176..188 'float_32_vec': vec2<f32>
            191..207 'acos(v...1.0f))': vec2<f32>
            196..206 'vec2(1.0f)': vec2<f32>
            201..205 '1.0f': f32
            217..229 'float_16_vec': vec2<f16>
            232..248 'acos(v...1.0h))': vec2<f16>
            237..247 'vec2(1.0h)': vec2<f16>
            242..246 '1.0h': f16
        "#]],
    );
}

#[test]
fn acosh() {
    check_infer(
        ExtensionsConfig {
            f16: true,
            ..Default::default()
        },
        "
enable f16;
fn foo() {
    let abstract_float = acosh(1.0);
    let float_32 = acosh(1.0f);
    let float_16 = acosh(1.0h);

    let abstract_float_vec = acosh(vec2(1.0));
    let float_32_vec = acosh(vec2(1.0f));
    let float_16_vec = acosh(vec2(1.0h));
}
",
        expect![[r#"
            31..45 'abstract_float': f32
            48..58 'acosh(1.0)': float
            54..57 '1.0': float
            68..76 'float_32': f32
            79..90 'acosh(1.0f)': f32
            85..89 '1.0f': f32
            100..108 'float_16': f16
            111..122 'acosh(1.0h)': f16
            117..121 '1.0h': f16
            133..151 'abstra...at_vec': vec2<f32>
            154..170 'acosh(...(1.0))': vec2<float>
            160..169 'vec2(1.0)': vec2<float>
            165..168 '1.0': float
            180..192 'float_32_vec': vec2<f32>
            195..212 'acosh(...1.0f))': vec2<f32>
            201..211 'vec2(1.0f)': vec2<f32>
            206..210 '1.0f': f32
            222..234 'float_16_vec': vec2<f16>
            237..254 'acosh(...1.0h))': vec2<f16>
            243..253 'vec2(1.0h)': vec2<f16>
            248..252 '1.0h': f16
        "#]],
    );
}

#[test]
fn asin() {
    check_infer(
        ExtensionsConfig {
            f16: true,
            ..Default::default()
        },
        "
enable f16;
fn foo() {
    let abstract_float = asin(1.0);
    let float_32 = asin(1.0f);
    let float_16 = asin(1.0h);

    let abstract_float_vec = asin(vec2(1.0));
    let float_32_vec = asin(vec2(1.0f));
    let float_16_vec = asin(vec2(1.0h));
}
",
        expect![[r#"
            31..45 'abstract_float': f32
            48..57 'asin(1.0)': float
            53..56 '1.0': float
            67..75 'float_32': f32
            78..88 'asin(1.0f)': f32
            83..87 '1.0f': f32
            98..106 'float_16': f16
            109..119 'asin(1.0h)': f16
            114..118 '1.0h': f16
            130..148 'abstra...at_vec': vec2<f32>
            151..166 'asin(vec2(1.0))': vec2<float>
            156..165 'vec2(1.0)': vec2<float>
            161..164 '1.0': float
            176..188 'float_32_vec': vec2<f32>
            191..207 'asin(v...1.0f))': vec2<f32>
            196..206 'vec2(1.0f)': vec2<f32>
            201..205 '1.0f': f32
            217..229 'float_16_vec': vec2<f16>
            232..248 'asin(v...1.0h))': vec2<f16>
            237..247 'vec2(1.0h)': vec2<f16>
            242..246 '1.0h': f16
        "#]],
    );
}

#[test]
fn asinh() {
    check_infer(
        ExtensionsConfig {
            f16: true,
            ..Default::default()
        },
        "
enable f16;
fn foo() {
    let abstract_float = asinh(1.0);
    let float_32 = asinh(1.0f);
    let float_16 = asinh(1.0h);

    let abstract_float_vec = asinh(vec2(1.0));
    let float_32_vec = asinh(vec2(1.0f));
    let float_16_vec = asinh(vec2(1.0h));
}
",
        expect![[r#"
            31..45 'abstract_float': f32
            48..58 'asinh(1.0)': float
            54..57 '1.0': float
            68..76 'float_32': f32
            79..90 'asinh(1.0f)': f32
            85..89 '1.0f': f32
            100..108 'float_16': f16
            111..122 'asinh(1.0h)': f16
            117..121 '1.0h': f16
            133..151 'abstra...at_vec': vec2<f32>
            154..170 'asinh(...(1.0))': vec2<float>
            160..169 'vec2(1.0)': vec2<float>
            165..168 '1.0': float
            180..192 'float_32_vec': vec2<f32>
            195..212 'asinh(...1.0f))': vec2<f32>
            201..211 'vec2(1.0f)': vec2<f32>
            206..210 '1.0f': f32
            222..234 'float_16_vec': vec2<f16>
            237..254 'asinh(...1.0h))': vec2<f16>
            243..253 'vec2(1.0h)': vec2<f16>
            248..252 '1.0h': f16
        "#]],
    );
}

#[test]
fn atan() {
    check_infer(
        ExtensionsConfig {
            f16: true,
            ..Default::default()
        },
        "
enable f16;
fn foo() {
    let abstract_float = atan(1.0);
    let float_32 = atan(1.0f);
    let float_16 = atan(1.0h);

    let abstract_float_vec = atan(vec2(1.0));
    let float_32_vec = atan(vec2(1.0f));
    let float_16_vec = atan(vec2(1.0h));
}
",
        expect![[r#"
            31..45 'abstract_float': f32
            48..57 'atan(1.0)': float
            53..56 '1.0': float
            67..75 'float_32': f32
            78..88 'atan(1.0f)': f32
            83..87 '1.0f': f32
            98..106 'float_16': f16
            109..119 'atan(1.0h)': f16
            114..118 '1.0h': f16
            130..148 'abstra...at_vec': vec2<f32>
            151..166 'atan(vec2(1.0))': vec2<float>
            156..165 'vec2(1.0)': vec2<float>
            161..164 '1.0': float
            176..188 'float_32_vec': vec2<f32>
            191..207 'atan(v...1.0f))': vec2<f32>
            196..206 'vec2(1.0f)': vec2<f32>
            201..205 '1.0f': f32
            217..229 'float_16_vec': vec2<f16>
            232..248 'atan(v...1.0h))': vec2<f16>
            237..247 'vec2(1.0h)': vec2<f16>
            242..246 '1.0h': f16
        "#]],
    );
}

#[test]
fn atanh() {
    check_infer(
        ExtensionsConfig {
            f16: true,
            ..Default::default()
        },
        "
enable f16;
fn foo() {
    let abstract_float = atanh(1.0);
    let float_32 = atanh(1.0f);
    let float_16 = atanh(1.0h);

    let abstract_float_vec = atanh(vec2(1.0));
    let float_32_vec = atanh(vec2(1.0f));
    let float_16_vec = atanh(vec2(1.0h));
}
",
        expect![[r#"
            31..45 'abstract_float': f32
            48..58 'atanh(1.0)': float
            54..57 '1.0': float
            68..76 'float_32': f32
            79..90 'atanh(1.0f)': f32
            85..89 '1.0f': f32
            100..108 'float_16': f16
            111..122 'atanh(1.0h)': f16
            117..121 '1.0h': f16
            133..151 'abstra...at_vec': vec2<f32>
            154..170 'atanh(...(1.0))': vec2<float>
            160..169 'vec2(1.0)': vec2<float>
            165..168 '1.0': float
            180..192 'float_32_vec': vec2<f32>
            195..212 'atanh(...1.0f))': vec2<f32>
            201..211 'vec2(1.0f)': vec2<f32>
            206..210 '1.0f': f32
            222..234 'float_16_vec': vec2<f16>
            237..254 'atanh(...1.0h))': vec2<f16>
            243..253 'vec2(1.0h)': vec2<f16>
            248..252 '1.0h': f16
        "#]],
    );
}

#[test]
fn atan2() {
    check_infer(
        ExtensionsConfig {
            f16: true,
            ..Default::default()
        },
        "
enable f16;
fn foo() {
    let abstract_float = atan2(1.0, 1.0);
    let float_32 = atan2(1.0f, 1.0f);
    let float_16 = atan2(1.0h, 1.0h);

    let abstract_float_vec = atan2(vec2(1.0), vec2(1.0));
    let float_32_vec = atan2(vec2(1.0f), vec2(1.0f));
    let float_16_vec = atan2(vec2(1.0h), vec2(1.0h));
}
",
        expect![[r#"
            31..45 'abstract_float': f32
            48..63 'atan2(1.0, 1.0)': float
            54..57 '1.0': float
            59..62 '1.0': float
            73..81 'float_32': f32
            84..101 'atan2(... 1.0f)': f32
            90..94 '1.0f': f32
            96..100 '1.0f': f32
            111..119 'float_16': f16
            122..139 'atan2(... 1.0h)': f16
            128..132 '1.0h': f16
            134..138 '1.0h': f16
            150..168 'abstra...at_vec': vec2<f32>
            171..198 'atan2(...(1.0))': vec2<float>
            177..186 'vec2(1.0)': vec2<float>
            182..185 '1.0': float
            188..197 'vec2(1.0)': vec2<float>
            193..196 '1.0': float
            208..220 'float_32_vec': vec2<f32>
            223..252 'atan2(...1.0f))': vec2<f32>
            229..239 'vec2(1.0f)': vec2<f32>
            234..238 '1.0f': f32
            241..251 'vec2(1.0f)': vec2<f32>
            246..250 '1.0f': f32
            262..274 'float_16_vec': vec2<f16>
            277..306 'atan2(...1.0h))': vec2<f16>
            283..293 'vec2(1.0h)': vec2<f16>
            288..292 '1.0h': f16
            295..305 'vec2(1.0h)': vec2<f16>
            300..304 '1.0h': f16
        "#]],
    );
}

#[test]
fn ceil() {
    check_infer(
        ExtensionsConfig {
            f16: true,
            ..Default::default()
        },
        "
enable f16;
fn foo() {
    let abstract_float = ceil(1.0);
    let float_32 = ceil(1.0f);
    let float_16 = ceil(1.0h);

    let abstract_float_vec = ceil(vec2(1.0));
    let float_32_vec = ceil(vec2(1.0f));
    let float_16_vec = ceil(vec2(1.0h));
}
",
        expect![[r#"
            31..45 'abstract_float': f32
            48..57 'ceil(1.0)': float
            53..56 '1.0': float
            67..75 'float_32': f32
            78..88 'ceil(1.0f)': f32
            83..87 '1.0f': f32
            98..106 'float_16': f16
            109..119 'ceil(1.0h)': f16
            114..118 '1.0h': f16
            130..148 'abstra...at_vec': vec2<f32>
            151..166 'ceil(vec2(1.0))': vec2<float>
            156..165 'vec2(1.0)': vec2<float>
            161..164 '1.0': float
            176..188 'float_32_vec': vec2<f32>
            191..207 'ceil(v...1.0f))': vec2<f32>
            196..206 'vec2(1.0f)': vec2<f32>
            201..205 '1.0f': f32
            217..229 'float_16_vec': vec2<f16>
            232..248 'ceil(v...1.0h))': vec2<f16>
            237..247 'vec2(1.0h)': vec2<f16>
            242..246 '1.0h': f16
        "#]],
    );
}

#[test]
fn clamp() {
    check_infer(
        ExtensionsConfig {
            f16: true,
            ..Default::default()
        },
        "
enable f16;
fn foo() {
    let abstract_int = clamp(1, 1, 1);
    let abstract_float = clamp(1.0, 1.0, 1.0);
    let signed_integer_32 = clamp(1i, 1i, 1i);
    let unsigned_integer_32 = clamp(1u, 1u, 1u);
    let float_32 = clamp(1.0f, 1.0f, 1.0f);
    let float_16 = clamp(1.0h, 1.0h, 1.0h);

    let abstract_int_vec = clamp(vec2(1), vec2(1), vec2(1));
    let abstract_float_vec = clamp(vec2(1.0), vec2(1.0), vec2(1.0));
    let signed_integer_32_vec = clamp(vec2(1i), vec2(1i), vec2(1i));
    let unsigned_integer_32_vec = clamp(vec2(1u), vec2(1u), vec2(1u));
    let float_32_vec = clamp(vec2(1.0f), vec2(1.0f), vec2(1.0f));
    let float_16_vec = clamp(vec2(1.0h), vec2(1.0h), vec2(1.0h));
}
",
        expect![[r#"
            31..43 'abstract_int': i32
            46..60 'clamp(1, 1, 1)': integer
            52..53 '1': integer
            55..56 '1': integer
            58..59 '1': integer
            70..84 'abstract_float': f32
            87..107 'clamp(..., 1.0)': float
            93..96 '1.0': float
            98..101 '1.0': float
            103..106 '1.0': float
            117..134 'signed...ger_32': i32
            137..154 'clamp(...i, 1i)': i32
            143..145 '1i': i32
            147..149 '1i': i32
            151..153 '1i': i32
            164..183 'unsign...ger_32': u32
            186..203 'clamp(...u, 1u)': u32
            192..194 '1u': u32
            196..198 '1u': u32
            200..202 '1u': u32
            213..221 'float_32': f32
            224..247 'clamp(... 1.0f)': f32
            230..234 '1.0f': f32
            236..240 '1.0f': f32
            242..246 '1.0f': f32
            257..265 'float_16': f16
            268..291 'clamp(... 1.0h)': f16
            274..278 '1.0h': f16
            280..284 '1.0h': f16
            286..290 '1.0h': f16
            302..318 'abstra...nt_vec': vec2<i32>
            321..353 'clamp(...c2(1))': vec2<integer>
            327..334 'vec2(1)': vec2<integer>
            332..333 '1': integer
            336..343 'vec2(1)': vec2<integer>
            341..342 '1': integer
            345..352 'vec2(1)': vec2<integer>
            350..351 '1': integer
            363..381 'abstra...at_vec': vec2<f32>
            384..422 'clamp(...(1.0))': vec2<float>
            390..399 'vec2(1.0)': vec2<float>
            395..398 '1.0': float
            401..410 'vec2(1.0)': vec2<float>
            406..409 '1.0': float
            412..421 'vec2(1.0)': vec2<float>
            417..420 '1.0': float
            432..453 'signed...32_vec': vec2<i32>
            456..491 'clamp(...2(1i))': vec2<i32>
            462..470 'vec2(1i)': vec2<i32>
            467..469 '1i': i32
            472..480 'vec2(1i)': vec2<i32>
            477..479 '1i': i32
            482..490 'vec2(1i)': vec2<i32>
            487..489 '1i': i32
            501..524 'unsign...32_vec': vec2<u32>
            527..562 'clamp(...2(1u))': vec2<u32>
            533..541 'vec2(1u)': vec2<u32>
            538..540 '1u': u32
            543..551 'vec2(1u)': vec2<u32>
            548..550 '1u': u32
            553..561 'vec2(1u)': vec2<u32>
            558..560 '1u': u32
            572..584 'float_32_vec': vec2<f32>
            587..628 'clamp(...1.0f))': vec2<f32>
            593..603 'vec2(1.0f)': vec2<f32>
            598..602 '1.0f': f32
            605..615 'vec2(1.0f)': vec2<f32>
            610..614 '1.0f': f32
            617..627 'vec2(1.0f)': vec2<f32>
            622..626 '1.0f': f32
            638..650 'float_16_vec': vec2<f16>
            653..694 'clamp(...1.0h))': vec2<f16>
            659..669 'vec2(1.0h)': vec2<f16>
            664..668 '1.0h': f16
            671..681 'vec2(1.0h)': vec2<f16>
            676..680 '1.0h': f16
            683..693 'vec2(1.0h)': vec2<f16>
            688..692 '1.0h': f16
        "#]],
    );
}

#[test]
fn cos() {
    check_infer(
        ExtensionsConfig {
            f16: true,
            ..Default::default()
        },
        "
enable f16;
fn foo() {
    let abstract_float = cos(1.0);
    let float_32 = cos(1.0f);
    let float_16 = cos(1.0h);

    let abstract_float_vec = cos(vec2(1.0));
    let float_32_vec = cos(vec2(1.0f));
    let float_16_vec = cos(vec2(1.0h));
}
",
        expect![[r#"
            31..45 'abstract_float': f32
            48..56 'cos(1.0)': float
            52..55 '1.0': float
            66..74 'float_32': f32
            77..86 'cos(1.0f)': f32
            81..85 '1.0f': f32
            96..104 'float_16': f16
            107..116 'cos(1.0h)': f16
            111..115 '1.0h': f16
            127..145 'abstra...at_vec': vec2<f32>
            148..162 'cos(vec2(1.0))': vec2<float>
            152..161 'vec2(1.0)': vec2<float>
            157..160 '1.0': float
            172..184 'float_32_vec': vec2<f32>
            187..202 'cos(vec2(1.0f))': vec2<f32>
            191..201 'vec2(1.0f)': vec2<f32>
            196..200 '1.0f': f32
            212..224 'float_16_vec': vec2<f16>
            227..242 'cos(vec2(1.0h))': vec2<f16>
            231..241 'vec2(1.0h)': vec2<f16>
            236..240 '1.0h': f16
        "#]],
    );
}

#[test]
fn cosh() {
    check_infer(
        ExtensionsConfig {
            f16: true,
            ..Default::default()
        },
        "
enable f16;
fn foo() {
    let abstract_float = cosh(1.0);
    let float_32 = cosh(1.0f);
    let float_16 = cosh(1.0h);

    let abstract_float_vec = cosh(vec2(1.0));
    let float_32_vec = cosh(vec2(1.0f));
    let float_16_vec = cosh(vec2(1.0h));
}
",
        expect![[r#"
            31..45 'abstract_float': f32
            48..57 'cosh(1.0)': float
            53..56 '1.0': float
            67..75 'float_32': f32
            78..88 'cosh(1.0f)': f32
            83..87 '1.0f': f32
            98..106 'float_16': f16
            109..119 'cosh(1.0h)': f16
            114..118 '1.0h': f16
            130..148 'abstra...at_vec': vec2<f32>
            151..166 'cosh(vec2(1.0))': vec2<float>
            156..165 'vec2(1.0)': vec2<float>
            161..164 '1.0': float
            176..188 'float_32_vec': vec2<f32>
            191..207 'cosh(v...1.0f))': vec2<f32>
            196..206 'vec2(1.0f)': vec2<f32>
            201..205 '1.0f': f32
            217..229 'float_16_vec': vec2<f16>
            232..248 'cosh(v...1.0h))': vec2<f16>
            237..247 'vec2(1.0h)': vec2<f16>
            242..246 '1.0h': f16
        "#]],
    );
}

#[test]
fn countLeadingZeros() {
    check_infer(
        ExtensionsConfig::default(),
        "
fn foo() {
    let signed_integer_32 = countLeadingZeros(1i);
    let unsigned_integer_32 = countLeadingZeros(1u);

    let signed_integer_32_vec = countLeadingZeros(vec2(1i));
    let unsigned_integer_32_vec = countLeadingZeros(vec2(1u));
}
",
        expect![[r#"
            19..36 'signed...ger_32': i32
            39..60 'countL...os(1i)': i32
            57..59 '1i': i32
            70..89 'unsign...ger_32': u32
            92..113 'countL...os(1u)': u32
            110..112 '1u': u32
            124..145 'signed...32_vec': vec2<i32>
            148..175 'countL...2(1i))': vec2<i32>
            166..174 'vec2(1i)': vec2<i32>
            171..173 '1i': i32
            185..208 'unsign...32_vec': vec2<u32>
            211..238 'countL...2(1u))': vec2<u32>
            229..237 'vec2(1u)': vec2<u32>
            234..236 '1u': u32
        "#]],
    );
}

#[test]
fn countOneBits() {
    check_infer(
        ExtensionsConfig::default(),
        "
fn foo() {
    let signed_integer_32 = countOneBits(1i);
    let unsigned_integer_32 = countOneBits(1u);

    let signed_integer_32_vec = countOneBits(vec2(1i));
    let unsigned_integer_32_vec = countOneBits(vec2(1u));
}
",
        expect![[r#"
            19..36 'signed...ger_32': i32
            39..55 'countO...ts(1i)': i32
            52..54 '1i': i32
            65..84 'unsign...ger_32': u32
            87..103 'countO...ts(1u)': u32
            100..102 '1u': u32
            114..135 'signed...32_vec': vec2<i32>
            138..160 'countO...2(1i))': vec2<i32>
            151..159 'vec2(1i)': vec2<i32>
            156..158 '1i': i32
            170..193 'unsign...32_vec': vec2<u32>
            196..218 'countO...2(1u))': vec2<u32>
            209..217 'vec2(1u)': vec2<u32>
            214..216 '1u': u32
        "#]],
    );
}

#[test]
fn countTrailingZeros() {
    check_infer(
        ExtensionsConfig::default(),
        "
fn foo() {
    let signed_integer_32 = countTrailingZeros(1i);
    let unsigned_integer_32 = countTrailingZeros(1u);

    let signed_integer_32_vec = countTrailingZeros(vec2(1i));
    let unsigned_integer_32_vec = countTrailingZeros(vec2(1u));
}
",
        expect![[r#"
            19..36 'signed...ger_32': i32
            39..61 'countT...os(1i)': i32
            58..60 '1i': i32
            71..90 'unsign...ger_32': u32
            93..115 'countT...os(1u)': u32
            112..114 '1u': u32
            126..147 'signed...32_vec': vec2<i32>
            150..178 'countT...2(1i))': vec2<i32>
            169..177 'vec2(1i)': vec2<i32>
            174..176 '1i': i32
            188..211 'unsign...32_vec': vec2<u32>
            214..242 'countT...2(1u))': vec2<u32>
            233..241 'vec2(1u)': vec2<u32>
            238..240 '1u': u32
        "#]],
    );
}

#[test]
fn cross() {
    check_infer(
        ExtensionsConfig {
            f16: true,
            ..Default::default()
        },
        "
enable f16;
fn foo() {
    let abstract_float_vec = cross(vec3(1.0), vec3(1.0));
    let float_32_vec = cross(vec3(1.0f), vec3(1.0f));
    let float_16_vec = cross(vec3(1.0h), vec3(1.0h));
}
",
        expect![[r#"
            31..49 'abstra...at_vec': vec3<f32>
            52..79 'cross(...(1.0))': vec3<float>
            58..67 'vec3(1.0)': vec3<float>
            63..66 '1.0': float
            69..78 'vec3(1.0)': vec3<float>
            74..77 '1.0': float
            89..101 'float_32_vec': vec3<f32>
            104..133 'cross(...1.0f))': vec3<f32>
            110..120 'vec3(1.0f)': vec3<f32>
            115..119 '1.0f': f32
            122..132 'vec3(1.0f)': vec3<f32>
            127..131 '1.0f': f32
            143..155 'float_16_vec': vec3<f16>
            158..187 'cross(...1.0h))': vec3<f16>
            164..174 'vec3(1.0h)': vec3<f16>
            169..173 '1.0h': f16
            176..186 'vec3(1.0h)': vec3<f16>
            181..185 '1.0h': f16
        "#]],
    );
}

#[test]
fn degrees() {
    check_infer(
        ExtensionsConfig {
            f16: true,
            ..Default::default()
        },
        "
enable f16;
fn foo() {
    let abstract_float = degrees(1.0);
    let float_32 = degrees(1.0f);
    let float_16 = degrees(1.0h);

    let abstract_float_vec = degrees(vec2(1.0));
    let float_32_vec = degrees(vec2(1.0f));
    let float_16_vec = degrees(vec2(1.0h));
}
",
        expect![[r#"
            31..45 'abstract_float': f32
            48..60 'degrees(1.0)': float
            56..59 '1.0': float
            70..78 'float_32': f32
            81..94 'degrees(1.0f)': f32
            89..93 '1.0f': f32
            104..112 'float_16': f16
            115..128 'degrees(1.0h)': f16
            123..127 '1.0h': f16
            139..157 'abstra...at_vec': vec2<f32>
            160..178 'degree...(1.0))': vec2<float>
            168..177 'vec2(1.0)': vec2<float>
            173..176 '1.0': float
            188..200 'float_32_vec': vec2<f32>
            203..222 'degree...1.0f))': vec2<f32>
            211..221 'vec2(1.0f)': vec2<f32>
            216..220 '1.0f': f32
            232..244 'float_16_vec': vec2<f16>
            247..266 'degree...1.0h))': vec2<f16>
            255..265 'vec2(1.0h)': vec2<f16>
            260..264 '1.0h': f16
        "#]],
    );
}

#[test]
fn determinant() {
    check_infer(
        ExtensionsConfig {
            f16: true,
            ..Default::default()
        },
        "
enable f16;
fn foo() {
    let abstract_float = determinant(mat2x2(1.0));
    let float_32 = determinant(mat2x2(1.0f));
    let float_16 = determinant(mat2x2(1.0h));
}
",
        expect![[r#"
            31..45 'abstract_float': f32
            48..72 'determ...(1.0))': float
            60..71 'mat2x2(1.0)': mat2x2<float>
            67..70 '1.0': float
            82..90 'float_32': f32
            93..118 'determ...1.0f))': f32
            105..117 'mat2x2(1.0f)': mat2x2<f32>
            112..116 '1.0f': f32
            128..136 'float_16': f16
            139..164 'determ...1.0h))': f16
            151..163 'mat2x2(1.0h)': mat2x2<f16>
            158..162 '1.0h': f16
        "#]],
    );
}

#[test]
fn distance() {
    check_infer(
        ExtensionsConfig {
            f16: true,
            ..Default::default()
        },
        "
enable f16;
fn foo() {
    let abstract_float = distance(1.0, 1.0);
    let float_32 = distance(1.0f, 1.0f);
    let float_16 = distance(1.0h, 1.0h);

    let abstract_float_vec = distance(vec2(1.0), vec2(1.0));
    let float_32_vec = distance(vec2(1.0f), vec2(1.0f));
    let float_16_vec = distance(vec2(1.0h), vec2(1.0h));
}
",
        expect![[r#"
            31..45 'abstract_float': f32
            48..66 'distan..., 1.0)': float
            57..60 '1.0': float
            62..65 '1.0': float
            76..84 'float_32': f32
            87..107 'distan... 1.0f)': f32
            96..100 '1.0f': f32
            102..106 '1.0f': f32
            117..125 'float_16': f16
            128..148 'distan... 1.0h)': f16
            137..141 '1.0h': f16
            143..147 '1.0h': f16
            159..177 'abstra...at_vec': f32
            180..210 'distan...(1.0))': float
            189..198 'vec2(1.0)': vec2<float>
            194..197 '1.0': float
            200..209 'vec2(1.0)': vec2<float>
            205..208 '1.0': float
            220..232 'float_32_vec': f32
            235..267 'distan...1.0f))': f32
            244..254 'vec2(1.0f)': vec2<f32>
            249..253 '1.0f': f32
            256..266 'vec2(1.0f)': vec2<f32>
            261..265 '1.0f': f32
            277..289 'float_16_vec': f16
            292..324 'distan...1.0h))': f16
            301..311 'vec2(1.0h)': vec2<f16>
            306..310 '1.0h': f16
            313..323 'vec2(1.0h)': vec2<f16>
            318..322 '1.0h': f16
        "#]],
    );
}

#[test]
fn dot4U8Packed() {
    check_infer(
        ExtensionsConfig {
            f16: true,
            ..Default::default()
        },
        "
enable f16;
fn foo() {
    let unsigned_integer_32 = dot4U8Packed(1u, 1u);
}
",
        expect![[r#"
            31..50 'unsign...ger_32': u32
            53..73 'dot4U8...u, 1u)': u32
            66..68 '1u': u32
            70..72 '1u': u32
        "#]],
    );
}

#[test]
fn dot4I8Packed() {
    check_infer(
        ExtensionsConfig::default(),
        "
fn foo() {
    let signed_integer_32 = dot4I8Packed(1u, 1u);
}
",
        expect![[r#"
            19..36 'signed...ger_32': i32
            39..59 'dot4I8...u, 1u)': i32
            52..54 '1u': u32
            56..58 '1u': u32
        "#]],
    );
}

#[test]
fn exp() {
    check_infer(
        ExtensionsConfig {
            f16: true,
            ..Default::default()
        },
        "
enable f16;
fn foo() {
    let abstract_float = exp(1.0);
    let float_32 = exp(1.0f);
    let float_16 = exp(1.0h);

    let abstract_float_vec = exp(vec2(1.0));
    let float_32_vec = exp(vec2(1.0f));
    let float_16_vec = exp(vec2(1.0h));
}
",
        expect![[r#"
            31..45 'abstract_float': f32
            48..56 'exp(1.0)': float
            52..55 '1.0': float
            66..74 'float_32': f32
            77..86 'exp(1.0f)': f32
            81..85 '1.0f': f32
            96..104 'float_16': f16
            107..116 'exp(1.0h)': f16
            111..115 '1.0h': f16
            127..145 'abstra...at_vec': vec2<f32>
            148..162 'exp(vec2(1.0))': vec2<float>
            152..161 'vec2(1.0)': vec2<float>
            157..160 '1.0': float
            172..184 'float_32_vec': vec2<f32>
            187..202 'exp(vec2(1.0f))': vec2<f32>
            191..201 'vec2(1.0f)': vec2<f32>
            196..200 '1.0f': f32
            212..224 'float_16_vec': vec2<f16>
            227..242 'exp(vec2(1.0h))': vec2<f16>
            231..241 'vec2(1.0h)': vec2<f16>
            236..240 '1.0h': f16
        "#]],
    );
}

#[test]
fn exp2() {
    check_infer(
        ExtensionsConfig {
            f16: true,
            ..Default::default()
        },
        "
enable f16;
fn foo() {
    let abstract_float = exp2(1.0);
    let float_32 = exp2(1.0f);
    let float_16 = exp2(1.0h);

    let abstract_float_vec = exp2(vec2(1.0));
    let float_32_vec = exp2(vec2(1.0f));
    let float_16_vec = exp2(vec2(1.0h));
}
",
        expect![[r#"
            31..45 'abstract_float': f32
            48..57 'exp2(1.0)': float
            53..56 '1.0': float
            67..75 'float_32': f32
            78..88 'exp2(1.0f)': f32
            83..87 '1.0f': f32
            98..106 'float_16': f16
            109..119 'exp2(1.0h)': f16
            114..118 '1.0h': f16
            130..148 'abstra...at_vec': vec2<f32>
            151..166 'exp2(vec2(1.0))': vec2<float>
            156..165 'vec2(1.0)': vec2<float>
            161..164 '1.0': float
            176..188 'float_32_vec': vec2<f32>
            191..207 'exp2(v...1.0f))': vec2<f32>
            196..206 'vec2(1.0f)': vec2<f32>
            201..205 '1.0f': f32
            217..229 'float_16_vec': vec2<f16>
            232..248 'exp2(v...1.0h))': vec2<f16>
            237..247 'vec2(1.0h)': vec2<f16>
            242..246 '1.0h': f16
        "#]],
    );
}

#[test]
fn extractBits_signed() {
    check_infer(
        ExtensionsConfig::default(),
        "
fn foo() {
    let signed_integer_32 = extractBits(1i, 0, 0);
    let signed_integer_32_vec = extractBits(vec2(1i), 0, 0);
}
",
        expect![[r#"
            19..36 'signed...ger_32': i32
            39..60 'extrac... 0, 0)': i32
            51..53 '1i': i32
            55..56 '0': integer
            58..59 '0': integer
            70..91 'signed...32_vec': vec2<i32>
            94..121 'extrac... 0, 0)': vec2<i32>
            106..114 'vec2(1i)': vec2<i32>
            111..113 '1i': i32
            116..117 '0': integer
            119..120 '0': integer
        "#]],
    );
}

#[test]
fn extractBits_unsigned() {
    check_infer(
        ExtensionsConfig::default(),
        "
fn foo() {
    let unsigned_integer_32 = extractBits(1u, 0, 0);
    let unsigned_integer_32_vec = extractBits(vec2(1u), 0, 0);
}
",
        expect![[r#"
            19..38 'unsign...ger_32': u32
            41..62 'extrac... 0, 0)': u32
            53..55 '1u': u32
            57..58 '0': integer
            60..61 '0': integer
            72..95 'unsign...32_vec': vec2<u32>
            98..125 'extrac... 0, 0)': vec2<u32>
            110..118 'vec2(1u)': vec2<u32>
            115..117 '1u': u32
            120..121 '0': integer
            123..124 '0': integer
        "#]],
    );
}

#[test]
fn faceForward() {
    check_infer(
        ExtensionsConfig {
            f16: true,
            ..Default::default()
        },
        "
enable f16;
fn foo() {
    let abstract_float_vec = faceForward(vec2(1.0), vec2(1.0), vec2(1.0));
    let float_32_vec = faceForward(vec2(1.0f), vec2(1.0f), vec2(1.0f));
    let float_16_vec = faceForward(vec2(1.0h), vec2(1.0h), vec2(1.0h));
}
",
        expect![[r#"
            31..49 'abstra...at_vec': vec2<f32>
            52..96 'faceFo...(1.0))': vec2<float>
            64..73 'vec2(1.0)': vec2<float>
            69..72 '1.0': float
            75..84 'vec2(1.0)': vec2<float>
            80..83 '1.0': float
            86..95 'vec2(1.0)': vec2<float>
            91..94 '1.0': float
            106..118 'float_32_vec': vec2<f32>
            121..168 'faceFo...1.0f))': vec2<f32>
            133..143 'vec2(1.0f)': vec2<f32>
            138..142 '1.0f': f32
            145..155 'vec2(1.0f)': vec2<f32>
            150..154 '1.0f': f32
            157..167 'vec2(1.0f)': vec2<f32>
            162..166 '1.0f': f32
            178..190 'float_16_vec': vec2<f16>
            193..240 'faceFo...1.0h))': vec2<f16>
            205..215 'vec2(1.0h)': vec2<f16>
            210..214 '1.0h': f16
            217..227 'vec2(1.0h)': vec2<f16>
            222..226 '1.0h': f16
            229..239 'vec2(1.0h)': vec2<f16>
            234..238 '1.0h': f16
        "#]],
    );
}

#[test]
fn firstLeadingBit_signed() {
    check_infer(
        ExtensionsConfig::default(),
        "
fn foo() {
    let signed_integer_32 = firstLeadingBit(1i);
    let signed_integer_32_vec = firstLeadingBit(vec2(1i));
}
",
        expect![[r#"
            19..36 'signed...ger_32': i32
            39..58 'firstL...it(1i)': i32
            55..57 '1i': i32
            68..89 'signed...32_vec': vec2<i32>
            92..117 'firstL...2(1i))': vec2<i32>
            108..116 'vec2(1i)': vec2<i32>
            113..115 '1i': i32
        "#]],
    );
}

#[test]
fn firstLeadingBit_unsigned() {
    check_infer(
        ExtensionsConfig::default(),
        "
fn foo() {
    let unsigned_integer_32 = firstLeadingBit(1u);
    let unsigned_integer_32_vec = firstLeadingBit(vec2(1u));
}
",
        expect![[r#"
            19..38 'unsign...ger_32': u32
            41..60 'firstL...it(1u)': u32
            57..59 '1u': u32
            70..93 'unsign...32_vec': vec2<u32>
            96..121 'firstL...2(1u))': vec2<u32>
            112..120 'vec2(1u)': vec2<u32>
            117..119 '1u': u32
        "#]],
    );
}

#[test]
fn firstTrailingBit_signed() {
    check_infer(
        ExtensionsConfig::default(),
        "
fn foo() {
    let signed_integer_32 = firstTrailingBit(1i);
    let signed_integer_32_vec = firstTrailingBit(vec2(1i));
}
",
        expect![[r#"
            19..36 'signed...ger_32': i32
            39..59 'firstT...it(1i)': i32
            56..58 '1i': i32
            69..90 'signed...32_vec': vec2<i32>
            93..119 'firstT...2(1i))': vec2<i32>
            110..118 'vec2(1i)': vec2<i32>
            115..117 '1i': i32
        "#]],
    );
}

#[test]
fn firstTrailingBit_unsigned() {
    check_infer(
        ExtensionsConfig::default(),
        "
fn foo() {
    let unsigned_integer_32 = firstTrailingBit(1u);
    let unsigned_integer_32_vec = firstTrailingBit(vec2(1u));
}
",
        expect![[r#"
            19..38 'unsign...ger_32': u32
            41..61 'firstT...it(1u)': u32
            58..60 '1u': u32
            71..94 'unsign...32_vec': vec2<u32>
            97..123 'firstT...2(1u))': vec2<u32>
            114..122 'vec2(1u)': vec2<u32>
            119..121 '1u': u32
        "#]],
    );
}

#[test]
fn floor() {
    check_infer(
        ExtensionsConfig {
            f16: true,
            ..Default::default()
        },
        "
enable f16;
fn foo() {
    let abstract_float = floor(1.0);
    let float_32 = floor(1.0f);
    let float_16 = floor(1.0h);

    let abstract_float_vec = floor(vec2(1.0));
    let float_32_vec = floor(vec2(1.0f));
    let float_16_vec = floor(vec2(1.0h));
}
",
        expect![[r#"
            31..45 'abstract_float': f32
            48..58 'floor(1.0)': float
            54..57 '1.0': float
            68..76 'float_32': f32
            79..90 'floor(1.0f)': f32
            85..89 '1.0f': f32
            100..108 'float_16': f16
            111..122 'floor(1.0h)': f16
            117..121 '1.0h': f16
            133..151 'abstra...at_vec': vec2<f32>
            154..170 'floor(...(1.0))': vec2<float>
            160..169 'vec2(1.0)': vec2<float>
            165..168 '1.0': float
            180..192 'float_32_vec': vec2<f32>
            195..212 'floor(...1.0f))': vec2<f32>
            201..211 'vec2(1.0f)': vec2<f32>
            206..210 '1.0f': f32
            222..234 'float_16_vec': vec2<f16>
            237..254 'floor(...1.0h))': vec2<f16>
            243..253 'vec2(1.0h)': vec2<f16>
            248..252 '1.0h': f16
        "#]],
    );
}

#[test]
fn fma() {
    check_infer(
        ExtensionsConfig {
            f16: true,
            ..Default::default()
        },
        "
enable f16;
fn foo() {
    let abstract_float = fma(1.0, 1.0, 1.0);
    let float_32 = fma(1.0f, 1.0f, 1.0f);
    let float_16 = fma(1.0h, 1.0h, 1.0h);

    let abstract_float_vec = fma(vec2(1.0), vec2(1.0), vec2(1.0));
    let float_32_vec = fma(vec2(1.0f), vec2(1.0f), vec2(1.0f));
    let float_16_vec = fma(vec2(1.0h), vec2(1.0h), vec2(1.0h));
}
",
        expect![[r#"
            31..45 'abstract_float': f32
            48..66 'fma(1...., 1.0)': float
            52..55 '1.0': float
            57..60 '1.0': float
            62..65 '1.0': float
            76..84 'float_32': f32
            87..108 'fma(1.... 1.0f)': f32
            91..95 '1.0f': f32
            97..101 '1.0f': f32
            103..107 '1.0f': f32
            118..126 'float_16': f16
            129..150 'fma(1.... 1.0h)': f16
            133..137 '1.0h': f16
            139..143 '1.0h': f16
            145..149 '1.0h': f16
            161..179 'abstra...at_vec': vec2<f32>
            182..218 'fma(ve...(1.0))': vec2<float>
            186..195 'vec2(1.0)': vec2<float>
            191..194 '1.0': float
            197..206 'vec2(1.0)': vec2<float>
            202..205 '1.0': float
            208..217 'vec2(1.0)': vec2<float>
            213..216 '1.0': float
            228..240 'float_32_vec': vec2<f32>
            243..282 'fma(ve...1.0f))': vec2<f32>
            247..257 'vec2(1.0f)': vec2<f32>
            252..256 '1.0f': f32
            259..269 'vec2(1.0f)': vec2<f32>
            264..268 '1.0f': f32
            271..281 'vec2(1.0f)': vec2<f32>
            276..280 '1.0f': f32
            292..304 'float_16_vec': vec2<f16>
            307..346 'fma(ve...1.0h))': vec2<f16>
            311..321 'vec2(1.0h)': vec2<f16>
            316..320 '1.0h': f16
            323..333 'vec2(1.0h)': vec2<f16>
            328..332 '1.0h': f16
            335..345 'vec2(1.0h)': vec2<f16>
            340..344 '1.0h': f16
        "#]],
    );
}

#[test]
fn fract() {
    check_infer(
        ExtensionsConfig {
            f16: true,
            ..Default::default()
        },
        "
enable f16;
fn foo() {
    let abstract_float = fract(1.0);
    let float_32 = fract(1.0f);
    let float_16 = fract(1.0h);

    let abstract_float_vec = fract(vec2(1.0));
    let float_32_vec = fract(vec2(1.0f));
    let float_16_vec = fract(vec2(1.0h));
}
",
        expect![[r#"
            31..45 'abstract_float': f32
            48..58 'fract(1.0)': float
            54..57 '1.0': float
            68..76 'float_32': f32
            79..90 'fract(1.0f)': f32
            85..89 '1.0f': f32
            100..108 'float_16': f16
            111..122 'fract(1.0h)': f16
            117..121 '1.0h': f16
            133..151 'abstra...at_vec': vec2<f32>
            154..170 'fract(...(1.0))': vec2<float>
            160..169 'vec2(1.0)': vec2<float>
            165..168 '1.0': float
            180..192 'float_32_vec': vec2<f32>
            195..212 'fract(...1.0f))': vec2<f32>
            201..211 'vec2(1.0f)': vec2<f32>
            206..210 '1.0f': f32
            222..234 'float_16_vec': vec2<f16>
            237..254 'fract(...1.0h))': vec2<f16>
            243..253 'vec2(1.0h)': vec2<f16>
            248..252 '1.0h': f16
        "#]],
    );
}

#[test]
fn frexp() {
    check_infer(
        ExtensionsConfig {
            f16: true,
            ..Default::default()
        },
        "
enable f16;
fn foo() {
    let abstract_float = frexp(1.0);
    let abstract_float_fract = abstract_float.fract;
    let abstract_float_exp = abstract_float.exp;
    let float_32 = frexp(1.0f);
    let float_32_fract = float_32.fract;
    let float_32_exp = float_32.exp;
    let float_16 = frexp(1.0h);
    let float_16_fract = float_16.fract;
    let float_16_exp = float_16.exp;

    let abstract_float_vec = frexp(vec2(1.0));
    let abstract_float_vec_fract = abstract_float_vec.fract;
    let abstract_float_vec_exp = abstract_float_vec.exp;
    let float_32_vec = frexp(vec2(1.0f));
    let float_32_vec_fract = float_32_vec.fract;
    let float_32_vec_exp = float_32_vec.exp;
    let float_16_vec = frexp(vec2(1.0h));
    let float_16_vec_fract = float_16_vec.fract;
    let float_16_vec_exp = float_16_vec.exp;
}
",
        expect![[r#"
            31..45 'abstract_float': __frexp_result_abstract
            48..58 'frexp(1.0)': __frexp_result_abstract
            54..57 '1.0': float
            68..88 'abstra..._fract': f32
            91..105 'abstract_float': __frexp_result_abstract
            91..111 'abstra....fract': float
            121..139 'abstra...at_exp': i32
            142..156 'abstract_float': __frexp_result_abstract
            142..160 'abstra...at.exp': integer
            170..178 'float_32': __frexp_result_f32
            181..192 'frexp(1.0f)': __frexp_result_f32
            187..191 '1.0f': f32
            202..216 'float_32_fract': f32
            219..227 'float_32': __frexp_result_f32
            219..233 'float_32.fract': f32
            243..255 'float_32_exp': i32
            258..266 'float_32': __frexp_result_f32
            258..270 'float_32.exp': i32
            280..288 'float_16': __frexp_result_f16
            291..302 'frexp(1.0h)': __frexp_result_f16
            297..301 '1.0h': f16
            312..326 'float_16_fract': f16
            329..337 'float_16': __frexp_result_f16
            329..343 'float_16.fract': f16
            353..365 'float_16_exp': i32
            368..376 'float_16': __frexp_result_f16
            368..380 'float_16.exp': i32
            391..409 'abstra...at_vec': __frexp_result_vec2_abstract
            412..428 'frexp(...(1.0))': __frexp_result_vec2_abstract
            418..427 'vec2(1.0)': vec2<float>
            423..426 '1.0': float
            438..462 'abstra..._fract': vec2<f32>
            465..483 'abstra...at_vec': __frexp_result_vec2_abstract
            465..489 'abstra....fract': vec2<float>
            499..521 'abstra...ec_exp': vec2<i32>
            524..542 'abstra...at_vec': __frexp_result_vec2_abstract
            524..546 'abstra...ec.exp': vec2<integer>
            556..568 'float_32_vec': __frexp_result_vec2_f32
            571..588 'frexp(...1.0f))': __frexp_result_vec2_f32
            577..587 'vec2(1.0f)': vec2<f32>
            582..586 '1.0f': f32
            598..616 'float_..._fract': vec2<f32>
            619..631 'float_32_vec': __frexp_result_vec2_f32
            619..637 'float_....fract': vec2<f32>
            647..663 'float_...ec_exp': vec2<i32>
            666..678 'float_32_vec': __frexp_result_vec2_f32
            666..682 'float_...ec.exp': vec2<i32>
            692..704 'float_16_vec': __frexp_result_vec2_f16
            707..724 'frexp(...1.0h))': __frexp_result_vec2_f16
            713..723 'vec2(1.0h)': vec2<f16>
            718..722 '1.0h': f16
            734..752 'float_..._fract': vec2<f16>
            755..767 'float_16_vec': __frexp_result_vec2_f16
            755..773 'float_....fract': vec2<f16>
            783..799 'float_...ec_exp': vec2<i32>
            802..814 'float_16_vec': __frexp_result_vec2_f16
            802..818 'float_...ec.exp': vec2<i32>
        "#]],
    );
}

#[test]
fn insertBits() {
    check_infer(
        ExtensionsConfig::default(),
        "
fn foo() {
    let signed_integer_32 = insertBits(1i, 1i, 0u, 0u);
    let unsigned_integer_32 = insertBits(1u, 1u, 0u, 0u);

    let signed_integer_32_vec = insertBits(vec2(1i), vec2(1i), 0u, 0u);
    let unsigned_integer_32_vec = insertBits(vec2(1u), vec2(1u), 0u, 0u);
}
",
        expect![[r#"
            19..36 'signed...ger_32': i32
            39..65 'insert...u, 0u)': i32
            50..52 '1i': i32
            54..56 '1i': i32
            58..60 '0u': u32
            62..64 '0u': u32
            75..94 'unsign...ger_32': u32
            97..123 'insert...u, 0u)': u32
            108..110 '1u': u32
            112..114 '1u': u32
            116..118 '0u': u32
            120..122 '0u': u32
            134..155 'signed...32_vec': vec2<i32>
            158..196 'insert...u, 0u)': vec2<i32>
            169..177 'vec2(1i)': vec2<i32>
            174..176 '1i': i32
            179..187 'vec2(1i)': vec2<i32>
            184..186 '1i': i32
            189..191 '0u': u32
            193..195 '0u': u32
            206..229 'unsign...32_vec': vec2<u32>
            232..270 'insert...u, 0u)': vec2<u32>
            243..251 'vec2(1u)': vec2<u32>
            248..250 '1u': u32
            253..261 'vec2(1u)': vec2<u32>
            258..260 '1u': u32
            263..265 '0u': u32
            267..269 '0u': u32
        "#]],
    );
}

#[test]
fn inverseSqrt() {
    check_infer(
        ExtensionsConfig {
            f16: true,
            ..Default::default()
        },
        "
enable f16;
fn foo() {
    let abstract_float = inverseSqrt(1.0);
    let float_32 = inverseSqrt(1.0f);
    let float_16 = inverseSqrt(1.0h);

    let abstract_float_vec = inverseSqrt(vec2(1.0));
    let float_32_vec = inverseSqrt(vec2(1.0f));
    let float_16_vec = inverseSqrt(vec2(1.0h));
}
",
        expect![[r#"
            31..45 'abstract_float': f32
            48..64 'invers...t(1.0)': float
            60..63 '1.0': float
            74..82 'float_32': f32
            85..102 'invers...(1.0f)': f32
            97..101 '1.0f': f32
            112..120 'float_16': f16
            123..140 'invers...(1.0h)': f16
            135..139 '1.0h': f16
            151..169 'abstra...at_vec': vec2<f32>
            172..194 'invers...(1.0))': vec2<float>
            184..193 'vec2(1.0)': vec2<float>
            189..192 '1.0': float
            204..216 'float_32_vec': vec2<f32>
            219..242 'invers...1.0f))': vec2<f32>
            231..241 'vec2(1.0f)': vec2<f32>
            236..240 '1.0f': f32
            252..264 'float_16_vec': vec2<f16>
            267..290 'invers...1.0h))': vec2<f16>
            279..289 'vec2(1.0h)': vec2<f16>
            284..288 '1.0h': f16
        "#]],
    );
}

#[test]
fn ldexp() {
    check_infer(
        ExtensionsConfig {
            f16: true,
            ..Default::default()
        },
        "
enable f16;
fn foo() {
    let abstract_float_abstract_integer = ldexp(1.0, 1);
    let float_32 = ldexp(1.0f, 1i);
    let float_16 = ldexp(1.0h, 1i);

    let abstract_float_abstract_integer_vec = ldexp(vec2(1.0), vec2(1));
    let float_32_vec = ldexp(vec2(1.0f), vec2(1i));
    let float_16_vec = ldexp(vec2(1.0h), vec2(1i));

    let automatic_concrete = ldexp(1.0, 1i);
    let automatic_concrete_vec = ldexp(vec2(1.0), vec2(1i));
}
",
        expect![[r#"
            31..62 'abstra...nteger': f32
            65..78 'ldexp(1.0, 1)': float
            71..74 '1.0': float
            76..77 '1': integer
            88..96 'float_32': f32
            99..114 'ldexp(1.0f, 1i)': f32
            105..109 '1.0f': f32
            111..113 '1i': i32
            124..132 'float_16': f16
            135..150 'ldexp(1.0h, 1i)': f16
            141..145 '1.0h': f16
            147..149 '1i': i32
            161..196 'abstra...er_vec': vec2<f32>
            199..224 'ldexp(...c2(1))': vec2<float>
            205..214 'vec2(1.0)': vec2<float>
            210..213 '1.0': float
            216..223 'vec2(1)': vec2<integer>
            221..222 '1': integer
            234..246 'float_32_vec': vec2<f32>
            249..276 'ldexp(...2(1i))': vec2<f32>
            255..265 'vec2(1.0f)': vec2<f32>
            260..264 '1.0f': f32
            267..275 'vec2(1i)': vec2<i32>
            272..274 '1i': i32
            286..298 'float_16_vec': vec2<f16>
            301..328 'ldexp(...2(1i))': vec2<f16>
            307..317 'vec2(1.0h)': vec2<f16>
            312..316 '1.0h': f16
            319..327 'vec2(1i)': vec2<i32>
            324..326 '1i': i32
            339..357 'automa...ncrete': f32
            360..374 'ldexp(1.0, 1i)': f32
            366..369 '1.0': float
            371..373 '1i': i32
            384..406 'automa...te_vec': vec2<f32>
            409..435 'ldexp(...2(1i))': vec2<f32>
            415..424 'vec2(1.0)': vec2<float>
            420..423 '1.0': float
            426..434 'vec2(1i)': vec2<i32>
            431..433 '1i': i32
        "#]],
    );
}

#[test]
fn length() {
    check_infer(
        ExtensionsConfig {
            f16: true,
            ..Default::default()
        },
        "
enable f16;
fn foo() {
    let abstract_float = length(1.0);
    let float_32 = length(1.0f);
    let float_16 = length(1.0h);

    let abstract_float_vec = length(vec2(1.0));
    let float_32_vec = length(vec2(1.0f));
    let float_16_vec = length(vec2(1.0h));
}
",
        expect![[r#"
            31..45 'abstract_float': f32
            48..59 'length(1.0)': float
            55..58 '1.0': float
            69..77 'float_32': f32
            80..92 'length(1.0f)': f32
            87..91 '1.0f': f32
            102..110 'float_16': f16
            113..125 'length(1.0h)': f16
            120..124 '1.0h': f16
            136..154 'abstra...at_vec': f32
            157..174 'length...(1.0))': float
            164..173 'vec2(1.0)': vec2<float>
            169..172 '1.0': float
            184..196 'float_32_vec': f32
            199..217 'length...1.0f))': f32
            206..216 'vec2(1.0f)': vec2<f32>
            211..215 '1.0f': f32
            227..239 'float_16_vec': f16
            242..260 'length...1.0h))': f16
            249..259 'vec2(1.0h)': vec2<f16>
            254..258 '1.0h': f16
        "#]],
    );
}

#[test]
fn log() {
    check_infer(
        ExtensionsConfig {
            f16: true,
            ..Default::default()
        },
        "
enable f16;
fn foo() {
    let abstract_float = log(1.0);
    let float_32 = log(1.0f);
    let float_16 = log(1.0h);

    let abstract_float_vec = log(vec2(1.0));
    let float_32_vec = log(vec2(1.0f));
    let float_16_vec = log(vec2(1.0h));
}
",
        expect![[r#"
            31..45 'abstract_float': f32
            48..56 'log(1.0)': float
            52..55 '1.0': float
            66..74 'float_32': f32
            77..86 'log(1.0f)': f32
            81..85 '1.0f': f32
            96..104 'float_16': f16
            107..116 'log(1.0h)': f16
            111..115 '1.0h': f16
            127..145 'abstra...at_vec': vec2<f32>
            148..162 'log(vec2(1.0))': vec2<float>
            152..161 'vec2(1.0)': vec2<float>
            157..160 '1.0': float
            172..184 'float_32_vec': vec2<f32>
            187..202 'log(vec2(1.0f))': vec2<f32>
            191..201 'vec2(1.0f)': vec2<f32>
            196..200 '1.0f': f32
            212..224 'float_16_vec': vec2<f16>
            227..242 'log(vec2(1.0h))': vec2<f16>
            231..241 'vec2(1.0h)': vec2<f16>
            236..240 '1.0h': f16
        "#]],
    );
}

#[test]
fn log2() {
    check_infer(
        ExtensionsConfig {
            f16: true,
            ..Default::default()
        },
        "
enable f16;
fn foo() {
    let abstract_float = log2(1.0);
    let float_32 = log2(1.0f);
    let float_16 = log2(1.0h);

    let abstract_float_vec = log2(vec2(1.0));
    let float_32_vec = log2(vec2(1.0f));
    let float_16_vec = log2(vec2(1.0h));
}
",
        expect![[r#"
            31..45 'abstract_float': f32
            48..57 'log2(1.0)': float
            53..56 '1.0': float
            67..75 'float_32': f32
            78..88 'log2(1.0f)': f32
            83..87 '1.0f': f32
            98..106 'float_16': f16
            109..119 'log2(1.0h)': f16
            114..118 '1.0h': f16
            130..148 'abstra...at_vec': vec2<f32>
            151..166 'log2(vec2(1.0))': vec2<float>
            156..165 'vec2(1.0)': vec2<float>
            161..164 '1.0': float
            176..188 'float_32_vec': vec2<f32>
            191..207 'log2(v...1.0f))': vec2<f32>
            196..206 'vec2(1.0f)': vec2<f32>
            201..205 '1.0f': f32
            217..229 'float_16_vec': vec2<f16>
            232..248 'log2(v...1.0h))': vec2<f16>
            237..247 'vec2(1.0h)': vec2<f16>
            242..246 '1.0h': f16
        "#]],
    );
}

#[test]
fn modf() {
    check_infer(
        ExtensionsConfig {
            f16: true,
            ..Default::default()
        },
        "
enable f16;
fn foo() {
    let abstract_float = modf(1.0);
    let abstract_float_fract = abstract_float.fract;
    let abstract_float_exp = abstract_float.whole;
    let float_32 = modf(1.0f);
    let float_32_fract = float_32.fract;
    let float_32_exp = float_32.whole;
    let float_16 = modf(1.0h);
    let float_16_fract = float_16.fract;
    let float_16_exp = float_16.whole;

    let abstract_float_vec = modf(vec2(1.0));
    let abstract_float_vec_fract = abstract_float_vec.fract;
    let abstract_float_vec_exp = abstract_float_vec.whole;
    let float_32_vec = modf(vec2(1.0f));
    let float_32_vec_fract = float_32_vec.fract;
    let float_32_vec_exp = float_32_vec.whole;
    let float_16_vec = modf(vec2(1.0h));
    let float_16_vec_fract = float_16_vec.fract;
    let float_16_vec_exp = float_16_vec.whole;
}
",
        expect![[r#"
            31..45 'abstract_float': __modf_result_abstract
            48..57 'modf(1.0)': __modf_result_abstract
            53..56 '1.0': float
            67..87 'abstra..._fract': f32
            90..104 'abstract_float': __modf_result_abstract
            90..110 'abstra....fract': float
            120..138 'abstra...at_exp': f32
            141..155 'abstract_float': __modf_result_abstract
            141..161 'abstra....whole': float
            171..179 'float_32': __modf_result_f32
            182..192 'modf(1.0f)': __modf_result_f32
            187..191 '1.0f': f32
            202..216 'float_32_fract': f32
            219..227 'float_32': __modf_result_f32
            219..233 'float_32.fract': f32
            243..255 'float_32_exp': f32
            258..266 'float_32': __modf_result_f32
            258..272 'float_32.whole': f32
            282..290 'float_16': __modf_result_f16
            293..303 'modf(1.0h)': __modf_result_f16
            298..302 '1.0h': f16
            313..327 'float_16_fract': f16
            330..338 'float_16': __modf_result_f16
            330..344 'float_16.fract': f16
            354..366 'float_16_exp': f16
            369..377 'float_16': __modf_result_f16
            369..383 'float_16.whole': f16
            394..412 'abstra...at_vec': __modf_result_vec2_abstract
            415..430 'modf(vec2(1.0))': __modf_result_vec2_abstract
            420..429 'vec2(1.0)': vec2<float>
            425..428 '1.0': float
            440..464 'abstra..._fract': vec2<f32>
            467..485 'abstra...at_vec': __modf_result_vec2_abstract
            467..491 'abstra....fract': vec2<float>
            501..523 'abstra...ec_exp': vec2<f32>
            526..544 'abstra...at_vec': __modf_result_vec2_abstract
            526..550 'abstra....whole': vec2<float>
            560..572 'float_32_vec': __modf_result_vec2_f32
            575..591 'modf(v...1.0f))': __modf_result_vec2_f32
            580..590 'vec2(1.0f)': vec2<f32>
            585..589 '1.0f': f32
            601..619 'float_..._fract': vec2<f32>
            622..634 'float_32_vec': __modf_result_vec2_f32
            622..640 'float_....fract': vec2<f32>
            650..666 'float_...ec_exp': vec2<f32>
            669..681 'float_32_vec': __modf_result_vec2_f32
            669..687 'float_....whole': vec2<f32>
            697..709 'float_16_vec': __modf_result_vec2_f16
            712..728 'modf(v...1.0h))': __modf_result_vec2_f16
            717..727 'vec2(1.0h)': vec2<f16>
            722..726 '1.0h': f16
            738..756 'float_..._fract': vec2<f16>
            759..771 'float_16_vec': __modf_result_vec2_f16
            759..777 'float_....fract': vec2<f16>
            787..803 'float_...ec_exp': vec2<f16>
            806..818 'float_16_vec': __modf_result_vec2_f16
            806..824 'float_....whole': vec2<f16>
        "#]],
    );
}

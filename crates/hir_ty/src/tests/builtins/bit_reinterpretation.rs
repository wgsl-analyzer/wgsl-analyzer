#![expect(clippy::too_many_lines, reason = "snapshot test data")]

use expect_test::expect;
use syntax::ExtensionsConfig;

use crate::tests::check_infer;

#[test]
fn bitcast_16() {
    check_infer(
        ExtensionsConfig {
            f16: true,
            ..Default::default()
        },
        "
enable f16;
fn foo() {
    let f16_f16 = bitcast<f16>(f16());
}
",
        expect![[r#"
            31..38 'f16_f16': f16
            41..60 'bitcas...f16())': f16
            54..59 'f16()': f16
        "#]],
    );
}

#[test]
fn bitcast_32() {
    check_infer(
        ExtensionsConfig {
            f16: true,
            ..Default::default()
        },
        "
enable f16;
fn foo() {
    let i32_i32 = bitcast<i32>(i32());
    let u32_u32 = bitcast<u32>(u32());
    let f32_f32 = bitcast<f32>(f32());
    let i32_u32 = bitcast<i32>(u32());
    let i32_f32 = bitcast<i32>(f32());
    let u32_i32 = bitcast<u32>(i32());
    let u32_f32 = bitcast<u32>(f32());
    let f32_i32 = bitcast<f32>(i32());
    let f32_u32 = bitcast<f32>(u32());

    let i32_vec2h = bitcast<i32>(vec2h());
    let u32_vec2h = bitcast<u32>(vec2h());
    let f32_vec2h = bitcast<f32>(vec2h());

    let vec2h_i32 = bitcast<vec2h>(i32());
    let vec2h_u32 = bitcast<vec2h>(u32());
    let vec2h_f32 = bitcast<vec2h>(f32());
}
",
        expect![[r#"
            31..38 'i32_i32': i32
            41..60 'bitcas...i32())': i32
            54..59 'i32()': i32
            70..77 'u32_u32': u32
            80..99 'bitcas...u32())': u32
            93..98 'u32()': u32
            109..116 'f32_f32': f32
            119..138 'bitcas...f32())': f32
            132..137 'f32()': f32
            148..155 'i32_u32': i32
            158..177 'bitcas...u32())': i32
            171..176 'u32()': u32
            187..194 'i32_f32': i32
            197..216 'bitcas...f32())': i32
            210..215 'f32()': f32
            226..233 'u32_i32': u32
            236..255 'bitcas...i32())': u32
            249..254 'i32()': i32
            265..272 'u32_f32': u32
            275..294 'bitcas...f32())': u32
            288..293 'f32()': f32
            304..311 'f32_i32': f32
            314..333 'bitcas...i32())': f32
            327..332 'i32()': i32
            343..350 'f32_u32': f32
            353..372 'bitcas...u32())': f32
            366..371 'u32()': u32
            383..392 'i32_vec2h': i32
            395..416 'bitcas...c2h())': i32
            408..415 'vec2h()': vec2<f16>
            426..435 'u32_vec2h': u32
            438..459 'bitcas...c2h())': u32
            451..458 'vec2h()': vec2<f16>
            469..478 'f32_vec2h': f32
            481..502 'bitcas...c2h())': f32
            494..501 'vec2h()': vec2<f16>
            513..522 'vec2h_i32': vec2<f16>
            525..546 'bitcas...i32())': vec2<f16>
            540..545 'i32()': i32
            556..565 'vec2h_u32': vec2<f16>
            568..589 'bitcas...u32())': vec2<f16>
            583..588 'u32()': u32
            599..608 'vec2h_f32': vec2<f16>
            611..632 'bitcas...f32())': vec2<f16>
            626..631 'f32()': f32
        "#]],
    );
}

#[test]
fn bitcast_N() {
    check_infer(
        ExtensionsConfig {
            f16: true,
            shader_int64: true,
            ..Default::default()
        },
        "
enable f16;
fn foo() {
    let vec2i_vec2i = bitcast<vec2i>(vec2i());
    let vec3i_vec3i = bitcast<vec3i>(vec3i());
    let vec4i_vec4i = bitcast<vec4i>(vec4i());
    let vec2u_vec2u = bitcast<vec2u>(vec2u());
    let vec3u_vec3u = bitcast<vec3u>(vec3u());
    let vec4u_vec4u = bitcast<vec4u>(vec4u());
    let vec2f_vec2f = bitcast<vec2f>(vec2f());
    let vec3f_vec3f = bitcast<vec3f>(vec3f());
    let vec4f_vec4f = bitcast<vec4f>(vec4f());
    let vec2h_vec2h = bitcast<vec2h>(vec2h());
    let vec3h_vec3h = bitcast<vec3h>(vec3h());
    let vec4h_vec4h = bitcast<vec4h>(vec4h());
    let vec2u64_vec2u64 = bitcast<vec2<u64>>(vec2<u64>());
    let vec3u64_vec3u64 = bitcast<vec3<u64>>(vec3<u64>());
    let vec4u64_vec4u64 = bitcast<vec4<u64>>(vec4<u64>());
    let vec2i64_vec2i64 = bitcast<vec2<i64>>(vec2<i64>());
    let vec3i64_vec3i64 = bitcast<vec3<i64>>(vec3<i64>());
    let vec4i64_vec4i64 = bitcast<vec4<i64>>(vec4<i64>());
    let vec2i_vec2u = bitcast<vec2i>(vec2u());
    let vec3i_vec3u = bitcast<vec3i>(vec3u());
    let vec4i_vec4u = bitcast<vec4i>(vec4u());
    let vec2i_vec2f = bitcast<vec2i>(vec2f());
    let vec3i_vec3f = bitcast<vec3i>(vec3f());
    let vec4i_vec4f = bitcast<vec4i>(vec4f());
    let vec2u_vec2i = bitcast<vec2u>(vec2i());
    let vec3u_vec3i = bitcast<vec3u>(vec3i());
    let vec4u_vec4i = bitcast<vec4u>(vec4i());
    let vec2u_vec2f = bitcast<vec2u>(vec2f());
    let vec3u_vec3f = bitcast<vec3u>(vec3f());
    let vec4u_vec4f = bitcast<vec4u>(vec4f());
    let vec2f_vec2i = bitcast<vec2f>(vec2i());
    let vec3f_vec3i = bitcast<vec3f>(vec3i());
    let vec4f_vec4i = bitcast<vec4f>(vec4i());
    let vec2f_vec2u = bitcast<vec2f>(vec2u());
    let vec3f_vec3u = bitcast<vec3f>(vec3u());
    let vec4f_vec4u = bitcast<vec4f>(vec4u());
    let vec2i64_vec2u64 = bitcast<vec2<i64>>(vec2<u64>());
    let vec3i64_vec3u64 = bitcast<vec3<i64>>(vec3<u64>());
    let vec4i64_vec4u64 = bitcast<vec4<i64>>(vec4<u64>());
    let vec2u64_vec2i64 = bitcast<vec2<u64>>(vec2<i64>());
    let vec3u64_vec3i64 = bitcast<vec3<u64>>(vec3<i64>());
    let vec4u64_vec4i64 = bitcast<vec4<u64>>(vec4<i64>());
}
",
        expect![[r#"
            31..42 'vec2i_vec2i': vec2<i32>
            45..68 'bitcas...c2i())': vec2<i32>
            60..67 'vec2i()': vec2<i32>
            78..89 'vec3i_vec3i': vec3<i32>
            92..115 'bitcas...c3i())': vec3<i32>
            107..114 'vec3i()': vec3<i32>
            125..136 'vec4i_vec4i': vec4<i32>
            139..162 'bitcas...c4i())': vec4<i32>
            154..161 'vec4i()': vec4<i32>
            172..183 'vec2u_vec2u': vec2<u32>
            186..209 'bitcas...c2u())': vec2<u32>
            201..208 'vec2u()': vec2<u32>
            219..230 'vec3u_vec3u': vec3<u32>
            233..256 'bitcas...c3u())': vec3<u32>
            248..255 'vec3u()': vec3<u32>
            266..277 'vec4u_vec4u': vec4<u32>
            280..303 'bitcas...c4u())': vec4<u32>
            295..302 'vec4u()': vec4<u32>
            313..324 'vec2f_vec2f': vec2<f32>
            327..350 'bitcas...c2f())': vec2<f32>
            342..349 'vec2f()': vec2<f32>
            360..371 'vec3f_vec3f': vec3<f32>
            374..397 'bitcas...c3f())': vec3<f32>
            389..396 'vec3f()': vec3<f32>
            407..418 'vec4f_vec4f': vec4<f32>
            421..444 'bitcas...c4f())': vec4<f32>
            436..443 'vec4f()': vec4<f32>
            454..465 'vec2h_vec2h': vec2<f16>
            468..491 'bitcas...c2h())': vec2<f16>
            483..490 'vec2h()': vec2<f16>
            501..512 'vec3h_vec3h': vec3<f16>
            515..538 'bitcas...c3h())': vec3<f16>
            530..537 'vec3h()': vec3<f16>
            548..559 'vec4h_vec4h': vec4<f16>
            562..585 'bitcas...c4h())': vec4<f16>
            577..584 'vec4h()': vec4<f16>
            595..610 'vec2u64_vec2u64': vec2<u64>
            613..644 'bitcas...64>())': vec2<u64>
            632..643 'vec2<u64>()': vec2<u64>
            654..669 'vec3u64_vec3u64': vec3<u64>
            672..703 'bitcas...64>())': vec3<u64>
            691..702 'vec3<u64>()': vec3<u64>
            713..728 'vec4u64_vec4u64': vec4<u64>
            731..762 'bitcas...64>())': vec4<u64>
            750..761 'vec4<u64>()': vec4<u64>
            772..787 'vec2i64_vec2i64': vec2<i64>
            790..821 'bitcas...64>())': vec2<i64>
            809..820 'vec2<i64>()': vec2<i64>
            831..846 'vec3i64_vec3i64': vec3<i64>
            849..880 'bitcas...64>())': vec3<i64>
            868..879 'vec3<i64>()': vec3<i64>
            890..905 'vec4i64_vec4i64': vec4<i64>
            908..939 'bitcas...64>())': vec4<i64>
            927..938 'vec4<i64>()': vec4<i64>
            949..960 'vec2i_vec2u': vec2<i32>
            963..986 'bitcas...c2u())': vec2<i32>
            978..985 'vec2u()': vec2<u32>
            996..1007 'vec3i_vec3u': vec3<i32>
            1010..1033 'bitcas...c3u())': vec3<i32>
            1025..1032 'vec3u()': vec3<u32>
            1043..1054 'vec4i_vec4u': vec4<i32>
            1057..1080 'bitcas...c4u())': vec4<i32>
            1072..1079 'vec4u()': vec4<u32>
            1090..1101 'vec2i_vec2f': vec2<i32>
            1104..1127 'bitcas...c2f())': vec2<i32>
            1119..1126 'vec2f()': vec2<f32>
            1137..1148 'vec3i_vec3f': vec3<i32>
            1151..1174 'bitcas...c3f())': vec3<i32>
            1166..1173 'vec3f()': vec3<f32>
            1184..1195 'vec4i_vec4f': vec4<i32>
            1198..1221 'bitcas...c4f())': vec4<i32>
            1213..1220 'vec4f()': vec4<f32>
            1231..1242 'vec2u_vec2i': vec2<u32>
            1245..1268 'bitcas...c2i())': vec2<u32>
            1260..1267 'vec2i()': vec2<i32>
            1278..1289 'vec3u_vec3i': vec3<u32>
            1292..1315 'bitcas...c3i())': vec3<u32>
            1307..1314 'vec3i()': vec3<i32>
            1325..1336 'vec4u_vec4i': vec4<u32>
            1339..1362 'bitcas...c4i())': vec4<u32>
            1354..1361 'vec4i()': vec4<i32>
            1372..1383 'vec2u_vec2f': vec2<u32>
            1386..1409 'bitcas...c2f())': vec2<u32>
            1401..1408 'vec2f()': vec2<f32>
            1419..1430 'vec3u_vec3f': vec3<u32>
            1433..1456 'bitcas...c3f())': vec3<u32>
            1448..1455 'vec3f()': vec3<f32>
            1466..1477 'vec4u_vec4f': vec4<u32>
            1480..1503 'bitcas...c4f())': vec4<u32>
            1495..1502 'vec4f()': vec4<f32>
            1513..1524 'vec2f_vec2i': vec2<f32>
            1527..1550 'bitcas...c2i())': vec2<f32>
            1542..1549 'vec2i()': vec2<i32>
            1560..1571 'vec3f_vec3i': vec3<f32>
            1574..1597 'bitcas...c3i())': vec3<f32>
            1589..1596 'vec3i()': vec3<i32>
            1607..1618 'vec4f_vec4i': vec4<f32>
            1621..1644 'bitcas...c4i())': vec4<f32>
            1636..1643 'vec4i()': vec4<i32>
            1654..1665 'vec2f_vec2u': vec2<f32>
            1668..1691 'bitcas...c2u())': vec2<f32>
            1683..1690 'vec2u()': vec2<u32>
            1701..1712 'vec3f_vec3u': vec3<f32>
            1715..1738 'bitcas...c3u())': vec3<f32>
            1730..1737 'vec3u()': vec3<u32>
            1748..1759 'vec4f_vec4u': vec4<f32>
            1762..1785 'bitcas...c4u())': vec4<f32>
            1777..1784 'vec4u()': vec4<u32>
            1795..1810 'vec2i64_vec2u64': vec2<i64>
            1813..1844 'bitcas...64>())': vec2<i64>
            1832..1843 'vec2<u64>()': vec2<u64>
            1854..1869 'vec3i64_vec3u64': vec3<i64>
            1872..1903 'bitcas...64>())': vec3<i64>
            1891..1902 'vec3<u64>()': vec3<u64>
            1913..1928 'vec4i64_vec4u64': vec4<i64>
            1931..1962 'bitcas...64>())': vec4<i64>
            1950..1961 'vec4<u64>()': vec4<u64>
            1972..1987 'vec2u64_vec2i64': vec2<u64>
            1990..2021 'bitcas...64>())': vec2<u64>
            2009..2020 'vec2<i64>()': vec2<i64>
            2031..2046 'vec3u64_vec3i64': vec3<u64>
            2049..2080 'bitcas...64>())': vec3<u64>
            2068..2079 'vec3<i64>()': vec3<i64>
            2090..2105 'vec4u64_vec4i64': vec4<u64>
            2108..2139 'bitcas...64>())': vec4<u64>
            2127..2138 'vec4<i64>()': vec4<i64>
        "#]],
    );
}

#[test]
fn bitcast_64() {
    check_infer(
        ExtensionsConfig {
            f16: true,
            shader_int64: true,
            ..Default::default()
        },
        "
enable f16;
fn foo() {
    let u64_u64 = bitcast<u64>(u64());
    let i64_i64 = bitcast<i64>(i64());
    let i64_u64 = bitcast<i64>(u64());
    let u64_i64 = bitcast<u64>(i64());

    let vec2i_i64 = bitcast<vec2i>(i64());
    let vec2u_i64 = bitcast<vec2u>(i64());
    let vec2f_i64 = bitcast<vec2f>(i64());
    let vec2i_u64 = bitcast<vec2i>(u64());
    let vec2u_u64 = bitcast<vec2u>(u64());
    let vec2f_u64 = bitcast<vec2f>(u64());

    let i64_vec2i = bitcast<i64>(vec2i());
    let i64_vec2u = bitcast<i64>(vec2u());
    let i64_vec2f = bitcast<i64>(vec2f());
    let i64_vec4h = bitcast<i64>(vec4h());
    let u64_vec2i = bitcast<u64>(vec2i());
    let u64_vec2u = bitcast<u64>(vec2u());
    let u64_vec2f = bitcast<u64>(vec2f());
    let u64_vec4h = bitcast<u64>(vec4h());

    let vec2i_vec4h = bitcast<vec2i>(vec4h());
    let vec2u_vec4h = bitcast<vec2u>(vec4h());
    let vec2f_vec4h = bitcast<vec2f>(vec4h());
    let vec4h_vec2i = bitcast<vec4h>(vec2i());
    let vec4h_vec2u = bitcast<vec4h>(vec2u());
    let vec4h_vec2f = bitcast<vec4h>(vec2f());

    let vec4h_i64 = bitcast<vec4h>(i64());
    let vec4h_u64 = bitcast<vec4h>(u64());
}
",
        expect![[r#"
            31..38 'u64_u64': u64
            41..60 'bitcas...u64())': u64
            54..59 'u64()': u64
            70..77 'i64_i64': i64
            80..99 'bitcas...i64())': i64
            93..98 'i64()': i64
            109..116 'i64_u64': i64
            119..138 'bitcas...u64())': i64
            132..137 'u64()': u64
            148..155 'u64_i64': u64
            158..177 'bitcas...i64())': u64
            171..176 'i64()': i64
            188..197 'vec2i_i64': vec2<i32>
            200..221 'bitcas...i64())': vec2<i32>
            215..220 'i64()': i64
            231..240 'vec2u_i64': vec2<u32>
            243..264 'bitcas...i64())': vec2<u32>
            258..263 'i64()': i64
            274..283 'vec2f_i64': vec2<f32>
            286..307 'bitcas...i64())': vec2<f32>
            301..306 'i64()': i64
            317..326 'vec2i_u64': vec2<i32>
            329..350 'bitcas...u64())': vec2<i32>
            344..349 'u64()': u64
            360..369 'vec2u_u64': vec2<u32>
            372..393 'bitcas...u64())': vec2<u32>
            387..392 'u64()': u64
            403..412 'vec2f_u64': vec2<f32>
            415..436 'bitcas...u64())': vec2<f32>
            430..435 'u64()': u64
            447..456 'i64_vec2i': i64
            459..480 'bitcas...c2i())': i64
            472..479 'vec2i()': vec2<i32>
            490..499 'i64_vec2u': i64
            502..523 'bitcas...c2u())': i64
            515..522 'vec2u()': vec2<u32>
            533..542 'i64_vec2f': i64
            545..566 'bitcas...c2f())': i64
            558..565 'vec2f()': vec2<f32>
            576..585 'i64_vec4h': i64
            588..609 'bitcas...c4h())': i64
            601..608 'vec4h()': vec4<f16>
            619..628 'u64_vec2i': u64
            631..652 'bitcas...c2i())': u64
            644..651 'vec2i()': vec2<i32>
            662..671 'u64_vec2u': u64
            674..695 'bitcas...c2u())': u64
            687..694 'vec2u()': vec2<u32>
            705..714 'u64_vec2f': u64
            717..738 'bitcas...c2f())': u64
            730..737 'vec2f()': vec2<f32>
            748..757 'u64_vec4h': u64
            760..781 'bitcas...c4h())': u64
            773..780 'vec4h()': vec4<f16>
            792..803 'vec2i_vec4h': vec2<i32>
            806..829 'bitcas...c4h())': vec2<i32>
            821..828 'vec4h()': vec4<f16>
            839..850 'vec2u_vec4h': vec2<u32>
            853..876 'bitcas...c4h())': vec2<u32>
            868..875 'vec4h()': vec4<f16>
            886..897 'vec2f_vec4h': vec2<f32>
            900..923 'bitcas...c4h())': vec2<f32>
            915..922 'vec4h()': vec4<f16>
            933..944 'vec4h_vec2i': vec4<f16>
            947..970 'bitcas...c2i())': vec4<f16>
            962..969 'vec2i()': vec2<i32>
            980..991 'vec4h_vec2u': vec4<f16>
            994..1017 'bitcas...c2u())': vec4<f16>
            1009..1016 'vec2u()': vec2<u32>
            1027..1038 'vec4h_vec2f': vec4<f16>
            1041..1064 'bitcas...c2f())': vec4<f16>
            1056..1063 'vec2f()': vec2<f32>
            1075..1084 'vec4h_i64': vec4<f16>
            1087..1108 'bitcas...i64())': vec4<f16>
            1102..1107 'i64()': i64
            1118..1127 'vec4h_u64': vec4<f16>
            1130..1151 'bitcas...u64())': vec4<f16>
            1145..1150 'u64()': u64
        "#]],
    );
}

#[test]
fn bitcast_concretization() {
    check_infer(
        ExtensionsConfig {
            shader_int64: true,
            ..Default::default()
        },
        "
fn foo() {
    let u32_abstract_int = bitcast<u32>(1);
    let vec2u_vec2_abstract_int = bitcast<vec2u>(vec2(1));
    let vec3u_vec3_abstract_int = bitcast<vec3u>(vec3(1));
    let vec4u_vec4_abstract_int = bitcast<vec4u>(vec4(1));

    // let u64_abstract_int = bitcast<u64>(1);
    // let vec2u64_vec2_abstract_int = bitcast<vec2<u64>>(vec2(1));
    // let vec3u64_vec3_abstract_int = bitcast<vec3<u64>>(vec3(1));
    // let vec4u64_vec4_abstract_int = bitcast<vec4<u64>>(vec4(1));
}
",
        expect![[r#"
            19..35 'u32_ab...ct_int': u32
            38..53 'bitcast<u32>(1)': u32
            51..52 '1': integer
            63..86 'vec2u_...ct_int': vec2<u32>
            89..112 'bitcas...c2(1))': vec2<u32>
            104..111 'vec2(1)': vec2<integer>
            109..110 '1': integer
            122..145 'vec3u_...ct_int': vec3<u32>
            148..171 'bitcas...c3(1))': vec3<u32>
            163..170 'vec3(1)': vec3<integer>
            168..169 '1': integer
            181..204 'vec4u_...ct_int': vec4<u32>
            207..230 'bitcas...c4(1))': vec4<u32>
            222..229 'vec4(1)': vec4<integer>
            227..228 '1': integer
        "#]],
    );
}

#[test]
fn bitcast_128() {
    check_infer(
        ExtensionsConfig {
            shader_int64: true,
            ..Default::default()
        },
        "
fn foo() {
    let vec2i64_vec4i = bitcast<vec2<i64>>(vec4i());
    let vec2i64_vec4u = bitcast<vec2<i64>>(vec4u());
    let vec2i64_vec4f = bitcast<vec2<i64>>(vec4f());
    let vec2u64_vec4i = bitcast<vec2<u64>>(vec4i());
    let vec2u64_vec4u = bitcast<vec2<u64>>(vec4u());
    let vec2u64_vec4f = bitcast<vec2<u64>>(vec4f());

    let vec4i_vec2i64 = bitcast<vec4i>(vec2<i64>());
    let vec4u_vec2i64 = bitcast<vec4u>(vec2<i64>());
    let vec4f_vec2i64 = bitcast<vec4f>(vec2<i64>());
    let vec4i_vec2u64 = bitcast<vec4i>(vec2<u64>());
    let vec4u_vec2u64 = bitcast<vec4u>(vec2<u64>());
    let vec4f_vec2u64 = bitcast<vec4f>(vec2<u64>());
}
",
        expect![[r#"
            19..32 'vec2i64_vec4i': vec2<i64>
            35..62 'bitcas...c4i())': vec2<i64>
            54..61 'vec4i()': vec4<i32>
            72..85 'vec2i64_vec4u': vec2<i64>
            88..115 'bitcas...c4u())': vec2<i64>
            107..114 'vec4u()': vec4<u32>
            125..138 'vec2i64_vec4f': vec2<i64>
            141..168 'bitcas...c4f())': vec2<i64>
            160..167 'vec4f()': vec4<f32>
            178..191 'vec2u64_vec4i': vec2<u64>
            194..221 'bitcas...c4i())': vec2<u64>
            213..220 'vec4i()': vec4<i32>
            231..244 'vec2u64_vec4u': vec2<u64>
            247..274 'bitcas...c4u())': vec2<u64>
            266..273 'vec4u()': vec4<u32>
            284..297 'vec2u64_vec4f': vec2<u64>
            300..327 'bitcas...c4f())': vec2<u64>
            319..326 'vec4f()': vec4<f32>
            338..351 'vec4i_vec2i64': vec4<i32>
            354..381 'bitcas...64>())': vec4<i32>
            369..380 'vec2<i64>()': vec2<i64>
            391..404 'vec4u_vec2i64': vec4<u32>
            407..434 'bitcas...64>())': vec4<u32>
            422..433 'vec2<i64>()': vec2<i64>
            444..457 'vec4f_vec2i64': vec4<f32>
            460..487 'bitcas...64>())': vec4<f32>
            475..486 'vec2<i64>()': vec2<i64>
            497..510 'vec4i_vec2u64': vec4<i32>
            513..540 'bitcas...64>())': vec4<i32>
            528..539 'vec2<u64>()': vec2<u64>
            550..563 'vec4u_vec2u64': vec4<u32>
            566..593 'bitcas...64>())': vec4<u32>
            581..592 'vec2<u64>()': vec2<u64>
            603..616 'vec4f_vec2u64': vec4<f32>
            619..646 'bitcas...64>())': vec4<f32>
            634..645 'vec2<u64>()': vec2<u64>
        "#]],
    );
}

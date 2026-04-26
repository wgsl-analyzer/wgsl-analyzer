use expect_test::expect;

use crate::tests::check;

#[test]
fn reserved_words_do_parse() {
    check(
        "
        var NULL = 0;
        var Self = 0;
        var abstract = 0;
        var active = 0;
        var alignas = 0;
        var alignof = 0;
        // WESL keyword
        // var as = 0;
        var asm = 0;
        var asm_fragment = 0;
        var async = 0;
        var attribute = 0;
        var auto = 0;
        var await = 0;
        var become = 0;
        var cast = 0;
        var catch = 0;
        var class = 0;
        var co_await = 0;
        var co_return = 0;
        var co_yield = 0;
        var coherent = 0;
        var column_major = 0;
        var common = 0;
        var compile = 0;
        var compile_fragment = 0;
        var concept = 0;
        var const_cast = 0;
        var consteval = 0;
        var constexpr = 0;
        var constinit = 0;
        var crate = 0;
        var debugger = 0;
        var decltype = 0;
        var delete = 0;
        var demote = 0;
        var demote_to_helper = 0;
        var do = 0;
        var dynamic_cast = 0;
        var enum = 0;
        var explicit = 0;
        var export = 0;
        var extends = 0;
        var extern = 0;
        var external = 0;
        var fallthrough = 0;
        var filter = 0;
        var final = 0;
        var finally = 0;
        var friend = 0;
        var from = 0;
        var fxgroup = 0;
        var get = 0;
        var goto = 0;
        var groupshared = 0;
        var highp = 0;
        var impl = 0;
        var implements = 0;
        // WESL keyword
        // var import = 0;
        var inline = 0;
        var instanceof = 0;
        var interface = 0;
        var layout = 0;
        var lowp = 0;
        var macro = 0;
        var macro_rules = 0;
        var match = 0;
        var mediump = 0;
        var meta = 0;
        var mod = 0;
        var module = 0;
        var move = 0;
        var mut = 0;
        var mutable = 0;
        var namespace = 0;
        var new = 0;
        var nil = 0;
        var noexcept = 0;
        var noinline = 0;
        var nointerpolation = 0;
        var non_coherent = 0;
        var noncoherent = 0;
        var noperspective = 0;
        var null = 0;
        var nullptr = 0;
        var of = 0;
        var operator = 0;
        // WESL keyword
        // var package = 0;
        var packoffset = 0;
        var partition = 0;
        var pass = 0;
        var patch = 0;
        var pixelfragment = 0;
        var precise = 0;
        var precision = 0;
        var premerge = 0;
        var priv = 0;
        var protected = 0;
        var pub = 0;
        var public = 0;
        var readonly = 0;
        var ref = 0;
        var regardless = 0;
        var register = 0;
        var reinterpret_cast = 0;
        var require = 0;
        var resource = 0;
        var restrict = 0;
        var self = 0;
        var set = 0;
        var shared = 0;
        var sizeof = 0;
        var smooth = 0;
        var snorm = 0;
        var static = 0;
        var static_assert = 0;
        var static_cast = 0;
        var std = 0;
        var subroutine = 0;
        // WESL keyword
        // var super = 0;
        var target = 0;
        var template = 0;
        var this = 0;
        var thread_local = 0;
        var throw = 0;
        var trait = 0;
        var try = 0;
        var type = 0;
        var typedef = 0;
        var typeid = 0;
        var typename = 0;
        var typeof = 0;
        var union = 0;
        var unless = 0;
        var unorm = 0;
        var unsafe = 0;
        var unsized = 0;
        var use = 0;
        var using = 0;
        var varying = 0;
        var virtual = 0;
        var volatile = 0;
        var wgsl = 0;
        var where = 0;
        var with = 0;
        var writeonly = 0;
        var yield = 0;
        ",
        expect![[r#"
            SourceFile@0..3743
              Blankspace@0..9 "\n        "
              VariableDeclaration@9..22
                Var@9..12 "var"
                Blankspace@12..13 " "
                Name@13..17
                  Identifier@13..17 "NULL"
                Blankspace@17..18 " "
                Equal@18..19 "="
                Blankspace@19..20 " "
                Literal@20..21
                  IntLiteral@20..21 "0"
                Semicolon@21..22 ";"
              Blankspace@22..31 "\n        "
              VariableDeclaration@31..44
                Var@31..34 "var"
                Blankspace@34..35 " "
                Name@35..39
                  Identifier@35..39 "Self"
                Blankspace@39..40 " "
                Equal@40..41 "="
                Blankspace@41..42 " "
                Literal@42..43
                  IntLiteral@42..43 "0"
                Semicolon@43..44 ";"
              Blankspace@44..53 "\n        "
              VariableDeclaration@53..70
                Var@53..56 "var"
                Blankspace@56..57 " "
                Name@57..65
                  Identifier@57..65 "abstract"
                Blankspace@65..66 " "
                Equal@66..67 "="
                Blankspace@67..68 " "
                Literal@68..69
                  IntLiteral@68..69 "0"
                Semicolon@69..70 ";"
              Blankspace@70..79 "\n        "
              VariableDeclaration@79..94
                Var@79..82 "var"
                Blankspace@82..83 " "
                Name@83..89
                  Identifier@83..89 "active"
                Blankspace@89..90 " "
                Equal@90..91 "="
                Blankspace@91..92 " "
                Literal@92..93
                  IntLiteral@92..93 "0"
                Semicolon@93..94 ";"
              Blankspace@94..103 "\n        "
              VariableDeclaration@103..119
                Var@103..106 "var"
                Blankspace@106..107 " "
                Name@107..114
                  Identifier@107..114 "alignas"
                Blankspace@114..115 " "
                Equal@115..116 "="
                Blankspace@116..117 " "
                Literal@117..118
                  IntLiteral@117..118 "0"
                Semicolon@118..119 ";"
              Blankspace@119..128 "\n        "
              VariableDeclaration@128..144
                Var@128..131 "var"
                Blankspace@131..132 " "
                Name@132..139
                  Identifier@132..139 "alignof"
                Blankspace@139..140 " "
                Equal@140..141 "="
                Blankspace@141..142 " "
                Literal@142..143
                  IntLiteral@142..143 "0"
                Semicolon@143..144 ";"
              Blankspace@144..153 "\n        "
              LineEndingComment@153..168 "// WESL keyword"
              Blankspace@168..177 "\n        "
              LineEndingComment@177..191 "// var as = 0;"
              Blankspace@191..200 "\n        "
              VariableDeclaration@200..212
                Var@200..203 "var"
                Blankspace@203..204 " "
                Name@204..207
                  Identifier@204..207 "asm"
                Blankspace@207..208 " "
                Equal@208..209 "="
                Blankspace@209..210 " "
                Literal@210..211
                  IntLiteral@210..211 "0"
                Semicolon@211..212 ";"
              Blankspace@212..221 "\n        "
              VariableDeclaration@221..242
                Var@221..224 "var"
                Blankspace@224..225 " "
                Name@225..237
                  Identifier@225..237 "asm_fragment"
                Blankspace@237..238 " "
                Equal@238..239 "="
                Blankspace@239..240 " "
                Literal@240..241
                  IntLiteral@240..241 "0"
                Semicolon@241..242 ";"
              Blankspace@242..251 "\n        "
              VariableDeclaration@251..265
                Var@251..254 "var"
                Blankspace@254..255 " "
                Name@255..260
                  Identifier@255..260 "async"
                Blankspace@260..261 " "
                Equal@261..262 "="
                Blankspace@262..263 " "
                Literal@263..264
                  IntLiteral@263..264 "0"
                Semicolon@264..265 ";"
              Blankspace@265..274 "\n        "
              VariableDeclaration@274..292
                Var@274..277 "var"
                Blankspace@277..278 " "
                Name@278..287
                  Identifier@278..287 "attribute"
                Blankspace@287..288 " "
                Equal@288..289 "="
                Blankspace@289..290 " "
                Literal@290..291
                  IntLiteral@290..291 "0"
                Semicolon@291..292 ";"
              Blankspace@292..301 "\n        "
              VariableDeclaration@301..314
                Var@301..304 "var"
                Blankspace@304..305 " "
                Name@305..309
                  Identifier@305..309 "auto"
                Blankspace@309..310 " "
                Equal@310..311 "="
                Blankspace@311..312 " "
                Literal@312..313
                  IntLiteral@312..313 "0"
                Semicolon@313..314 ";"
              Blankspace@314..323 "\n        "
              VariableDeclaration@323..337
                Var@323..326 "var"
                Blankspace@326..327 " "
                Name@327..332
                  Identifier@327..332 "await"
                Blankspace@332..333 " "
                Equal@333..334 "="
                Blankspace@334..335 " "
                Literal@335..336
                  IntLiteral@335..336 "0"
                Semicolon@336..337 ";"
              Blankspace@337..346 "\n        "
              VariableDeclaration@346..361
                Var@346..349 "var"
                Blankspace@349..350 " "
                Name@350..356
                  Identifier@350..356 "become"
                Blankspace@356..357 " "
                Equal@357..358 "="
                Blankspace@358..359 " "
                Literal@359..360
                  IntLiteral@359..360 "0"
                Semicolon@360..361 ";"
              Blankspace@361..370 "\n        "
              VariableDeclaration@370..383
                Var@370..373 "var"
                Blankspace@373..374 " "
                Name@374..378
                  Identifier@374..378 "cast"
                Blankspace@378..379 " "
                Equal@379..380 "="
                Blankspace@380..381 " "
                Literal@381..382
                  IntLiteral@381..382 "0"
                Semicolon@382..383 ";"
              Blankspace@383..392 "\n        "
              VariableDeclaration@392..406
                Var@392..395 "var"
                Blankspace@395..396 " "
                Name@396..401
                  Identifier@396..401 "catch"
                Blankspace@401..402 " "
                Equal@402..403 "="
                Blankspace@403..404 " "
                Literal@404..405
                  IntLiteral@404..405 "0"
                Semicolon@405..406 ";"
              Blankspace@406..415 "\n        "
              VariableDeclaration@415..429
                Var@415..418 "var"
                Blankspace@418..419 " "
                Name@419..424
                  Identifier@419..424 "class"
                Blankspace@424..425 " "
                Equal@425..426 "="
                Blankspace@426..427 " "
                Literal@427..428
                  IntLiteral@427..428 "0"
                Semicolon@428..429 ";"
              Blankspace@429..438 "\n        "
              VariableDeclaration@438..455
                Var@438..441 "var"
                Blankspace@441..442 " "
                Name@442..450
                  Identifier@442..450 "co_await"
                Blankspace@450..451 " "
                Equal@451..452 "="
                Blankspace@452..453 " "
                Literal@453..454
                  IntLiteral@453..454 "0"
                Semicolon@454..455 ";"
              Blankspace@455..464 "\n        "
              VariableDeclaration@464..482
                Var@464..467 "var"
                Blankspace@467..468 " "
                Name@468..477
                  Identifier@468..477 "co_return"
                Blankspace@477..478 " "
                Equal@478..479 "="
                Blankspace@479..480 " "
                Literal@480..481
                  IntLiteral@480..481 "0"
                Semicolon@481..482 ";"
              Blankspace@482..491 "\n        "
              VariableDeclaration@491..508
                Var@491..494 "var"
                Blankspace@494..495 " "
                Name@495..503
                  Identifier@495..503 "co_yield"
                Blankspace@503..504 " "
                Equal@504..505 "="
                Blankspace@505..506 " "
                Literal@506..507
                  IntLiteral@506..507 "0"
                Semicolon@507..508 ";"
              Blankspace@508..517 "\n        "
              VariableDeclaration@517..534
                Var@517..520 "var"
                Blankspace@520..521 " "
                Name@521..529
                  Identifier@521..529 "coherent"
                Blankspace@529..530 " "
                Equal@530..531 "="
                Blankspace@531..532 " "
                Literal@532..533
                  IntLiteral@532..533 "0"
                Semicolon@533..534 ";"
              Blankspace@534..543 "\n        "
              VariableDeclaration@543..564
                Var@543..546 "var"
                Blankspace@546..547 " "
                Name@547..559
                  Identifier@547..559 "column_major"
                Blankspace@559..560 " "
                Equal@560..561 "="
                Blankspace@561..562 " "
                Literal@562..563
                  IntLiteral@562..563 "0"
                Semicolon@563..564 ";"
              Blankspace@564..573 "\n        "
              VariableDeclaration@573..588
                Var@573..576 "var"
                Blankspace@576..577 " "
                Name@577..583
                  Identifier@577..583 "common"
                Blankspace@583..584 " "
                Equal@584..585 "="
                Blankspace@585..586 " "
                Literal@586..587
                  IntLiteral@586..587 "0"
                Semicolon@587..588 ";"
              Blankspace@588..597 "\n        "
              VariableDeclaration@597..613
                Var@597..600 "var"
                Blankspace@600..601 " "
                Name@601..608
                  Identifier@601..608 "compile"
                Blankspace@608..609 " "
                Equal@609..610 "="
                Blankspace@610..611 " "
                Literal@611..612
                  IntLiteral@611..612 "0"
                Semicolon@612..613 ";"
              Blankspace@613..622 "\n        "
              VariableDeclaration@622..647
                Var@622..625 "var"
                Blankspace@625..626 " "
                Name@626..642
                  Identifier@626..642 "compile_fragment"
                Blankspace@642..643 " "
                Equal@643..644 "="
                Blankspace@644..645 " "
                Literal@645..646
                  IntLiteral@645..646 "0"
                Semicolon@646..647 ";"
              Blankspace@647..656 "\n        "
              VariableDeclaration@656..672
                Var@656..659 "var"
                Blankspace@659..660 " "
                Name@660..667
                  Identifier@660..667 "concept"
                Blankspace@667..668 " "
                Equal@668..669 "="
                Blankspace@669..670 " "
                Literal@670..671
                  IntLiteral@670..671 "0"
                Semicolon@671..672 ";"
              Blankspace@672..681 "\n        "
              VariableDeclaration@681..700
                Var@681..684 "var"
                Blankspace@684..685 " "
                Name@685..695
                  Identifier@685..695 "const_cast"
                Blankspace@695..696 " "
                Equal@696..697 "="
                Blankspace@697..698 " "
                Literal@698..699
                  IntLiteral@698..699 "0"
                Semicolon@699..700 ";"
              Blankspace@700..709 "\n        "
              VariableDeclaration@709..727
                Var@709..712 "var"
                Blankspace@712..713 " "
                Name@713..722
                  Identifier@713..722 "consteval"
                Blankspace@722..723 " "
                Equal@723..724 "="
                Blankspace@724..725 " "
                Literal@725..726
                  IntLiteral@725..726 "0"
                Semicolon@726..727 ";"
              Blankspace@727..736 "\n        "
              VariableDeclaration@736..754
                Var@736..739 "var"
                Blankspace@739..740 " "
                Name@740..749
                  Identifier@740..749 "constexpr"
                Blankspace@749..750 " "
                Equal@750..751 "="
                Blankspace@751..752 " "
                Literal@752..753
                  IntLiteral@752..753 "0"
                Semicolon@753..754 ";"
              Blankspace@754..763 "\n        "
              VariableDeclaration@763..781
                Var@763..766 "var"
                Blankspace@766..767 " "
                Name@767..776
                  Identifier@767..776 "constinit"
                Blankspace@776..777 " "
                Equal@777..778 "="
                Blankspace@778..779 " "
                Literal@779..780
                  IntLiteral@779..780 "0"
                Semicolon@780..781 ";"
              Blankspace@781..790 "\n        "
              VariableDeclaration@790..804
                Var@790..793 "var"
                Blankspace@793..794 " "
                Name@794..799
                  Identifier@794..799 "crate"
                Blankspace@799..800 " "
                Equal@800..801 "="
                Blankspace@801..802 " "
                Literal@802..803
                  IntLiteral@802..803 "0"
                Semicolon@803..804 ";"
              Blankspace@804..813 "\n        "
              VariableDeclaration@813..830
                Var@813..816 "var"
                Blankspace@816..817 " "
                Name@817..825
                  Identifier@817..825 "debugger"
                Blankspace@825..826 " "
                Equal@826..827 "="
                Blankspace@827..828 " "
                Literal@828..829
                  IntLiteral@828..829 "0"
                Semicolon@829..830 ";"
              Blankspace@830..839 "\n        "
              VariableDeclaration@839..856
                Var@839..842 "var"
                Blankspace@842..843 " "
                Name@843..851
                  Identifier@843..851 "decltype"
                Blankspace@851..852 " "
                Equal@852..853 "="
                Blankspace@853..854 " "
                Literal@854..855
                  IntLiteral@854..855 "0"
                Semicolon@855..856 ";"
              Blankspace@856..865 "\n        "
              VariableDeclaration@865..880
                Var@865..868 "var"
                Blankspace@868..869 " "
                Name@869..875
                  Identifier@869..875 "delete"
                Blankspace@875..876 " "
                Equal@876..877 "="
                Blankspace@877..878 " "
                Literal@878..879
                  IntLiteral@878..879 "0"
                Semicolon@879..880 ";"
              Blankspace@880..889 "\n        "
              VariableDeclaration@889..904
                Var@889..892 "var"
                Blankspace@892..893 " "
                Name@893..899
                  Identifier@893..899 "demote"
                Blankspace@899..900 " "
                Equal@900..901 "="
                Blankspace@901..902 " "
                Literal@902..903
                  IntLiteral@902..903 "0"
                Semicolon@903..904 ";"
              Blankspace@904..913 "\n        "
              VariableDeclaration@913..938
                Var@913..916 "var"
                Blankspace@916..917 " "
                Name@917..933
                  Identifier@917..933 "demote_to_helper"
                Blankspace@933..934 " "
                Equal@934..935 "="
                Blankspace@935..936 " "
                Literal@936..937
                  IntLiteral@936..937 "0"
                Semicolon@937..938 ";"
              Blankspace@938..947 "\n        "
              VariableDeclaration@947..958
                Var@947..950 "var"
                Blankspace@950..951 " "
                Name@951..953
                  Identifier@951..953 "do"
                Blankspace@953..954 " "
                Equal@954..955 "="
                Blankspace@955..956 " "
                Literal@956..957
                  IntLiteral@956..957 "0"
                Semicolon@957..958 ";"
              Blankspace@958..967 "\n        "
              VariableDeclaration@967..988
                Var@967..970 "var"
                Blankspace@970..971 " "
                Name@971..983
                  Identifier@971..983 "dynamic_cast"
                Blankspace@983..984 " "
                Equal@984..985 "="
                Blankspace@985..986 " "
                Literal@986..987
                  IntLiteral@986..987 "0"
                Semicolon@987..988 ";"
              Blankspace@988..997 "\n        "
              VariableDeclaration@997..1010
                Var@997..1000 "var"
                Blankspace@1000..1001 " "
                Name@1001..1005
                  Identifier@1001..1005 "enum"
                Blankspace@1005..1006 " "
                Equal@1006..1007 "="
                Blankspace@1007..1008 " "
                Literal@1008..1009
                  IntLiteral@1008..1009 "0"
                Semicolon@1009..1010 ";"
              Blankspace@1010..1019 "\n        "
              VariableDeclaration@1019..1036
                Var@1019..1022 "var"
                Blankspace@1022..1023 " "
                Name@1023..1031
                  Identifier@1023..1031 "explicit"
                Blankspace@1031..1032 " "
                Equal@1032..1033 "="
                Blankspace@1033..1034 " "
                Literal@1034..1035
                  IntLiteral@1034..1035 "0"
                Semicolon@1035..1036 ";"
              Blankspace@1036..1045 "\n        "
              VariableDeclaration@1045..1060
                Var@1045..1048 "var"
                Blankspace@1048..1049 " "
                Name@1049..1055
                  Identifier@1049..1055 "export"
                Blankspace@1055..1056 " "
                Equal@1056..1057 "="
                Blankspace@1057..1058 " "
                Literal@1058..1059
                  IntLiteral@1058..1059 "0"
                Semicolon@1059..1060 ";"
              Blankspace@1060..1069 "\n        "
              VariableDeclaration@1069..1085
                Var@1069..1072 "var"
                Blankspace@1072..1073 " "
                Name@1073..1080
                  Identifier@1073..1080 "extends"
                Blankspace@1080..1081 " "
                Equal@1081..1082 "="
                Blankspace@1082..1083 " "
                Literal@1083..1084
                  IntLiteral@1083..1084 "0"
                Semicolon@1084..1085 ";"
              Blankspace@1085..1094 "\n        "
              VariableDeclaration@1094..1109
                Var@1094..1097 "var"
                Blankspace@1097..1098 " "
                Name@1098..1104
                  Identifier@1098..1104 "extern"
                Blankspace@1104..1105 " "
                Equal@1105..1106 "="
                Blankspace@1106..1107 " "
                Literal@1107..1108
                  IntLiteral@1107..1108 "0"
                Semicolon@1108..1109 ";"
              Blankspace@1109..1118 "\n        "
              VariableDeclaration@1118..1135
                Var@1118..1121 "var"
                Blankspace@1121..1122 " "
                Name@1122..1130
                  Identifier@1122..1130 "external"
                Blankspace@1130..1131 " "
                Equal@1131..1132 "="
                Blankspace@1132..1133 " "
                Literal@1133..1134
                  IntLiteral@1133..1134 "0"
                Semicolon@1134..1135 ";"
              Blankspace@1135..1144 "\n        "
              VariableDeclaration@1144..1164
                Var@1144..1147 "var"
                Blankspace@1147..1148 " "
                Name@1148..1159
                  Identifier@1148..1159 "fallthrough"
                Blankspace@1159..1160 " "
                Equal@1160..1161 "="
                Blankspace@1161..1162 " "
                Literal@1162..1163
                  IntLiteral@1162..1163 "0"
                Semicolon@1163..1164 ";"
              Blankspace@1164..1173 "\n        "
              VariableDeclaration@1173..1188
                Var@1173..1176 "var"
                Blankspace@1176..1177 " "
                Name@1177..1183
                  Identifier@1177..1183 "filter"
                Blankspace@1183..1184 " "
                Equal@1184..1185 "="
                Blankspace@1185..1186 " "
                Literal@1186..1187
                  IntLiteral@1186..1187 "0"
                Semicolon@1187..1188 ";"
              Blankspace@1188..1197 "\n        "
              VariableDeclaration@1197..1211
                Var@1197..1200 "var"
                Blankspace@1200..1201 " "
                Name@1201..1206
                  Identifier@1201..1206 "final"
                Blankspace@1206..1207 " "
                Equal@1207..1208 "="
                Blankspace@1208..1209 " "
                Literal@1209..1210
                  IntLiteral@1209..1210 "0"
                Semicolon@1210..1211 ";"
              Blankspace@1211..1220 "\n        "
              VariableDeclaration@1220..1236
                Var@1220..1223 "var"
                Blankspace@1223..1224 " "
                Name@1224..1231
                  Identifier@1224..1231 "finally"
                Blankspace@1231..1232 " "
                Equal@1232..1233 "="
                Blankspace@1233..1234 " "
                Literal@1234..1235
                  IntLiteral@1234..1235 "0"
                Semicolon@1235..1236 ";"
              Blankspace@1236..1245 "\n        "
              VariableDeclaration@1245..1260
                Var@1245..1248 "var"
                Blankspace@1248..1249 " "
                Name@1249..1255
                  Identifier@1249..1255 "friend"
                Blankspace@1255..1256 " "
                Equal@1256..1257 "="
                Blankspace@1257..1258 " "
                Literal@1258..1259
                  IntLiteral@1258..1259 "0"
                Semicolon@1259..1260 ";"
              Blankspace@1260..1269 "\n        "
              VariableDeclaration@1269..1282
                Var@1269..1272 "var"
                Blankspace@1272..1273 " "
                Name@1273..1277
                  Identifier@1273..1277 "from"
                Blankspace@1277..1278 " "
                Equal@1278..1279 "="
                Blankspace@1279..1280 " "
                Literal@1280..1281
                  IntLiteral@1280..1281 "0"
                Semicolon@1281..1282 ";"
              Blankspace@1282..1291 "\n        "
              VariableDeclaration@1291..1307
                Var@1291..1294 "var"
                Blankspace@1294..1295 " "
                Name@1295..1302
                  Identifier@1295..1302 "fxgroup"
                Blankspace@1302..1303 " "
                Equal@1303..1304 "="
                Blankspace@1304..1305 " "
                Literal@1305..1306
                  IntLiteral@1305..1306 "0"
                Semicolon@1306..1307 ";"
              Blankspace@1307..1316 "\n        "
              VariableDeclaration@1316..1328
                Var@1316..1319 "var"
                Blankspace@1319..1320 " "
                Name@1320..1323
                  Identifier@1320..1323 "get"
                Blankspace@1323..1324 " "
                Equal@1324..1325 "="
                Blankspace@1325..1326 " "
                Literal@1326..1327
                  IntLiteral@1326..1327 "0"
                Semicolon@1327..1328 ";"
              Blankspace@1328..1337 "\n        "
              VariableDeclaration@1337..1350
                Var@1337..1340 "var"
                Blankspace@1340..1341 " "
                Name@1341..1345
                  Identifier@1341..1345 "goto"
                Blankspace@1345..1346 " "
                Equal@1346..1347 "="
                Blankspace@1347..1348 " "
                Literal@1348..1349
                  IntLiteral@1348..1349 "0"
                Semicolon@1349..1350 ";"
              Blankspace@1350..1359 "\n        "
              VariableDeclaration@1359..1379
                Var@1359..1362 "var"
                Blankspace@1362..1363 " "
                Name@1363..1374
                  Identifier@1363..1374 "groupshared"
                Blankspace@1374..1375 " "
                Equal@1375..1376 "="
                Blankspace@1376..1377 " "
                Literal@1377..1378
                  IntLiteral@1377..1378 "0"
                Semicolon@1378..1379 ";"
              Blankspace@1379..1388 "\n        "
              VariableDeclaration@1388..1402
                Var@1388..1391 "var"
                Blankspace@1391..1392 " "
                Name@1392..1397
                  Identifier@1392..1397 "highp"
                Blankspace@1397..1398 " "
                Equal@1398..1399 "="
                Blankspace@1399..1400 " "
                Literal@1400..1401
                  IntLiteral@1400..1401 "0"
                Semicolon@1401..1402 ";"
              Blankspace@1402..1411 "\n        "
              VariableDeclaration@1411..1424
                Var@1411..1414 "var"
                Blankspace@1414..1415 " "
                Name@1415..1419
                  Identifier@1415..1419 "impl"
                Blankspace@1419..1420 " "
                Equal@1420..1421 "="
                Blankspace@1421..1422 " "
                Literal@1422..1423
                  IntLiteral@1422..1423 "0"
                Semicolon@1423..1424 ";"
              Blankspace@1424..1433 "\n        "
              VariableDeclaration@1433..1452
                Var@1433..1436 "var"
                Blankspace@1436..1437 " "
                Name@1437..1447
                  Identifier@1437..1447 "implements"
                Blankspace@1447..1448 " "
                Equal@1448..1449 "="
                Blankspace@1449..1450 " "
                Literal@1450..1451
                  IntLiteral@1450..1451 "0"
                Semicolon@1451..1452 ";"
              Blankspace@1452..1461 "\n        "
              LineEndingComment@1461..1476 "// WESL keyword"
              Blankspace@1476..1485 "\n        "
              LineEndingComment@1485..1503 "// var import = 0;"
              Blankspace@1503..1512 "\n        "
              VariableDeclaration@1512..1527
                Var@1512..1515 "var"
                Blankspace@1515..1516 " "
                Name@1516..1522
                  Identifier@1516..1522 "inline"
                Blankspace@1522..1523 " "
                Equal@1523..1524 "="
                Blankspace@1524..1525 " "
                Literal@1525..1526
                  IntLiteral@1525..1526 "0"
                Semicolon@1526..1527 ";"
              Blankspace@1527..1536 "\n        "
              VariableDeclaration@1536..1555
                Var@1536..1539 "var"
                Blankspace@1539..1540 " "
                Name@1540..1550
                  Identifier@1540..1550 "instanceof"
                Blankspace@1550..1551 " "
                Equal@1551..1552 "="
                Blankspace@1552..1553 " "
                Literal@1553..1554
                  IntLiteral@1553..1554 "0"
                Semicolon@1554..1555 ";"
              Blankspace@1555..1564 "\n        "
              VariableDeclaration@1564..1582
                Var@1564..1567 "var"
                Blankspace@1567..1568 " "
                Name@1568..1577
                  Identifier@1568..1577 "interface"
                Blankspace@1577..1578 " "
                Equal@1578..1579 "="
                Blankspace@1579..1580 " "
                Literal@1580..1581
                  IntLiteral@1580..1581 "0"
                Semicolon@1581..1582 ";"
              Blankspace@1582..1591 "\n        "
              VariableDeclaration@1591..1606
                Var@1591..1594 "var"
                Blankspace@1594..1595 " "
                Name@1595..1601
                  Identifier@1595..1601 "layout"
                Blankspace@1601..1602 " "
                Equal@1602..1603 "="
                Blankspace@1603..1604 " "
                Literal@1604..1605
                  IntLiteral@1604..1605 "0"
                Semicolon@1605..1606 ";"
              Blankspace@1606..1615 "\n        "
              VariableDeclaration@1615..1628
                Var@1615..1618 "var"
                Blankspace@1618..1619 " "
                Name@1619..1623
                  Identifier@1619..1623 "lowp"
                Blankspace@1623..1624 " "
                Equal@1624..1625 "="
                Blankspace@1625..1626 " "
                Literal@1626..1627
                  IntLiteral@1626..1627 "0"
                Semicolon@1627..1628 ";"
              Blankspace@1628..1637 "\n        "
              VariableDeclaration@1637..1651
                Var@1637..1640 "var"
                Blankspace@1640..1641 " "
                Name@1641..1646
                  Identifier@1641..1646 "macro"
                Blankspace@1646..1647 " "
                Equal@1647..1648 "="
                Blankspace@1648..1649 " "
                Literal@1649..1650
                  IntLiteral@1649..1650 "0"
                Semicolon@1650..1651 ";"
              Blankspace@1651..1660 "\n        "
              VariableDeclaration@1660..1680
                Var@1660..1663 "var"
                Blankspace@1663..1664 " "
                Name@1664..1675
                  Identifier@1664..1675 "macro_rules"
                Blankspace@1675..1676 " "
                Equal@1676..1677 "="
                Blankspace@1677..1678 " "
                Literal@1678..1679
                  IntLiteral@1678..1679 "0"
                Semicolon@1679..1680 ";"
              Blankspace@1680..1689 "\n        "
              VariableDeclaration@1689..1703
                Var@1689..1692 "var"
                Blankspace@1692..1693 " "
                Name@1693..1698
                  Identifier@1693..1698 "match"
                Blankspace@1698..1699 " "
                Equal@1699..1700 "="
                Blankspace@1700..1701 " "
                Literal@1701..1702
                  IntLiteral@1701..1702 "0"
                Semicolon@1702..1703 ";"
              Blankspace@1703..1712 "\n        "
              VariableDeclaration@1712..1728
                Var@1712..1715 "var"
                Blankspace@1715..1716 " "
                Name@1716..1723
                  Identifier@1716..1723 "mediump"
                Blankspace@1723..1724 " "
                Equal@1724..1725 "="
                Blankspace@1725..1726 " "
                Literal@1726..1727
                  IntLiteral@1726..1727 "0"
                Semicolon@1727..1728 ";"
              Blankspace@1728..1737 "\n        "
              VariableDeclaration@1737..1750
                Var@1737..1740 "var"
                Blankspace@1740..1741 " "
                Name@1741..1745
                  Identifier@1741..1745 "meta"
                Blankspace@1745..1746 " "
                Equal@1746..1747 "="
                Blankspace@1747..1748 " "
                Literal@1748..1749
                  IntLiteral@1748..1749 "0"
                Semicolon@1749..1750 ";"
              Blankspace@1750..1759 "\n        "
              VariableDeclaration@1759..1771
                Var@1759..1762 "var"
                Blankspace@1762..1763 " "
                Name@1763..1766
                  Identifier@1763..1766 "mod"
                Blankspace@1766..1767 " "
                Equal@1767..1768 "="
                Blankspace@1768..1769 " "
                Literal@1769..1770
                  IntLiteral@1769..1770 "0"
                Semicolon@1770..1771 ";"
              Blankspace@1771..1780 "\n        "
              VariableDeclaration@1780..1795
                Var@1780..1783 "var"
                Blankspace@1783..1784 " "
                Name@1784..1790
                  Identifier@1784..1790 "module"
                Blankspace@1790..1791 " "
                Equal@1791..1792 "="
                Blankspace@1792..1793 " "
                Literal@1793..1794
                  IntLiteral@1793..1794 "0"
                Semicolon@1794..1795 ";"
              Blankspace@1795..1804 "\n        "
              VariableDeclaration@1804..1817
                Var@1804..1807 "var"
                Blankspace@1807..1808 " "
                Name@1808..1812
                  Identifier@1808..1812 "move"
                Blankspace@1812..1813 " "
                Equal@1813..1814 "="
                Blankspace@1814..1815 " "
                Literal@1815..1816
                  IntLiteral@1815..1816 "0"
                Semicolon@1816..1817 ";"
              Blankspace@1817..1826 "\n        "
              VariableDeclaration@1826..1838
                Var@1826..1829 "var"
                Blankspace@1829..1830 " "
                Name@1830..1833
                  Identifier@1830..1833 "mut"
                Blankspace@1833..1834 " "
                Equal@1834..1835 "="
                Blankspace@1835..1836 " "
                Literal@1836..1837
                  IntLiteral@1836..1837 "0"
                Semicolon@1837..1838 ";"
              Blankspace@1838..1847 "\n        "
              VariableDeclaration@1847..1863
                Var@1847..1850 "var"
                Blankspace@1850..1851 " "
                Name@1851..1858
                  Identifier@1851..1858 "mutable"
                Blankspace@1858..1859 " "
                Equal@1859..1860 "="
                Blankspace@1860..1861 " "
                Literal@1861..1862
                  IntLiteral@1861..1862 "0"
                Semicolon@1862..1863 ";"
              Blankspace@1863..1872 "\n        "
              VariableDeclaration@1872..1890
                Var@1872..1875 "var"
                Blankspace@1875..1876 " "
                Name@1876..1885
                  Identifier@1876..1885 "namespace"
                Blankspace@1885..1886 " "
                Equal@1886..1887 "="
                Blankspace@1887..1888 " "
                Literal@1888..1889
                  IntLiteral@1888..1889 "0"
                Semicolon@1889..1890 ";"
              Blankspace@1890..1899 "\n        "
              VariableDeclaration@1899..1911
                Var@1899..1902 "var"
                Blankspace@1902..1903 " "
                Name@1903..1906
                  Identifier@1903..1906 "new"
                Blankspace@1906..1907 " "
                Equal@1907..1908 "="
                Blankspace@1908..1909 " "
                Literal@1909..1910
                  IntLiteral@1909..1910 "0"
                Semicolon@1910..1911 ";"
              Blankspace@1911..1920 "\n        "
              VariableDeclaration@1920..1932
                Var@1920..1923 "var"
                Blankspace@1923..1924 " "
                Name@1924..1927
                  Identifier@1924..1927 "nil"
                Blankspace@1927..1928 " "
                Equal@1928..1929 "="
                Blankspace@1929..1930 " "
                Literal@1930..1931
                  IntLiteral@1930..1931 "0"
                Semicolon@1931..1932 ";"
              Blankspace@1932..1941 "\n        "
              VariableDeclaration@1941..1958
                Var@1941..1944 "var"
                Blankspace@1944..1945 " "
                Name@1945..1953
                  Identifier@1945..1953 "noexcept"
                Blankspace@1953..1954 " "
                Equal@1954..1955 "="
                Blankspace@1955..1956 " "
                Literal@1956..1957
                  IntLiteral@1956..1957 "0"
                Semicolon@1957..1958 ";"
              Blankspace@1958..1967 "\n        "
              VariableDeclaration@1967..1984
                Var@1967..1970 "var"
                Blankspace@1970..1971 " "
                Name@1971..1979
                  Identifier@1971..1979 "noinline"
                Blankspace@1979..1980 " "
                Equal@1980..1981 "="
                Blankspace@1981..1982 " "
                Literal@1982..1983
                  IntLiteral@1982..1983 "0"
                Semicolon@1983..1984 ";"
              Blankspace@1984..1993 "\n        "
              VariableDeclaration@1993..2017
                Var@1993..1996 "var"
                Blankspace@1996..1997 " "
                Name@1997..2012
                  Identifier@1997..2012 "nointerpolation"
                Blankspace@2012..2013 " "
                Equal@2013..2014 "="
                Blankspace@2014..2015 " "
                Literal@2015..2016
                  IntLiteral@2015..2016 "0"
                Semicolon@2016..2017 ";"
              Blankspace@2017..2026 "\n        "
              VariableDeclaration@2026..2047
                Var@2026..2029 "var"
                Blankspace@2029..2030 " "
                Name@2030..2042
                  Identifier@2030..2042 "non_coherent"
                Blankspace@2042..2043 " "
                Equal@2043..2044 "="
                Blankspace@2044..2045 " "
                Literal@2045..2046
                  IntLiteral@2045..2046 "0"
                Semicolon@2046..2047 ";"
              Blankspace@2047..2056 "\n        "
              VariableDeclaration@2056..2076
                Var@2056..2059 "var"
                Blankspace@2059..2060 " "
                Name@2060..2071
                  Identifier@2060..2071 "noncoherent"
                Blankspace@2071..2072 " "
                Equal@2072..2073 "="
                Blankspace@2073..2074 " "
                Literal@2074..2075
                  IntLiteral@2074..2075 "0"
                Semicolon@2075..2076 ";"
              Blankspace@2076..2085 "\n        "
              VariableDeclaration@2085..2107
                Var@2085..2088 "var"
                Blankspace@2088..2089 " "
                Name@2089..2102
                  Identifier@2089..2102 "noperspective"
                Blankspace@2102..2103 " "
                Equal@2103..2104 "="
                Blankspace@2104..2105 " "
                Literal@2105..2106
                  IntLiteral@2105..2106 "0"
                Semicolon@2106..2107 ";"
              Blankspace@2107..2116 "\n        "
              VariableDeclaration@2116..2129
                Var@2116..2119 "var"
                Blankspace@2119..2120 " "
                Name@2120..2124
                  Identifier@2120..2124 "null"
                Blankspace@2124..2125 " "
                Equal@2125..2126 "="
                Blankspace@2126..2127 " "
                Literal@2127..2128
                  IntLiteral@2127..2128 "0"
                Semicolon@2128..2129 ";"
              Blankspace@2129..2138 "\n        "
              VariableDeclaration@2138..2154
                Var@2138..2141 "var"
                Blankspace@2141..2142 " "
                Name@2142..2149
                  Identifier@2142..2149 "nullptr"
                Blankspace@2149..2150 " "
                Equal@2150..2151 "="
                Blankspace@2151..2152 " "
                Literal@2152..2153
                  IntLiteral@2152..2153 "0"
                Semicolon@2153..2154 ";"
              Blankspace@2154..2163 "\n        "
              VariableDeclaration@2163..2174
                Var@2163..2166 "var"
                Blankspace@2166..2167 " "
                Name@2167..2169
                  Identifier@2167..2169 "of"
                Blankspace@2169..2170 " "
                Equal@2170..2171 "="
                Blankspace@2171..2172 " "
                Literal@2172..2173
                  IntLiteral@2172..2173 "0"
                Semicolon@2173..2174 ";"
              Blankspace@2174..2183 "\n        "
              VariableDeclaration@2183..2200
                Var@2183..2186 "var"
                Blankspace@2186..2187 " "
                Name@2187..2195
                  Identifier@2187..2195 "operator"
                Blankspace@2195..2196 " "
                Equal@2196..2197 "="
                Blankspace@2197..2198 " "
                Literal@2198..2199
                  IntLiteral@2198..2199 "0"
                Semicolon@2199..2200 ";"
              Blankspace@2200..2209 "\n        "
              LineEndingComment@2209..2224 "// WESL keyword"
              Blankspace@2224..2233 "\n        "
              LineEndingComment@2233..2252 "// var package = 0;"
              Blankspace@2252..2261 "\n        "
              VariableDeclaration@2261..2280
                Var@2261..2264 "var"
                Blankspace@2264..2265 " "
                Name@2265..2275
                  Identifier@2265..2275 "packoffset"
                Blankspace@2275..2276 " "
                Equal@2276..2277 "="
                Blankspace@2277..2278 " "
                Literal@2278..2279
                  IntLiteral@2278..2279 "0"
                Semicolon@2279..2280 ";"
              Blankspace@2280..2289 "\n        "
              VariableDeclaration@2289..2307
                Var@2289..2292 "var"
                Blankspace@2292..2293 " "
                Name@2293..2302
                  Identifier@2293..2302 "partition"
                Blankspace@2302..2303 " "
                Equal@2303..2304 "="
                Blankspace@2304..2305 " "
                Literal@2305..2306
                  IntLiteral@2305..2306 "0"
                Semicolon@2306..2307 ";"
              Blankspace@2307..2316 "\n        "
              VariableDeclaration@2316..2329
                Var@2316..2319 "var"
                Blankspace@2319..2320 " "
                Name@2320..2324
                  Identifier@2320..2324 "pass"
                Blankspace@2324..2325 " "
                Equal@2325..2326 "="
                Blankspace@2326..2327 " "
                Literal@2327..2328
                  IntLiteral@2327..2328 "0"
                Semicolon@2328..2329 ";"
              Blankspace@2329..2338 "\n        "
              VariableDeclaration@2338..2352
                Var@2338..2341 "var"
                Blankspace@2341..2342 " "
                Name@2342..2347
                  Identifier@2342..2347 "patch"
                Blankspace@2347..2348 " "
                Equal@2348..2349 "="
                Blankspace@2349..2350 " "
                Literal@2350..2351
                  IntLiteral@2350..2351 "0"
                Semicolon@2351..2352 ";"
              Blankspace@2352..2361 "\n        "
              VariableDeclaration@2361..2383
                Var@2361..2364 "var"
                Blankspace@2364..2365 " "
                Name@2365..2378
                  Identifier@2365..2378 "pixelfragment"
                Blankspace@2378..2379 " "
                Equal@2379..2380 "="
                Blankspace@2380..2381 " "
                Literal@2381..2382
                  IntLiteral@2381..2382 "0"
                Semicolon@2382..2383 ";"
              Blankspace@2383..2392 "\n        "
              VariableDeclaration@2392..2408
                Var@2392..2395 "var"
                Blankspace@2395..2396 " "
                Name@2396..2403
                  Identifier@2396..2403 "precise"
                Blankspace@2403..2404 " "
                Equal@2404..2405 "="
                Blankspace@2405..2406 " "
                Literal@2406..2407
                  IntLiteral@2406..2407 "0"
                Semicolon@2407..2408 ";"
              Blankspace@2408..2417 "\n        "
              VariableDeclaration@2417..2435
                Var@2417..2420 "var"
                Blankspace@2420..2421 " "
                Name@2421..2430
                  Identifier@2421..2430 "precision"
                Blankspace@2430..2431 " "
                Equal@2431..2432 "="
                Blankspace@2432..2433 " "
                Literal@2433..2434
                  IntLiteral@2433..2434 "0"
                Semicolon@2434..2435 ";"
              Blankspace@2435..2444 "\n        "
              VariableDeclaration@2444..2461
                Var@2444..2447 "var"
                Blankspace@2447..2448 " "
                Name@2448..2456
                  Identifier@2448..2456 "premerge"
                Blankspace@2456..2457 " "
                Equal@2457..2458 "="
                Blankspace@2458..2459 " "
                Literal@2459..2460
                  IntLiteral@2459..2460 "0"
                Semicolon@2460..2461 ";"
              Blankspace@2461..2470 "\n        "
              VariableDeclaration@2470..2483
                Var@2470..2473 "var"
                Blankspace@2473..2474 " "
                Name@2474..2478
                  Identifier@2474..2478 "priv"
                Blankspace@2478..2479 " "
                Equal@2479..2480 "="
                Blankspace@2480..2481 " "
                Literal@2481..2482
                  IntLiteral@2481..2482 "0"
                Semicolon@2482..2483 ";"
              Blankspace@2483..2492 "\n        "
              VariableDeclaration@2492..2510
                Var@2492..2495 "var"
                Blankspace@2495..2496 " "
                Name@2496..2505
                  Identifier@2496..2505 "protected"
                Blankspace@2505..2506 " "
                Equal@2506..2507 "="
                Blankspace@2507..2508 " "
                Literal@2508..2509
                  IntLiteral@2508..2509 "0"
                Semicolon@2509..2510 ";"
              Blankspace@2510..2519 "\n        "
              VariableDeclaration@2519..2531
                Var@2519..2522 "var"
                Blankspace@2522..2523 " "
                Name@2523..2526
                  Identifier@2523..2526 "pub"
                Blankspace@2526..2527 " "
                Equal@2527..2528 "="
                Blankspace@2528..2529 " "
                Literal@2529..2530
                  IntLiteral@2529..2530 "0"
                Semicolon@2530..2531 ";"
              Blankspace@2531..2540 "\n        "
              VariableDeclaration@2540..2555
                Var@2540..2543 "var"
                Blankspace@2543..2544 " "
                Name@2544..2550
                  Identifier@2544..2550 "public"
                Blankspace@2550..2551 " "
                Equal@2551..2552 "="
                Blankspace@2552..2553 " "
                Literal@2553..2554
                  IntLiteral@2553..2554 "0"
                Semicolon@2554..2555 ";"
              Blankspace@2555..2564 "\n        "
              VariableDeclaration@2564..2581
                Var@2564..2567 "var"
                Blankspace@2567..2568 " "
                Name@2568..2576
                  Identifier@2568..2576 "readonly"
                Blankspace@2576..2577 " "
                Equal@2577..2578 "="
                Blankspace@2578..2579 " "
                Literal@2579..2580
                  IntLiteral@2579..2580 "0"
                Semicolon@2580..2581 ";"
              Blankspace@2581..2590 "\n        "
              VariableDeclaration@2590..2602
                Var@2590..2593 "var"
                Blankspace@2593..2594 " "
                Name@2594..2597
                  Identifier@2594..2597 "ref"
                Blankspace@2597..2598 " "
                Equal@2598..2599 "="
                Blankspace@2599..2600 " "
                Literal@2600..2601
                  IntLiteral@2600..2601 "0"
                Semicolon@2601..2602 ";"
              Blankspace@2602..2611 "\n        "
              VariableDeclaration@2611..2630
                Var@2611..2614 "var"
                Blankspace@2614..2615 " "
                Name@2615..2625
                  Identifier@2615..2625 "regardless"
                Blankspace@2625..2626 " "
                Equal@2626..2627 "="
                Blankspace@2627..2628 " "
                Literal@2628..2629
                  IntLiteral@2628..2629 "0"
                Semicolon@2629..2630 ";"
              Blankspace@2630..2639 "\n        "
              VariableDeclaration@2639..2656
                Var@2639..2642 "var"
                Blankspace@2642..2643 " "
                Name@2643..2651
                  Identifier@2643..2651 "register"
                Blankspace@2651..2652 " "
                Equal@2652..2653 "="
                Blankspace@2653..2654 " "
                Literal@2654..2655
                  IntLiteral@2654..2655 "0"
                Semicolon@2655..2656 ";"
              Blankspace@2656..2665 "\n        "
              VariableDeclaration@2665..2690
                Var@2665..2668 "var"
                Blankspace@2668..2669 " "
                Name@2669..2685
                  Identifier@2669..2685 "reinterpret_cast"
                Blankspace@2685..2686 " "
                Equal@2686..2687 "="
                Blankspace@2687..2688 " "
                Literal@2688..2689
                  IntLiteral@2688..2689 "0"
                Semicolon@2689..2690 ";"
              Blankspace@2690..2699 "\n        "
              VariableDeclaration@2699..2715
                Var@2699..2702 "var"
                Blankspace@2702..2703 " "
                Name@2703..2710
                  Identifier@2703..2710 "require"
                Blankspace@2710..2711 " "
                Equal@2711..2712 "="
                Blankspace@2712..2713 " "
                Literal@2713..2714
                  IntLiteral@2713..2714 "0"
                Semicolon@2714..2715 ";"
              Blankspace@2715..2724 "\n        "
              VariableDeclaration@2724..2741
                Var@2724..2727 "var"
                Blankspace@2727..2728 " "
                Name@2728..2736
                  Identifier@2728..2736 "resource"
                Blankspace@2736..2737 " "
                Equal@2737..2738 "="
                Blankspace@2738..2739 " "
                Literal@2739..2740
                  IntLiteral@2739..2740 "0"
                Semicolon@2740..2741 ";"
              Blankspace@2741..2750 "\n        "
              VariableDeclaration@2750..2767
                Var@2750..2753 "var"
                Blankspace@2753..2754 " "
                Name@2754..2762
                  Identifier@2754..2762 "restrict"
                Blankspace@2762..2763 " "
                Equal@2763..2764 "="
                Blankspace@2764..2765 " "
                Literal@2765..2766
                  IntLiteral@2765..2766 "0"
                Semicolon@2766..2767 ";"
              Blankspace@2767..2776 "\n        "
              VariableDeclaration@2776..2789
                Var@2776..2779 "var"
                Blankspace@2779..2780 " "
                Name@2780..2784
                  Identifier@2780..2784 "self"
                Blankspace@2784..2785 " "
                Equal@2785..2786 "="
                Blankspace@2786..2787 " "
                Literal@2787..2788
                  IntLiteral@2787..2788 "0"
                Semicolon@2788..2789 ";"
              Blankspace@2789..2798 "\n        "
              VariableDeclaration@2798..2810
                Var@2798..2801 "var"
                Blankspace@2801..2802 " "
                Name@2802..2805
                  Identifier@2802..2805 "set"
                Blankspace@2805..2806 " "
                Equal@2806..2807 "="
                Blankspace@2807..2808 " "
                Literal@2808..2809
                  IntLiteral@2808..2809 "0"
                Semicolon@2809..2810 ";"
              Blankspace@2810..2819 "\n        "
              VariableDeclaration@2819..2834
                Var@2819..2822 "var"
                Blankspace@2822..2823 " "
                Name@2823..2829
                  Identifier@2823..2829 "shared"
                Blankspace@2829..2830 " "
                Equal@2830..2831 "="
                Blankspace@2831..2832 " "
                Literal@2832..2833
                  IntLiteral@2832..2833 "0"
                Semicolon@2833..2834 ";"
              Blankspace@2834..2843 "\n        "
              VariableDeclaration@2843..2858
                Var@2843..2846 "var"
                Blankspace@2846..2847 " "
                Name@2847..2853
                  Identifier@2847..2853 "sizeof"
                Blankspace@2853..2854 " "
                Equal@2854..2855 "="
                Blankspace@2855..2856 " "
                Literal@2856..2857
                  IntLiteral@2856..2857 "0"
                Semicolon@2857..2858 ";"
              Blankspace@2858..2867 "\n        "
              VariableDeclaration@2867..2882
                Var@2867..2870 "var"
                Blankspace@2870..2871 " "
                Name@2871..2877
                  Identifier@2871..2877 "smooth"
                Blankspace@2877..2878 " "
                Equal@2878..2879 "="
                Blankspace@2879..2880 " "
                Literal@2880..2881
                  IntLiteral@2880..2881 "0"
                Semicolon@2881..2882 ";"
              Blankspace@2882..2891 "\n        "
              VariableDeclaration@2891..2905
                Var@2891..2894 "var"
                Blankspace@2894..2895 " "
                Name@2895..2900
                  Identifier@2895..2900 "snorm"
                Blankspace@2900..2901 " "
                Equal@2901..2902 "="
                Blankspace@2902..2903 " "
                Literal@2903..2904
                  IntLiteral@2903..2904 "0"
                Semicolon@2904..2905 ";"
              Blankspace@2905..2914 "\n        "
              VariableDeclaration@2914..2929
                Var@2914..2917 "var"
                Blankspace@2917..2918 " "
                Name@2918..2924
                  Identifier@2918..2924 "static"
                Blankspace@2924..2925 " "
                Equal@2925..2926 "="
                Blankspace@2926..2927 " "
                Literal@2927..2928
                  IntLiteral@2927..2928 "0"
                Semicolon@2928..2929 ";"
              Blankspace@2929..2938 "\n        "
              VariableDeclaration@2938..2960
                Var@2938..2941 "var"
                Blankspace@2941..2942 " "
                Name@2942..2955
                  Identifier@2942..2955 "static_assert"
                Blankspace@2955..2956 " "
                Equal@2956..2957 "="
                Blankspace@2957..2958 " "
                Literal@2958..2959
                  IntLiteral@2958..2959 "0"
                Semicolon@2959..2960 ";"
              Blankspace@2960..2969 "\n        "
              VariableDeclaration@2969..2989
                Var@2969..2972 "var"
                Blankspace@2972..2973 " "
                Name@2973..2984
                  Identifier@2973..2984 "static_cast"
                Blankspace@2984..2985 " "
                Equal@2985..2986 "="
                Blankspace@2986..2987 " "
                Literal@2987..2988
                  IntLiteral@2987..2988 "0"
                Semicolon@2988..2989 ";"
              Blankspace@2989..2998 "\n        "
              VariableDeclaration@2998..3010
                Var@2998..3001 "var"
                Blankspace@3001..3002 " "
                Name@3002..3005
                  Identifier@3002..3005 "std"
                Blankspace@3005..3006 " "
                Equal@3006..3007 "="
                Blankspace@3007..3008 " "
                Literal@3008..3009
                  IntLiteral@3008..3009 "0"
                Semicolon@3009..3010 ";"
              Blankspace@3010..3019 "\n        "
              VariableDeclaration@3019..3038
                Var@3019..3022 "var"
                Blankspace@3022..3023 " "
                Name@3023..3033
                  Identifier@3023..3033 "subroutine"
                Blankspace@3033..3034 " "
                Equal@3034..3035 "="
                Blankspace@3035..3036 " "
                Literal@3036..3037
                  IntLiteral@3036..3037 "0"
                Semicolon@3037..3038 ";"
              Blankspace@3038..3047 "\n        "
              LineEndingComment@3047..3062 "// WESL keyword"
              Blankspace@3062..3071 "\n        "
              LineEndingComment@3071..3088 "// var super = 0;"
              Blankspace@3088..3097 "\n        "
              VariableDeclaration@3097..3112
                Var@3097..3100 "var"
                Blankspace@3100..3101 " "
                Name@3101..3107
                  Identifier@3101..3107 "target"
                Blankspace@3107..3108 " "
                Equal@3108..3109 "="
                Blankspace@3109..3110 " "
                Literal@3110..3111
                  IntLiteral@3110..3111 "0"
                Semicolon@3111..3112 ";"
              Blankspace@3112..3121 "\n        "
              VariableDeclaration@3121..3138
                Var@3121..3124 "var"
                Blankspace@3124..3125 " "
                Name@3125..3133
                  Identifier@3125..3133 "template"
                Blankspace@3133..3134 " "
                Equal@3134..3135 "="
                Blankspace@3135..3136 " "
                Literal@3136..3137
                  IntLiteral@3136..3137 "0"
                Semicolon@3137..3138 ";"
              Blankspace@3138..3147 "\n        "
              VariableDeclaration@3147..3160
                Var@3147..3150 "var"
                Blankspace@3150..3151 " "
                Name@3151..3155
                  Identifier@3151..3155 "this"
                Blankspace@3155..3156 " "
                Equal@3156..3157 "="
                Blankspace@3157..3158 " "
                Literal@3158..3159
                  IntLiteral@3158..3159 "0"
                Semicolon@3159..3160 ";"
              Blankspace@3160..3169 "\n        "
              VariableDeclaration@3169..3190
                Var@3169..3172 "var"
                Blankspace@3172..3173 " "
                Name@3173..3185
                  Identifier@3173..3185 "thread_local"
                Blankspace@3185..3186 " "
                Equal@3186..3187 "="
                Blankspace@3187..3188 " "
                Literal@3188..3189
                  IntLiteral@3188..3189 "0"
                Semicolon@3189..3190 ";"
              Blankspace@3190..3199 "\n        "
              VariableDeclaration@3199..3213
                Var@3199..3202 "var"
                Blankspace@3202..3203 " "
                Name@3203..3208
                  Identifier@3203..3208 "throw"
                Blankspace@3208..3209 " "
                Equal@3209..3210 "="
                Blankspace@3210..3211 " "
                Literal@3211..3212
                  IntLiteral@3211..3212 "0"
                Semicolon@3212..3213 ";"
              Blankspace@3213..3222 "\n        "
              VariableDeclaration@3222..3236
                Var@3222..3225 "var"
                Blankspace@3225..3226 " "
                Name@3226..3231
                  Identifier@3226..3231 "trait"
                Blankspace@3231..3232 " "
                Equal@3232..3233 "="
                Blankspace@3233..3234 " "
                Literal@3234..3235
                  IntLiteral@3234..3235 "0"
                Semicolon@3235..3236 ";"
              Blankspace@3236..3245 "\n        "
              VariableDeclaration@3245..3257
                Var@3245..3248 "var"
                Blankspace@3248..3249 " "
                Name@3249..3252
                  Identifier@3249..3252 "try"
                Blankspace@3252..3253 " "
                Equal@3253..3254 "="
                Blankspace@3254..3255 " "
                Literal@3255..3256
                  IntLiteral@3255..3256 "0"
                Semicolon@3256..3257 ";"
              Blankspace@3257..3266 "\n        "
              VariableDeclaration@3266..3279
                Var@3266..3269 "var"
                Blankspace@3269..3270 " "
                Name@3270..3274
                  Identifier@3270..3274 "type"
                Blankspace@3274..3275 " "
                Equal@3275..3276 "="
                Blankspace@3276..3277 " "
                Literal@3277..3278
                  IntLiteral@3277..3278 "0"
                Semicolon@3278..3279 ";"
              Blankspace@3279..3288 "\n        "
              VariableDeclaration@3288..3304
                Var@3288..3291 "var"
                Blankspace@3291..3292 " "
                Name@3292..3299
                  Identifier@3292..3299 "typedef"
                Blankspace@3299..3300 " "
                Equal@3300..3301 "="
                Blankspace@3301..3302 " "
                Literal@3302..3303
                  IntLiteral@3302..3303 "0"
                Semicolon@3303..3304 ";"
              Blankspace@3304..3313 "\n        "
              VariableDeclaration@3313..3328
                Var@3313..3316 "var"
                Blankspace@3316..3317 " "
                Name@3317..3323
                  Identifier@3317..3323 "typeid"
                Blankspace@3323..3324 " "
                Equal@3324..3325 "="
                Blankspace@3325..3326 " "
                Literal@3326..3327
                  IntLiteral@3326..3327 "0"
                Semicolon@3327..3328 ";"
              Blankspace@3328..3337 "\n        "
              VariableDeclaration@3337..3354
                Var@3337..3340 "var"
                Blankspace@3340..3341 " "
                Name@3341..3349
                  Identifier@3341..3349 "typename"
                Blankspace@3349..3350 " "
                Equal@3350..3351 "="
                Blankspace@3351..3352 " "
                Literal@3352..3353
                  IntLiteral@3352..3353 "0"
                Semicolon@3353..3354 ";"
              Blankspace@3354..3363 "\n        "
              VariableDeclaration@3363..3378
                Var@3363..3366 "var"
                Blankspace@3366..3367 " "
                Name@3367..3373
                  Identifier@3367..3373 "typeof"
                Blankspace@3373..3374 " "
                Equal@3374..3375 "="
                Blankspace@3375..3376 " "
                Literal@3376..3377
                  IntLiteral@3376..3377 "0"
                Semicolon@3377..3378 ";"
              Blankspace@3378..3387 "\n        "
              VariableDeclaration@3387..3401
                Var@3387..3390 "var"
                Blankspace@3390..3391 " "
                Name@3391..3396
                  Identifier@3391..3396 "union"
                Blankspace@3396..3397 " "
                Equal@3397..3398 "="
                Blankspace@3398..3399 " "
                Literal@3399..3400
                  IntLiteral@3399..3400 "0"
                Semicolon@3400..3401 ";"
              Blankspace@3401..3410 "\n        "
              VariableDeclaration@3410..3425
                Var@3410..3413 "var"
                Blankspace@3413..3414 " "
                Name@3414..3420
                  Identifier@3414..3420 "unless"
                Blankspace@3420..3421 " "
                Equal@3421..3422 "="
                Blankspace@3422..3423 " "
                Literal@3423..3424
                  IntLiteral@3423..3424 "0"
                Semicolon@3424..3425 ";"
              Blankspace@3425..3434 "\n        "
              VariableDeclaration@3434..3448
                Var@3434..3437 "var"
                Blankspace@3437..3438 " "
                Name@3438..3443
                  Identifier@3438..3443 "unorm"
                Blankspace@3443..3444 " "
                Equal@3444..3445 "="
                Blankspace@3445..3446 " "
                Literal@3446..3447
                  IntLiteral@3446..3447 "0"
                Semicolon@3447..3448 ";"
              Blankspace@3448..3457 "\n        "
              VariableDeclaration@3457..3472
                Var@3457..3460 "var"
                Blankspace@3460..3461 " "
                Name@3461..3467
                  Identifier@3461..3467 "unsafe"
                Blankspace@3467..3468 " "
                Equal@3468..3469 "="
                Blankspace@3469..3470 " "
                Literal@3470..3471
                  IntLiteral@3470..3471 "0"
                Semicolon@3471..3472 ";"
              Blankspace@3472..3481 "\n        "
              VariableDeclaration@3481..3497
                Var@3481..3484 "var"
                Blankspace@3484..3485 " "
                Name@3485..3492
                  Identifier@3485..3492 "unsized"
                Blankspace@3492..3493 " "
                Equal@3493..3494 "="
                Blankspace@3494..3495 " "
                Literal@3495..3496
                  IntLiteral@3495..3496 "0"
                Semicolon@3496..3497 ";"
              Blankspace@3497..3506 "\n        "
              VariableDeclaration@3506..3518
                Var@3506..3509 "var"
                Blankspace@3509..3510 " "
                Name@3510..3513
                  Identifier@3510..3513 "use"
                Blankspace@3513..3514 " "
                Equal@3514..3515 "="
                Blankspace@3515..3516 " "
                Literal@3516..3517
                  IntLiteral@3516..3517 "0"
                Semicolon@3517..3518 ";"
              Blankspace@3518..3527 "\n        "
              VariableDeclaration@3527..3541
                Var@3527..3530 "var"
                Blankspace@3530..3531 " "
                Name@3531..3536
                  Identifier@3531..3536 "using"
                Blankspace@3536..3537 " "
                Equal@3537..3538 "="
                Blankspace@3538..3539 " "
                Literal@3539..3540
                  IntLiteral@3539..3540 "0"
                Semicolon@3540..3541 ";"
              Blankspace@3541..3550 "\n        "
              VariableDeclaration@3550..3566
                Var@3550..3553 "var"
                Blankspace@3553..3554 " "
                Name@3554..3561
                  Identifier@3554..3561 "varying"
                Blankspace@3561..3562 " "
                Equal@3562..3563 "="
                Blankspace@3563..3564 " "
                Literal@3564..3565
                  IntLiteral@3564..3565 "0"
                Semicolon@3565..3566 ";"
              Blankspace@3566..3575 "\n        "
              VariableDeclaration@3575..3591
                Var@3575..3578 "var"
                Blankspace@3578..3579 " "
                Name@3579..3586
                  Identifier@3579..3586 "virtual"
                Blankspace@3586..3587 " "
                Equal@3587..3588 "="
                Blankspace@3588..3589 " "
                Literal@3589..3590
                  IntLiteral@3589..3590 "0"
                Semicolon@3590..3591 ";"
              Blankspace@3591..3600 "\n        "
              VariableDeclaration@3600..3617
                Var@3600..3603 "var"
                Blankspace@3603..3604 " "
                Name@3604..3612
                  Identifier@3604..3612 "volatile"
                Blankspace@3612..3613 " "
                Equal@3613..3614 "="
                Blankspace@3614..3615 " "
                Literal@3615..3616
                  IntLiteral@3615..3616 "0"
                Semicolon@3616..3617 ";"
              Blankspace@3617..3626 "\n        "
              VariableDeclaration@3626..3639
                Var@3626..3629 "var"
                Blankspace@3629..3630 " "
                Name@3630..3634
                  Identifier@3630..3634 "wgsl"
                Blankspace@3634..3635 " "
                Equal@3635..3636 "="
                Blankspace@3636..3637 " "
                Literal@3637..3638
                  IntLiteral@3637..3638 "0"
                Semicolon@3638..3639 ";"
              Blankspace@3639..3648 "\n        "
              VariableDeclaration@3648..3662
                Var@3648..3651 "var"
                Blankspace@3651..3652 " "
                Name@3652..3657
                  Identifier@3652..3657 "where"
                Blankspace@3657..3658 " "
                Equal@3658..3659 "="
                Blankspace@3659..3660 " "
                Literal@3660..3661
                  IntLiteral@3660..3661 "0"
                Semicolon@3661..3662 ";"
              Blankspace@3662..3671 "\n        "
              VariableDeclaration@3671..3684
                Var@3671..3674 "var"
                Blankspace@3674..3675 " "
                Name@3675..3679
                  Identifier@3675..3679 "with"
                Blankspace@3679..3680 " "
                Equal@3680..3681 "="
                Blankspace@3681..3682 " "
                Literal@3682..3683
                  IntLiteral@3682..3683 "0"
                Semicolon@3683..3684 ";"
              Blankspace@3684..3693 "\n        "
              VariableDeclaration@3693..3711
                Var@3693..3696 "var"
                Blankspace@3696..3697 " "
                Name@3697..3706
                  Identifier@3697..3706 "writeonly"
                Blankspace@3706..3707 " "
                Equal@3707..3708 "="
                Blankspace@3708..3709 " "
                Literal@3709..3710
                  IntLiteral@3709..3710 "0"
                Semicolon@3710..3711 ";"
              Blankspace@3711..3720 "\n        "
              VariableDeclaration@3720..3734
                Var@3720..3723 "var"
                Blankspace@3723..3724 " "
                Name@3724..3729
                  Identifier@3724..3729 "yield"
                Blankspace@3729..3730 " "
                Equal@3730..3731 "="
                Blankspace@3731..3732 " "
                Literal@3732..3733
                  IntLiteral@3732..3733 "0"
                Semicolon@3733..3734 ";"
              Blankspace@3734..3743 "\n        ""#]],
    );
}

#[test]
fn keywords_do_not_parse() {
    check(
        "
        var alias=0;
        var break=0;
        var case=0;
        var const=0;
        var const_assert=0;
        var continue=0;
        var continuing=0;
        var default=0;
        var diagnostic=0;
        var discard=0;
        var else=0;
        var enable=0;
        var false=0;
        var fn=0;
        var for=0;
        var if=0;
        var let=0;
        var loop=0;
        var override=0;
        var requires=0;
        var return=0;
        var struct=0;
        var switch=0;
        var true=0;
        var var=0;
        var while=0;
        ",
        expect![[r#"
            SourceFile@0..573
              Blankspace@0..9 "\n        "
              VariableDeclaration@9..13
                Var@9..12 "var"
                Blankspace@12..13 " "
              TypeAliasDeclaration@13..21
                Alias@13..18 "alias"
                Equal@18..19 "="
                TypeSpecifier@19..20
                  Path@19..20
                    Error@19..20
                      IntLiteral@19..20 "0"
                Semicolon@20..21 ";"
              Blankspace@21..30 "\n        "
              VariableDeclaration@30..42
                Var@30..33 "var"
                Blankspace@33..34 " "
                Error@34..39
                  Break@34..39 "break"
                Equal@39..40 "="
                Literal@40..41
                  IntLiteral@40..41 "0"
                Semicolon@41..42 ";"
              Blankspace@42..51 "\n        "
              VariableDeclaration@51..62
                Var@51..54 "var"
                Blankspace@54..55 " "
                Error@55..59
                  Case@55..59 "case"
                Equal@59..60 "="
                Literal@60..61
                  IntLiteral@60..61 "0"
                Semicolon@61..62 ";"
              Blankspace@62..71 "\n        "
              VariableDeclaration@71..75
                Var@71..74 "var"
                Blankspace@74..75 " "
              ConstantDeclaration@75..83
                Const@75..80 "const"
                Equal@80..81 "="
                Literal@81..82
                  IntLiteral@81..82 "0"
                Semicolon@82..83 ";"
              Blankspace@83..92 "\n        "
              VariableDeclaration@92..96
                Var@92..95 "var"
                Blankspace@95..96 " "
              AssertStatement@96..108
                ConstantAssert@96..108 "const_assert"
              Error@108..110
                Equal@108..109 "="
                IntLiteral@109..110 "0"
              Semicolon@110..111 ";"
              Blankspace@111..120 "\n        "
              VariableDeclaration@120..135
                Var@120..123 "var"
                Blankspace@123..124 " "
                Error@124..132
                  Continue@124..132 "continue"
                Equal@132..133 "="
                Literal@133..134
                  IntLiteral@133..134 "0"
                Semicolon@134..135 ";"
              Blankspace@135..144 "\n        "
              VariableDeclaration@144..161
                Var@144..147 "var"
                Blankspace@147..148 " "
                Error@148..158
                  Continuing@148..158 "continuing"
                Equal@158..159 "="
                Literal@159..160
                  IntLiteral@159..160 "0"
                Semicolon@160..161 ";"
              Blankspace@161..170 "\n        "
              VariableDeclaration@170..184
                Var@170..173 "var"
                Blankspace@173..174 " "
                Error@174..181
                  Default@174..181 "default"
                Equal@181..182 "="
                Literal@182..183
                  IntLiteral@182..183 "0"
                Semicolon@183..184 ";"
              Blankspace@184..193 "\n        "
              VariableDeclaration@193..197
                Var@193..196 "var"
                Blankspace@196..197 " "
              DiagnosticDirective@197..210
                Diagnostic@197..207 "diagnostic"
                DiagnosticControl@207..209
                  DiagnosticRuleName@207..209
                    Error@207..209
                      Equal@207..208 "="
                      IntLiteral@208..209 "0"
                Semicolon@209..210 ";"
              Blankspace@210..219 "\n        "
              VariableDeclaration@219..233
                Var@219..222 "var"
                Blankspace@222..223 " "
                Error@223..230
                  Discard@223..230 "discard"
                Equal@230..231 "="
                Literal@231..232
                  IntLiteral@231..232 "0"
                Semicolon@232..233 ";"
              Blankspace@233..242 "\n        "
              VariableDeclaration@242..253
                Var@242..245 "var"
                Blankspace@245..246 " "
                Error@246..250
                  Else@246..250 "else"
                Equal@250..251 "="
                Literal@251..252
                  IntLiteral@251..252 "0"
                Semicolon@252..253 ";"
              Blankspace@253..262 "\n        "
              VariableDeclaration@262..266
                Var@262..265 "var"
                Blankspace@265..266 " "
              EnableDirective@266..275
                Enable@266..272 "enable"
                Error@272..274
                  Equal@272..273 "="
                  IntLiteral@273..274 "0"
                Semicolon@274..275 ";"
              Blankspace@275..284 "\n        "
              VariableDeclaration@284..296
                Var@284..287 "var"
                Blankspace@287..288 " "
                Error@288..293
                  False@288..293 "false"
                Equal@293..294 "="
                Literal@294..295
                  IntLiteral@294..295 "0"
                Semicolon@295..296 ";"
              Blankspace@296..305 "\n        "
              VariableDeclaration@305..309
                Var@305..308 "var"
                Blankspace@308..309 " "
              FunctionDeclaration@309..313
                Fn@309..311 "fn"
                FunctionParameters@311..313
                  Error@311..313
                    Equal@311..312 "="
                    IntLiteral@312..313 "0"
              Semicolon@313..314 ";"
              Blankspace@314..323 "\n        "
              VariableDeclaration@323..333
                Var@323..326 "var"
                Blankspace@326..327 " "
                Error@327..330
                  For@327..330 "for"
                Equal@330..331 "="
                Literal@331..332
                  IntLiteral@331..332 "0"
                Semicolon@332..333 ";"
              Blankspace@333..342 "\n        "
              VariableDeclaration@342..351
                Var@342..345 "var"
                Blankspace@345..346 " "
                Error@346..348
                  If@346..348 "if"
                Equal@348..349 "="
                Literal@349..350
                  IntLiteral@349..350 "0"
                Semicolon@350..351 ";"
              Blankspace@351..360 "\n        "
              VariableDeclaration@360..364
                Var@360..363 "var"
                Blankspace@363..364 " "
              Error@364..370
                Let@364..367 "let"
                Equal@367..368 "="
                Literal@368..369
                  IntLiteral@368..369 "0"
                Semicolon@369..370 ";"
              Blankspace@370..379 "\n        "
              VariableDeclaration@379..390
                Var@379..382 "var"
                Blankspace@382..383 " "
                Error@383..387
                  Loop@383..387 "loop"
                Equal@387..388 "="
                Literal@388..389
                  IntLiteral@388..389 "0"
                Semicolon@389..390 ";"
              Blankspace@390..399 "\n        "
              VariableDeclaration@399..403
                Var@399..402 "var"
                Blankspace@402..403 " "
              OverrideDeclaration@403..414
                Override@403..411 "override"
                Equal@411..412 "="
                Literal@412..413
                  IntLiteral@412..413 "0"
                Semicolon@413..414 ";"
              Blankspace@414..423 "\n        "
              VariableDeclaration@423..427
                Var@423..426 "var"
                Blankspace@426..427 " "
              RequiresDirective@427..438
                Requires@427..435 "requires"
                Error@435..437
                  Equal@435..436 "="
                  IntLiteral@436..437 "0"
                Semicolon@437..438 ";"
              Blankspace@438..447 "\n        "
              VariableDeclaration@447..460
                Var@447..450 "var"
                Blankspace@450..451 " "
                Error@451..457
                  Return@451..457 "return"
                Equal@457..458 "="
                Literal@458..459
                  IntLiteral@458..459 "0"
                Semicolon@459..460 ";"
              Blankspace@460..469 "\n        "
              VariableDeclaration@469..473
                Var@469..472 "var"
                Blankspace@472..473 " "
              StructDeclaration@473..479
                Struct@473..479 "struct"
              Error@479..481
                Equal@479..480 "="
                IntLiteral@480..481 "0"
              Semicolon@481..482 ";"
              Blankspace@482..491 "\n        "
              VariableDeclaration@491..504
                Var@491..494 "var"
                Blankspace@494..495 " "
                Error@495..501
                  Switch@495..501 "switch"
                Equal@501..502 "="
                Literal@502..503
                  IntLiteral@502..503 "0"
                Semicolon@503..504 ";"
              Blankspace@504..513 "\n        "
              VariableDeclaration@513..524
                Var@513..516 "var"
                Blankspace@516..517 " "
                Error@517..521
                  True@517..521 "true"
                Equal@521..522 "="
                Literal@522..523
                  IntLiteral@522..523 "0"
                Semicolon@523..524 ";"
              Blankspace@524..533 "\n        "
              VariableDeclaration@533..537
                Var@533..536 "var"
                Blankspace@536..537 " "
              VariableDeclaration@537..543
                Var@537..540 "var"
                Equal@540..541 "="
                Literal@541..542
                  IntLiteral@541..542 "0"
                Semicolon@542..543 ";"
              Blankspace@543..552 "\n        "
              VariableDeclaration@552..564
                Var@552..555 "var"
                Blankspace@555..556 " "
                Error@556..561
                  While@556..561 "while"
                Equal@561..562 "="
                Literal@562..563
                  IntLiteral@562..563 "0"
                Semicolon@563..564 ";"
              Blankspace@564..573 "\n        "

            error at 13..18: invalid syntax, expected one of: '@', '{', '}', ',', '=', <identifier>, ')', ';', <template start>
            error at 18..19: invalid syntax, expected: <identifier>
            error at 19..20: invalid syntax, expected one of: <identifier>, 'package', 'super'
            error at 34..39: invalid syntax, expected one of: '@', '{', '}', ',', '=', <identifier>, ')', ';', <template start>
            error at 55..59: invalid syntax, expected one of: '@', '{', '}', ',', '=', <identifier>, ')', ';', <template start>
            error at 75..80: invalid syntax, expected one of: '@', '{', '}', ',', '=', <identifier>, ')', ';', <template start>
            error at 80..81: invalid syntax, expected: <identifier>
            error at 96..108: invalid syntax, expected one of: '@', '{', '}', ',', '=', <identifier>, ')', ';', <template start>
            error at 108..109: invalid syntax, expected one of: '&', '!', 'false', <floating point literal>, <identifier>, <integer literal>, '-', 'package', '(', '*', 'super', '~', 'true'
            error at 124..132: invalid syntax, expected one of: '@', '{', '}', ',', '=', <identifier>, ')', ';', <template start>
            error at 148..158: invalid syntax, expected one of: '@', '{', '}', ',', '=', <identifier>, ')', ';', <template start>
            error at 174..181: invalid syntax, expected one of: '@', '{', '}', ',', '=', <identifier>, ')', ';', <template start>
            error at 197..207: invalid syntax, expected one of: '@', '{', '}', ',', '=', <identifier>, ')', ';', <template start>
            error at 197..207: directives must come before other items
            error at 207..208: invalid syntax, expected: '('
            error at 223..230: invalid syntax, expected one of: '@', '{', '}', ',', '=', <identifier>, ')', ';', <template start>
            error at 246..250: invalid syntax, expected one of: '@', '{', '}', ',', '=', <identifier>, ')', ';', <template start>
            error at 266..272: invalid syntax, expected one of: '@', '{', '}', ',', '=', <identifier>, ')', ';', <template start>
            error at 266..272: directives must come before other items
            error at 272..273: invalid syntax, expected: <identifier>
            error at 272..272: unknown extension 
            error at 288..293: invalid syntax, expected one of: '@', '{', '}', ',', '=', <identifier>, ')', ';', <template start>
            error at 309..311: invalid syntax, expected one of: '@', '{', '}', ',', '=', <identifier>, ')', ';', <template start>
            error at 311..312: invalid syntax, expected: <identifier>
            error at 327..330: invalid syntax, expected one of: '@', '{', '}', ',', '=', <identifier>, ')', ';', <template start>
            error at 346..348: invalid syntax, expected one of: '@', '{', '}', ',', '=', <identifier>, ')', ';', <template start>
            error at 364..367: invalid syntax, expected one of: '@', '{', '}', ',', '=', <identifier>, ')', ';', <template start>
            error at 367..368: invalid syntax, expected: <identifier>
            error at 364..370: global let declarations are not allowed
            error at 383..387: invalid syntax, expected one of: '@', '{', '}', ',', '=', <identifier>, ')', ';', <template start>
            error at 403..411: invalid syntax, expected one of: '@', '{', '}', ',', '=', <identifier>, ')', ';', <template start>
            error at 411..412: invalid syntax, expected: <identifier>
            error at 427..435: invalid syntax, expected one of: '@', '{', '}', ',', '=', <identifier>, ')', ';', <template start>
            error at 427..435: directives must come before other items
            error at 435..436: invalid syntax, expected: <identifier>
            error at 451..457: invalid syntax, expected one of: '@', '{', '}', ',', '=', <identifier>, ')', ';', <template start>
            error at 473..479: invalid syntax, expected one of: '@', '{', '}', ',', '=', <identifier>, ')', ';', <template start>
            error at 479..480: invalid syntax, expected: <identifier>
            error at 495..501: invalid syntax, expected one of: '@', '{', '}', ',', '=', <identifier>, ')', ';', <template start>
            error at 517..521: invalid syntax, expected one of: '@', '{', '}', ',', '=', <identifier>, ')', ';', <template start>
            error at 537..540: invalid syntax, expected one of: '@', '{', '}', ',', '=', <identifier>, ')', ';', <template start>
            error at 540..541: invalid syntax, expected: <identifier>
            error at 556..561: invalid syntax, expected one of: '@', '{', '}', ',', '=', <identifier>, ')', ';', <template start>"#]],
    );
}

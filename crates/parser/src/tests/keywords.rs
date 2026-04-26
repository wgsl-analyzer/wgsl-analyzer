use expect_test::expect;

use crate::tests::check;

#[test]
fn reserved_words_do_not_parse() {
    check(
        "
        NULL,Self,abstract,active,alignas,alignof,as,asm,asm_fragment,async,attribute,auto,await,become,cast,catch,class,co_await,co_return,co_yield,coherent,column_major,common,compile,compile_fragment,concept,const_cast,consteval,constexpr,constinit,crate,debugger,decltype,delete,demote,demote_to_helper,do,dynamic_cast,enum,explicit,export,extends,extern,external,fallthrough,filter,final,finally,friend,from,fxgroup,get,goto,groupshared,highp,impl,implements,import,inline,instanceof,interface,layout,lowp,macro,macro_rules,match,mediump,meta,mod,module,move,mut,mutable,namespace,new,nil,noexcept,noinline,nointerpolation,non_coherent,noncoherent,noperspective,null,nullptr,of,operator,package,packoffset,partition,pass,patch,pixelfragment,precise,precision,premerge,priv,protected,pub,public,readonly,ref,regardless,register,reinterpret_cast,require,resource,restrict,self,set,shared,sizeof,smooth,snorm,static,static_assert,static_cast,std,subroutine,super,target,template,this,thread_local,throw,trait,try,type,typedef,typeid,typename,typeof,union,unless,unorm,unsafe,unsized,use,using,varying,virtual,volatile,wgsl,where,with,writeonly,yield",
        expect![[r#"
            SourceFile@0..1152
              Blankspace@0..9 "\n        "
              Error@9..1152
                Reserved@9..13 "NULL"
                Comma@13..14 ","
                Reserved@14..18 "Self"
                Comma@18..19 ","
                Reserved@19..27 "abstract"
                Comma@27..28 ","
                Reserved@28..34 "active"
                Comma@34..35 ","
                Reserved@35..42 "alignas"
                Comma@42..43 ","
                Reserved@43..50 "alignof"
                Comma@50..51 ","
                Reserved@51..53 "as"
                Comma@53..54 ","
                Reserved@54..57 "asm"
                Comma@57..58 ","
                Reserved@58..70 "asm_fragment"
                Comma@70..71 ","
                Reserved@71..76 "async"
                Comma@76..77 ","
                Reserved@77..86 "attribute"
                Comma@86..87 ","
                Reserved@87..91 "auto"
                Comma@91..92 ","
                Reserved@92..97 "await"
                Comma@97..98 ","
                Reserved@98..104 "become"
                Comma@104..105 ","
                Reserved@105..109 "cast"
                Comma@109..110 ","
                Reserved@110..115 "catch"
                Comma@115..116 ","
                Reserved@116..121 "class"
                Comma@121..122 ","
                Reserved@122..130 "co_await"
                Comma@130..131 ","
                Reserved@131..140 "co_return"
                Comma@140..141 ","
                Reserved@141..149 "co_yield"
                Comma@149..150 ","
                Reserved@150..158 "coherent"
                Comma@158..159 ","
                Reserved@159..171 "column_major"
                Comma@171..172 ","
                Reserved@172..178 "common"
                Comma@178..179 ","
                Reserved@179..186 "compile"
                Comma@186..187 ","
                Reserved@187..203 "compile_fragment"
                Comma@203..204 ","
                Reserved@204..211 "concept"
                Comma@211..212 ","
                Reserved@212..222 "const_cast"
                Comma@222..223 ","
                Reserved@223..232 "consteval"
                Comma@232..233 ","
                Reserved@233..242 "constexpr"
                Comma@242..243 ","
                Reserved@243..252 "constinit"
                Comma@252..253 ","
                Reserved@253..258 "crate"
                Comma@258..259 ","
                Reserved@259..267 "debugger"
                Comma@267..268 ","
                Reserved@268..276 "decltype"
                Comma@276..277 ","
                Reserved@277..283 "delete"
                Comma@283..284 ","
                Reserved@284..290 "demote"
                Comma@290..291 ","
                Reserved@291..307 "demote_to_helper"
                Comma@307..308 ","
                Reserved@308..310 "do"
                Comma@310..311 ","
                Reserved@311..323 "dynamic_cast"
                Comma@323..324 ","
                Reserved@324..328 "enum"
                Comma@328..329 ","
                Reserved@329..337 "explicit"
                Comma@337..338 ","
                Reserved@338..344 "export"
                Comma@344..345 ","
                Reserved@345..352 "extends"
                Comma@352..353 ","
                Reserved@353..359 "extern"
                Comma@359..360 ","
                Reserved@360..368 "external"
                Comma@368..369 ","
                Reserved@369..380 "fallthrough"
                Comma@380..381 ","
                Reserved@381..387 "filter"
                Comma@387..388 ","
                Reserved@388..393 "final"
                Comma@393..394 ","
                Reserved@394..401 "finally"
                Comma@401..402 ","
                Reserved@402..408 "friend"
                Comma@408..409 ","
                Reserved@409..413 "from"
                Comma@413..414 ","
                Reserved@414..421 "fxgroup"
                Comma@421..422 ","
                Reserved@422..425 "get"
                Comma@425..426 ","
                Reserved@426..430 "goto"
                Comma@430..431 ","
                Reserved@431..442 "groupshared"
                Comma@442..443 ","
                Reserved@443..448 "highp"
                Comma@448..449 ","
                Reserved@449..453 "impl"
                Comma@453..454 ","
                Reserved@454..464 "implements"
                Comma@464..465 ","
                Reserved@465..471 "import"
                Comma@471..472 ","
                Reserved@472..478 "inline"
                Comma@478..479 ","
                Reserved@479..489 "instanceof"
                Comma@489..490 ","
                Reserved@490..499 "interface"
                Comma@499..500 ","
                Reserved@500..506 "layout"
                Comma@506..507 ","
                Reserved@507..511 "lowp"
                Comma@511..512 ","
                Reserved@512..517 "macro"
                Comma@517..518 ","
                Reserved@518..529 "macro_rules"
                Comma@529..530 ","
                Reserved@530..535 "match"
                Comma@535..536 ","
                Reserved@536..543 "mediump"
                Comma@543..544 ","
                Reserved@544..548 "meta"
                Comma@548..549 ","
                Reserved@549..552 "mod"
                Comma@552..553 ","
                Reserved@553..559 "module"
                Comma@559..560 ","
                Reserved@560..564 "move"
                Comma@564..565 ","
                Reserved@565..568 "mut"
                Comma@568..569 ","
                Reserved@569..576 "mutable"
                Comma@576..577 ","
                Reserved@577..586 "namespace"
                Comma@586..587 ","
                Reserved@587..590 "new"
                Comma@590..591 ","
                Reserved@591..594 "nil"
                Comma@594..595 ","
                Reserved@595..603 "noexcept"
                Comma@603..604 ","
                Reserved@604..612 "noinline"
                Comma@612..613 ","
                Reserved@613..628 "nointerpolation"
                Comma@628..629 ","
                Reserved@629..641 "non_coherent"
                Comma@641..642 ","
                Reserved@642..653 "noncoherent"
                Comma@653..654 ","
                Reserved@654..667 "noperspective"
                Comma@667..668 ","
                Reserved@668..672 "null"
                Comma@672..673 ","
                Reserved@673..680 "nullptr"
                Comma@680..681 ","
                Reserved@681..683 "of"
                Comma@683..684 ","
                Reserved@684..692 "operator"
                Comma@692..693 ","
                Reserved@693..700 "package"
                Comma@700..701 ","
                Reserved@701..711 "packoffset"
                Comma@711..712 ","
                Reserved@712..721 "partition"
                Comma@721..722 ","
                Reserved@722..726 "pass"
                Comma@726..727 ","
                Reserved@727..732 "patch"
                Comma@732..733 ","
                Reserved@733..746 "pixelfragment"
                Comma@746..747 ","
                Reserved@747..754 "precise"
                Comma@754..755 ","
                Reserved@755..764 "precision"
                Comma@764..765 ","
                Reserved@765..773 "premerge"
                Comma@773..774 ","
                Reserved@774..778 "priv"
                Comma@778..779 ","
                Reserved@779..788 "protected"
                Comma@788..789 ","
                Reserved@789..792 "pub"
                Comma@792..793 ","
                Reserved@793..799 "public"
                Comma@799..800 ","
                Reserved@800..808 "readonly"
                Comma@808..809 ","
                Reserved@809..812 "ref"
                Comma@812..813 ","
                Reserved@813..823 "regardless"
                Comma@823..824 ","
                Reserved@824..832 "register"
                Comma@832..833 ","
                Reserved@833..849 "reinterpret_cast"
                Comma@849..850 ","
                Reserved@850..857 "require"
                Comma@857..858 ","
                Reserved@858..866 "resource"
                Comma@866..867 ","
                Reserved@867..875 "restrict"
                Comma@875..876 ","
                Reserved@876..880 "self"
                Comma@880..881 ","
                Reserved@881..884 "set"
                Comma@884..885 ","
                Reserved@885..891 "shared"
                Comma@891..892 ","
                Reserved@892..898 "sizeof"
                Comma@898..899 ","
                Reserved@899..905 "smooth"
                Comma@905..906 ","
                Reserved@906..911 "snorm"
                Comma@911..912 ","
                Reserved@912..918 "static"
                Comma@918..919 ","
                Reserved@919..932 "static_assert"
                Comma@932..933 ","
                Reserved@933..944 "static_cast"
                Comma@944..945 ","
                Reserved@945..948 "std"
                Comma@948..949 ","
                Reserved@949..959 "subroutine"
                Comma@959..960 ","
                Reserved@960..965 "super"
                Comma@965..966 ","
                Reserved@966..972 "target"
                Comma@972..973 ","
                Reserved@973..981 "template"
                Comma@981..982 ","
                Reserved@982..986 "this"
                Comma@986..987 ","
                Reserved@987..999 "thread_local"
                Comma@999..1000 ","
                Reserved@1000..1005 "throw"
                Comma@1005..1006 ","
                Reserved@1006..1011 "trait"
                Comma@1011..1012 ","
                Reserved@1012..1015 "try"
                Comma@1015..1016 ","
                Reserved@1016..1020 "type"
                Comma@1020..1021 ","
                Reserved@1021..1028 "typedef"
                Comma@1028..1029 ","
                Reserved@1029..1035 "typeid"
                Comma@1035..1036 ","
                Reserved@1036..1044 "typename"
                Comma@1044..1045 ","
                Reserved@1045..1051 "typeof"
                Comma@1051..1052 ","
                Reserved@1052..1057 "union"
                Comma@1057..1058 ","
                Reserved@1058..1064 "unless"
                Comma@1064..1065 ","
                Reserved@1065..1070 "unorm"
                Comma@1070..1071 ","
                Reserved@1071..1077 "unsafe"
                Comma@1077..1078 ","
                Reserved@1078..1085 "unsized"
                Comma@1085..1086 ","
                Reserved@1086..1089 "use"
                Comma@1089..1090 ","
                Reserved@1090..1095 "using"
                Comma@1095..1096 ","
                Reserved@1096..1103 "varying"
                Comma@1103..1104 ","
                Reserved@1104..1111 "virtual"
                Comma@1111..1112 ","
                Reserved@1112..1120 "volatile"
                Comma@1120..1121 ","
                Reserved@1121..1125 "wgsl"
                Comma@1125..1126 ","
                Reserved@1126..1131 "where"
                Comma@1131..1132 ","
                Reserved@1132..1136 "with"
                Comma@1136..1137 ","
                Reserved@1137..1146 "writeonly"
                Comma@1146..1147 ","
                Reserved@1147..1152 "yield"

            error at 9..13: 'NULL' is a reserved word in WGSL
            error at 14..18: 'Self' is a reserved word in WGSL
            error at 19..27: 'abstract' is a reserved word in WGSL
            error at 28..34: 'active' is a reserved word in WGSL
            error at 35..42: 'alignas' is a reserved word in WGSL
            error at 43..50: 'alignof' is a reserved word in WGSL
            error at 51..53: 'as' is a reserved word in WGSL
            error at 50..53: switch to WESL to use `as`
            error at 54..57: 'asm' is a reserved word in WGSL
            error at 58..70: 'asm_fragment' is a reserved word in WGSL
            error at 71..76: 'async' is a reserved word in WGSL
            error at 77..86: 'attribute' is a reserved word in WGSL
            error at 87..91: 'auto' is a reserved word in WGSL
            error at 92..97: 'await' is a reserved word in WGSL
            error at 98..104: 'become' is a reserved word in WGSL
            error at 105..109: 'cast' is a reserved word in WGSL
            error at 110..115: 'catch' is a reserved word in WGSL
            error at 116..121: 'class' is a reserved word in WGSL
            error at 122..130: 'co_await' is a reserved word in WGSL
            error at 131..140: 'co_return' is a reserved word in WGSL
            error at 141..149: 'co_yield' is a reserved word in WGSL
            error at 150..158: 'coherent' is a reserved word in WGSL
            error at 159..171: 'column_major' is a reserved word in WGSL
            error at 172..178: 'common' is a reserved word in WGSL
            error at 179..186: 'compile' is a reserved word in WGSL
            error at 187..203: 'compile_fragment' is a reserved word in WGSL
            error at 204..211: 'concept' is a reserved word in WGSL
            error at 212..222: 'const_cast' is a reserved word in WGSL
            error at 223..232: 'consteval' is a reserved word in WGSL
            error at 233..242: 'constexpr' is a reserved word in WGSL
            error at 243..252: 'constinit' is a reserved word in WGSL
            error at 253..258: 'crate' is a reserved word in WGSL
            error at 259..267: 'debugger' is a reserved word in WGSL
            error at 268..276: 'decltype' is a reserved word in WGSL
            error at 277..283: 'delete' is a reserved word in WGSL
            error at 284..290: 'demote' is a reserved word in WGSL
            error at 291..307: 'demote_to_helper' is a reserved word in WGSL
            error at 308..310: 'do' is a reserved word in WGSL
            error at 311..323: 'dynamic_cast' is a reserved word in WGSL
            error at 324..328: 'enum' is a reserved word in WGSL
            error at 329..337: 'explicit' is a reserved word in WGSL
            error at 338..344: 'export' is a reserved word in WGSL
            error at 345..352: 'extends' is a reserved word in WGSL
            error at 353..359: 'extern' is a reserved word in WGSL
            error at 360..368: 'external' is a reserved word in WGSL
            error at 369..380: 'fallthrough' is a reserved word in WGSL
            error at 381..387: 'filter' is a reserved word in WGSL
            error at 388..393: 'final' is a reserved word in WGSL
            error at 394..401: 'finally' is a reserved word in WGSL
            error at 402..408: 'friend' is a reserved word in WGSL
            error at 409..413: 'from' is a reserved word in WGSL
            error at 414..421: 'fxgroup' is a reserved word in WGSL
            error at 422..425: 'get' is a reserved word in WGSL
            error at 426..430: 'goto' is a reserved word in WGSL
            error at 431..442: 'groupshared' is a reserved word in WGSL
            error at 443..448: 'highp' is a reserved word in WGSL
            error at 449..453: 'impl' is a reserved word in WGSL
            error at 454..464: 'implements' is a reserved word in WGSL
            error at 465..471: 'import' is a reserved word in WGSL
            error at 464..471: switch to WESL to use `import`
            error at 472..478: 'inline' is a reserved word in WGSL
            error at 479..489: 'instanceof' is a reserved word in WGSL
            error at 490..499: 'interface' is a reserved word in WGSL
            error at 500..506: 'layout' is a reserved word in WGSL
            error at 507..511: 'lowp' is a reserved word in WGSL
            error at 512..517: 'macro' is a reserved word in WGSL
            error at 518..529: 'macro_rules' is a reserved word in WGSL
            error at 530..535: 'match' is a reserved word in WGSL
            error at 536..543: 'mediump' is a reserved word in WGSL
            error at 544..548: 'meta' is a reserved word in WGSL
            error at 549..552: 'mod' is a reserved word in WGSL
            error at 553..559: 'module' is a reserved word in WGSL
            error at 560..564: 'move' is a reserved word in WGSL
            error at 565..568: 'mut' is a reserved word in WGSL
            error at 569..576: 'mutable' is a reserved word in WGSL
            error at 577..586: 'namespace' is a reserved word in WGSL
            error at 587..590: 'new' is a reserved word in WGSL
            error at 591..594: 'nil' is a reserved word in WGSL
            error at 595..603: 'noexcept' is a reserved word in WGSL
            error at 604..612: 'noinline' is a reserved word in WGSL
            error at 613..628: 'nointerpolation' is a reserved word in WGSL
            error at 629..641: 'non_coherent' is a reserved word in WGSL
            error at 642..653: 'noncoherent' is a reserved word in WGSL
            error at 654..667: 'noperspective' is a reserved word in WGSL
            error at 668..672: 'null' is a reserved word in WGSL
            error at 673..680: 'nullptr' is a reserved word in WGSL
            error at 681..683: 'of' is a reserved word in WGSL
            error at 684..692: 'operator' is a reserved word in WGSL
            error at 693..700: 'package' is a reserved word in WGSL
            error at 692..700: switch to WESL to use `package`
            error at 701..711: 'packoffset' is a reserved word in WGSL
            error at 712..721: 'partition' is a reserved word in WGSL
            error at 722..726: 'pass' is a reserved word in WGSL
            error at 727..732: 'patch' is a reserved word in WGSL
            error at 733..746: 'pixelfragment' is a reserved word in WGSL
            error at 747..754: 'precise' is a reserved word in WGSL
            error at 755..764: 'precision' is a reserved word in WGSL
            error at 765..773: 'premerge' is a reserved word in WGSL
            error at 774..778: 'priv' is a reserved word in WGSL
            error at 779..788: 'protected' is a reserved word in WGSL
            error at 789..792: 'pub' is a reserved word in WGSL
            error at 793..799: 'public' is a reserved word in WGSL
            error at 800..808: 'readonly' is a reserved word in WGSL
            error at 809..812: 'ref' is a reserved word in WGSL
            error at 813..823: 'regardless' is a reserved word in WGSL
            error at 824..832: 'register' is a reserved word in WGSL
            error at 833..849: 'reinterpret_cast' is a reserved word in WGSL
            error at 850..857: 'require' is a reserved word in WGSL
            error at 858..866: 'resource' is a reserved word in WGSL
            error at 867..875: 'restrict' is a reserved word in WGSL
            error at 876..880: 'self' is a reserved word in WGSL
            error at 881..884: 'set' is a reserved word in WGSL
            error at 885..891: 'shared' is a reserved word in WGSL
            error at 892..898: 'sizeof' is a reserved word in WGSL
            error at 899..905: 'smooth' is a reserved word in WGSL
            error at 906..911: 'snorm' is a reserved word in WGSL
            error at 912..918: 'static' is a reserved word in WGSL
            error at 919..932: 'static_assert' is a reserved word in WGSL
            error at 933..944: 'static_cast' is a reserved word in WGSL
            error at 945..948: 'std' is a reserved word in WGSL
            error at 949..959: 'subroutine' is a reserved word in WGSL
            error at 960..965: 'super' is a reserved word in WGSL
            error at 959..965: switch to WESL to use `super`
            error at 966..972: 'target' is a reserved word in WGSL
            error at 973..981: 'template' is a reserved word in WGSL
            error at 982..986: 'this' is a reserved word in WGSL
            error at 987..999: 'thread_local' is a reserved word in WGSL
            error at 1000..1005: 'throw' is a reserved word in WGSL
            error at 1006..1011: 'trait' is a reserved word in WGSL
            error at 1012..1015: 'try' is a reserved word in WGSL
            error at 1016..1020: 'type' is a reserved word in WGSL
            error at 1021..1028: 'typedef' is a reserved word in WGSL
            error at 1029..1035: 'typeid' is a reserved word in WGSL
            error at 1036..1044: 'typename' is a reserved word in WGSL
            error at 1045..1051: 'typeof' is a reserved word in WGSL
            error at 1052..1057: 'union' is a reserved word in WGSL
            error at 1058..1064: 'unless' is a reserved word in WGSL
            error at 1065..1070: 'unorm' is a reserved word in WGSL
            error at 1071..1077: 'unsafe' is a reserved word in WGSL
            error at 1078..1085: 'unsized' is a reserved word in WGSL
            error at 1086..1089: 'use' is a reserved word in WGSL
            error at 1090..1095: 'using' is a reserved word in WGSL
            error at 1096..1103: 'varying' is a reserved word in WGSL
            error at 1104..1111: 'virtual' is a reserved word in WGSL
            error at 1112..1120: 'volatile' is a reserved word in WGSL
            error at 1121..1125: 'wgsl' is a reserved word in WGSL
            error at 1126..1131: 'where' is a reserved word in WGSL
            error at 1132..1136: 'with' is a reserved word in WGSL
            error at 1137..1146: 'writeonly' is a reserved word in WGSL
            error at 1147..1152: 'yield' is a reserved word in WGSL
            error at 9..13: invalid syntax, expected one of: 'alias', '@', 'const', 'const_assert', 'diagnostic', <end of file>, 'enable', 'fn', 'import', 'let', 'override', 'requires', ';', 'struct', 'var'"#]],
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

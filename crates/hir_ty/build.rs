use std::{collections::BTreeMap, fs::File, path::PathBuf, str::FromStr};

#[derive(Default, Debug)]
struct Builtin {
    overloads: Vec<Overload>,
}
#[derive(Debug)]
enum Generic {
    VecSize,
    Type,
    TexelFormat,
}
#[derive(Debug)]
struct Overload {
    generics: BTreeMap<char, (usize, Generic)>,
    return_type: Option<Type>,
    parameters: Vec<(Type, Option<String>)>,
}
#[derive(Debug)]
enum Type {
    Vec(VecSize, Box<Type>),
    Matrix(VecSize, VecSize, Box<Type>),
    Texture(TextureType),
    Sampler { comparison: bool },
    Bool,
    F32,
    I32,
    U32,
    RuntimeArray(Box<Type>),
    Ptr(Box<Type>),
    Atomic(Box<Type>),
    Bound(usize),
    StorageTypeOfTexelFormat(usize),
}
enum VecSize {
    Two,
    Three,
    Four,
    Bound(usize),
}

impl std::fmt::Debug for VecSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Two => write!(f, "Two"),
            Self::Three => write!(f, "Three"),
            Self::Four => write!(f, "Four"),
            Self::Bound(var) => write!(f, "BoundVar(BoundVar {{ index: {} }})", var),
        }
    }
}

#[derive(Debug)]
struct TextureType {
    pub dimension: TextureDimensionality,
    pub arrayed: bool,
    pub multisampled: bool,
    pub kind: TextureKind,
}

#[derive(Debug)]
enum TexelFormat {
    Any,
    Bound(usize),
}
#[derive(Debug)]
enum AccessMode {
    ReadWrite,
    Read,
    Write,
    Any,
}
impl FromStr for AccessMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "read_write" => AccessMode::ReadWrite,
            "read" => AccessMode::Read,
            "write" => AccessMode::Write,
            "_" => AccessMode::Any,
            _ => return Err(()),
        })
    }
}

#[derive(Debug)]
enum TextureKind {
    Sampled(Box<Type>),
    #[allow(unused)]
    Storage(TexelFormat, AccessMode),
    Depth,
    External,
}

#[derive(Debug)]
enum TextureDimensionality {
    D1,
    D2,
    D3,
    Cube,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=builtins.wgsl");

    let dir = PathBuf::from(std::env::var("OUT_DIR")?).join("generated");
    std::fs::create_dir_all(&dir)?;
    let path = dir.join("builtins.rs");
    let mut file = File::create(path)?;

    let mut builtins: BTreeMap<String, Builtin> = BTreeMap::new();

    let builtins_file = std::fs::read_to_string("builtins.wgsl")?;
    for line in builtins_file.lines() {
        if line.is_empty() || line.starts_with("//") {
            continue;
        }
        let (name, overload) = parse_line(line);
        let builtin = builtins.entry(name.to_string()).or_default();
        builtin.overloads.push(overload);
    }

    // panic!("{:#?}", builtins);

    for (name, builtin) in &builtins {
        builtin_to_rust(&mut file, name, builtin)?;
    }
    foo(&mut file, &builtins)?;

    Ok(())
}

fn foo(f: &mut dyn std::io::Write, builtins: &BTreeMap<String, Builtin>) -> std::io::Result<()> {
    write!(
        f,
        r#"
impl Builtin {{
    pub fn for_name(db: &dyn HirDatabase, name: &Name) -> Option<Builtin> {{
        match name.as_str() {{"#
    )?;

    for name in builtins.keys() {
        if name.starts_with("op") {
            continue;
        }

        write!(
            f,
            r#""{name}" => Some(Builtin::builtin_{name}(db)),"#,
            name = name
        )?;
    }
    write!(
        f,
        r#"_ => None,
        }}
    }}
}}
    "#
    )?;

    write!(
        f,
        "impl Builtin {{
    pub const ALL_BUILTINS: &'static [&'static str] = &["
    )?;

    for name in builtins.keys() {
        if name.starts_with("op") {
            continue;
        }
        write!(f, r#""{}", "#, name)?;
    }

    write!(f, "    ];\n}}")?;

    Ok(())
}

fn parse_line(line: &str) -> (&str, Overload) {
    let (name, line) = line.split_once('(').unwrap();
    let (parameters, line) = line.split_once(')').unwrap();
    let return_type = line.trim_start_matches(" ->").trim();

    let mut generics = BTreeMap::<char, (usize, Generic)>::default();
    let parameters: Vec<_> = parameters
        .split(',')
        .filter(|param| !param.is_empty())
        .map(|ty| match ty.find(":") {
            Some(idx) if !ty[idx..].starts_with("::") => {
                let (name, ty) = ty.split_at(idx);
                let ty = &ty[1..];
                let name = name.trim();
                let name = (!name.is_empty()).then(|| name.to_string());
                (parse_ty(&mut generics, ty.trim()), name)
            }
            _ => (parse_ty(&mut generics, ty.trim()), None),
        })
        .collect();

    let return_type = match return_type {
        "" => None,
        _ => Some(parse_ty(&mut generics, return_type.trim())),
    };

    (
        name,
        Overload {
            generics,
            return_type,
            parameters,
        },
    )
}

fn parse_generic(ty: &str) -> Option<(&str, &str)> {
    let (ty, rest) = ty.split_once('<')?;
    let inner = rest.strip_suffix('>')?;
    Some((ty, inner))
}

fn parse_vec_size(generics: &mut BTreeMap<char, (usize, Generic)>, size: char) -> VecSize {
    match size {
        '2' => VecSize::Two,
        '3' => VecSize::Three,
        '4' => VecSize::Four,
        other => {
            let len = generics.len();
            let (i, _) = generics.entry(other).or_insert((len, Generic::VecSize));
            VecSize::Bound(*i)
        }
    }
}

fn parse_texel_format(
    generics: &mut BTreeMap<char, (usize, Generic)>,
    format: char,
) -> TexelFormat {
    match format {
        '_' => TexelFormat::Any,
        other => {
            let len = generics.len();
            let (i, _) = generics.entry(other).or_insert((len, Generic::TexelFormat));
            TexelFormat::Bound(*i)
        }
    }
}

fn only_char(input: &str) -> char {
    let mut chars = input.chars();
    let val = chars.next().unwrap();
    assert!(chars.next().is_none());

    val
}

fn parse_ty(generics: &mut BTreeMap<char, (usize, Generic)>, ty: &str) -> Type {
    if let Some((ty, inner)) = parse_generic(ty) {
        if let Some(size) = ty.strip_prefix("vec") {
            let size = only_char(size);

            let size = parse_vec_size(generics, size);
            let inner = parse_ty(generics, inner);
            return Type::Vec(size, Box::new(inner));
        } else if let Some(texture) = ty.strip_prefix("texture_storage_") {
            let (format, mode) = inner.split_once(';').unwrap();
            let format = parse_texel_format(generics, only_char(format));
            let mode = mode.parse().unwrap();

            #[rustfmt::skip]
            let texture_type = match texture {
                "1d" => TextureType { dimension: TextureDimensionality::D1, arrayed: false, multisampled: false, kind: TextureKind::Storage(format, mode) },
                "2d" => TextureType { dimension: TextureDimensionality::D2, arrayed: false, multisampled: false, kind: TextureKind::Storage(format, mode) },
                "2d_array" => TextureType { dimension: TextureDimensionality::D2, arrayed: true, multisampled: false, kind: TextureKind::Storage(format, mode) },
                "3d" => TextureType { dimension: TextureDimensionality::D3, arrayed: false, multisampled: false, kind: TextureKind::Storage(format, mode) },
                _ => unimplemented!("{}", ty),
            };
            return Type::Texture(texture_type);
        } else if let Some(texture) = ty.strip_prefix("texture_") {
            let inner = parse_ty(generics, inner);
            #[rustfmt::skip]
            let texture_type = match texture {
                "1d" => TextureType { dimension: TextureDimensionality::D1, arrayed: false, multisampled: false, kind: TextureKind::Sampled(Box::new(inner)) },
                "2d" => TextureType { dimension: TextureDimensionality::D2, arrayed: false, multisampled: false, kind: TextureKind::Sampled(Box::new(inner)) },
                "2d_array" => TextureType { dimension: TextureDimensionality::D2, arrayed: true, multisampled: false, kind: TextureKind::Sampled(Box::new(inner)) },
                "3d" => TextureType { dimension: TextureDimensionality::D3, arrayed: false, multisampled: false, kind: TextureKind::Sampled(Box::new(inner)) },
                "cube" => TextureType { dimension: TextureDimensionality::Cube, arrayed: false, multisampled: false, kind: TextureKind::Sampled(Box::new(inner)) },
                "cube_array" => TextureType { dimension: TextureDimensionality::Cube, arrayed: true, multisampled: false, kind: TextureKind::Sampled(Box::new(inner)) },
                "multisampled_2d" => TextureType { dimension: TextureDimensionality::D2, arrayed: false, multisampled: true, kind: TextureKind::Sampled(Box::new(inner)) },
                _ => unimplemented!("{}", ty),
            };
            return Type::Texture(texture_type);
        } else if let Some(size) = ty.strip_prefix("mat") {
            let mut chars = size.chars();
            let columns = chars.next().unwrap();
            assert!(chars.next().unwrap() == 'x');
            let rows = chars.next().unwrap();
            assert!(chars.next().is_none());

            let columns = parse_vec_size(generics, columns);
            let rows = parse_vec_size(generics, rows);

            let inner = parse_ty(generics, inner);
            return Type::Matrix(columns, rows, Box::new(inner));
        } else if ty == "array" {
            let inner = parse_ty(generics, inner);
            return Type::RuntimeArray(Box::new(inner));
        } else if ty == "ptr" {
            let inner = parse_ty(generics, inner);
            return Type::Ptr(Box::new(inner));
        } else if ty == "atomic" {
            let inner = parse_ty(generics, inner);
            return Type::Atomic(Box::new(inner));
        } else {
            unimplemented!("{}", ty);
        }
    }

    if let Some(texture) = ty.strip_prefix("texture_") {
        #[rustfmt::skip]
        let texture_type = match texture {
            "depth_2d" => TextureType { dimension: TextureDimensionality::D2, arrayed: false, multisampled: false, kind: TextureKind::Depth },
            "depth_2d_array" => TextureType { dimension: TextureDimensionality::D2, arrayed: true, multisampled: false, kind: TextureKind::Depth },
            "depth_cube" => TextureType { dimension: TextureDimensionality::Cube, arrayed: false, multisampled: false, kind: TextureKind::Depth },
            "depth_cube_array" => TextureType { dimension: TextureDimensionality::Cube, arrayed: true, multisampled: false, kind: TextureKind::Depth },
            "depth_multisampled_2d" => TextureType { dimension: TextureDimensionality::D2, arrayed: false, multisampled: true, kind: TextureKind::Depth },
            "external" => TextureType { dimension: TextureDimensionality::D2, arrayed: false, multisampled: false, kind: TextureKind::External },
            _ => unimplemented!("{}", ty),
        };
        return Type::Texture(texture_type);
    }

    if ty.len() == 1 {
        let generic = ty.chars().next().unwrap();
        let len = generics.len();
        let (i, _) = generics.entry(generic).or_insert((len, Generic::Type));
        return Type::Bound(*i);
    }

    match ty {
        "bool" => Type::Bool,
        "f32" => Type::F32,
        "i32" => Type::I32,
        "u32" => Type::U32,
        "sampler" => Type::Sampler { comparison: false },
        "sampler_comparison" => Type::Sampler { comparison: true },
        "F::StorageType" => {
            let var = generics.get(&'F').unwrap().0;
            Type::StorageTypeOfTexelFormat(var)
        }
        other => unimplemented!("{}", other),
    }
}

fn type_to_rust(ty: &Type) -> String {
    match ty {
        Type::Vec(size, inner) => format!(
            "TyKind::Vector(VectorType {{ size: VecSize::{:?}, inner: {} }}).intern(db)",
            size,
            type_to_rust(inner)
        ),

        Type::Matrix(columns, rows, inner) => format!(
            "TyKind::Matrix(MatrixType {{ columns: VecSize::{:?}, rows: VecSize::{:?}, inner: {} }}).intern(db)",
            columns, rows,
            type_to_rust(inner)
        ),

        ty @ (Type::Bool | Type::F32 | Type::I32 | Type::U32) => {
            format!("TyKind::Scalar(ScalarType::{:?}).intern(db)", ty)
        }
        Type::Bound(i) => {
            format!("TyKind::BoundVar(BoundVar {{ index: {} }}).intern(db)", i,)
        }
        Type::Texture(texture) => {
            format!(
                "TyKind::Texture(TextureType {{
                            kind: TextureKind::{},
                            arrayed: {},
                            multisampled: {},
                            dimension: TextureDimensionality::{:?},
                        }}).intern(db)",
                match &texture.kind {
                    TextureKind::Sampled(inner) => format!("Sampled({})", type_to_rust(inner)),
                    TextureKind::Storage(texel_format, access_mode) => {
                        let texel_format = match texel_format {
                            TexelFormat::Any => "Any".to_string(),
                            TexelFormat::Bound(var) => format!("BoundVar(BoundVar {{ index: {} }})",var),
                        };

                        format!("Storage(TexelFormat::{}, AccessMode::{:?})", texel_format, access_mode)
                    },
                    TextureKind::Depth => "Depth".to_string(),
                    TextureKind::External => "External".to_string(),
                },
                texture.arrayed,
                texture.multisampled,
                texture.dimension,
            )
        }
        Type::Sampler { comparison } => format!(
            "TyKind::Sampler(SamplerType {{ comparison: {}  }}).intern(db)",
            comparison
        ),
        Type::RuntimeArray(inner) => format!("TyKind::Array(ArrayType {{
            size: ArraySize::Dynamic,
            binding_array: false,
            inner: {}
        }}).intern(db)", type_to_rust(inner)),
        Type::Ptr(inner) => format!("TyKind::Ptr(Ptr {{
            inner: {},
            access_mode: AccessMode::ReadWrite,
            storage_class: StorageClass::Private,
        }}).intern(db)", type_to_rust(inner)),
        Type::Atomic(inner) => format!("TyKind::Atomic(AtomicType {{
            inner: {},
        }}).intern(db)", type_to_rust(inner)),
        Type::StorageTypeOfTexelFormat(var) => format!("TyKind::StorageTypeOfTexelFormat(BoundVar {{ index: {} }}).intern(db)", var),
    }
}
fn builtin_to_rust(
    f: &mut dyn std::io::Write,
    name: &str,
    builtin: &Builtin,
) -> std::io::Result<()> {
    write!(
        f,
        r#"impl Builtin {{
    #[allow(non_snake_case)]
    pub fn builtin_{name}(db: &dyn HirDatabase) -> Self {{
        let name = Name::from("{name}");
        let overloads = vec!["#,
        name = name
    )?;

    for overload in &builtin.overloads {
        write!(
            f,
            "
            BuiltinOverload {{
                generics: vec![{generics}],
                ty: FunctionDetails {{
                    return_type: {return_ty},
                    parameters: vec![",
            return_ty = overload
                .return_type
                .as_ref()
                .map(|ty| format!("Some({})", type_to_rust(ty)))
                .unwrap_or_else(|| "None".to_string()),
            generics = overload
                .generics
                .values()
                .map(|val| match val.1 {
                    Generic::VecSize => "GenericArgKind::VecSize, ",
                    Generic::Type => "GenericArgKind::Type, ",
                    Generic::TexelFormat => "GenericArgKind::TexelFormat, ",
                })
                .collect::<String>()
        )?;
        for (param, name) in &overload.parameters {
            write!(
                f,
                "
                        ({ty}, {name}),",
                ty = type_to_rust(param),
                name = match name {
                    Some(name) => format!("Name::from({name:?})"),
                    None => "Name::missing()".to_string(),
                }
            )?;
        }
        write!(
            f,
            r#"
                    ],
                }}
                .intern(db),
            }},"#,
        )?;
    }

    write!(
        f,
        r#"
        ];
        Builtin {{ name, overloads }}
    }}
}}
"#,
    )
}

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
    F16,
    F32,
    I32,
    U32,
    RuntimeArray(Box<Type>),
    Pointer(Box<Type>),
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
    fn fmt(
        &self,
        #[expect(clippy::min_ident_chars, reason = "trait impl")] f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::Two => write!(f, "Two"),
            Self::Three => write!(f, "Three"),
            Self::Four => write!(f, "Four"),
            Self::Bound(var) => write!(f, "BoundVar(BoundVar {{ index: {var} }})"),
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

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        Ok(match string {
            "read_write" => Self::ReadWrite,
            "read" => Self::Read,
            "write" => Self::Write,
            "_" => Self::Any,
            _ => return Err(()),
        })
    }
}

#[derive(Debug)]
enum TextureKind {
    Sampled(Box<Type>),
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

    let directory = PathBuf::from(std::env::var("OUT_DIR")?).join("generated");
    std::fs::create_dir_all(&directory)?;
    let path = directory.join("builtins.rs");
    let mut file = File::create(path)?;

    let mut builtins: BTreeMap<String, Builtin> = BTreeMap::new();

    let builtins_file = std::fs::read_to_string("builtins.wgsl")?;
    for line in builtins_file.lines() {
        if line.is_empty() || line.starts_with("//") {
            continue;
        }
        let (name, overload) = parse_line(line);
        let builtin = builtins.entry(name.to_owned()).or_default();
        builtin.overloads.push(overload);
    }

    // panic!("{:#?}", builtins);

    for (name, builtin) in &builtins {
        builtin_to_rust(&mut file, name, builtin)?;
    }
    foo(&mut file, &builtins)?;

    Ok(())
}

fn foo(
    destination: &mut dyn std::io::Write,
    builtins: &BTreeMap<String, Builtin>,
) -> std::io::Result<()> {
    write!(
        destination,
        "
impl Builtin {{
    pub fn for_name(database: &dyn HirDatabase, name: &Name) -> Option<Builtin> {{
        match name.as_str() {{"
    )?;

    for name in builtins.keys() {
        if name.starts_with("op") {
            continue;
        }

        write!(
            destination,
            r#""{name}" => Some(Builtin::builtin_{name}(database)),"#
        )?;
    }
    write!(
        destination,
        "_ => None,
        }}
    }}
}}
    "
    )?;

    write!(
        destination,
        "impl Builtin {{
    pub const ALL_BUILTINS: &'static [&'static str] = &["
    )?;

    for name in builtins.keys() {
        if name.starts_with("op") {
            continue;
        }
        write!(destination, r#""{name}", "#)?;
    }

    write!(destination, "    ];\n}}")?;

    Ok(())
}

fn parse_line(line: &str) -> (&str, Overload) {
    let (name, line) = line.split_once('(').unwrap();
    let (parameters, line) = line.split_once(')').unwrap();
    let return_type = line.trim_start_matches(" ->").trim();

    let mut generics = BTreeMap::<char, (usize, Generic)>::default();
    let parameters: Vec<_> = parameters
        .split(',')
        .filter(|parameter| !parameter.is_empty())
        .map(|r#type| match r#type.find(':') {
            Some(index) if !r#type[index..].starts_with("::") => {
                let (name, r#type) = r#type.split_at(index);
                let r#type = &r#type[1..];
                let name = name.trim();
                let name = (!name.is_empty()).then(|| name.to_owned());
                (parse_ty(&mut generics, r#type.trim()), name)
            },
            _ => (parse_ty(&mut generics, r#type.trim()), None),
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

fn parse_generic(r#type: &str) -> Option<(&str, &str)> {
    let (r#type, rest) = r#type.split_once('<')?;
    let inner = rest.strip_suffix('>')?;
    Some((r#type, inner))
}

fn parse_vec_size(
    generics: &mut BTreeMap<char, (usize, Generic)>,
    size: char,
) -> VecSize {
    match size {
        '2' => VecSize::Two,
        '3' => VecSize::Three,
        '4' => VecSize::Four,
        other => {
            let length = generics.len();
            let (i, _) = generics.entry(other).or_insert((length, Generic::VecSize));
            VecSize::Bound(*i)
        },
    }
}

fn parse_texel_format(
    generics: &mut BTreeMap<char, (usize, Generic)>,
    format: char,
) -> TexelFormat {
    match format {
        '_' => TexelFormat::Any,
        other => {
            let length = generics.len();
            let (i, _) = generics
                .entry(other)
                .or_insert((length, Generic::TexelFormat));
            TexelFormat::Bound(*i)
        },
    }
}

fn only_char(input: &str) -> char {
    let mut characters = input.chars();
    let value = characters.next().unwrap();
    assert!(characters.next().is_none());

    value
}

#[expect(clippy::unimplemented, reason = "TODO")]
fn parse_ty(
    generics: &mut BTreeMap<char, (usize, Generic)>,
    r#type: &str,
) -> Type {
    if let Some((r#type, inner)) = parse_generic(r#type) {
        if let Some(size) = r#type.strip_prefix("vec") {
            let size = only_char(size);

            let size = parse_vec_size(generics, size);
            let inner = parse_ty(generics, inner);
            return Type::Vec(size, Box::new(inner));
        } else if let Some(texture) = r#type.strip_prefix("texture_storage_") {
            let (format, mode) = inner.split_once(';').unwrap();
            let format = parse_texel_format(generics, only_char(format));
            let mode = mode.parse().unwrap();

            #[rustfmt::skip]
            let texture_type = match texture {
                "1d" => TextureType { dimension: TextureDimensionality::D1, arrayed: false, multisampled: false, kind: TextureKind::Storage(format, mode) },
                "2d" => TextureType { dimension: TextureDimensionality::D2, arrayed: false, multisampled: false, kind: TextureKind::Storage(format, mode) },
                "2d_array" => TextureType { dimension: TextureDimensionality::D2, arrayed: true, multisampled: false, kind: TextureKind::Storage(format, mode) },
                "3d" => TextureType { dimension: TextureDimensionality::D3, arrayed: false, multisampled: false, kind: TextureKind::Storage(format, mode) },
                _ => unimplemented!("{}", r#type),
            };
            return Type::Texture(texture_type);
        } else if let Some(texture) = r#type.strip_prefix("texture_") {
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
                _ => unimplemented!("{}", r#type),
            };
            return Type::Texture(texture_type);
        } else if let Some(size) = r#type.strip_prefix("mat") {
            let mut characters = size.chars();
            let columns = characters.next().unwrap();
            assert!(characters.next().unwrap() == 'x');
            let rows = characters.next().unwrap();
            assert!(characters.next().is_none());

            let columns = parse_vec_size(generics, columns);
            let rows = parse_vec_size(generics, rows);

            let inner = parse_ty(generics, inner);
            return Type::Matrix(columns, rows, Box::new(inner));
        } else if r#type == "array" {
            let inner = parse_ty(generics, inner);
            return Type::RuntimeArray(Box::new(inner));
        } else if r#type == "ptr" {
            let inner = parse_ty(generics, inner);
            return Type::Pointer(Box::new(inner));
        } else if r#type == "atomic" {
            let inner = parse_ty(generics, inner);
            return Type::Atomic(Box::new(inner));
        }
        unimplemented!("{}", r#type);
    }

    if let Some(texture) = r#type.strip_prefix("texture_") {
        #[rustfmt::skip]
        let texture_type = match texture {
            "depth_2d" => TextureType { dimension: TextureDimensionality::D2, arrayed: false, multisampled: false, kind: TextureKind::Depth },
            "depth_2d_array" => TextureType { dimension: TextureDimensionality::D2, arrayed: true, multisampled: false, kind: TextureKind::Depth },
            "depth_cube" => TextureType { dimension: TextureDimensionality::Cube, arrayed: false, multisampled: false, kind: TextureKind::Depth },
            "depth_cube_array" => TextureType { dimension: TextureDimensionality::Cube, arrayed: true, multisampled: false, kind: TextureKind::Depth },
            "depth_multisampled_2d" => TextureType { dimension: TextureDimensionality::D2, arrayed: false, multisampled: true, kind: TextureKind::Depth },
            "external" => TextureType { dimension: TextureDimensionality::D2, arrayed: false, multisampled: false, kind: TextureKind::External },
            _ => unimplemented!("{}", r#type),
        };
        return Type::Texture(texture_type);
    }

    if r#type.len() == 1 {
        let generic = r#type.chars().next().unwrap();
        let length = generics.len();
        let (i, _) = generics.entry(generic).or_insert((length, Generic::Type));
        return Type::Bound(*i);
    }

    match r#type {
        "bool" => Type::Bool,
        "f16" => Type::F16,
        "f32" => Type::F32,
        "i32" => Type::I32,
        "u32" => Type::U32,
        "sampler" => Type::Sampler { comparison: false },
        "sampler_comparison" => Type::Sampler { comparison: true },
        "F::StorageType" => {
            let var = generics.get(&'F').unwrap().0;
            Type::StorageTypeOfTexelFormat(var)
        },
        other => unimplemented!("{}", other),
    }
}

fn type_to_rust(r#type: &Type) -> String {
    match r#type {
        Type::Vec(size, component_type) => format!(
            "TyKind::Vector(crate::ty::VectorType {{ size: VecSize::{:?}, component_type: {} }}).intern(database)",
            size,
            type_to_rust(component_type)
        ),

        Type::Matrix(columns, rows, inner) => format!(
            "TyKind::Matrix(crate::ty::MatrixType {{ columns: VecSize::{:?}, rows: VecSize::{:?}, inner: {} }}).intern(database)",
            columns,
            rows,
            type_to_rust(inner)
        ),

        (Type::Bool | Type::F32 | Type::I32 | Type::U32 | Type::F16) => {
            format!("TyKind::Scalar(ScalarType::{type:?}).intern(database)")
        },
        Type::Bound(i) => {
            format!("TyKind::BoundVar(BoundVar {{ index: {i} }}).intern(database)",)
        },
        Type::Texture(texture) => {
            format!(
                "TyKind::Texture(TextureType {{
                            kind: TextureKind::{},
                            arrayed: {},
                            multisampled: {},
                            dimension: TextureDimensionality::{:?},
                        }}).intern(database)",
                match &texture.kind {
                    TextureKind::Sampled(inner) => format!("Sampled({})", type_to_rust(inner)),
                    TextureKind::Storage(texel_format, access_mode) => {
                        let texel_format = match texel_format {
                            TexelFormat::Any => "Any".to_owned(),
                            TexelFormat::Bound(var) => {
                                format!("BoundVar(BoundVar {{ index: {var} }})")
                            },
                        };

                        format!("Storage(TexelFormat::{texel_format}, AccessMode::{access_mode:?})")
                    },
                    TextureKind::Depth => "Depth".to_owned(),
                    TextureKind::External => "External".to_owned(),
                },
                texture.arrayed,
                texture.multisampled,
                texture.dimension,
            )
        },
        Type::Sampler { comparison } => {
            format!("TyKind::Sampler(SamplerType {{ comparison: {comparison}  }}).intern(database)")
        },
        Type::RuntimeArray(inner) => format!(
            "TyKind::Array(ArrayType {{
            size: ArraySize::Dynamic,
            binding_array: false,
            inner: {}
        }}).intern(database)",
            type_to_rust(inner)
        ),
        Type::Pointer(inner) => format!(
            "TyKind::Pointer(Pointer {{
            inner: {},
            access_mode: AccessMode::ReadWrite,
            address_space: AddressSpace::Private,
        }}).intern(database)",
            type_to_rust(inner)
        ),
        Type::Atomic(inner) => format!(
            "TyKind::Atomic(AtomicType {{
            inner: {},
        }}).intern(database)",
            type_to_rust(inner)
        ),
        Type::StorageTypeOfTexelFormat(var) => {
            format!(
                "TyKind::StorageTypeOfTexelFormat(BoundVar {{ index: {var} }}).intern(database)"
            )
        },
    }
}

fn builtin_to_rust(
    sink: &mut dyn std::io::Write,
    name: &str,
    builtin: &Builtin,
) -> std::io::Result<()> {
    write!(
        sink,
        r#"impl Builtin {{
    #[allow(non_snake_case)]
    pub fn builtin_{name}(database: &dyn HirDatabase) -> Self {{
        let name = Name::from("{name}");
        let overloads = vec!["#
    )?;

    for overload in &builtin.overloads {
        write!(
            sink,
            "
            BuiltinOverload {{
                generics: vec![{generics}],
                r#type: FunctionDetails {{
                    return_type: {return_ty},
                    parameters: vec![",
            return_ty = overload.return_type.as_ref().map_or_else(
                || "None".to_owned(),
                |r#type| format!("Some({})", type_to_rust(r#type))
            ),
            generics = overload
                .generics
                .values()
                .map(|value| match value.1 {
                    Generic::VecSize => "GenericArgKind::VecSize, ",
                    Generic::Type => "GenericArgKind::Type, ",
                    Generic::TexelFormat => "GenericArgKind::TexelFormat, ",
                })
                .collect::<String>()
        )?;
        for (parameter, name) in &overload.parameters {
            write!(
                sink,
                "
                        ({ty}, {name}),",
                ty = type_to_rust(parameter),
                name = match name {
                    Some(name) => format!("Name::from({name:?})"),
                    None => "Name::missing()".to_owned(),
                }
            )?;
        }
        write!(
            sink,
            "
                    ],
                }}
                .intern(database),
            }},",
        )?;
    }

    write!(
        sink,
        "
        ];
        Builtin {{ name, overloads }}
    }}
}}
",
    )
}

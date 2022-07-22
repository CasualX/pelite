/*!
Datamaps manage entity state.
*/

#![allow(bad_style)]

use pelite;
use pelite::pattern as pat;
use pelite::pe32::*;
use pelite::{util::CStr, Pod};

//----------------------------------------------------------------

pub fn print(client: PeFile) {
    let datamaps = datamaps(client).unwrap();

    tprint! {
        "### Datamaps\n\n"
        for class in (&datamaps) {
            "<details>\n"
            "<summary><code>class "{class.name}
            if let Some(base) = (class.base) {
                " extends "{base}
            }
            "</code></summary>\n\n"
            "```\n"
            "{{\n"
            for field in (&class.fields) {
                "\t"{field.name}": "{field.ty}",\n"
            }
            "}}\n"
            "```\n\n"
            "#### Offsets\n\n"
            "```\n"
            for field in (&class.fields) {
                {class.name}"!"{field.offset;#06x}" "{field.name}"\n"
            }
            "```\n"
            "</details>\n"
        }
        "\n"
    }
}

//----------------------------------------------------------------

const TD_OFFSET_COUNT: usize = 2;

#[derive(Pod, Debug)]
#[repr(C)]
struct typedescription_t {
    fieldType: i32,
    fieldName: Ptr<CStr>,
    fieldOffset: [i32; TD_OFFSET_COUNT],
    fieldSize: u16,
    flags: i16,
    externalName: Ptr<CStr>,
    pSaveRestoreOps: Va,
    td: Ptr<datamap_t>,
    fieldSizeInBytes: i32,
    override_field: Ptr<typedescription_t>,
    override_count: i32,
    fieldTolerance: f32,
    unk: [i32; 3],
}
// sizeof(typedescription_t) == 60

#[derive(Pod, Debug)]
#[repr(C)]
struct datamap_t {
    dataDesc: Ptr<[typedescription_t]>,
    dataNumFields: i32,
    dataClassName: Ptr<CStr>,
    baseMap: Ptr<datamap_t>,
    chains_validated: u8,        //bool
    packed_offsets_computed: u8, //bool
    _pad: [u8; 2],
    packed_size: i32,
}

static FIELD_TYPES: [&str; 29] = [
    "Void",
    "Float",
    "String",
    "Vector",
    "Quaternion",
    "Integer",
    "Boolean",
    "Short",
    "Char",
    "Color32",
    "Embedded",
    "Custom",
    "ClassPtr",
    "EHandle",
    "EDict",
    "PositionVector",
    "Time",
    "Tick",
    "ModelName",
    "SoundName",
    "Input",
    "Function",
    "VMatrix",
    "VMatrixWorldSpace",
    "Matrix3x4WorldSpace",
    "Interval",
    "ModelIndex",
    "MaterialIndex",
    "Vector2D",
];

//----------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Class<'a> {
    pub name: &'a str,
    pub base: Option<&'a str>,
    pub fields: Vec<Field<'a>>,
}
#[derive(Copy, Clone, Debug)]
pub struct Field<'a> {
    pub ty: &'a str,
    pub name: &'a str,
    pub offset: i32,
}

pub fn datamaps<'a>(client: PeFile<'a>) -> pelite::Result<Vec<Class<'a>>> {
    let mut save = [0; 4];
    let mut classes = Vec::new();
    // The datamaps aren't fully constructed yet, find these constructors
    // ```
    // mov s_DataMap.dataNumFields, sizeof(s_TypeDescriptors) / sizeof(typedescription_t)
    // mov s_DataMap.dataDesc, offset s_TypeDescriptors
    // ret
    // .align 16
    // ```
    let mut matches = client.scanner().matches_code(pat!(
        "@4 C705????'???? C705*{'}*{'} C3 CCCCCCCCCCCCCCCCCCCCCC"
    ));
    while matches.next(&mut save) {
        let num = client.derva_copy::<i32>(save[1]).unwrap();
        let datamap = client.derva::<datamap_t>(save[2]);
        let tydescs = client.derva_slice::<typedescription_t>(save[3], num as usize);

        // The match isn't perfect and includes some false positives, skip these
        if let (Ok(datamap), Ok(tydescs)) = (datamap, tydescs) {
            // Ignore further false positives
            if let Ok(class) = dataclass(client, datamap, tydescs) {
                classes.push(class);
            }
        }
    }
    classes.sort_unstable_by_key(|cls| cls.name);
    Ok(classes)
}

fn dataclass<'a>(
    client: PeFile<'a>,
    datamap: &datamap_t,
    tydescs: &[typedescription_t],
) -> pelite::Result<Class<'a>> {
    let mut fields = Vec::new();
    for tydesc in tydescs {
        if let Ok(field_name) = client.deref_c_str(tydesc.fieldName) {
            let ty = client
                .deref(tydesc.td)
                .and_then(|td| client.deref_c_str(td.dataClassName))
                .map(|name| name.to_str().unwrap())
                .unwrap_or(*FIELD_TYPES.get(tydesc.fieldType as usize).unwrap_or(&"?"));
            fields.push(Field {
                name: field_name.to_str().unwrap(),
                ty,
                offset: tydesc.fieldOffset[0],
            });
        }
    }
    fields.sort_by_key(|field| field.offset);

    let class_name = client.deref_c_str(datamap.dataClassName)?.to_str().unwrap();
    let base_class = client
        .deref(datamap.baseMap)
        .and_then(|basemap| Ok(client.deref_c_str(basemap.dataClassName)?.to_str().unwrap()))
        .ok();

    Ok(Class {
        name: class_name,
        base: base_class,
        fields,
    })
}

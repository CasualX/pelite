use std::collections::BTreeMap;

// ============================================================================
// Core address types
// ============================================================================

/// Virtual address inside the loaded image.
///
/// This is deliberately distinct from file offsets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Address(pub u64);

/// Half-open image range `[start, end)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AddressRange {
	pub start: Address,
	pub end: Address,
}

impl AddressRange {
	pub fn len(self) -> u64 {
		self.end.0 - self.start.0
	}

	pub fn is_empty(self) -> bool {
		self.start == self.end
	}

	pub fn contains(self, address: Address) -> bool {
		self.start <= address && address < self.end
	}
}


// ============================================================================
// Database
// ============================================================================

/// Complete persistent analysis database.
///
/// Layers are ordered from lowest priority to highest priority.
///
/// Individual layers may contain any combination of facts. A loader may
/// populate symbols and types, a disassembler may primarily populate
/// instruction starts and xrefs, and the user's working layer may contain
/// edits spanning all of them.
///
/// Layers can be reordered, removed, edited, or merged.
#[derive(Debug, Clone)]
pub struct Database {
	/// Identity/information about the binary this database describes.
	pub image: ImageInfo,

	/// Ordered analysis layers, lowest priority first.
	pub layers: Vec<Layer>,
}


// ============================================================================
// Image
// ============================================================================

/// Information required to associate the database with an input image.
///
/// The actual binary bytes do not necessarily have to be embedded in the
/// database file.
#[derive(Debug, Clone)]
pub struct ImageInfo {
	/// Preferred/load base used by addresses in this database.
	pub image_base: Address,

	/// Size of the mapped image, if known.
	pub image_size: Option<u64>,

	/// Optional content hash used to verify that the correct binary was
	/// opened.
	pub hash: Option<Vec<u8>>,

	/// Optional original filename, for display purposes only.
	pub filename: Option<String>,
}


// ============================================================================
// Layer
// ============================================================================

/// One independently manageable contribution to the analysis database.
///
/// A layer does not have a particular semantic purpose. Any producer may
/// populate any subset of these stores.
///
/// Empty stores are perfectly valid.
#[derive(Debug, Clone)]
pub struct Layer {
	/// Stable identity of this layer within the database.
	pub id: LayerId,

	/// Human-readable layer name.
	pub name: String,

	/// Optional lightweight information about where this layer came from.
	///
	/// This is intentionally coarse-grained. Facts themselves do not need
	/// individual provenance records.
	pub producer: Option<String>,

	/// Instruction-start information.
	pub instructions: InstructionMap,

	/// Address-anchored image objects.
	pub symbols: SymbolMap,

	/// Function-specific metadata.
	pub functions: FunctionMap,

	/// Reusable type definitions.
	pub types: TypeDatabase,

	/// Directed address-to-address relationships.
	pub xrefs: XrefMap,

	/// Arbitrary address-associated annotations.
	pub annotations: AnnotationMap,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LayerId(pub u64);


// ============================================================================
// Instructions
// ============================================================================

/// Instruction-start information contributed by a layer.
///
/// This consists of address-ranged bitmaps. Bit `n` describes the address
/// `base + n`.
///
/// `1` means an instruction begins at that address.
/// `0` means no instruction begins at that address.
///
/// A bitmap only describes addresses within its own range. Multiple bitmaps
/// may exist and may overlap.
#[derive(Debug, Clone, Default)]
pub struct InstructionMap {
	pub bitmaps: Vec<InstructionBitmap>,
}

/// Dense instruction-start information for a contiguous range of addresses.
#[derive(Debug, Clone)]
pub struct InstructionBitmap {
	/// Address represented by bit zero.
	pub base: Address,

	/// Number of meaningful address bits represented by this bitmap.
	///
	/// This need not be a multiple of eight.
	pub len: u64,

	/// Packed little-bit-order bitmap data.
	///
	/// The exact bit packing convention should be specified by the serialized
	/// format; the in-memory representation is free to use something else.
	pub bits: Vec<u8>,
}


// ============================================================================
// Symbols
// ============================================================================

/// Address-anchored information about objects in the image.
///
/// A symbol always belongs to an image address but does not necessarily have
/// a name.
///
/// Optional extent and type information allow this same structure to describe
/// labels, sections, functions, typed globals, strings, structures, arrays,
/// jump tables, and other image objects.
#[derive(Debug, Clone, Default)]
pub struct SymbolMap {
	pub symbols: Vec<Symbol>,
}

/// Information anchored at one address in the image.
#[derive(Debug, Clone)]
pub struct Symbol {
	/// Address at which this object is anchored.
	pub address: Address,

	/// Optional human-readable name.
	///
	/// An unnamed symbol remains useful for attaching an extent, type, or
	/// classification to an address.
	pub name: Option<String>,

	/// Optional extent beginning at `address`.
	///
	/// When absent, the symbol describes only its anchor address.
	///
	/// An extent is expressed as a size because the region is always anchored
	/// to the symbol address.
	pub size: Option<u64>,

	/// Optional type/value interpretation for this object.
	pub ty: Option<TypeId>,

	/// Optional broad classification.
	///
	/// This is intentionally extensible and should not determine fundamental
	/// database semantics.
	pub kind: Option<SymbolKind>,

	/// Additional format-specific or analysis-specific properties that do not
	/// warrant first-class fields.
	pub metadata: Metadata,
}

/// Broad classification of an address-anchored object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SymbolKind {
	/// Plain named or unnamed location.
	Label,

	/// Function entry/body object.
	Function,

	/// Imported symbol.
	Import,

	/// Exported symbol.
	Export,

	/// Mapped executable/data section.
	Section,

	/// General data object.
	Data,

	/// String-like object.
	String,

	/// Jump table or switch table.
	JumpTable,

	/// Padding/alignment area.
	Padding,

	/// Format-specific object not covered above.
	Other(String),
}


// ============================================================================
// Functions
// ============================================================================

/// Function-specific metadata keyed by function entry address.
///
/// Symbols remain responsible for names, ranges, and generic type
/// interpretations. This store contains information meaningful specifically
/// to functions.
#[derive(Debug, Clone, Default)]
pub struct FunctionMap {
	pub functions: Vec<Function>,
}

#[derive(Debug, Clone)]
pub struct Function {
	/// Entry address of the function.
	pub address: Address,

	/// Function signature, when known.
	///
	/// This should refer to a `Type::Function`.
	pub signature: Option<TypeId>,

	/// Calling convention, when known independently of the function type or
	/// when useful as explicit metadata.
	pub calling_convention: Option<CallingConvention>,

	/// Function-specific attributes discovered by analysis or supplied by the
	/// user.
	pub attributes: FunctionAttributes,

	/// Additional extensible metadata.
	pub metadata: Metadata,
}

#[derive(Debug, Clone, Default)]
pub struct FunctionAttributes {
	pub no_return: bool,
	pub variadic: bool,

	/// Function is believed to merely forward execution elsewhere.
	pub thunk: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CallingConvention {
	Cdecl,
	Stdcall,
	Fastcall,
	Thiscall,

	/// Windows x86-64 ABI.
	Win64,

	/// System V AMD64 ABI.
	SysV64,

	/// Architecture/platform-specific convention.
	Other(String),
}


// ============================================================================
// Types
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TypeId(pub u64);

/// Reusable type definitions.
///
/// Type IDs are local to the database/layer model; the runtime implementation
/// may intern, deduplicate, or canonicalize them as desired.
#[derive(Debug, Clone, Default)]
pub struct TypeDatabase {
	pub types: Vec<TypeDefinition>,
}

#[derive(Debug, Clone)]
pub struct TypeDefinition {
	pub id: TypeId,

	/// Optional name such as `IMAGE_NT_HEADERS64`, `DWORD`, or
	/// `MyPacketHeader`.
	pub name: Option<String>,

	pub ty: Type,

	/// Optional preferred rendering/interpretation of values of this type.
	///
	/// For example, two logically similar unsigned 32-bit types could prefer
	/// decimal vs hexadecimal presentation.
	pub format: Option<ValueFormat>,

	pub metadata: Metadata,
}

#[derive(Debug, Clone)]
pub enum Type {
	Void,

	Integer {
		bits: u16,
		signed: bool,
	},

	Float {
		format: FloatFormat,
	},

	Pointer {
		pointee: TypeId,
	},

	Array {
		element: TypeId,
		count: Option<u64>,
	},

	Struct {
		size: Option<u64>,
		fields: Vec<StructField>,
	},

	Union {
		size: Option<u64>,
		fields: Vec<StructField>,
	},

	Enum {
		underlying: TypeId,
		values: Vec<EnumValue>,
	},

	Function {
		return_type: TypeId,
		parameters: Vec<FunctionParameter>,
		variadic: bool,
	},

	/// Type alias preserving a distinct name/identity.
	Alias {
		target: TypeId,
	},
}

#[derive(Debug, Clone)]
pub enum FloatFormat {
	F16,
	F32,
	F64,
	F80,
	F128,
}

#[derive(Debug, Clone)]
pub struct StructField {
	pub offset: u64,
	pub name: Option<String>,
	pub ty: TypeId,
}

#[derive(Debug, Clone)]
pub struct EnumValue {
	pub value: i128,
	pub name: String,
}

#[derive(Debug, Clone)]
pub struct FunctionParameter {
	pub name: Option<String>,
	pub ty: TypeId,
}

/// Preferred textual interpretation of a typed value.
///
/// This is intentionally small for now. More specialized formats can be
/// introduced once their actual requirements become clear.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValueFormat {
	Default,

	Decimal,

	Hexadecimal,

	Binary,

	Character,

	/// Interpret packed bytes as ASCII-like characters.
	///
	/// Useful for things such as four-byte immediates.
	PackedAscii,

	Other(String),
}


// ============================================================================
// Xrefs
// ============================================================================

/// Directed references between addresses.
///
/// Only the source -> target direction is persisted. Reverse lookup indexes
/// are derived at runtime.
#[derive(Debug, Clone, Default)]
pub struct XrefMap {
	pub xrefs: Vec<Xref>,
}

#[derive(Debug, Clone)]
pub struct Xref {
	pub source: Address,
	pub target: Address,

	/// Optional classification of the relationship.
	pub kind: Option<XrefKind>,

	pub metadata: Metadata,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum XrefKind {
	Call,
	Jump,
	ConditionalJump,

	Read,
	Write,

	/// Address/pointer/reference whose exact semantic use is not specified.
	Reference,

	Other(String),
}


// ============================================================================
// Annotations
// ============================================================================

/// Arbitrary address-associated information.
///
/// Unlike symbols, annotations do not claim that an address represents an
/// image object. They simply attach additional information to a location.
///
/// The UI chooses how to present an annotation based on its kind, MIME-like
/// type, metadata, or other application conventions.
#[derive(Debug, Clone, Default)]
pub struct AnnotationMap {
	pub annotations: Vec<Annotation>,
}

#[derive(Debug, Clone)]
pub struct Annotation {
	pub address: Address,

	/// Broad annotation classification.
	pub kind: AnnotationKind,

	/// Arbitrary annotation payload.
	///
	/// Text is sufficient for comments and many analysis results. This could
	/// later become a richer tagged value if actual use cases require it.
	pub value: String,

	pub metadata: Metadata,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnnotationKind {
	Comment,
	Note,
	Warning,

	/// Presentation hint associated with this address.
	Display,

	/// Analysis-specific information whose representation is selected by the
	/// UI/plugin that understands it.
	Analysis,

	Other(String),
}


// ============================================================================
// Generic metadata
// ============================================================================

/// Extensible string metadata.
///
/// This deliberately avoids introducing first-class fields for every obscure
/// loader, analysis, or UI property.
///
/// The runtime database can of course compile commonly queried metadata into
/// faster structures.
pub type Metadata = BTreeMap<String, String>;

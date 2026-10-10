// SPDX-License-Identifier: GPL-3.0-only

const textDecoder = new TextDecoder("utf-8");
const textEncoder = new TextEncoder();

/** @type {*} */
let result = null;
let imports = {
	env: {
		// Returns data in the form of serialized JSON
		/** @param {number} ptr @param {number} len */
		returnJSON(ptr, len) {
			let json = textDecoder.decode(new Uint8Array(instance.exports.memory.buffer, ptr, len));
			result = JSON.parse(json);
		},
		// Returns an UTF-8 string
		/** @param {number} ptr @param {number} len */
		returnString(ptr, len) {
			result = textDecoder.decode(new Uint8Array(instance.exports.memory.buffer, ptr, len));
		},
		// Returns an error
		/** @param {number} ptr @param {number} len */
		returnError(ptr, len) {
			let message = textDecoder.decode(new Uint8Array(instance.exports.memory.buffer, ptr, len));
			result = new Error(message);
		},
		// Returns a copied byte array
		/** @param {number} ptr @param {number} len */
		returnUint8Array(ptr, len) {
			result = new Uint8Array(instance.exports.memory.buffer, ptr, len).slice();
		},
		// Returns a byte array borrowing the WebAssembly memory
		/** @param {number} ptr @param {number} len */
		returnUint8Slice(ptr, len) {
			result = new Uint8Array(instance.exports.memory.buffer, ptr, len);
		},
		returnNull() {
			result = null;
		},
		// Debugging and logging
		/** @param {number} ptr @param {number} len */
		consoleLog(ptr, len) {
			let string = textDecoder.decode(new Uint8Array(instance.exports.memory.buffer, ptr, len));
			console.log(string);
		},
	},
};

/** @type {*} */
let instance;

/** @returns {*} The value supplied by the most recent WebAssembly call. */
function takeResult() {
	let value = result;
	result = null;
	return value;
}

class WasmBytes {
	/** @param {Uint8Array} bytes */
	constructor(bytes) {
		this.length = bytes.length;
		this.address = instance.exports.bytesAllocate(this.length);

		new Uint8Array(instance.exports.memory.buffer, this.address, this.length).set(bytes);
	}

	forget() {
		this.address = 0;
		this.length = 0;
	}

	dispose() {
		if (this.address !== 0) {
			instance.exports.bytesFree(this.address, this.length);
			this.address = 0;
			this.length = 0;
		}
	}
}

class WasmString {
	/** @param {string} string */
	constructor(string) {
		let bytes = textEncoder.encode(string);

		this.length = bytes.length;
		this.address = instance.exports.bytesAllocate(this.length);

		new Uint8Array(instance.exports.memory.buffer, this.address, this.length).set(bytes);
	}

	dispose() {
		if (this.address !== 0) {
			instance.exports.bytesFree(this.address, this.length);
			this.address = 0;
			this.length = 0;
		}
	}
}

const wasmURL = new URL("./pelite.wasm", import.meta.url);
const response = await fetch(wasmURL);
if (!response.ok) {
	throw new Error(`Unable to load ${wasmURL}: ${response.status} ${response.statusText}`);
}
({ instance } = await WebAssembly.instantiate(await response.arrayBuffer(), imports));

/**
 * A value returned by the WebAssembly module, or an error reported by it.
 *
 * @template T
 * @typedef {T | Error} Result
 */

/**
 * @typedef {object} DisassembledInstruction
 * @property {string} address The formatted section name and virtual address.
 * @property {number[]} bytes The encoded instruction bytes.
 * @property {string} instruction The formatted instruction.
 */

/**
 * @typedef {object} ScannerMatch
 * @property {number} rva The RVA at which the pattern matched.
 * @property {number[]} save The pattern's captured values.
 */

/**
 * @typedef {object} ScannerMatchOptions
 * @property {number} [limit=0] Maximum number of matches to return; zero means unlimited.
 */

/**
 * @typedef {object} ReadOptions
 * @property {boolean} [zerofill=false] Allow scalar/pointer reads from virtual section tails.
 * @property {number} [string_preview_length=256] Maximum bytes/code units in string previews.
 * @property {number} [max_dynamic_array_length=1024] Maximum element count for field-length arrays.
 */

/**
 * @typedef {{ $error: string, $rva: number | null }} ReadError
 */

/**
 * @typedef {number | string | null | ReadError | ReadValue[] | { [field: string]: ReadValue }} ReadValue
 */

/**
 * @typedef {object} PeFileHeader
 * @property {number} Machine
 * @property {number} NumberOfSections
 * @property {number} TimeDateStamp
 * @property {number} PointerToSymbolTable
 * @property {number} NumberOfSymbols
 * @property {number} SizeOfOptionalHeader
 * @property {number} Characteristics
 */

/**
 * @typedef {object} PeOptionalHeader
 * @property {number} Magic
 * @property {string} LinkerVersion
 * @property {number} SizeOfCode
 * @property {number} SizeOfInitializedData
 * @property {number} SizeOfUninitializedData
 * @property {number} AddressOfEntryPoint
 * @property {number} [BaseOfData] Present only in PE32.
 * @property {number} BaseOfCode
 * @property {number} ImageBase
 * @property {number} SectionAlignment
 * @property {number} FileAlignment
 * @property {string} OperatingSystemVersion
 * @property {string} ImageVersion
 * @property {string} SubsystemVersion
 * @property {number} Win32VersionValue
 * @property {number} SizeOfImage
 * @property {number} SizeOfHeaders
 * @property {number} CheckSum
 * @property {number} Subsystem
 * @property {number} DllCharacteristics
 * @property {number} SizeOfStackReserve
 * @property {number} SizeOfStackCommit
 * @property {number} SizeOfHeapReserve
 * @property {number} SizeOfHeapCommit
 * @property {number} LoaderFlags
 * @property {number} NumberOfRvaAndSizes
 */

/** @typedef {{ Name: string, VirtualAddress: number, VirtualSize: number, SizeOfRawData: number, PointerToRawData: number, Characteristics: number }} PeSectionHeader */
/** @typedef {{ VirtualAddress: number, Size: number }} PeDataDirectory */
/** @typedef {{ Signature: number, FileHeader: PeFileHeader, OptionalHeader: PeOptionalHeader }} PeNtHeaders */
/** @typedef {{ DosHeader: PeDosHeader, NtHeaders: PeNtHeaders, SectionHeaders: PeSectionHeader[], DataDirectory: PeDataDirectory[], details: Record<string, unknown> & { 'DataDirectory.Names': (string | null)[] } }} PeHeaders */

/** @typedef {{ ByName: { name: string, hint: number }, ByOrdinal?: never } | { ByOrdinal: { ord: number }, ByName?: never }} PeImportedSymbol */
/** @typedef {{ address: number, import: PeImportedSymbol | null }} PeImportEntry */
/** @typedef {{ OriginalFirstThunk: number, TimeDateStamp: number, ForwarderChain: number, Name: number, FirstThunk: number }} PeImportDescriptorImage */
/** @typedef {{ image: PeImportDescriptorImage, dll_name: string | null, imports: PeImportEntry[] | null }} PeImportDescriptor */

/** @typedef {{ image: Record<string, unknown>, dll_name: string | null, ordinal_base: number, functions: (number | null | string)[], names: Record<string, number> }} PeExportDirectory */
/** @typedef {{ product: number, build: number, count: number }} PeRichRecord */
/** @typedef {{ xor_key: number, checksum: number, records: PeRichRecord[] }} PeRichStructure */
/** @typedef {{ fixed: Record<string, number | string> | null, strings: Record<string, Record<string, string>>, langs: string[] }} PeVersionInfo */
/** @typedef {{ image: { OffsetToData: number, Size: number, CodePage: number, Reserved: number }, size: number, code_page: number, bytes: string | null }} PeResourceData */
/** @typedef {{ name: ResourceName | null, directory?: PeResourceTreeEntry[] | null, data?: PeResourceData | null }} PeResourceTreeEntry */

/**
 * @typedef {object} PeDosHeader
 * @property {number} e_magic
 * @property {number} e_cblp
 * @property {number} e_cp
 * @property {number} e_crlc
 * @property {number} e_cparhdr
 * @property {number} e_minalloc
 * @property {number} e_maxalloc
 * @property {number} e_ss
 * @property {number} e_sp
 * @property {number} e_csum
 * @property {number} e_ip
 * @property {number} e_cs
 * @property {number} e_lfarlc
 * @property {number} e_ovno
 * @property {number[]} e_res
 * @property {number} e_oemid
 * @property {number} e_oeminfo
 * @property {number[]} e_res2
 * @property {number} e_lfanew
 */

/** Relocation RVAs and types are parallel arrays. @typedef {{ rvas: number[], types: number[] }} PeBaseRelocations */
/** @typedef {{ Flags: number, Catalog: number, CatalogOffset: number, Reserved: number }} PeLoadConfigCodeIntegrity */

/**
 * @typedef {object} PeLoadConfigImage
 * @property {number} Size
 * @property {number} TimeDateStamp
 * @property {string} Version
 * @property {number} GlobalFlagsClear
 * @property {number} GlobalFlagsSet
 * @property {number} CriticalSectionDefaultTimeout
 * @property {number} DeCommitFreeBlockThreshold
 * @property {number} DeCommitTotalFreeThreshold
 * @property {number} LockPrefixTable
 * @property {number} MaximumAllocationSize
 * @property {number} VirtualMemoryThreshold
 * @property {number} ProcessAffinityMask
 * @property {number} ProcessHeapFlags
 * @property {number} CSDVersion
 * @property {number} DependentLoadFlags
 * @property {number} EditList
 * @property {number} SecurityCookie
 * @property {number} SEHandlerTable
 * @property {number} SEHandlerCount
 * @property {number} GuardCFCheckFunctionPointer
 * @property {number} GuardCFDispatchFunctionPointer
 * @property {number} GuardCFFunctionTable
 * @property {number} GuardCFFunctionCount
 * @property {number} GuardFlags
 * @property {PeLoadConfigCodeIntegrity} CodeIntegrity
 * @property {number} GuardAddressTakenIatEntryTable
 * @property {number} GuardAddressTakenIatEntryCount
 * @property {number} GuardLongJumpTargetTable
 * @property {number} GuardLongJumpTargetCount
 * @property {number} DynamicValueRelocTable
 * @property {number} CHPEMetadataPointer
 * @property {number} GuardRFFailureRoutine
 * @property {number} GuardRFFailureRoutineFunctionPointer
 * @property {number} DynamicValueRelocTableOffset
 * @property {number} DynamicValueRelocTableSection
 * @property {number} Reserved2
 * @property {number} GuardRFVerifyStackPointerFunctionPointer
 * @property {number} HotPatchTableOffset
 * @property {number} Reserved3
 * @property {number} EnclaveConfigurationPointer
 * @property {number} VolatileMetadataPointer
 * @property {number} GuardEHContinuationTable
 * @property {number} GuardEHContinuationCount
 * @property {number} GuardXFGCheckFunctionPointer
 * @property {number} GuardXFGDispatchFunctionPointer
 * @property {number} GuardXFGTableDispatchFunctionPointer
 * @property {number} CastGuardOsDeterminedFailureMode
 * @property {number} GuardMemcpyFunctionPointer
 * @property {number} UmaFunctionPointers
 */

/** Missing fields in older load-config revisions are zero-filled in image.
 * @typedef {{ image: PeLoadConfigImage, size: number, security_cookie: number | null, se_handler_table: number[] | null }} PeLoadConfig
 */

/** @typedef {{ BeginAddress: number, EndAddress: number, UnwindData: number }} PeRuntimeFunctionImage */
/** @typedef {{ CodeOffset: number, UnwindOpInfo: number }} PeUnwindCode */
/** @typedef {{ version: number, flags: number, size_of_prolog: number, frame_register: number, frame_offset: number, unwind_codes: PeUnwindCode[] }} PeUnwindInfo */
/** @typedef {{ image: PeRuntimeFunctionImage, unwind_info: PeUnwindInfo | null }} PeRuntimeFunction */

/**
 * @typedef {object} PeDebugDirectoryImage
 * @property {number} Characteristics
 * @property {number} TimeDateStamp
 * @property {string} Version
 * @property {number} Type
 * @property {number} SizeOfData
 * @property {number} AddressOfRawData
 * @property {number} PointerToRawData
 */

/**
 * @typedef {object} PeCodeView20Image
 * @property {number} CvSignature
 * @property {number} Offset
 * @property {number} TimeDateStamp
 * @property {number} Age
 */

/**
 * @typedef {object} PeCodeView70Image
 * @property {number} CvSignature
 * @property {string} Signature
 * @property {number} Age
 */

/**
 * @typedef {object} PeDebugMiscImage
 * @property {number} DataType
 * @property {number} Length
 * @property {number} Unicode
 */

/** @typedef {{ format: 'NB10', image: PeCodeView20Image, pdb_file_name: string } | { format: 'RSDS', image: PeCodeView70Image, pdb_file_name: string }} PeCodeView */
/** Unicode debug names retain their UTF-16 code units. @typedef {{ image: PeDebugMiscImage, name: string | number[] }} PeDebugMisc */
/** @typedef {{ rva: number, size: number, name: string }} PePgoSection */
/** These are serialized Rust results, not JavaScript Error instances.
 * @typedef {{ signature: string, sections: ({ Ok: PePgoSection, Err?: never } | { Err: string, Ok?: never })[] }} PePgo
 */
/** Unsupported or unreadable debug entries have null content/type.
 * @typedef {{ image: PeDebugDirectoryImage, type: string | null, entry: PeCodeView | PeDebugMisc | PePgo | null }} PeDebugDirectoryEntry
 */

/** @typedef {{ dwLength: number, wRevision: number, wCertificateType: number }} PeCertificateImage */
/** certificate_data is base64-encoded. @typedef {{ image: PeCertificateImage, certificate_type: number, certificate_data: string }} PeSecurityDirectory */

/** A resource name or unsigned 32-bit resource ID. @typedef {string | number} ResourceName */

/**
 * File MD5/SHA-256 and conventional import hash, encoded as lowercase hex.
 * A null imphash means imports could not be fully parsed. Absent imports hash the empty sequence.
 * @typedef {{ md5: string, sha256: string, imphash: string | null }} PeHashes
 */

/**
 * Parsed TLS directory. A null callback array can mean an absent pointer or unreadable data;
 * image.AddressOfCallBacks distinguishes the absent pointer (zero).
 * @typedef {{ image: Record<string, number> & { AddressOfCallBacks: number }, raw_data: string | null, slot: number | null, callbacks: number[] | null }} PeTls
 */

/**
 * Shannon entropy (0–8 bits per byte) of a section's raw data and up to 16 evenly sized samples.
 * Empty or unreadable raw data has null entropy and an empty sample array.
 * @typedef {{ entropy: number | null, samples: number[] }} SectionEntropy
 */

/**
 * @param {ResourceName} name
 * @returns {string}
 */
function resourceName(name) {
	if (typeof name === "string") {
		return name;
	}
	if (Number.isInteger(name) && name >= 0 && name <= 0xffff_ffff) {
		return `#${name}`;
	}
	throw new TypeError("A resource name must be a string or an unsigned 32-bit integer");
}

/**
 * Read-only access to a PE image backed by WebAssembly memory.
 *
 * Parsing methods return an `Error` value when the image cannot satisfy a request.
 * Dispose the file when it is no longer needed.
 */
export class PeFile {
	/**
	 * Parses a PE file from a byte array.
	 *
	 * @param {Uint8Array} bytes The complete PE file.
	 */
	constructor(bytes) {
		if (!(bytes instanceof Uint8Array)) {
			throw new TypeError("PeFile expects a Uint8Array");
		}
		const wasmBytes = new WasmBytes(bytes);
		try {
			/** @private @type {number} */
			this.p = instance.exports.pefileNew(wasmBytes.address, wasmBytes.length);
			wasmBytes.forget();
			if (!this.p) {
				throw takeResult() ?? new Error("Unable to construct PeFile");
			}
			takeResult();
		}
		finally {
			wasmBytes.dispose();
		}
	}

	/** Releases the PE image from WebAssembly memory. */
	dispose() {
		if (this.p !== 0) {
			instance.exports.pefileDrop(this.p);
			this.p = 0;
		}
	}

	/** Releases the PE image when used with explicit resource management. */
	[Symbol.dispose]() {
		this.dispose();
	}

	/** @returns {Result<PeDosHeader>} Parsed DOS header JSON. */
	dosHeader() {
		instance.exports.pefileDosHeader(this.p);
		return takeResult();
	}

	/** @returns {Result<PeNtHeaders>} Parsed NT headers JSON. */
	ntHeaders() {
		instance.exports.pefileNtHeaders(this.p);
		return takeResult();
	}

	/** @returns {Result<PeFileHeader>} Parsed COFF file header JSON. */
	fileHeader() {
		instance.exports.pefileFileHeader(this.p);
		return takeResult();
	}

	/** @returns {Result<PeOptionalHeader>} Parsed optional header JSON. */
	optionalHeader() {
		instance.exports.pefileOptionalHeader(this.p);
		return takeResult();
	}

	/** @returns {Result<PeSectionHeader[]>} Parsed section headers JSON. */
	sectionHeaders() {
		instance.exports.pefileSectionHeaders(this.p);
		return takeResult();
	}

	/** @returns {Result<PeHeaders>} Parsed PE headers JSON. */
	headers() {
		instance.exports.pefileHeaders(this.p);
		return takeResult();
	}

	/**
	 * Converts a virtual address to an RVA.
	 *
	 * @param {number | bigint} va Virtual address.
	 * @returns {Result<number>} The corresponding RVA.
	 */
	vaToRva(va) {
		let value = instance.exports.pefileVaToRva(this.p, BigInt(va));
		return takeResult() ?? (value >>> 0);
	}

	/**
	 * Converts an RVA to a virtual address.
	 *
	 * @param {number} rva Relative virtual address.
	 * @returns {Result<bigint>} The corresponding virtual address.
	 */
	rvaToVa(rva) {
		let value = instance.exports.pefileRvaToVa(this.p, rva);
		return takeResult() ?? BigInt.asUintN(64, value);
	}

	/**
	 * Converts an RVA to a file offset; zero-filled data returns an Error.
	 * @param {number} rva
	 * @returns {Result<number>}
	 */
	rvaToFileOffset(rva) {
		let value = instance.exports.pefileRvaToFileOffset(this.p, rva);
		return takeResult() ?? (value >>> 0);
	}

	/**
	 * Converts a virtual address to a file offset; zero-filled data returns an Error.
	 * @param {number | bigint} va
	 * @returns {Result<number>}
	 */
	vaToFileOffset(va) {
		let value = instance.exports.pefileVaToFileOffset(this.p, BigInt(va));
		return takeResult() ?? (value >>> 0);
	}

	/**
	 * Converts a file offset to an RVA; unmapped raw data returns an Error.
	 * @param {number} offset
	 * @returns {Result<number>}
	 */
	fileOffsetToRva(offset) {
		let value = instance.exports.pefileFileOffsetToRva(this.p, offset);
		return takeResult() ?? (value >>> 0);
	}

	/**
	 * Converts a file offset to a virtual address; unmapped raw data returns an Error.
	 * @param {number} offset
	 * @returns {Result<bigint>}
	 */
	fileOffsetToVa(offset) {
		let value = instance.exports.pefileFileOffsetToVa(this.p, offset);
		return takeResult() ?? BigInt.asUintN(64, value);
	}

	/**
	 * Disassemble instructions starting in the half-open RVA range [start, end).
	 *
	 * @param {number} start Start RVA.
	 * @param {number | null} [end=null] End RVA; defaults to one byte after `start`.
	 * @returns {Result<DisassembledInstruction[]>}
	 */
	disasm(start, end = null) {
		if (end === null) {
			end = start + 1;
		}
		instance.exports.pefileDisasm(this.p, start, end);
		return takeResult();
	}

	/**
	 * Interprets typed data at an RVA using the CLI read type syntax.
	 *
	 * Types include u8/u16/u32/u64, i8/i16/i32/i64, f32/f64, cstr, utf16lez,
	 * pointers (*T), arrays ([T; N] or [T; field]), structs, and unions.
	 * Fields use natural C alignment; pointers use the PE image's bitness.
	 * Null pointers return null; opaque pointers (*unk, *code, *fn) return RVAs.
	 * Reading code, fn, or unk directly returns an error.
	 * Syntax/options errors and failed root reads return Error. Failed nested
	 * reads return inline {$error, $rva} objects alongside successful values.
	 * Integers return JS numbers; u64/i64 can lose precision beyond 53 bits.
	 * String previews end with … when truncated and require file-backed bytes.
	 *
	 * @param {number} rva Relative virtual address.
	 * @param {string} type Type expression.
	 * @param {ReadOptions} [options]
	 * @returns {Result<ReadValue>}
	 * @example pefile.read(0x2000, "struct { count: u32, values: *[u16; count] }")
	 */
	read(rva, type, options = {}) {
		if (!Number.isInteger(rva) || rva < 0 || rva > 0xffff_ffff || typeof type !== "string") {
			return new Error("read expects an unsigned 32-bit RVA and a type string");
		}
		const argsWasm = new WasmString(JSON.stringify({ type, options }));
		try {
			instance.exports.pefileRead(this.p, rva, argsWasm.address, argsWasm.length);
			return takeResult();
		}
		finally {
			argsWasm.dispose();
		}
	}

	/**
	 * Copies exactly `bytes` bytes from the virtual image at an RVA.
	 *
	 * Gaps, missing file bytes, and section bytes beyond min(VirtualSize, SizeOfRawData)
	 * are zero-filled. Returns an Error if the range extends beyond SizeOfImage.
	 *
	 * @param {number} rva Relative virtual address.
	 * @param {number} bytes Number of bytes to copy.
	 * @returns {Result<Uint8Array>}
	 */
	hexDumpBytes(rva, bytes) {
		if (!Number.isInteger(rva) || rva < 0 || rva > 0xffff_ffff ||
			!Number.isInteger(bytes) || bytes < 0 || bytes > 0xffff_ffff) {
			return new Error("hexDumpBytes expects an unsigned 32-bit RVA and byte count");
		}
		instance.exports.pefileHexDumpBytes(this.p, rva, bytes);
		return takeResult();
	}

	/**
	 * Returns a packed mask for `bytes` bytes of the virtual image at an RVA.
	 *
	 * Bit i is 1 for a file-backed byte and 0 for zero-fill, even when the on-disk
	 * value is zero. Byte i uses bit (i % 8) of mask byte floor(i / 8), LSB first.
	 * The mask has ceil(bytes / 8) bytes; unused final bits are zero.
	 * Returns an Error if the range extends beyond SizeOfImage.
	 *
	 * @param {number} rva Relative virtual address.
	 * @param {number} bytes Number of virtual image bytes to describe.
	 * @returns {Result<Uint8Array>}
	 */
	hexDumpMask(rva, bytes) {
		if (!Number.isInteger(rva) || rva < 0 || rva > 0xffff_ffff ||
			!Number.isInteger(bytes) || bytes < 0 || bytes > 0xffff_ffff) {
			return new Error("hexDumpMask expects an unsigned 32-bit RVA and byte count");
		}
		instance.exports.pefileHexDumpMask(this.p, rva, bytes);
		return takeResult();
	}

	/**
	 * Returns a view into the PE image starting at an RVA.
	 *
	 * The view borrows WebAssembly memory rather than copying it and may be invalidated if that memory grows.
	 *
	 * @param {number} rva Relative virtual address.
	 * @param {number} [min_size=0] Minimum number of bytes required.
	 * @param {number} [align_of=1] Required alignment.
	 * @returns {Result<Uint8Array>}
	 */
	sliceBytes(rva, min_size = 0, align_of = 1) {
		instance.exports.pefileSliceBytes(this.p, rva, min_size, align_of);
		return takeResult();
	}

	/**
	 * Returns a view into the PE image starting at a virtual address.
	 *
	 * The view borrows WebAssembly memory rather than copying it and may be invalidated if that memory grows.
	 *
	 * @param {number | bigint} va Virtual address.
	 * @param {number} [min_size=0] Minimum number of bytes required.
	 * @param {number} [align_of=1] Required alignment.
	 * @returns {Result<Uint8Array>}
	 */
	readBytes(va, min_size = 0, align_of = 1) {
		instance.exports.pefileReadBytes(this.p, BigInt(va), min_size, align_of);
		return takeResult();
	}

	/** @returns {Result<PeHashes>} File fingerprints. */
	hashes() {
		instance.exports.pefileHashes(this.p);
		return takeResult();
	}

	/** @returns {Result<SectionEntropy[]>} Entropy entries in sectionHeaders() order, including empty/unreadable sections. */
	sectionEntropy() {
		instance.exports.pefileSectionEntropy(this.p);
		return takeResult();
	}


	/** @returns {Result<PeRichStructure | null>} Parsed Rich structure JSON. */
	richStructure() {
		instance.exports.pefileRichStructure(this.p);
		return takeResult();
	}

	/** @returns {Result<PeImportDescriptor[] | null>} Parsed imports JSON. */
	imports() {
		instance.exports.pefileImports(this.p);
		return takeResult();
	}

	/** @returns {Result<PeExportDirectory | null>} Parsed exports JSON. */
	exports() {
		instance.exports.pefileExports(this.p);
		return takeResult();
	}

	/** @returns {Result<PeBaseRelocations | null>} Parsed base relocations JSON. */
	baseRelocations() {
		instance.exports.pefileBaseRelocations(this.p);
		return takeResult();
	}

	/** @returns {Result<PeLoadConfig | null>} Parsed load configuration JSON. */
	loadConfig() {
		instance.exports.pefileLoadConfig(this.p);
		return takeResult();
	}

	/** @returns {Result<PeTls | null>} Parsed TLS directory, or null if absent. */
	tls() {
		instance.exports.pefileTls(this.p);
		return takeResult();
	}

	/** @returns {Result<PeRuntimeFunction[] | null>} Parsed x64 exception directory JSON, or `null` for PE32. */
	exceptionsX64() {
		instance.exports.pefileExceptionsX64(this.p);
		return takeResult();
	}

	/** @returns {Result<string | null>} Embedded PDB file name/path, or null if absent. Malformed debug data or invalid UTF-8 returns an Error. */
	pdbFileName() {
		instance.exports.pefilePdbFileName(this.p);
		return takeResult();
	}

	/** @returns {Result<PeDebugDirectoryEntry[] | null>} Parsed debug directory JSON. */
	debug() {
		instance.exports.pefileDebug(this.p);
		return takeResult();
	}

	/** @returns {Result<PeSecurityDirectory | null>} Parsed Authenticode security directory JSON. */
	security() {
		instance.exports.pefileSecurity(this.p);
		return takeResult();
	}

	/** @returns {Result<PeResourceTreeEntry[] | null>} Parsed resource tree JSON. */
	resourcesTree() {
		instance.exports.pefileResourcesTree(this.p);
		return takeResult();
	}

	/**
	 * Reads resource data by its slash-separated resource path.
	 *
	 * @param {string} path Resource path such as `/#16/#1/#1033`.
	 * @returns {Result<Uint8Array | null>} A copy of the resource bytes, or `null` if not found.
	 */
	resourcesGetResource(path) {
		const nameWasm = new WasmString(path);
		try {
			instance.exports.pefileResourcesGetResource(this.p, nameWasm.address, nameWasm.length);
			return takeResult();
		}
		finally {
			nameWasm.dispose();
		}
	}

	/** @returns {Result<string | null>} The manifest text, or `null` if not found. */
	resourcesManifest() {
		instance.exports.pefileResourcesManifest(this.p);
		return takeResult();
	}

	/** @returns {Result<PeVersionInfo | null>} Parsed version information JSON. */
	resourcesVersionInfo() {
		instance.exports.pefileResourcesVersionInfo(this.p);
		return takeResult();
	}

	/** @returns {Result<Array<ResourceName> | null>} Available icon names, or `null` if absent. */
	resourcesListIcons() {
		instance.exports.pefileListIcons(this.p);
		return takeResult();
	}

	/**
	 * @param {ResourceName} name Icon name or numeric resource ID.
	 * @returns {Result<Uint8Array | null>} A copy of the ICO file, or `null` if not found.
	 */
	resourcesGetIcon(name) {
		const nameWasm = new WasmString(resourceName(name));
		try {
			instance.exports.pefileGetIcon(this.p, nameWasm.address, nameWasm.length);
			return takeResult();
		}
		finally {
			nameWasm.dispose();
		}
	}

	/** @returns {Result<Array<ResourceName> | null>} Available cursor names, or `null` if absent. */
	resourcesListCursors() {
		instance.exports.pefileListCursors(this.p);
		return takeResult();
	}

	/**
	 * @param {ResourceName} name Cursor name or numeric resource ID.
	 * @returns {Result<Uint8Array | null>} A copy of the CUR file, or `null` if not found.
	 */
	resourcesGetCursor(name) {
		const nameWasm = new WasmString(resourceName(name));
		try {
			instance.exports.pefileGetCursor(this.p, nameWasm.address, nameWasm.length);
			return takeResult();
		}
		finally {
			nameWasm.dispose();
		}
	}

	/**
	 * Executes a pattern at exactly one RVA.
	 *
	 * @param {number} rva Relative virtual address.
	 * @param {string} pattern Pattern expression.
	 * @returns {Result<number[] | null>} Captured values, or `null` when the pattern does not match.
	 */
	scannerExec(rva, pattern) {
		const patternWasm = new WasmString(pattern);
		try {
			instance.exports.pefileScannerExec(this.p, rva, patternWasm.address, patternWasm.length);
			return takeResult();
		}
		finally {
			patternWasm.dispose();
		}
	}

	/**
	 * Finds the first pattern match in selected sections.
	 *
	 * @param {string} pattern Pattern expression.
	 * @param {string | number | null} [section] Section name or zero-based section index; omitted selects executable sections.
	 * @returns {Result<ScannerMatch | null>}
	 */
	scannerFind(pattern, section) {
		let argString = JSON.stringify({ pattern, section });
		const argWasm = new WasmString(argString);
		try {
			instance.exports.pefileScannerFind(this.p, argWasm.address, argWasm.length);
			return takeResult();
		}
		finally {
			argWasm.dispose();
		}
	}

	/**
	 * Finds pattern matches in selected sections.
	 *
	 * @param {string} pattern Pattern expression.
	 * @param {string | number | null} [section] Section name or zero-based section index; omitted selects executable sections.
	 * @param {ScannerMatchOptions} [options]
	 * @returns {Result<ScannerMatch[]>}
	 */
	scannerMatches(pattern, section, options = {}) {
		let argString = JSON.stringify({ pattern, section, ...options });
		const argWasm = new WasmString(argString);
		try {
			instance.exports.pefileScannerMatches(this.p, argWasm.address, argWasm.length);
			return takeResult();
		}
		finally {
			argWasm.dispose();
		}
	}
}

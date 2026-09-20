const textDecoder = new TextDecoder("utf-8");
const textEncoder = new TextEncoder();

/**
 * A value produced by parsing JSON returned from the WebAssembly module.
 *
 * The detailed schema depends on the PE structure requested.
 *
 * @typedef {object | null} JSONValue
 */

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

/** A resource name or unsigned 32-bit resource ID. @typedef {string | number} ResourceName */

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

const wasmURL = new URL("./pelite.wasm", import.meta.url);

let wasmResponse = await fetch(wasmURL);
if (!wasmResponse.ok) {
	throw new Error(`Unable to load ${wasmURL}: ${wasmResponse.status} ${wasmResponse.statusText}`);
}
let wasmArrayBuffer = await wasmResponse.arrayBuffer();
let { module, instance } = /** @type {*} */ (await WebAssembly.instantiate(wasmArrayBuffer, imports));

/** @returns {*} The value supplied by the most recent WebAssembly call. */
function takeResult() {
	let value = result;
	result = null;
	return value;
}

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

	[Symbol.dispose]() {
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

	[Symbol.dispose]() {
		if (this.address !== 0) {
			instance.exports.bytesFree(this.address, this.length);
			this.address = 0;
			this.length = 0;
		}
	}
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
		using wasmBytes = new WasmBytes(bytes);
		this.p = instance.exports.pefileNew(wasmBytes.address, wasmBytes.length);
		wasmBytes.forget();
		if (!this.p) {
			throw takeResult() ?? new Error("Unable to construct PeFile");
		}
		takeResult();
	}

	/** Releases the PE image from WebAssembly memory. */
	[Symbol.dispose]() {
		if (this.p !== 0) {
			instance.exports.pefileDrop(this.p);
			this.p = 0;
		}
	}

	/** @returns {Result<JSONValue>} Parsed DOS header JSON. */
	dosHeader() {
		instance.exports.pefileDosHeader(this.p);
		return takeResult();
	}

	/** @returns {Result<JSONValue>} Parsed NT headers JSON. */
	ntHeaders() {
		instance.exports.pefileNtHeaders(this.p);
		return takeResult();
	}

	/** @returns {Result<JSONValue>} Parsed COFF file header JSON. */
	fileHeader() {
		instance.exports.pefileFileHeader(this.p);
		return takeResult();
	}

	/** @returns {Result<JSONValue>} Parsed optional header JSON. */
	optionalHeader() {
		instance.exports.pefileOptionalHeader(this.p);
		return takeResult();
	}

	/** @returns {Result<JSONValue>} Parsed section headers JSON. */
	sectionHeaders() {
		instance.exports.pefileSectionHeaders(this.p);
		return takeResult();
	}

	/** @returns {Result<JSONValue>} Parsed PE headers JSON. */
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
		return takeResult() ?? value;
	}

	/**
	 * Converts an RVA to a virtual address.
	 *
	 * @param {number} rva Relative virtual address.
	 * @returns {Result<bigint>} The corresponding virtual address.
	 */
	rvaToVa(rva) {
		let value = instance.exports.pefileRvaToVa(this.p, rva);
		return takeResult() ?? value;
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
	 * Reads a null-terminated string at an RVA.
	 *
	 * @param {number} rva Relative virtual address.
	 * @param {string} [encoding="utf8"] Use `utf8` for strict UTF-8; any other value is lossy.
	 * @returns {Result<string>}
	 */
	sliceCString(rva, encoding = "utf8") {
		instance.exports.pefileSliceCString(this.p, rva, encoding === "utf8");
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

	/**
	 * Reads a null-terminated string at a virtual address.
	 *
	 * @param {number | bigint} va Virtual address.
	 * @param {string} [encoding="utf8"] Use `utf8` for strict UTF-8; any other value is lossy.
	 * @returns {Result<string>}
	 */
	readCString(va, encoding = "utf8") {
		instance.exports.pefileReadCString(this.p, BigInt(va), encoding === "utf8");
		return takeResult();
	}

	/** @returns {Result<JSONValue>} Parsed Rich structure JSON. */
	richStructure() {
		instance.exports.pefileRichStructure(this.p);
		return takeResult();
	}

	/** @returns {Result<JSONValue>} Parsed imports JSON. */
	imports() {
		instance.exports.pefileImports(this.p);
		return takeResult();
	}

	/** @returns {Result<JSONValue>} Parsed exports JSON. */
	exports() {
		instance.exports.pefileExports(this.p);
		return takeResult();
	}

	/** @returns {Result<JSONValue>} Parsed base relocations JSON. */
	baseRelocations() {
		instance.exports.pefileBaseRelocations(this.p);
		return takeResult();
	}

	/** @returns {Result<JSONValue>} Parsed load configuration JSON. */
	loadConfig() {
		instance.exports.pefileLoadConfig(this.p);
		return takeResult();
	}

	/** @returns {Result<JSONValue>} Parsed TLS directory JSON. */
	tls() {
		instance.exports.pefileTls(this.p);
		return takeResult();
	}

	/** @returns {Result<JSONValue>} Parsed x64 exception directory JSON, or `null` for PE32. */
	exceptionsX64() {
		instance.exports.pefileExceptionsX64(this.p);
		return takeResult();
	}

	/** @returns {Result<JSONValue>} Parsed debug directory JSON. */
	debug() {
		instance.exports.pefileDebug(this.p);
		return takeResult();
	}

	/** @returns {Result<JSONValue>} Parsed Authenticode security directory JSON. */
	security() {
		instance.exports.pefileSecurity(this.p);
		return takeResult();
	}

	/** @returns {Result<JSONValue>} Parsed resource tree JSON. */
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
		using nameWasm = new WasmString(path);
		instance.exports.pefileResourcesGetResource(this.p, nameWasm.address, nameWasm.length);
		return takeResult();
	}

	/** @returns {Result<string | null>} The manifest text, or `null` if not found. */
	resourcesManifest() {
		instance.exports.pefileResourcesManifest(this.p);
		return takeResult();
	}

	/** @returns {Result<JSONValue>} Parsed version information JSON. */
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
		using nameWasm = new WasmString(resourceName(name));
		instance.exports.pefileGetIcon(this.p, nameWasm.address, nameWasm.length);
		return takeResult();
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
		using nameWasm = new WasmString(resourceName(name));
		instance.exports.pefileGetCursor(this.p, nameWasm.address, nameWasm.length);
		return takeResult();
	}

	/**
	 * Executes a pattern at exactly one RVA.
	 *
	 * @param {number} rva Relative virtual address.
	 * @param {string} pattern Pattern expression.
	 * @returns {Result<number[] | null>} Captured values, or `null` when the pattern does not match.
	 */
	scannerExec(rva, pattern) {
		using patternWasm = new WasmString(pattern);
		instance.exports.pefileScannerExec(this.p, rva, patternWasm.address, patternWasm.length);
		return takeResult();
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
		using argWasm = new WasmString(argString);
		instance.exports.pefileScannerFind(this.p, argWasm.address, argWasm.length);
		return takeResult();
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
		using argWasm = new WasmString(argString);
		instance.exports.pefileScannerMatches(this.p, argWasm.address, argWasm.length);
		return takeResult();
	}
}


/**
 * Fetches the pelite wasm module and initializes its shared resources.
 *
 * @remarks
 * It is currently not possible to fetch and load wasm code synchronously so this API is async.
 *
 * @example
 * ```
 * // Load the PeLite wasm library.
 * let pelite = await pelite("pelite.wasm");
 * // Instantiate the PeFile class with an ArrayBuffer containing the executable.
 * let pefile = new pelite.PeFile(arrayBuffer);
 * ```
 *
 * @param {string} wasmPath - Path to the wasm module.
 * @returns {PeLite}
 */
async function pelite(wasmPath) {
	let textDecoder = new TextDecoder('utf-8');
	let textEncoder = new TextEncoder('utf-8');
	let result = null;
	let arrayConstructor = Uint8Array;
	let imports = {
		env: {
			// Return data in the form of serialized JSON
			setJSON: function(ptr, len) {
				let json = textDecoder.decode(new Uint8Array(instance.exports.memory.buffer, ptr, len));
				result = JSON.parse(json);
			},
			// Return an UTF-8 string
			setString: function(ptr, len) {
				result = textDecoder.decode(new Uint8Array(instance.exports.memory.buffer, ptr, len));
			},
			// Return an error string
			setError: function(ptr, len) {
				let message = textDecoder.decode(new Uint8Array(instance.exports.memory.buffer, ptr, len));
				result = new Error(message);
			},
			// Return an array referencing the underlying image
			setData: function(ptr, len) {
				result = new arrayConstructor(instance.exports.memory.buffer, ptr, len);
			},
			// Debugging and logging
			setAlert: function(ptr, len) {
				let string = textDecoder.decode(new Uint8Array(instance.exports.memory.buffer, ptr, len));
				console.log(string);
			},
		}
	};
	// Reads the result
	function wrapResult() {
		let localResult = result;
		result = null;
		arrayConstructor = Uint8Array;
		return localResult;
	}
	async function load(wasmPath, imports) {
		let response = await fetch(wasmPath);
		let arrayBuffer = await response.arrayBuffer();
		let wasm = await WebAssembly.instantiate(arrayBuffer, imports);
		return wasm;
	}
	let { module, instance } = await load(wasmPath, imports);
	function copyString(string) {
		let array = textEncoder.encode(string);
		let ptr = instance.exports.bytesAllocate(array.length);
		new Uint8Array(instance.exports.memory.buffer, ptr, array.length).set(array);
		return [ptr, array.length];
	}
	function copyResourceName(name) {
		if (typeof name === "string") {
			return copyString(name);
		}
		else if (typeof name === "number") {
			return [0, name | 0]
		}
		else {
			throw new Error("Resource names must be string or number, typeof("+name+") is "+typeof name);
		}
	}
	/**
	 * @typedef {Object} PeLite
	 * @property {PeFile} PeFile - The PeFile parser class.
	 */
	/**
	 * The PeFile parser.
	 */
	class PeFile {
		/**
		 * Constructor.
		 * @remarks Only the PE header is parsed and validated.
		 * @param {Number} ptr The address where the image is located in the WASM linear memory.
		 * @param {Number} len The length of the image.
		 */
		 constructor(ptr, len) {
			this.ptr = ptr;
			this.len = len;
			this.data = new Uint8Array(instance.exports.memory.buffer, ptr, len);
			this.address = instance.exports.pefileNew(ptr, len);
			if (this.address == 0)
				throw wrapResult();
		}
		/**
		 * Constructs a PeFile from ArrayBuffer.
		 * @remarks Copies the ArrayBuffer to the WASM linear memory.
		 * @param {ArrayBuffer} buffer - The bytes to parse as a PeFile.
		 * @returns {PeFile}
		 */
		static fromBuffer(buffer) {
			// Allocate and copy the buffer's contents to the WASM linear memory
			let ptr = instance.exports.bytesAllocate(buffer.byteLength);
			let len = buffer.byteLength;
			let data = new Uint8Array(instance.exports.memory.buffer, ptr, len);
			data.set(new Uint8Array(buffer));
			// Construct the PeFile
			return new PeFile(ptr, len);
		}
		/**
		 * @typedef {Object} DosHeader
		 * @property {Number} e_magic
		 * @property {Number} e_lfanew
		 */
		/**
		 * Gets the DOS header.
		 * @returns {DosHeader}
		 */
		dosHeader() {
			instance.exports.pefileDosHeader(this.address);
			return wrapResult();
		}
		dosImage() {
			instance.exports.pefileDosImage(this.address);
			return wrapResult();
		}
		/**
		 * @typedef {Object} NtHeaders
		 * @property {Number} Signature
		 * @property {FileHeader} FileHeader
		 * @property {OptionalHeader} OptionalHeader
		 */
		/**
		 * Gets the NT headers.
		 * @returns {NtHeaders}
		 */
		ntHeaders() {
			instance.exports.pefileNtHeaders(this.address);
			return wrapResult();
		}
		/**
		 * @typedef {Object} FileHeader
		 * @property {Number} Machine
		 * @property {Number} NumberOfSections
		 * @property {Number} TimeDateStamp
		 * @property {Number} SizeOfOptionalHeader
		 * @property {Number} Characteristics
		 */
		/**
		 * Gets the file header.
		 * @remarks
		 * The file header is part of the NT headers and can also be accessed with `file.ntHeaders().FileHeader`.
		 * @returns {FileHeader}
		 */
		fileHeader() {
			instance.exports.pefileFileHeader(this.address);
			return wrapResult();
		}
		/**
		 * @typedef {Object} OptionalHeader
		 */
		/**
		 * Gets the optional header.
		 * @remarks
		 * The optional header is part of the NT headers and can also be accessed with `file.ntHeaders().OptionalHeader`.
		 * @returns {OptionalHeader}
		 */
		optionalHeader() {
			instance.exports.pefileOptionalHeader(this.address);
			return wrapResult();
		}
		/**
		 * @typedef {Object} DataDirectory
		 * @property {Number} VirtualAddress
		 * @property {Number} Size
		 */
		/**
		 * Gets the data directory.
		 * @returns {DataDirectory[]}
		 */
		dataDirectory() {
			instance.exports.pefileDataDirectory(this.address);
			return wrapResult();
		}
		/**
		 * @typedef {Object} SectionHeader
		 * @property {string} Name
		 * @property {Number} VirtualSize
		 * @property {Number} VirtualAddress
		 * @property {Number} SizeOfRawData
		 * @property {Number} PointerToRawData
		 * @property {Number} Characteristics
		 */
		/**
		 * Gets the section headers.
		 * @returns {SectionHeader[]}
		 */
		sectionHeaders() {
			instance.exports.pefileSectionHeaders(this.address);
			return wrapResult();
		}
		/**
		 * @typedef {Object} HeaderDetails
		 */
		/**
		 * Returns all the PE headers and additional details.
		 * @returns {{DosHeader: DosHeader, NtHeaders: NtHeaders, DataDirectory: DataDirectory[], SectionHeaders: SectionHeader[], details: HeaderDetails}}
		 */
		headers() {
			instance.exports.pefileHeaders(this.address);
			return wrapResult();
		}
		/**
		 * Reads a subslice from the image starting at the given address with unbounded length.
		 * @param {Number} rva The relative virtual address.
		 * @param {Number} minSize Minimum number of bytes that must be read.
		 * @param {Number} align Alignment requirement, must be a power of two greater than zero.
		 * @returns {Uint8Array}
		 */
		slice(rva, minSize, align) {
			instance.exports.pefileSlice(this.address, rva, minSize, align);
			return wrapResult();
		}
		/**
		 * Reads a subslice from the image starting at the given address with unbounded length.
		 * @remarks The returned array is a subarray of `this.data`.
		 * @param {Number} rva The relative virtual address.
		 * @returns {Uint8Array}
		 */
		sliceBytes(rva) {
			instance.exports.pefileSliceBytes(this.address, rva);
			return wrapResult();
		}
		/**
		 * Reads and interprets a subslice of the image as a DataView.
		 * @param {Number} rva The relative virtual address.
		 * @param {Number} len The length of the structure in bytes.
		 * @param {Number} align Alignment requirement, must be a power of two greater than zero.
		 * @returns {DataView}
		 */
		sliceDataView(rva, len, align) {
			arrayConstructor = DataView;
			instance.exports.pefileSliceArray(this.address, rva, len, align);
			return wrapResult();
		}
		/**
		 * Reads and interprets a subslice of the image as a TypedArray.
		 * @remarks The returned array is a subarray of `this.data`.
		 * @param {Number} rva The relative virtual address.
		 * @param {Number} len The length of the array.
		 * @param {TypedArrayClass} typedArray TypedArray class.
		 * @returns {TypedArray}
		 */
		sliceArray(rva, len, typedArray) {
			arrayConstructor = typedArray;
			instance.exports.pefileSliceArray(this.address, rva, len, typedArray.BYTES_PER_ELEMENT);
			return wrapResult();
		}
		/**
		 * Reads a nul-terminated C string from the image.
		 * @remarks The returned 'string' is a subarray of `this.data`.
		 * @param {Number} rva The relative virtual address.
		 * @returns {Uint8Array}
		 */
		sliceCString(rva) {
			instance.exports.pefileSliceCString(this.address, rva);
			return wrapResult();
		}
		/**
		 * Reads a nul-terminated UTF-8 string from the image.
		 * @param {Number} rva The relative virtual address.
		 * @returns {string}
		 */
		sliceUtf8String(rva) {
			instance.exports.pefileSliceCString(this.address, rva);
			let string = wrapResult();
			if (string instanceof Uint8Array) {
				string = textDecoder.decode(string);
			}
			return string;
		}
		/**
		 * @typedef {Object} RichStructure
		 * @property {Number} xor_key
		 * @property {Number} checksum
		 * @property {{product: Number, build: Number, count: Number}[]} records
		 */
		/**
		 * Parses the Rich Structure.
		 * @returns {RichStructure|null}
		 */
		richStructure() {
			instance.exports.pefileRichStructure(this.address);
			return wrapResult();
		}
		/**
		 * @typedef {Object} Exports
		 * @property {string} dll_name
		 * @property {Number} time_date_stamp
		 * @property {string} version
		 * @property {Number} ordinal_base
		 * @property {Number[]} functions
		 * @property {Object.<string, Number>} names
		 */
		/**
		 * Parses the Exports Directory.
		 * @returns {Exports|null}
		 */
		exports() {
			instance.exports.pefileExports(this.address);
			return wrapResult();
		}
		/**
		 * @typedef {Object} ImportDescriptor
		 * @property {string} dll_name
		 * @property {({ByName: {hint: Number, name: string}}|{ByOrdinal: {ord: Number}})[]} int
		 */
		/**
		 * Parses the Imports Directory.
		 * @returns {ImportDescriptor[]|null}
		 */
		imports() {
			instance.exports.pefileImports(this.address);
			return wrapResult();
		}
		/**
		 * @typedef {Object} BaseRelocs
		 * @property {Number[]} rvas
		 * @property {Number[]} types
		 */
		/**
		 * Parses the Base Reloctions Directory.
		 * @returns {BaseRelocs|null}
		 */
		baseRelocs() {
			instance.exports.pefileBaseRelocs(this.address);
			return wrapResult();
		}
		/**
		 * @typedef {Object} LoadConfig
		 * @property {Number} security_cookie
		 * @property {Number[]} se_handler_table
		 */
		/**
		 * Parses the Load Config Directory.
		 * @returns {LoadConfig|null}
		 */
		loadConfig() {
			instance.exports.pefileLoadConfig(this.address);
			return wrapResult();
		}
		/**
		 * @typedef {Object} TLS
		 * @property {string} raw_data - Base64 encoded
		 * @property {Number[]} callbacks
		 */
		/**
		 * Parses the TLS Directory.
		 * @returns {TLS|null}
		 */
		tls() {
			instance.exports.pefileTls(this.address);
			return wrapResult();
		}
		/**
		 * @typedef {Object} DebugEntry
		 * @property {Object} entry
		 * @property {Number} time_date_stamp
		 * @property {string} type
		 * @property {string} version
		 */
		/**
		 * Parses the Debug Directory.
		 * @returns {DebugEntry[]|null}
		 */
		debug() {
			instance.exports.pefileDebug(this.address);
			return wrapResult();
		}
		/**
		 * Finds the Manifest.
		 * @returns {string|null}
		 */
		resourcesManifest() {
			instance.exports.pefileResourcesManifest(this.address);
			return wrapResult();
		}
		/**
		 * @typedef {Object} VersionInfo
		 * @property {Object} fixed
		 * @property {string[]} langs
		 * @property {Object.<string, Object.<string, string>>} strings
		 */
		/**
		 * Finds the Version Info.
		 * @returns {VersionInfo|null}
		 */
		resourcesVersionInfo() {
			instance.exports.pefileResourcesVersionInfo(this.address);
			return wrapResult();
		}
		/**
		 * @typedef {Object} DirectoryEntry
		 * @property {string} name
		 * @property {DirectoryEntry[]} directory
		 * @property {DataEntry} data
		 */
		/**
		 * @typedef {Object} DataEntry
		 * @property {Number} address
		 * @property {Number} size
		 * @property {Number} code_page
		 */
		/**
		 * Parses the Resources Directory.
		 * @returns {DirectoryEntry[]|null}
		 */
		resourcesTree() {
			instance.exports.pefileResourcesTree(this.address);
			return wrapResult();
		}
		/**
		 * Finds a data entry at the given path.
		 * @param {string|null} path Absolute path to the resource.
		 * Resource paths must start and separete components with a forward slash.
		 * Id entries must be prefixed with a #.
		 * @returns {DataEntry|null}
		 */
		resourcesFindData(path) {
			if (!path) {
				return null;
			}
			let [pathPtr, pathLen] = copyString(path);
			instance.exports.pefileResourcesFindData(this.address, pathPtr, pathLen);
			return wrapResult();
		}
		/**
		 * Returns the bytes for the given data entry.
		 * @remarks The returned array is a subarray of `this.data`.
		 * @param {DataEntry|null} dataEntry The data entry.
		 * @returns {Uint8Array|null}
		 */
		resourcesReadData(dataEntry) {
			if (!dataEntry) {
				return null;
			}
			instance.exports.pefileSliceArray(this.address, dataEntry.address, dataEntry.size, 1);
			return wrapResult();
		}
		/**
		 * Returns the first resource with matching type and name.
		 * @remarks The returned array is a subarray of `this.data`.
		 * @param {Number|string} type The type component of a resource.
		 * @param {Number|string} name The name component of a resource.
		 * @returns {Uint8Array|null}
		 */
		resourcesFindResource(type, name) {
			let [typePtr, typeLen] = copyResourceName(type);
			let [namePtr, nameLen] = copyResourceName(name);
			instance.exports.pefileResourcesFindResource(this.address, typePtr, typeLen, namePtr, nameLen);
			return wrapResult();
		}
		/**
		 * Returns the resource given its type, name and lang components.
		 * @remarks The returned array is a subarray of `this.data`.
		 * @param {Number|string} type The type component of a resource.
		 * @param {Number|string} name The name component of a resource.
		 * @param {Number|string} lang The lang component of a resource.
		 * @returns {Uint8Array|null}
		 */
		resourcesFindResourceEx(type, name, lang) {
			let [typePtr, typeLen] = copyResourceName(type);
			let [namePtr, nameLen] = copyResourceName(name);
			let [langPtr, langLen] = copyResourceName(lang);
			instance.exports.pefileResourcesFindResourceEx(this.address, typePtr, typeLen, namePtr, nameLen, langPtr, langLen);
			return wrapResult();
		}
		/**
		 * Executes the pattern at the given address.
		 * @param {Number} rva The relative virtual address.
		 * @param {string} pattern The pattern to match at the given address.
		 * @returns {Number[]|null}
		 */
		scannerExec(rva, pattern) {
			let [patPtr, patLen] = copyString(pattern);
			instance.exports.pefileScannerExec(this.address, rva, patPtr, patLen);
			return wrapResult();
		}
		/**
		 * Finds a single unique match for the given pattern.
		 * @remarks If there is more than one match null is returned.
		 * @param {string} pattern The pattern to search for.
		 * @param {Number} start The relative virtual address to start searching.
		 * @param {Number} end The relative virtual address to stop searching.
		 * @returns {Number[]|null}
		 */
		scannerFind(pattern, start, end) {
			let [patPtr, patLen] = copyString(pattern);
			instance.exports.pefileScannerFinds(this.address, patPtr, patLen, start, end);
			return wrapResult();
		}
		/**
		 * Finds a single unique code match for the given pattern.
		 * @param {string} pattern The pattern to search for.
		 * @returns {Number[]|null}
		 */
		scannerFindCode(pattern) {
			let [patPtr, patLen] = copyString(pattern);
			instance.exports.pefileScannerFindsCode(this.address, patPtr, patLen);
			return wrapResult();
		}
		/**
		 * Finds all matches of the given pattern.
		 * @param {string} pattern The pattern to search for.
		 * @param {Number} start The relative virtual address to start searching.
		 * @param {Number} end The relative virtual address to stop searching.
		 * @param {Number} offset Skip the first X matches.
		 * @param {Number} limit Stop when X matches have been found.
		 * @returns {Number[][]}
		 */
		scannerMatches(pattern, start, end, offset, limit) {
			let [patPtr, patLen] = copyString(pattern);
			instance.exports.pefileScannerMatches(this.address, patPtr, patLen, start, end, offset, limit);
			return wrapResult();
		}
		/**
		 * Finds all code matches of the given pattern.
		 * @param {string} pattern The pattern to search for.
		 * @param {Number} offset Skip the first X matches.
		 * @param {Number} limit Stop when X matches have been found.
		 * @returns {Number[][]}
		 */
		scannerMatchesCode(pattern, offset, limit) {
			let [patPtr, patLen] = copyString(pattern);
			instance.exports.pefileScannerMatchesCode(this.address, patPtr, patLen, offset, limit);
			return wrapResult();
		}
		/**
		 * Disposes the PeFile instance and releases its resources.
		 * @remarks This invalidates all the returned typed array buffers!
		 */
		dispose() {
			instance.exports.pefileDrop(this.address);
		}
	}
	return {
		module: module,
		instance: instance,
		PeFile: PeFile,
	};
}

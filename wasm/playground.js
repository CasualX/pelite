import createPeLite from "./pelite.js";

const sampleInput = document.querySelector("#sample");
const status = document.querySelector("#status");
const queryForm = document.querySelector("#query");
const operationSelect = document.querySelector("#operation");
const argumentsElement = document.querySelector("#arguments");
const output = document.querySelector("#output");
const copyButton = document.querySelector("#copy");

const number = (label, value = "0") => ({ label, value, parse: parseNumber });
const text = (label, value = "") => ({ label, value, parse: String });
const resourceName = (label) => ({ label, value: "", parse: parseResourceName });
const typedArrays = { Uint8Array, Uint16Array, Uint32Array, Int8Array, Int16Array, Int32Array, Float32Array, Float64Array };

const operations = {
	headers: { label: "All headers", group: "Headers" },
	dosHeader: { label: "DOS header", group: "Headers" },
	dosImage: { label: "DOS image bytes", group: "Headers" },
	ntHeaders: { label: "NT headers", group: "Headers" },
	fileHeader: { label: "File header", group: "Headers" },
	optionalHeader: { label: "Optional header", group: "Headers" },
	dataDirectory: { label: "Data directory", group: "Headers" },
	sectionHeaders: { label: "Section headers", group: "Headers" },
	richStructure: { label: "Rich structure", group: "Directories" },
	exports: { label: "Exports", group: "Directories" },
	imports: { label: "Imports", group: "Directories" },
	baseRelocs: { label: "Base relocations", group: "Directories" },
	loadConfig: { label: "Load config", group: "Directories" },
	tls: { label: "TLS", group: "Directories" },
	debug: { label: "Debug", group: "Directories" },
	resourcesManifest: { label: "Manifest", group: "Resources" },
	resourcesVersionInfo: { label: "Version info", group: "Resources" },
	resourcesTree: { label: "Resource tree", group: "Resources" },
	resourcesFindData: {
		label: "Find data entry by path",
		group: "Resources",
		fields: [text("Path", "/#16/#1/#1033")],
	},
	resourceBytes: {
		label: "Read resource bytes by path",
		group: "Resources",
		fields: [text("Path", "/#16/#1/#1033")],
		run(file, path) {
			return file.resourcesReadData(file.resourcesFindData(path));
		},
	},
	resourcesFindResource: {
		label: "Find resource by type and name",
		group: "Resources",
		fields: [resourceName("Type (text or #id)"), resourceName("Name (text or #id)")],
	},
	resourcesFindResourceEx: {
		label: "Find resource by type, name and language",
		group: "Resources",
		fields: [resourceName("Type (text or #id)"), resourceName("Name (text or #id)"), resourceName("Language (text or #id)")],
	},
	slice: {
		label: "Read bytes with size and alignment",
		group: "Addresses",
		fields: [number("RVA"), number("Minimum size", "1"), number("Alignment", "1")],
	},
	sliceBytes: { label: "Read bytes from RVA", group: "Addresses", fields: [number("RVA")] },
	sliceDataView: {
		label: "Read DataView",
		group: "Addresses",
		fields: [number("RVA"), number("Byte length", "1"), number("Alignment", "1")],
	},
	sliceArray: {
		label: "Read typed array",
		group: "Addresses",
		fields: [number("RVA"), number("Element count", "1"), text("Type", "Uint8Array")],
		run(file, rva, length, type) {
			if (!(type in typedArrays)) {
				throw new Error(`Unknown typed array “${type}”. Use one of: ${Object.keys(typedArrays).join(", ")}.`);
			}
			return file.sliceArray(rva, length, typedArrays[type]);
		},
	},
	sliceCString: { label: "Read C string bytes", group: "Addresses", fields: [number("RVA")] },
	sliceUtf8String: { label: "Read UTF-8 string", group: "Addresses", fields: [number("RVA")] },
	scannerExec: {
		label: "Execute pattern at RVA",
		group: "Scanner",
		fields: [number("RVA"), text("Pattern")],
	},
	scannerFindCode: { label: "Find unique code match", group: "Scanner", fields: [text("Pattern")] },
	scannerMatchesCode: {
		label: "Find code matches",
		group: "Scanner",
		fields: [text("Pattern"), number("Offset"), number("Limit", "100")],
	},
	scannerFind: {
		label: "Find unique match in range",
		group: "Scanner",
		fields: [text("Pattern"), number("Start RVA"), number("End RVA")],
	},
	scannerMatches: {
		label: "Find matches in range",
		group: "Scanner",
		fields: [text("Pattern"), number("Start RVA"), number("End RVA"), number("Offset"), number("Limit", "100")],
	},
};

let library;
let peFile;
let fileSummary = "";

populateOperations();
operationSelect.addEventListener("change", renderArguments);
sampleInput.addEventListener("change", loadSample);
queryForm.addEventListener("submit", runOperation);
copyButton.addEventListener("click", copyResult);
renderArguments();
loadLibrary();

async function loadLibrary() {
	try {
		library = await createPeLite();
		status.textContent = "PeLite is ready. Select a file.";
		sampleInput.disabled = false;
	} catch (error) {
		showError(error, "Could not load PeLite");
		sampleInput.disabled = true;
	}
}

async function loadSample() {
	const [file] = sampleInput.files;
	if (!file || !library) return;

	try {
		status.textContent = `Reading ${file.name}…`;
		const buffer = await file.arrayBuffer();
		peFile?.dispose();
		peFile = undefined;
		peFile = library.PeFile.fromBuffer(buffer);
		fileSummary = `${file.name} (${formatBytes(file.size)})`;
		status.textContent = `${fileSummary} loaded.`;
		queryForm.hidden = false;
		operationSelect.value = "headers";
		renderArguments();
		runSelectedOperation();
	} catch (error) {
		queryForm.hidden = true;
		showError(error, `Could not parse ${file.name}`);
	}
}

function populateOperations() {
	const groups = new Map();
	for (const [name, operation] of Object.entries(operations)) {
		let group = groups.get(operation.group);
		if (!group) {
			group = document.createElement("optgroup");
			group.label = operation.group;
			groups.set(operation.group, group);
			operationSelect.append(group);
		}
		group.append(new Option(operation.label, name));
	}
}

function renderArguments() {
	argumentsElement.replaceChildren();
	const fields = operations[operationSelect.value].fields ?? [];
	for (const [index, field] of fields.entries()) {
		const wrapper = document.createElement("div");
		const label = document.createElement("label");
		const input = document.createElement("input");
		input.id = `argument-${index}`;
		input.name = `argument-${index}`;
		input.type = "text";
		input.value = field.value;
		input.required = true;
		label.htmlFor = input.id;
		label.textContent = field.label;
		wrapper.append(label, input);
		argumentsElement.append(wrapper);
	}
}

function runOperation(event) {
	event.preventDefault();
	runSelectedOperation();
}

function runSelectedOperation() {
	if (!peFile) return;

	try {
		const name = operationSelect.value;
		const operation = operations[name];
		const values = (operation.fields ?? []).map((field, index) => {
			const value = queryForm.elements[`argument-${index}`].value;
			return field.parse(value);
		});
		const result = operation.run ? operation.run(peFile, ...values) : peFile[name](...values);
		showResult(result);
	} catch (error) {
		showError(error, "Operation failed");
	}
}

function showResult(result) {
	output.textContent = formatResult(result);
	status.textContent = `${fileSummary}: ${operations[operationSelect.value].label}.`;
	copyButton.disabled = false;
}

function showError(error, prefix) {
	const message = error instanceof Error ? error.message : String(error);
	status.textContent = `${prefix}: ${message}`;
	output.textContent = `${prefix}\n\n${message}`;
	copyButton.disabled = false;
}

function formatResult(value) {
	if (typeof value === "string") return value;
	if (ArrayBuffer.isView(value)) {
		const bytes = new Uint8Array(value.buffer, value.byteOffset, value.byteLength);
		const limit = 4096;
		return JSON.stringify({
			type: value.constructor.name,
			byteLength: value.byteLength,
			bytes: Array.from(bytes.subarray(0, limit)),
			truncated: bytes.length > limit,
		}, null, 2);
	}
	return JSON.stringify(value, null, 2);
}

function parseNumber(value) {
	const parsed = Number(value);
	if (!Number.isSafeInteger(parsed) || parsed < 0) {
		throw new Error(`Expected a non-negative integer, received “${value}”.`);
	}
	return parsed;
}

function parseResourceName(value) {
	if (value.startsWith("#")) return parseNumber(value.slice(1));
	return value;
}

function formatBytes(bytes) {
	if (bytes < 1024) return `${bytes} bytes`;
	return `${(bytes / 1024).toFixed(1)} KiB`;
}

async function copyResult() {
	try {
		await navigator.clipboard.writeText(output.textContent);
		copyButton.textContent = "Copied";
		setTimeout(() => { copyButton.textContent = "Copy"; }, 1200);
	}
	catch (error) {
		const message = error instanceof Error ? error.message : String(error);
		status.textContent = `Could not copy result: ${message}`;
	}
}

window.addEventListener("pagehide", () => peFile?.dispose());

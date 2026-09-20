document.addEventListener("alpine:init", () => {
	Alpine.data("apiReference", () => ({
		intro: "",
		groups: [],
		footnote: "",
		error: "",

		async load() {
			try {
				const response = await fetch("./pelite.docs.json");
				if (!response.ok) {
					throw new Error(`${response.status} ${response.statusText}`);
				}
				const docs = await response.json();
				this.intro = docs.intro;
				this.groups = docs.groups;
				this.footnote = docs.footnote;
			}
			catch (error) {
				this.error = `Unable to load API reference: ${error.message}`;
			}
		},
	}));
});

const fileInput = document.querySelector("#fileInput");
const codeInput = document.querySelector("#codeInput");
const runButton = document.querySelector("#runButton");
const output = document.querySelector("#output");
const status = document.querySelector("#status");
const demoButtons = document.querySelectorAll("[data-demo]");
let demoFile = null;

runButton.addEventListener("click", runScript);
codeInput.addEventListener("keydown", event => {
	if ((event.ctrlKey || event.metaKey) && event.key === "Enter") {
		event.preventDefault();
		runScript();
	}
});
fileInput.addEventListener("change", () => {
	const file = fileInput.files[0];
	demoFile = null;
	setActiveDemo(null);
	status.textContent = file ? `${file.name} · ${formatSize(file.size)}` : "No file selected";
});
demoButtons.forEach(button => button.addEventListener("click", () => selectDemo(button)));

async function selectDemo(button) {
	demoButtons.forEach(item => item.disabled = true);
	status.textContent = `Loading ${button.dataset.demo}…`;
	try {
		const response = await fetch(new URL(button.dataset.demo, document.baseURI));
		if (!response.ok) {
			throw new Error(`Unable to load demo: ${response.status} ${response.statusText}`);
		}
		const bytes = new Uint8Array(await response.arrayBuffer());
		demoFile = { name: button.dataset.demo, bytes };
		fileInput.value = "";
		setActiveDemo(button);
		status.textContent = `${demoFile.name} · ${formatSize(bytes.length)}`;
	}
	catch (error) {
		demoFile = null;
		setActiveDemo(null);
		status.textContent = "Could not load demo";
		output.textContent = error?.stack ?? String(error);
	}
	finally {
		demoButtons.forEach(item => item.disabled = false);
	}
}

function setActiveDemo(activeButton) {
	demoButtons.forEach(button => button.setAttribute("aria-pressed", String(button === activeButton)));
}

function formatSize(bytes) {
	if (bytes < 1024) {
		return `${bytes} B`;
	}
	if (bytes < 1024 ** 2) {
		return `${(bytes / 1024).toFixed(1)} KB`;
	}
	return `${(bytes / 1024 ** 2).toFixed(1)} MB`;
}
function stringify(value) {
	if (value instanceof Error) {
		return value.stack ?? value.message;
	}
	if (value instanceof Uint8Array) {
		return Array.from(value, byte => byte.toString(16).padStart(2, "0")).join(" ");
	}
	const json = JSON.stringify(value, (key, item) => typeof item === "bigint" ? `0x${item.toString(16)}` : item, 2);
	return json ?? String(value);
}

async function runScript() {
	if (runButton.disabled) {
		return;
	}
	runButton.disabled = true;
	runButton.textContent = "Running…";
	output.textContent = "";
	try {
		const file = fileInput.files[0];
		if (!file && !demoFile) {
			throw new Error("Please select a local file or one of the demos first.");
		}
		const bytes = demoFile?.bytes ?? new Uint8Array(await file.arrayBuffer());
		const { PeFile } = await import("./pelite.js");
		using pefile = new PeFile(bytes);
		const result = eval(codeInput.value);
		output.textContent = stringify(await result);
	}
	catch (error) {
		console.error(error);
		output.textContent = error?.stack ?? String(error);
	}
	finally {
		runButton.disabled = false;
		runButton.textContent = "Run";
	}
}

# pelite-js

Parse Windows PE32 and PE32+ executables in a web browser with WebAssembly.
The JavaScript API is a single ES module with no runtime dependencies.

## Use without npm

Copy `dist/pelite.js` and `dist/pelite.wasm` into your project's `deps/` folder.
Keep them together and serve your project over HTTP:

```html
<script type="module">
  import { PeFile } from "./deps/pelite.js";

  const response = await fetch("example.dll");
  if (!response.ok) throw new Error(`Unable to load example.dll: ${response.status}`);
  using pefile = new PeFile(new Uint8Array(await response.arrayBuffer()));
  const headers = pefile.headers();
  if (headers instanceof Error) throw headers;
  console.log(headers);
</script>
```

The module fetches its adjacent `pelite.wasm` during evaluation, so `PeFile` is
ready when the import completes. It requires a browser supporting ES modules,
top-level await, WebAssembly, and `using` declarations. Your project needs no
package manager or build step. Optionally copy `dist/pelite.d.ts` for editor type hints.

Construction throws for invalid files; parsing methods return an `Error` value
on failure. A `using` declaration disposes the handle at the end of its scope;
otherwise call `dispose()` when finished. Byte views from `sliceBytes()` and `readBytes()` borrow
WASM memory and may be invalidated by memory growth or disposal; copy them with
`.slice()` to retain them.

## Facts layers

`Facts` owns a collection of layers independently of `PeFile`. Pass the
target pointer width in bits (32 or 64) when constructing it. Import and export
native `#factmap` text; labels are stored separately.

```js
import { Facts } from "pelite-js";

using layers = new Facts(64);
const index = layers.addLayer('#factmap\nSx1000 code "entry"\n');
if (index instanceof Error) throw index;
const labeled = layers.setLayerLabel(index, "Analysis");
if (labeled instanceof Error) throw labeled;
const annotated = layers.addFact(index, 'Cx1000 "Entry point"');
if (annotated instanceof Error) throw annotated;
const text = layers.getLayer(index); // canonical #factmap text
if (text instanceof Error) throw text;
```

Indices start at zero; negative indices count from the end (`-1` is the last
layer). `addLayer(facts)` appends a layer with an empty label and returns
its index. Omitting `facts` creates an empty layer.
`shiftLayer(index, destination)` moves to the final destination index. `mergeLayer(base, index)` appends the source records to the base,
retains the base label, and removes the source. Indices resolve before mutation.
Use `getLayerLabel`, `setLayerLabel`, and `deleteLayer` to manage
layers. Other mutations return `null` on success. All operations return `Error`
on failure; failed validation leaves the layers unchanged.

Record order and duplicates are preserved; original whitespace and file comments
are discarded on export. `addFact` takes one record without a header or newline.
Call `dispose()` when finished, or use a `using` declaration. Layers do not yet
affect `PeFile` operations.

## Install from a GitHub Release

The browser package is distributed as a prebuilt tarball attached to a GitHub Release.

```sh
npm install https://github.com/CasualX/pelite/releases/download/pelite-js-v0.1.0/pelite-js-0.1.0.tgz
```

After installing, import by the package name in your browser application:

```js
import { PeFile } from "pelite-js";
```

Configure your bundler
to support top-level await and emit the WASM asset referenced by
`new URL("./pelite.wasm", import.meta.url)`. The asset is also exported as
`pelite-js/pelite.wasm`. This package targets browsers.
Installing the tarball requires no npm account or Rust toolchain.

## Build and test

```text
pelite-js/
  src/pelite.js       browser loader, WASM runtime, and public API
  rust/               Rust crate in the repository's Cargo workspace
  scripts/build.mjs   package build
  html/               index.html playground and tests.html
  dist/               generated pelite.js, pelite.wasm, and pelite.d.ts
```

Build from this repository checkout with Node.js, npm, and Rust:

```sh
rustup target add wasm32-unknown-unknown
cd pelite-js
npm ci
npm run build
npm test
```

## License

The WebAssembly bindings and JavaScript API are licensed under GPL-3.0-only.
See [license.txt](license.txt) and include it when redistributing these files.
The core Rust `pelite` library retains its MIT license.

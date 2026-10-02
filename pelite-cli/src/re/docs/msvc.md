When to use:

Investigate Microsoft C++ compiler patterns in a PE binary. The `rtti`
command reports class names, inheritance, and vtables in PE32 and PE32+
images, selecting the scanner from the image type. The independent `throws`
command extracts thrown exception types, catchable conversions, and copy and
destruction helpers from x86/x64 images, including code compiled with /GR-.

Examples:

    pelite-cli re msvc rtti sample.dll
    pelite-cli re msvc rtti sample64.dll --format=json-pretty
    pelite-cli re msvc throws sample64.dll --demangle --format=json-pretty

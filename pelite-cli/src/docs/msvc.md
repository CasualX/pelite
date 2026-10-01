When to use:

Investigate Microsoft C++ compiler patterns in a PE binary. The `rtti`
command reports class names, inheritance, and vtables in PE32 and PE32+
images, selecting the scanner from the image type.

Examples:

    pelite-cli msvc rtti sample.dll
    pelite-cli msvc rtti sample64.dll --format=json-pretty

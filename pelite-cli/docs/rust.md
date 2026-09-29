The data scans support PE32 and PE32+ images. These analyses recognize
toolchain-specific layouts and report candidates; matches can be missed or
unrelated data can be reported.

Examples:

    pelite-cli rust fmt-template sample.exe
    pelite-cli rust panic-locations sample.exe --format=json-pretty

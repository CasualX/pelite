#pragma once

#include <cstdint>

extern "C" {


struct Pattern {
	const uint16_t* ptr;
	size_t len;
};

struct Pattern PelitePatternParse(const char* string);

void PelitePatternFree(struct Pattern pattern);



struct PeView64 {
	const uint8_t* ptr;
	size_t len;
	uint64_t va;
};

struct PeView64 PeliteView64(const uint8_t* image, uint64_t base_address);

bool PeliteView64Finds(const struct PeView64* view, const char* pat, const char* sect, uint32_t* save_ptr, size_t save_len);



struct PeViewMatches64 {
	size_t inner[0x38 / 8];
};

struct PeViewMatches64 PeliteView64Matches(const struct PeView64* view, const Pattern* pat, const char* sect);

bool PeliteView64MatchesNext(struct PeViewMatches64* matches, uint32_t* save_ptr, size_t save_len);



struct PeFile64 {
	const uint8_t* ptr;
	size_t len;
};

struct PeFile64 PeliteFile64(const uint8_t* ptr, size_t len);

bool PeliteFile64Finds(const struct PeFile64* file, const char* pat, const char* sect, uint32_t* save_ptr, size_t save_len);



struct PeFileMatches64 {
	size_t inner[0x30 / 8];
};

struct PeFileMatches64 PeliteFile64Matches(const struct PeFile64* file, const Pattern* pat, const char* sect);

bool PeliteFile64MatchesNext(struct PeFileMatches64* matches, uint32_t* save_ptr, size_t save_len);


}

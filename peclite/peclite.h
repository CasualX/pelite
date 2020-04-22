#pragma once

#include <cstdint>

// Pelite scanner error codes.
enum pelite_scanner_error {
	pelite_scanner_success = 0,
	pelite_scanner_not_found = 1,
	// There was an error with the image, it is not a valid PE file.
	pelite_scanner_error_pefile = 2,
	// There was an error with the pattern, it is not valid UTF-8 or the syntax is incorrect.
	pelite_scanner_error_pattern = 3,
};

// Pattern interpreter, returns if the pattern matches the binary image at the given rva.
//
// The pattern may contain instructions to capture interesting addresses, these are stored in the save array.
// Out of bounds stores are simply ignored, ensure the save array is large enough for the given pattern.
//
// In case of mismatch, ie. returns `pelite_scanner_not_found`, the save array is still overwritten with temporary data and should be considered trashed.
// Keep a copy, invoke with a fresh save array or reexecute the pattern at the saved cursor to get around this.
extern "C" uint8_t pelite_scanner_exec(const void* image_ptr, size_t image_len, uint32_t rva, const char* patstring_ptr, size_t patstring_len, uint32_t* save_ptr, size_t save_len);

// Finds the unique match for the pattern in the given section.
//
// The pattern may contain instructions to capture interesting addresses, these are stored in the save array.
// Out of bounds stores are simply ignored, ensure the save array is large enough for the given pattern.
//
// In case of mismatch, ie. returns `pelite_scanner_not_found`, the save array is still overwritten with temporary data and should be considered trashed.
// Keep a copy, invoke with a fresh save array or reexecute the pattern at the saved cursor to get around this.
//
// Returns `pelite_scanner_not_found` if no match is found or multiple matches are found to prevent subtle bugs where a pattern goes stale by not being unique any more.
//
// Use the matches API if just the first match is desired.
extern "C" uint8_t pelite_scanner_find(const void* image_ptr, size_t image_len, const char* patstring_ptr, size_t patstring_len, size_t section_index, uint32_t* save_ptr, size_t save_len);

// The matches object stores the state of the scanner while looking for matches.
typedef struct { void* _; } PeliteScannerMatches;

// Creates an iterator over the matches of a pattern within the given section.
//
// The resulting matches object holds a reference to the image, do not touch it until after the matches object has been deleted.
extern "C" uint8_t pelite_scanner_matches_new(const void* image_ptr, size_t image_len, const char* patstring_ptr, size_t patstring_len, size_t section_index, PeliteScannerMatches* matches);

// Deletes the matches object.
extern "C" void pelite_scanner_matches_delete(PeliteScannerMatches* matches);

// Finds the next match, returns false if there are no more matches.
extern "C" bool pelite_scanner_matches_next(PeliteScannerMatches* matches, uint32_t* save_ptr, size_t save_len);

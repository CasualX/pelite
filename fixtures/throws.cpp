#include <cstdio>

// Deliberately nonpolymorphic: exception metadata must not depend on vtables
// or /GR. Runtime checks cover the conversions advertised by ThrowInfo.
namespace fixture {
struct PlainException { int value; };
struct BaseException { int base = 11; };
struct OtherException { int other = 22; };
struct DerivedException : BaseException, OtherException { int value = 33; };
struct VirtualException : virtual BaseException { int value = 44; };
static int live = 0;
static int copies = 0;
struct OwnedException {
	int value = 55;
	OwnedException() { ++live; }
	OwnedException(const OwnedException& other) : value(other.value) { ++live; ++copies; }
	~OwnedException() { --live; }
};

__declspec(noinline) void throw_int() { throw 7; }
__declspec(noinline) void throw_plain() { throw PlainException{9}; }
__declspec(noinline) void throw_derived() { throw DerivedException{}; }
__declspec(noinline) void throw_virtual() { throw VirtualException{}; }
__declspec(noinline) void throw_owned() { OwnedException object; throw object; }
__declspec(noinline) void throw_pointer(PlainException* object) { throw object; }
__declspec(noinline) void throw_const_pointer(const PlainException* object) { throw object; }
__declspec(noinline) void rethrow() { try { throw_int(); } catch (...) { throw; } }
}

int main() {
	using namespace fixture;
	int passed = 0;
	try { throw_int(); } catch (int value) { passed += value == 7; }
	try { throw_plain(); } catch (const PlainException& value) { passed += value.value == 9; }
	try { throw_derived(); } catch (const OtherException& value) { passed += value.other == 22; }
	try { throw_virtual(); } catch (const BaseException& value) { passed += value.base == 11; }
	try { throw_owned(); } catch (OwnedException value) { passed += value.value == 55; }
	passed += live == 0 && copies >= 1;
	PlainException object{66};
	try { throw_pointer(&object); } catch (PlainException* value) { passed += value == &object; }
	try { throw_const_pointer(&object); } catch (const PlainException* value) { passed += value == &object; }
	try { rethrow(); } catch (int value) { passed += value == 7; }
	std::puts(passed == 9 ? "throw checks passed" : "throw checks FAILED");
	return passed == 9 ? 0 : 1;
}

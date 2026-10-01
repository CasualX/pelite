#include <cstdio>
#include <typeinfo>

#define FIXTURE_NOINLINE __declspec(noinline)

// Distinct method bodies and /OPT:NOICF keep vtable entries easy to recognize.
namespace fixture {
class Root {
public:
	int value = 7;
	virtual ~Root() = default;
	virtual int id() const { return value; }
};
class Single : public Root {
public:
	int id() const override { return value + 10; }
};
class Multilevel final : public Single {
public:
	int id() const override { return value + 20; }
};
class Other {
public:
	int other = 11;
	virtual ~Other() = default;
	virtual int second() const { return other; }
};
class Multiple final : public Root, public Other {
public:
	int id() const override { return value + 30; }
	int second() const override { return other + 30; }
};
class Left : public Root {};
class Right : public Root {};
class Diamond final : public Left, public Right {};
class VirtualSingle : virtual public Root {
public:
	int id() const override { return value + 40; }
};
class VirtualLeft : virtual public Root {};
class VirtualRight : virtual public Root {};
class VirtualDiamond final : public VirtualLeft, public VirtualRight {
public:
	int id() const override { return value + 50; }
};
class Mixed final : public Left, public VirtualRight {};
class Private final : private Root {
public:
	Root* base() { return this; }
	int id() const override { return value + 60; }
};
class Protected final : protected Root {
public:
	Root* base() { return this; }
	int id() const override { return value + 70; }
};
class Interface {
public:
	virtual ~Interface() = default;
	virtual int execute() const = 0;
};
class Implementation final : public Interface, public Other {
public:
	int execute() const override { return 80; }
};
// A nonpolymorphic base contributes layout and RTTI hierarchy, but no vtable.
class Plain { public: int plain = 13; };
class WithPlain final : public Plain, public Root {};
template<class T> class Template final : public Root {
public:
	T payload = 17;
	int id() const override { return value + static_cast<int>(payload); }
};

FIXTURE_NOINLINE int observe(const Root& object) {
	std::printf("%s: %d\n", typeid(object).name(), object.id());
	return object.id();
}
FIXTURE_NOINLINE int observe_other(const Other& object) {
	std::printf("%s: %d\n", typeid(object).name(), object.second());
	return object.second();
}
} // namespace fixture

int main() {
	using namespace fixture;
	Root root;
	Single single;
	Multilevel multilevel;
	Other other;
	Multiple multiple;
	Left left;
	Right right;
	Diamond diamond;
	VirtualSingle virtual_single;
	VirtualLeft virtual_left;
	VirtualRight virtual_right;
	VirtualDiamond virtual_diamond;
	Mixed mixed;
	Private private_object;
	Protected protected_object;
	Implementation implementation;
	WithPlain with_plain;
	Template<int> template_object;
	observe(root); observe(single); observe(multilevel); observe_other(other);
	observe(multiple); observe_other(multiple); observe(left); observe(right);
	observe(static_cast<Left&>(diamond)); observe(static_cast<Right&>(diamond));
	observe(virtual_single); observe(virtual_left); observe(virtual_right);
	observe(virtual_diamond);
	observe(static_cast<Left&>(mixed)); observe(static_cast<VirtualRight&>(mixed));
	observe(*private_object.base()); observe(*protected_object.base());
	observe_other(implementation); observe(with_plain); observe(template_object);
	Interface* interface_pointer = &implementation;
	Root* root_pointer = &multiple;
	Other* cross_cast = dynamic_cast<Other*>(root_pointer);
	Root* shared_left = static_cast<VirtualLeft*>(&virtual_diamond);
	Root* shared_right = static_cast<VirtualRight*>(&virtual_diamond);
	Root* repeated_left = static_cast<Left*>(&diamond);
	Root* repeated_right = static_cast<Right*>(&diamond);
	bool ok = cross_cast == static_cast<Other*>(&multiple)
		&& dynamic_cast<Multiple*>(root_pointer) == &multiple
		&& dynamic_cast<Single*>(root_pointer) == nullptr
		&& shared_left == shared_right && repeated_left != repeated_right
		&& dynamic_cast<Private*>(private_object.base()) == nullptr
		&& dynamic_cast<Protected*>(protected_object.base()) == nullptr
		&& interface_pointer->execute() == 80;
	std::puts(ok ? "inheritance checks passed" : "inheritance checks FAILED");
	return ok ? 0 : 1;
}

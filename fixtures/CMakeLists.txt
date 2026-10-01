cmake_minimum_required(VERSION 3.26)
project(pelite_inheritance_fixture LANGUAGES CXX)

if(NOT WIN32 OR NOT CMAKE_CXX_COMPILER_ID STREQUAL "MSVC")
    message(FATAL_ERROR "This fixture requires MSVC on Windows")
endif()

if(NOT CMAKE_CONFIGURATION_TYPES AND NOT CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE Release CACHE STRING "Build configuration" FORCE)
endif()

if(CMAKE_SIZEOF_VOID_P EQUAL 8)
    set(fixture_arch x64)
elseif(CMAKE_SIZEOF_VOID_P EQUAL 4)
    set(fixture_arch x86)
else()
    message(FATAL_ERROR "This fixture currently supports x86 and x64 only")
endif()

add_executable(inheritance inheritance.cpp)
target_compile_features(inheritance PRIVATE cxx_std_17)
set_target_properties(inheritance PROPERTIES
    CXX_EXTENSIONS OFF
    OUTPUT_NAME "inheritance-${fixture_arch}"
    MSVC_RUNTIME_LIBRARY MultiThreaded
    INTERPROCEDURAL_OPTIMIZATION FALSE)

target_compile_options(inheritance PRIVATE /EHsc /GR /Od /Ob0 /W4 /WX)
target_link_options(inheritance PRIVATE
    "LINKER:/INCREMENTAL:NO" "LINKER:/OPT:NOICF" "LINKER:/DYNAMICBASE"
    "LINKER:/FIXED:NO" "LINKER:/Brepro")

set(fixture_manifest "${CMAKE_CURRENT_BINARY_DIR}/$<CONFIG>/inheritance-${fixture_arch}.json")
add_custom_command(TARGET inheritance POST_BUILD
    COMMAND "${CMAKE_COMMAND}"
        "-DBINARY=$<TARGET_FILE:inheritance>"
        "-DSOURCE=${CMAKE_CURRENT_SOURCE_DIR}/inheritance.cpp"
        "-DOUTPUT=${fixture_manifest}"
        "-DARCH=${fixture_arch}"
        "-DCOMPILER_ID=${CMAKE_CXX_COMPILER_ID}"
        "-DCOMPILER_VERSION=${CMAKE_CXX_COMPILER_VERSION}"
        "-DFRONTEND=${CMAKE_CXX_COMPILER_FRONTEND_VARIANT}"
        "-DTARGET_SYSTEM=${CMAKE_SYSTEM_NAME}"
        "-DCONFIG=$<CONFIG>"
        -P "${CMAKE_CURRENT_SOURCE_DIR}/cmake/manifest.cmake"
    VERBATIM)

include(CTest)
if(BUILD_TESTING)
    add_test(NAME inheritance-runtime COMMAND inheritance)
    set_tests_properties(inheritance-runtime PROPERTIES PASS_REGULAR_EXPRESSION "inheritance checks passed")
endif()

# Build in an isolated directory; explicitly install to refresh committed fixtures.
install(TARGETS inheritance RUNTIME DESTINATION bin)
install(FILES "${fixture_manifest}" DESTINATION bin)

cmake_minimum_required( VERSION 3.24 )

project( AngelScript_C LANGUAGES CXX C )

add_library( AngelScriptC STATIC src/angelscript_c.cpp src/angelscript_c.h )

add_subdirectory( "${CMAKE_SOURCE_DIR}/vendor/sdk/angelscript/projects/cmake" )

target_link_libraries( AngelScriptC angelscript )


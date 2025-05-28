cmake_minimum_required(VERSION 3.5)
project(libCombineWrapper LANGUAGES C CXX)

# Enable both C and CXX languages before including libCombine
enable_language(C)
enable_language(CXX)

# Add the libCombine subdirectory
add_subdirectory(${CMAKE_CURRENT_SOURCE_DIR}/../../submodules/libCombine libcombine) 
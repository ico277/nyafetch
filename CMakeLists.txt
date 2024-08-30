cmake_minimum_required(VERSION 3.5)
project(nyafetch)

set(CMAKE_CXX_STANDARD 17)

## options for features
option(ENABLE_PCI "Enable GPU info using libpci" true)
option(STATIC "Static build - clashes with ENABLE_PCI" false)
option(DEBUG "Build in debug mode" false)

if(DEBUG)
    add_definitions(-DDEBUG -g)
endif()

if(STATIC AND ENABLE_PCI)
    message(FATAL_ERROR "Cannot build statically with libpci enabled!")
endif()

# dependencies 
if(ENABLE_PCI)
    add_definitions(-DPCI)
    find_package(PkgConfig REQUIRED)
    pkg_check_modules(PCILIB REQUIRED libpci)
    include_directories(${PCILIB_INCLUDE_DIRS})
endif()
if(STATIC)
    set(CMAKE_EXE_LINKER_FLAGS "-static")
endif()
include(FetchContent)
FetchContent_Declare(
    tomlplusplus
    GIT_REPOSITORY https://github.com/marzer/tomlplusplus.git
    GIT_TAG v3.4.0
)
FetchContent_MakeAvailable(tomlplusplus)
# executable
add_definitions(-O2)
add_executable(nyafetch src/nyafetch.cpp)
target_link_libraries(nyafetch ${PCILIB_LIBRARIES})
target_link_libraries(nyafetch tomlplusplus::tomlplusplus)
# installation
install(TARGETS nyafetch
    RUNTIME DESTINATION bin
)
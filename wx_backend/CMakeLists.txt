cmake_minimum_required(VERSION 3.0)
project(wx_backend)

# Find the wxWidgets library
find_package(wxWidgets REQUIRED)
include(${wxWidgets_USE_FILE})

# Add the source files
file(GLOB SOURCE_FILES "wx_backend.cpp")

# Create the wx_backend library, link the wxWidgets library, set the include directories and output directories
add_library(wx_backend SHARED ${SOURCE_FILES})
execute_process(COMMAND wx-config --libs OUTPUT_VARIABLE wx_libs OUTPUT_STRIP_TRAILING_WHITESPACE)
target_link_libraries(wx_backend PRIVATE ${wx_libs})
execute_process(COMMAND wx-config --cxxflags OUTPUT_VARIABLE wx_cxxflags OUTPUT_STRIP_TRAILING_WHITESPACE)
set(wx_cxxflags "${wx_cxxflags} -std=c++11")
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} ${wx_cxxflags}")

install(TARGETS wx_backend DESTINATION .)
cmake_minimum_required(VERSION 3.24)
project(topdown_game)

set(CMAKE_CXX_STANDARD 17)

include_directories(${STB_LIBRARY})
add_executable(topdown_game src/main.cpp ${STB_LIBRARY}/stb_image.h)

find_package(GLEW REQUIRED)
find_package(OpenGL REQUIRED)
find_package(glm REQUIRED)
find_package(glfw3 REQUIRED)
find_package(nlohmann_json REQUIRED)

target_link_libraries(topdown_game ${OPENGL_LIBRARIES} glfw GLEW::GLEW glm::glm nlohmann_json::nlohmann_json)
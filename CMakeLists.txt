cmake_minimum_required(VERSION 3.25)

project(shakva LANGUAGES C)

if(NOT DEFINED CMAKE_C_STANDARD)
    set(CMAKE_C_STANDARD 23)
endif()

set(CMAKE_C_STANDARD_REQUIRED ON)

add_subdirectory(src)

set(SHAKVA_SRC main.c)

add_executable(shakva ${SHAKVA_SRC})
target_include_directories(shakva PRIVATE ${CMAKE_SOURCE_DIR}/include)

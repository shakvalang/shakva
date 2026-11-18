set(SHAKVA_SRC main.c driver.c parse.c)

add_executable(shakva ${SHAKVA_SRC})
target_include_directories(shakva PRIVATE ${CMAKE_SOURCE_DIR}/include)
target_link_libraries(shakva PRIVATE sds stb)

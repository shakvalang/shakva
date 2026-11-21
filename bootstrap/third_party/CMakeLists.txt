include(FetchContent)

FetchContent_Declare(
    sds
    GIT_REPOSITORY https://github.com/antirez/sds
    GIT_TAG        master
)
FetchContent_MakeAvailable(sds)
add_library(sds STATIC ${sds_SOURCE_DIR}/sds.c)
target_include_directories(sds PUBLIC ${sds_SOURCE_DIR})

FetchContent_Declare(
    stb
    GIT_REPOSITORY https://github.com/nothings/stb.git
    GIT_TAG        master
)
FetchContent_MakeAvailable(stb)
add_library(stb STATIC stb_impl.c)
target_include_directories(stb PUBLIC ${stb_SOURCE_DIR})

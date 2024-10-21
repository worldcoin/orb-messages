#
set(TARGET ORB_MCU_MESSAGING_LIB)

if (CONFIG_ORB_MCU_MESSAGING_LIB)
    # check that the proto compiler is installed
    find_program(PROTOC protoc)
    if (NOT PROTOC)
        message(FATAL_ERROR "'protoc' not found, please ensure protoc is installed\
 and in path. See https://docs.zephyrproject.org/latest/samples/modules/nanopb/README.html")
    endif ()

    # import nanopb function from the library itself
    if (NOT DEFINED WORKSPACE_DIR)
        get_filename_component(WORKSPACE_DIR ${ZEPHYR_BASE} DIRECTORY)
    endif ()
    set(CMAKE_MODULE_PATH ${CMAKE_MODULE_PATH} ${WORKSPACE_DIR}/modules/lib/nanopb/extra)
    find_package(Nanopb REQUIRED)

    set(NANOPB_OPTIONS "-I${CMAKE_CURRENT_SOURCE_DIR}/../messages/")

    set(SRC_FILES)
    # look for sec_priv.proto in private first, then in private_stub
    set(PRIV_INCLUDE_DIR ${CMAKE_CURRENT_SOURCE_DIR}/../messages/private)
    if (NOT EXISTS ${PRIV_INCLUDE_DIR}/sec_priv.proto)
        set(PRIV_INCLUDE_DIR ${CMAKE_CURRENT_SOURCE_DIR}/../messages/private_stub)
        if (NOT EXISTS ${PRIV_INCLUDE_DIR}/sec_priv.proto)
            message(FATAL_ERROR "sec_priv.proto not found in private or private_stub")
        endif ()
    endif ()
    # sec_priv.proto must be first in the list
    list(APPEND PROTO_FILES ${PRIV_INCLUDE_DIR}/sec_priv.proto ../messages/mcu.proto ../messages/main.proto ../messages/sec.proto ../messages/common.proto)
    list(APPEND SRC_FILES ${CMAKE_CURRENT_BINARY_DIR}/sec_priv.pb.c ${CMAKE_CURRENT_BINARY_DIR}/mcu.pb.c ${CMAKE_CURRENT_BINARY_DIR}/main.pb.c ${CMAKE_CURRENT_BINARY_DIR}/sec.pb.c ${CMAKE_CURRENT_BINARY_DIR}/common.pb.c)
    list(APPEND NANOPB_OPTIONS "-I${PRIV_INCLUDE_DIR}")

    # use provided function to generate C files
    # it will create a custom command that is launched if ${proto_sources} are dependencies
    nanopb_generate_cpp(proto_sources proto_headers
            ${PROTO_FILES}
    )

    # Files are not automatically generated so force
    # file generation if they are missing or if *.proto changes since last build
    add_custom_target(
            generate_proto_files
            DEPENDS
            ${SRC_FILES}
            ${PROTO_FILES}
            ${proto_headers}
    )

    zephyr_interface_library_named(${TARGET})

    target_include_directories(${TARGET} INTERFACE
            ${CMAKE_CURRENT_BINARY_DIR}
    )

    zephyr_library()
    zephyr_library_sources(
            ${SRC_FILES}
    )
    zephyr_library_link_libraries(${TARGET})
    target_link_libraries(${TARGET} INTERFACE zephyr_interface)
endif ()

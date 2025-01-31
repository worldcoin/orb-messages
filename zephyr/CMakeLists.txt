#
set(TARGET ORB_MCU_MESSAGING_LIB)

if (CONFIG_ORB_MCU_MESSAGING_LIB)
    # include nanopb to perform checks, see nanopb.cmake
    list(APPEND CMAKE_MODULE_PATH ${ZEPHYR_BASE}/modules/nanopb)
    include(nanopb)

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

    # see nanopb.cmake
    add_dependencies(nanopb_generated_headers generate_proto_files)

    zephyr_interface_library_named(${TARGET})
    zephyr_include_directories(${CMAKE_CURRENT_BINARY_DIR})
    zephyr_library_sources(
            ${SRC_FILES}
    )
    zephyr_link_libraries_ifdef(CONFIG_ORB_MCU_MESSAGING_LIB ${TARGET})
endif ()

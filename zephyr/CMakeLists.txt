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

    set(NANOPB_OPTIONS "-I${CMAKE_CURRENT_SOURCE_DIR}/../ -I${CMAKE_CURRENT_SOURCE_DIR}/../private")

    set(SRC_FILES)
    if (CONFIG_BOARD_PEARL_SECURITY OR CONFIG_BOARD_DIAMOND_SECURITY)
        # use private definitions if available
        file(GLOB_RECURSE PRIVATE_FILES ${CMAKE_CURRENT_SOURCE_DIR}/../private/*.proto ${CMAKE_CURRENT_SOURCE_DIR}/../private/*.options)
        if (PRIVATE_FILES)
            message(STATUS "Found private proto files, creating symlinks")
            foreach (PRIVATE_FILE ${PRIVATE_FILES})
                get_filename_component(PRIVATE_FILE_NAME ${PRIVATE_FILE} NAME)
                execute_process(COMMAND ${CMAKE_COMMAND} -E create_symlink ${PRIVATE_FILE} ${CMAKE_CURRENT_SOURCE_DIR}/../${PRIVATE_FILE_NAME})
            endforeach ()
        else ()
            message(WARNING "Private proto definitions not found for security board, using obfuscated definitions but expect errors.\nEnsure you have private definitions in ../private")
            execute_process(COMMAND ${CMAKE_COMMAND} -E create_symlink ${CMAKE_CURRENT_SOURCE_DIR}/../mcu_messaging_sec_obfuscated.proto ${CMAKE_CURRENT_SOURCE_DIR}/../mcu_messaging_sec_priv.proto)
        endif ()
        # ../private/mcu_messaging_sec_priv.proto must be first in the list
        list(APPEND PROTO_FILES ../private/mcu_messaging_sec_priv.proto ../mcu_messaging_sec.proto ../mcu_messaging_common.proto)
        list(APPEND SRC_FILES ${CMAKE_CURRENT_BINARY_DIR}/mcu_messaging_sec.pb.c ${CMAKE_CURRENT_BINARY_DIR}/mcu_messaging_common.pb.c ${CMAKE_CURRENT_BINARY_DIR}/mcu_messaging_sec_priv.pb.c)
    else ()
        list(APPEND PROTO_FILES ../mcu_messaging_main.proto ../mcu_messaging_common.proto)
        list(APPEND SRC_FILES ${CMAKE_CURRENT_BINARY_DIR}/mcu_messaging_main.pb.c ${CMAKE_CURRENT_BINARY_DIR}/mcu_messaging_common.pb.c)
    endif ()

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

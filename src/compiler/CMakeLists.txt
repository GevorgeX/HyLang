add_executable(Compiler compiler.cpp
        lexer.cpp
        lexer.h
        parser.cpp
        parser.h
)

add_custom_command(
        TARGET Compiler POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy
        ${CMAKE_SOURCE_DIR}/code.txt
        ${CMAKE_CURRENT_BINARY_DIR}/code.txt)
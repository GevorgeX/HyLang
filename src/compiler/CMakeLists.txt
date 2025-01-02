add_executable(Compiler compiler.cpp
        lexer/lexer.cpp
        lexer/lexer.h
        parser/parser.cpp
        parser/parser.h
        ast/node.h
        ast/visitor.cpp
        ast/visitor.h
        ast/nodes/integerExpNode.cpp
        ast/nodes/integerExpNode.h
        ast/nodes/binaryExpNode.cpp
        ast/nodes/binaryExpNode.h
        ast/operations.h
        ast/nodes/unaryExpNode.cpp
        ast/nodes/unaryExpNode.h
)

add_custom_command(
        TARGET Compiler POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy
        ${CMAKE_SOURCE_DIR}/code.txt
        ${CMAKE_CURRENT_BINARY_DIR}/code.txt)
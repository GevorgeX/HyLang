#include <fstream>
#include <iostream>
#include <sstream>

#include "ast/nodes/binaryExpNode.h"
#include "ast/nodes/integerExpNode.h"
#include "ast/nodes/unaryExpNode.h"
#include "lexer/lexer.h"
#include "parser/parser.h"

void print_ast( Node& root,std::string ident = "") {
    #define instanceOf(type, target) auto* exp = dynamic_cast<type*>(&target)
    std::cout << ident <<"+- ";
    ident.append("|  ");

    if (instanceOf(IntegerExpNode,root)) {
        std::cout << "Integer: " << exp->value <<"\n";
    }
    else if (instanceOf(BinaryExpNode,root)) {
        std::cout<<"Binary op: "<<exp->opType<<"\n";
        print_ast(*exp->left,ident);
        print_ast(*exp->right,ident);
    }
    else if (instanceOf(UnaryExpNode,root)) {
        std::cout<<"Unary op: "<<exp->opType<<"\n";
        print_ast(*exp->value,ident);
    }
}

int main(int argc, char **argv) {
    std::ifstream file;
    std::stringstream  buffer;
    std::string code;

    file.open("code.txt");
    if (!file.is_open()) {
        std::cerr << "Failed to open file" << std::endl;
        return 1;
    }
    buffer << file.rdbuf();
    code = buffer.str();

    file.close();
    Lexer lexer(code);
    auto tokens = lexer.parse();
    for (auto token : tokens) {
        std::cout << token<< "\n";
    }

    Parser parser(tokens, code);
    auto t = parser.parse();

    print_ast( *t);
}

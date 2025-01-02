#include <fstream>
#include <iostream>
#include <sstream>

#include "lexer/lexer.h"

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
}
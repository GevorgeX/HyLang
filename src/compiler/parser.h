#ifndef PARSER_H
#define PARSER_H
#include <vector>

#include "lexer.h"


class Parser {
    int index;
    std::vector<Token>& tokens;

    void require_token(TokenType type);
public:
    explicit Parser(std::vector<Token>&);
    void parse();
};



#endif //PARSER_H

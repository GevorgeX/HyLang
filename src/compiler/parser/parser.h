#ifndef PARSER_H
#define PARSER_H
#include <vector>

#include "../ast/node.h"
#include "../lexer/lexer.h"

#include
#include


class Parser {
    int index;
    std::vector<Token>& tokens;
    std::string& chars;


    std::unique_ptr<Node> primary();
    std::unique_ptr<Node> unary();
    std::unique_ptr<Node> multiplicative();
    std::unique_ptr<Node> additive();
    std::unique_ptr<Node> conditional();
    std::unique_ptr<Node> expression();
    void require_token(TokenType type);
public:
    explicit Parser(std::vector<Token>&, std::string&);
    std::unique_ptr<Node> parse();
};



#endif //PARSER_H

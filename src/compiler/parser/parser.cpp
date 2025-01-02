#include "parser.h"

#include "../ast/nodes/integerExpNode.h"
#include "../ast/nodes/unaryExpNode.h"

std::unique_ptr<Node> Parser::primary()
{
    auto token = tokens[index++];

    switch (token.type)
    {
    case TokenType::NUMBER:
        int number =std::atoi(chars.substr(token.start, token.end - token.start).c_str());
        return std::make_unique<Node>(IntegerExpNode(number));
    default: //throw std::runtime_error("Invalid token");
    }
}

std::unique_ptr<Node> Parser::unary()
{
    auto token = tokens[index++];
    std::unique_ptr<Node> res;
    switch (token.type)
    {
    case TokenType::PLUS:
        res = primary();
        return res;
    case TokenType::MINUS:
        res = primary();
        return std::make_unique<UnaryExpNode>(std::move(res),UnaryOperation::Negate));
    default: //throw std::runtime_error("Invalid token");
    }
}

std::unique_ptr<Node> Parser::multiplicative()
{
}

std::unique_ptr<Node> Parser::additive()
{
}

std::unique_ptr<Node> Parser::conditional()
{
}

std::unique_ptr<Node> Parser::expression()
{

}

void Parser::require_token(TokenType type)
{
    auto token = tokens[index];
    if(token.type == type)
    {
        index++;
        return;
    }

    //throw
}

Parser::Parser(std::vector<Token>& tokens, std::string& chars): tokens(tokens), chars(chars), index(0)
{
}

std::unique_ptr<Node> Parser::parse()
{
    return expression();
}

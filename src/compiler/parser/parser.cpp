#include "parser.h"

#include "../ast/nodes/binaryExpNode.h"
#include "../ast/nodes/integerExpNode.h"
#include "../ast/nodes/unaryExpNode.h"

std::unique_ptr<Node> Parser::primary()
{
    auto token = tokens[index++];
    int number = 0;
    std::unique_ptr<Node> expr;
    switch (token.type)
    {
    case TokenType::NUMBER:
        number = std::atoi(chars.substr(token.start, token.end - token.start).c_str());
        return std::make_unique<IntegerExpNode>(number);
    case TokenType::LEFT_PARENTHESIS:
        expr = expression();
        require_token(TokenType::RIGHT_PARENTHESIS);
        return expr;
    // default: break;//throw std::runtime_error("Invalid token");
    }
}

std::unique_ptr<Node> Parser::unary()
{
    auto token = tokens[index];
    std::unique_ptr<Node> res;
    switch (token.type)
    {
    case TokenType::MINUS:
        index++;
        res = primary();
        return std::make_unique<UnaryExpNode>(std::move(res),UnaryOperation::Negate);
    default: return primary();
    }
}

std::unique_ptr<Node> Parser::multiplicative()
{
    std::unique_ptr<Node> res = unary();
    std::unique_ptr<Node> un;
    while (true) {
        auto token = tokens[index];
        switch (token.type)
        {
            case TokenType::STAR:
                index++;
                un = unary();
                res = std::make_unique<BinaryExpNode>(std::move(res), std::move(un), BinaryOperation::Multiply);
                break;
            case TokenType::SLASH:
                index++;
                un = unary();
                res = std::make_unique<BinaryExpNode>(std::move(res), std::move(un), BinaryOperation::Divide);
                break;
            default: return res;
        }
    }

}

std::unique_ptr<Node> Parser::additive()
{
    std::unique_ptr<Node> res = multiplicative();
    std::unique_ptr<Node> mul;
    while (true) {
        auto token = tokens[index];
        switch (token.type)
        {
            case TokenType::PLUS:
                index++;
                mul = multiplicative();
                res = std::make_unique<BinaryExpNode>(std::move(res), std::move(mul), BinaryOperation::Add);
                break;
            case TokenType::MINUS:
                index++;
                mul = multiplicative();
                res = std::make_unique<BinaryExpNode>(std::move(res), std::move(mul), BinaryOperation::Subtract);
                break;
            default: return res;
        }
    }
}

std::unique_ptr<Node> Parser::conditional()
{
    return additive();
}

std::unique_ptr<Node> Parser::expression()
{
    return conditional();
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

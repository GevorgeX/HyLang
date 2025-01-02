#include "lexer.h"

#include <iostream>
#include <locale>

std::string tokenTypeToString(TokenType type) {
    switch (type) {
        case EOF_: return "EOF_";
        case WORD: return "WORD";
        case NUMBER: return "NUMBER";
        case TRUE: return "TRUE";
        case FALSE: return "FALSE";
        case PLUS: return "PLUS";
        case MINUS: return "MINUS";
        case STAR: return "STAR";
        case SLASH: return "SLASH";
        case LEFT_PARENTHESIS: return "LEFT_PARENTHESIS";
        case RIGHT_PARENTHESIS: return "RIGHT_PARENTHESIS";
        case LEFT_BRACE: return "LEFT_BRACE";
        case RIGHT_BRACE: return "RIGHT_BRACE";
        case EQUAL: return "EQUAL";
        case DOUBLE_EQUAL: return "DOUBLE_EQUAL";
        case IF: return "IF";
        case ELSE: return "ELSE";
        case AND: return "AND";
        case OR: return "OR";
        case LESS: return "LESS";
        case LESS_EQUAL: return "LESS_EQUAL";
        case GREATER: return "GREATER";
        case GREATER_EQUAL: return "GREATER_EQUAL";
        case NOT: return "NOT";
        case NOT_EQUAL: return "NOT_EQUAL";
        case WHILE: return "WHILE";
        case BREAK: return "BREAK";
        case CONTINUE: return "CONTINUE";
        case PLUS_EQUAL: return "PLUS_EQUAL";
        case MINUS_EQUAL: return "MINUS_EQUAL";
        case STAR_EQUAL: return "STAR_EQUAL";
        case SLASH_EQUAL: return "SLASH_EQUAL";
        default: return "UNKNOWN";
    }
}

std::ostream & operator<<(std::ostream &os, const Token &token) {
    os <<"{ "<<"type: "<<tokenTypeToString(token.type)<<" start: "<<token.start<<" end: "
    <<token.end<<" line: "<<token.line<<" }";
    return os;
}

bool Lexer::is_end() {
    return index >= chars.size();
}

Token Lexer::make_token(TokenType type) {
    return Token{type,start, index, line};
}

bool Lexer::match(char c) {
    if (is_end())
        return false;
    if (chars[index] == c) {
        index++;
        return true;
    }

    return false;
}

void Lexer::skip_ws() {
    while (!is_end()) {
        switch (chars[index]) {
            case ' ':
            case '\t':
            case '\r':
                index++;
                break;
            case '\n':
                index++;
                line++;
                break;
            default: return;;
        }
    }
}

Token Lexer::number() {
    while (!is_end() && isdigit(chars[index])) {
        index++;
    }

    if (!is_end() && chars[index] == '.' && isdigit(chars[index + 1])) {
        index++;
        while (!is_end() && isdigit(chars[index])) {
            index++;
        }
    }

    return make_token(TokenType::NUMBER);
}

Token Lexer::ident() {
    while (!is_end() &&
        (isalpha(chars[index]) || isdigit(chars[index]))
        ) {
        index++;
    }

    TokenType type;
    auto word = chars.substr(start, index - start);

        if(word ==  "ճիշտ")
            type = TokenType::TRUE;
        else if(word == "եթե")
            type = TokenType::IF;
        else if(word == "ուրիշ")
            type = TokenType::ELSE;
        else if(word == "կեխծ")
            type = TokenType::FALSE;
        else if(word == "և")
            type = TokenType::AND;
        else if(word == "կամ")
            type = TokenType::OR;
        else if(word == "ցիկլ")
            type = TokenType::WHILE;
        else if(word == "կայնի")
            type = TokenType::BREAK;
        else if(word == "շարունակի")
            type = TokenType::CONTINUE;
        else
            type = TokenType::WORD;

        return make_token(type);

}

Lexer::Lexer(std::string& chars):chars(chars), start(0), index(0), line(1) {}

std::vector<Token> Lexer::parse() {
    std::vector<Token> tokens;
    while (true) {
        skip_ws();

        if (is_end())
            break;
        start = index;

        switch (chars[index++]) {
            case '+':
                if (match('='))
                    tokens.push_back(make_token(TokenType::PLUS_EQUAL));
                else
                    tokens.push_back(make_token(TokenType::PLUS));
                break;

            case '-':
                if (match('='))
                    tokens.push_back(make_token(TokenType::MINUS_EQUAL));
                else
                    tokens.push_back(make_token(TokenType::MINUS));
                break;
            case '*':
                if (match('='))
                    tokens.push_back(make_token(TokenType::STAR_EQUAL));
                else
                    tokens.push_back(make_token(TokenType::STAR));
                break;
            case '/':
                if (match('='))
                    tokens.push_back(make_token(TokenType::SLASH_EQUAL));
                else
                    tokens.push_back(make_token(TokenType::SLASH));
                break;
            case '(':
                tokens.push_back(make_token(TokenType::LEFT_PARENTHESIS));
                break;
            case ')':
                tokens.push_back(make_token(TokenType::RIGHT_PARENTHESIS));
                break;
            case '=':
                if (match('='))
                    tokens.push_back(make_token(TokenType::DOUBLE_EQUAL));
                else
                    tokens.push_back(make_token(TokenType::EQUAL));
                break;
            case '!':
                if (match('='))
                    tokens.push_back(make_token(TokenType::NOT_EQUAL));
                else
                    tokens.push_back(make_token(TokenType::NOT));
                break;
            case '<':
                if (match('='))
                    tokens.push_back(make_token(TokenType::LESS_EQUAL));
                else
                    tokens.push_back(make_token(TokenType::LESS));
                break;
            case '{':
                tokens.push_back(make_token(TokenType::LEFT_BRACE));
                break;
            case '}':
                tokens.push_back(make_token(TokenType::RIGHT_BRACE));
                break;
            default:
                char chr = chars[index-1];
                if (std::isdigit(chr) ) {
                    auto token = number();
                    tokens.push_back(token);
                }
                else if (std::isalpha(chr) || chr == '_') {
                    auto token = ident();
                    tokens.push_back(token);
                }
                else {
                    //throw
                    std::cerr<<"EROR";
                }
        }
    }
    tokens.push_back(make_token(TokenType::EOF_));
    return tokens;
}

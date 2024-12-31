#ifndef LEXER_H
#define LEXER_H
#include <string>
#include <vector>

enum TokenType {
    EOF_ = 0,
    WORD ,
    NUMBER,
    TRUE,
    FALSE,
    PLUS,
    MINUS,
    STAR,
    SLASH,
    LEFT_PARENTHESIS,
    RIGHT_PARENTHESIS,
    LEFT_BRACE,
    RIGHT_BRACE,
    EQUAL,
    DOUBLE_EQUAL,
    IF,
    ELSE,
    AND,
    OR,
    LESS,
    LESS_EQUAL,
    GREATER,
    GREATER_EQUAL,
    NOT,
    NOT_EQUAL,
    WHILE,
    BREAK,
    CONTINUE,
    PLUS_EQUAL,
    MINUS_EQUAL,
    STAR_EQUAL,
    SLASH_EQUAL,
};

struct Token {
    TokenType type;
    int start;
    int end;
    int line;
};

std::ostream& operator<<(std::ostream&, const Token&);

class Lexer {
    int index;
    int start;
    int line;
    std::string& chars;

    bool is_end();
    Token make_token(TokenType);
    bool match(char);
    void skip_ws();
    Token number();
    Token ident();
public:
    explicit Lexer(std::string&);
    std::vector<Token> parse();
};



#endif //LEXER_H

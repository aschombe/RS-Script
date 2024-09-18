enum class Var_Types {
    INT,
    FLOAT,
    BOOL,
    STRING
}

std::string to_string(Var_Types type) {
    switch (type) {
        case Var_Types::INT:
            return "int";
        case Var_Types::FLOAT:
            return "float";
        case Var_Types::BOOL:
            return "bool";
        case Var_Types::STRING:
            return "string";
    }
}

enum class Func_Types {
    VOID,
    INT,
    FLOAT,
    BOOL,
    STRING
}

std::string to_string(Func_Types type) {
    switch (type) {
        case Func_Types::VOID:
            return "void";
        case Func_Types::INT:
            return "int";
        case Func_Types::FLOAT:
            return "float";
        case Func_Types::BOOL:
            return "bool";
        case Func_Types::STRING:
            return "string";
    }
}
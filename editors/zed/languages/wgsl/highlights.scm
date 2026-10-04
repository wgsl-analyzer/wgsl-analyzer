; Catch-all: prevents outer language string color from bleeding through injections
(identifier) @variable

; Comments
(line_comment) @comment
(block_comment) @comment

; Types - Built-ins
; Scalar types
((identifier) @type
  (#any-of? @type "bool" "i32" "u32" "f32" "i64" "u64" "f64"))

; Vector types
((identifier) @type
  (#match? @type "^vec[2-4][fihu]?$"))

; Matrix types
((identifier) @type
  (#match? @type "^mat[2-4]x[2-4][fh]?$"))

; Special built-in types
((identifier) @type
  (#any-of? @type "atomic" "array" "texture" "sampler"))

; Custom types (capitalized identifiers)
((identifier) @type
  (#match? @type "^[A-Z]"))

; Struct declarations
(struct_declaration
  name: (_) @type)

; Constants
(bool_literal) @boolean
(int_literal) @number
(hex_int_literal) @number
(float_literal) @number

((identifier) @constant
  (#match? @constant "^[A-Z0-9_]+$"))

; Functions
(function_declaration
  (function_header
    (identifier) @function))

(call_expression
  (identifier) @function.call)

(func_call_statement
  (identifier) @function.call)

; Templates
(template_list) @punctuation

; Storage modifiers - FIXED
(variable_declaration
  (template_list
    (identifier) @keyword
    (#any-of? @keyword "private" "storage" "uniform" "workgroup" "read" "write" "read_write")))

; Type templates
(type_specifier
  (template_list
    (identifier) @type))

; Attributes
(attribute
  (identifier) @attribute) @attribute

(attribute
  (identifier) @attribute
  (argument_list
    (identifier) @variable.builtin)
  (#eq? @attribute "builtin"))

; Variables
(param
  (identifier) @variable.parameter)

(variable_declaration
  (identifier) @variable)

(struct_member
  name: (_) @property)

(named_component_expression
  component: (_) @property)

; Control flow keywords
[
  "if"
  "else"
  "loop"
  "for"
  "while"
  "switch"
  "case"
  "default"
  "break"
  "continue"
  "continuing"
  "return"
  "discard"
] @keyword.control

; WESL extensions
[
  "import"
  "as"
] @keyword.control.import

; Declaration keywords
[
  "var"
  "let"
  "const"
  "override"
  "fn"
  "struct"
  "alias"
  "virtual"
  "diagnostic"
  "enable"
  "requires"
  "const_assert"
] @keyword


; Operators
[
  "-" "!" "~" "*" "&"  ; unary
  "^" "|" "/" "%" "+"  ; binary
  (shift_left) (shift_right)
] @operator

; Comparison operators
[
  (less_than) (greater_than)
  (less_than_equal) (greater_than_equal)
  "==" "!="
] @operator

; Assignment operators
[
  "+=" "-=" "*=" "/=" "%=" "|=" "^=" "++" "--" "="
] @operator

; Logical operators
[
  "&&" "||"
] @operator

; Punctuation
[ "(" ")" "[" "]" "{" "}" ] @punctuation.bracket
[ "," "." ":" ";" "->" ] @punctuation.delimiter

; Preprocessor
[ (preproc_directive) "#import" ] @keyword.directive

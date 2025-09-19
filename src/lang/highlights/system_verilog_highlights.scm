; Comments
(comment) @comment
(enum_name_declaration) @constant
(simple_identifier) @variable
(variable_lvalue) @variable
[(system_tf_identifier)
  (edge_identifier)
  (event_expression)
]

@keyword
(constant_primary) @constant
(primary) @number
(always_keyword) @function
; Strings
(string_literal) @string
(quoted_string) @string
(system_lib_string) @string

; Keywords
[
  "begin" "end" "this"
  "input" "output" "inout" "ref"
  "alias" "and" "assert" "assign" "assume" "before" "bind"
  "break" "case" "checker" "class" "clocking" "config" "const"
  "constraint" "cover" "covergroup" "coverpoint" "cross"
  "default" "defparam" "disable" "do" "else" "endcase" "endclass"
  "endclocking" "endfunction" "endgenerate" "endgroup"
  "endinterface" "endmodule" "endpackage" "endprogram"
  "endproperty" "endsequence" "endtask" "enum" "extends"
  "extern" "final" "for" "foreach" "forever" "fork" "function"
  "generate" "genvar" "if" "iff" "import" "initial" "inside"
  "interface" "join" "join_any" "join_none" "local" "localparam"
  "modport" "new" "null" "package" "packed" "parameter"
  "priority" "program" "property" "protected" "pure" "rand"
  "randc" "release" "repeat" "return" "sequence" "soft" "solve"
  "static" "struct" "super" "tagged" "task" "timeprecision"
  "timeunit" "typedef" "union" "unique" "unique0" "unsigned"
  "virtual" "wait" "while" "with"
] @keyword

; Preprocessor
;[
;  "`include" "`define" "`ifdef" "`ifndef" "`endif" "`else" "`elsif"
;  "`timescale" "`default_nettype" "`undef" "`pragma"
;  "`__FILE__" "`__LINE__"
;] @constant

; Operators
[
  ";" ":" "," "::" "." "=" "?" "==" "!=" "===" "!==" "<" "<=" ">" ">="
  "+" "-" "*" "/" "%" "**" "&&" "||" "!" "~" "&" "|" "^" "~&" "~|" "~^"
  "<<" ">>" "<<<" ">>>"
  "@"
  "#"
  "->" "->>" "|->" "|=>"
] @operator

; Brackets / punctuation
[ "(" ")" "[" "]" "{" "}" ] @punctuation.bracket

; Numbers
[
  (decimal_number)
  (hex_number)
  (octal_number)
  (binary_number)
] @number

; Identifiers
;(identifier) @variable

; Types
[
  "bit" "logic" "reg" "byte" "shortint" "int" "longint" "integer" "time"
  "shortreal" "real" "realtime" "string" "event" "chandle"
  "signed" "unsigned"
] @type

; System tasks / functions
[
  "$fatal" "$error" "$warning" "$info"
  "$stop" "$finish" "$exit"
] @function.builtin

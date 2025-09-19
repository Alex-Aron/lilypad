module adder4 (
    input  logic [3:0] a,
    input  logic [3:0] b,
    output logic [3:0] sum,
    output logic carry

);
    logic [4:0] inner_sum
    assign inner_sum = a + b;
    assign carry = inner_sum[4];
    assign sum = inner_sum[3:0];
endmodule
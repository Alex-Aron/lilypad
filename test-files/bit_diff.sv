////////////////////////////////////////////////////////////////////////////
// FSMD implementations
// Source: Greg Stitt, University of Florida: https://github.com/ARC-Lab-UF/sv-tutorial/blob/main/fsmd/bit_diff.sv
// Module: bit_dff_fsmd_1p
// Description: A 1-process FSMD implementation of the calculator.
//
// See the FSMD illustration in bit_diff.pdf for a graphical representation
// of this module.

module bit_diff_fsmd_2p #(
    parameter int WIDTH
) (
    input  logic                                clk,
    input  logic                                rst,
    input  logic                                go,
    input  logic        [            WIDTH-1:0] data,
    output logic signed [$clog2(2*WIDTH+1)-1:0] result,
    output logic                                done
);
    typedef enum logic [1:0] {
        START,
        COMPUTE,
        RESTART,
        XXX = 'x
    } state_t;

    // For a 2-process FSMD, every register needs a variable for the output of
    // the register, which is the current value represented by the _r suffix,
    // and a variable for the input to the register (i.e., the value for the
    // next cycle), which is determined by combinational logic.
    state_t state_r, next_state;
    logic done_r, next_done;
    logic [$bits(data)-1:0] data_r, next_data;
    logic [$bits(result)-1:0] result_r, next_result;
    logic [$clog2(WIDTH)-1:0] count_r, next_count;
    logic signed [$bits(result)-1:0] diff_r, next_diff;

    assign result = result_r;
    assign done   = done_r;

    // The first process simply implements all the registers.
    always_ff @(posedge clk or posedge rst) begin
        if (rst) begin
            result_r <= '0;
            done_r   <= 1'b0;
            diff_r   <= '0;
            count_r  <= '0;
            data_r   <= '0;
            state_r  <= START;
        end else begin
            result_r <= next_result;
            done_r   <= next_done;
            diff_r   <= next_diff;
            count_r  <= next_count;
            data_r   <= next_data;
            state_r  <= next_state;
        end
    end

    // The second process implements any combinational logic, which includes
    // the inputs to all the registers, and any other combinational logic. For
    // example, in this module the done output is not registered like in the
    // the 1-process model. Although the 2-process model seems like overkill for
    // this example, the advantage is that you can control exactly what is
    // registered. For complex designs, registering everything is usually not
    // ideal, which makes the 2-process model useful.
    always_comb begin

        // Since this is combinational logic, we should never be assigning a
        // _r version of the signals. The left hand side should either be a next_
        // signal, or other variables that correspond to combinational logic.
        //
        // Here we assign default values to all the register inputs to make sure
        // we don't have latches. For a register, a good default value is usually
        // the current value because then we only have to assign the signal later
        // if the register is going to change.
        next_result = result_r;
        next_done   = done_r;
        next_diff   = diff_r;
        next_data   = data_r;
        next_count  = count_r;
        next_state  = state_r;

        case (state_r)
            START: begin
                next_done   = 1'b0;
                next_result = '0;
                next_diff   = '0;
                next_data   = data;
                next_count  = '0;

                // Without the default assignment at the beginning of the block,
                // this would result in a latch in the 2-process FSMD.
                if (go) begin
                    next_state = COMPUTE;
                end
            end

            COMPUTE: begin

                next_diff  = data_r[0] ? diff_r + 1 : diff_r - 1;
                next_data  = data_r >> 1;
                next_count = count_r + 1'b1;

                // Here, we could compare with next_count also and get rid of the
                // -1. However, that would be non-ideal for two reasons. First,
                // The addition for the count becomes an input to the comparator
                // without a register in between, which could increase the the
                // length of the critical path and slow down the clock. Second,
                // the count variable would need an extra bit for the new condition
                // to ever be true, which would increase the size of the adder, the
                // comparator, and the register. 
                if (count_r == WIDTH - 1) begin
                    next_state  = RESTART;

                    // This looks potentially like a combinational loop because
                    // the always_comb block is sensitive to any variable that
                    // appears on the RHS of a statement. Because the block assigns
                    // next_diff while being sensitive to next_diff, this could loop
                    // forever, causing the simualtor to kill the job.
                    // However, the SystemVerilog recognizes this risk and does NOT
                    // make an always_comb sensitive to any variable assigned within
                    // the block.
                    next_result = next_diff;
                    next_done   = 1'b1;
                end
            end

            // The restart state here doesn't really do anything differently than
            // the start state, so we could easily combine them like before.
            RESTART: begin
                next_diff  = '0;
                next_count = '0;
                next_data  = data;

                if (go) begin
                    // We have to clear done here to ensure the register updates
                    // one cycle after go is asserted.
                    next_done  = 1'b0;
                    next_state = COMPUTE;
                end
            end

            default: next_state = XXX;
        endcase
    end
endmodule
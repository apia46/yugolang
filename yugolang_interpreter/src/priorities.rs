pub mod binary_op {
    // remember that these also take up the slot above them
    pub const ADD_SUB:i64 = -6;
    pub const MUL_DIV:i64 = -4;
    pub const COMPARE:i64 = -8;
}

pub mod control_flow {
    pub const IF:i64 = -1;
}

pub const LET:i64 = -9;
pub const SET:i64 = -10;

pub const FN:i64 = 0;


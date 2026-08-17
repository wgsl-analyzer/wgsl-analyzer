use expect_test::expect;

use crate::tests::check_infer;

#[test]
#[expect(
    clippy::too_many_lines,
    clippy::non_ascii_literal,
    reason = "snapshot test data"
)]
fn microsoft_qdk() {
    check_infer(
        r#"
// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// See https://webgpufundamentals.org/webgpu/lessons/webgpu-wgsl.html for an overview
// See https://www.w3.org/TR/WGSL/ for the details
// See https://webgpu.github.io/webgpu-samples/ for examples

//#region Compile time replaced constants

// WGSL has pipeline overridables, but they're a pain and limited, so just string replace constants here
const QUBIT_COUNT: i32 = 8; // REPLACE
const RESULT_COUNT: u32 = 8; // REPLACE
const WORKGROUPS_PER_SHOT: i32 = 1; // REPLACE
const ENTRIES_PER_THREAD: i32 = 5; // REPLACE
const THREADS_PER_WORKGROUP: i32 = 32; // REPLACE
const MAX_QUBIT_COUNT: i32 = 27; // REPLACE
const MAX_QUBITS_PER_WORKGROUP: i32 = 5; // REPLACE
const NOISE_TABLE_COUNT: u32 = 1; // REPLACE
const NOISE_ENTRY_COUNT: u32 = 1; // REPLACE
const MAX_REGISTERS: u32 = 256; // REPLACE
const MAX_MEMORY: u32 = 256; // REPLACE
const INSTRUCTIONS_SIZE: u32 = 0; // REPLACE
const BLOCK_TABLE_SIZE: u32 = 0; // REPLACE
const FUNCTION_TABLE_SIZE: u32 = 0; // REPLACE
const PHI_TABLE_SIZE: u32 = 0; // REPLACE
const SWITCH_CASES_SIZE: u32 = 0; // REPLACE
const CALL_ARGS_SIZE: u32 = 0; // REPLACE
const CONSTANT_DATA_SIZE: u32 = 0; // REPLACE

// Selects the adaptive (QIR bytecode interpreter) code paths when true, or the
// base (linear op-list) paths when false. String-replaced by the host per run.
// Because it is a `const`, the compiler folds the branches and eliminates the
// unused path, so there is no runtime cost to the shared kernels below.
const IS_ADAPTIVE: bool = false; // REPLACE

//#endregion

//#region Error codes

const ERR_INVALID_PROBS = 1u;
const ERR_INVALID_THREAD_TOTAL = 2u;
const ERR_CALL_STACK_OVERFLOW = 3u;
const ERR_CALL_STACK_UNDERFLOW = 4u;
const ERR_INVALID_INSTRUCTION = 5u;
const ERR_ALLOCA_OUT_OF_BOUNDS = 6u;
const ERR_MEMORY_OUT_OF_BOUNDS = 7u;
const ERR_UNSUPPORTED_LOSS_POLICY = 32u;

//#endregion

//#region Operation IDs
const OPID_ID      = 0u;
const OPID_RESETZ  = 1u;
const OPID_X       = 2u;
const OPID_Y       = 3u;
const OPID_Z       = 4u;
const OPID_H       = 5u;
const OPID_S       = 6u;
const OPID_SAdj    = 7u;
const OPID_T       = 8u;
const OPID_TAdj    = 9u;
const OPID_RX      = 12u;
const OPID_RY      = 13u;
const OPID_RZ      = 14u;
const OPID_CX      = 15u;
const OPID_CZ      = 16u;
const OPID_RXX     = 17u;
const OPID_RYY     = 18u;
const OPID_RZZ     = 19u;
const OPID_MZ      = 21u;
const OPID_MRESETZ = 22u;
const OPID_SWAP    = 24u;
const OPID_MAT1Q   = 25u;
const OPID_MAT2Q   = 26u;
const OPID_CY      = 29u;

const OPID_PAULI_NOISE_1Q = 128u;
const OPID_PAULI_NOISE_2Q = 129u;
const OPID_LOSS_NOISE = 130u;
const OPID_CORRELATED_NOISE = 131u;

// If the application of noise results in a custom matrix, it will have been stored in the shot buffer
// These OPIDs indicate to use that matrix and for how many qubits. (The qubit ids are in the original Op)
const OPID_SHOT_BUFF_1Q = 256u;
const OPID_SHOT_BUFF_2Q = 257u;

//#endregion

//#region Misc constants

// Tolerance for probabilities to sum to 1.0
const PROB_THRESHOLD: f32 = 0.0001;

// Always use 32 threads per workgroup for max concurrency on most current GPU hardware
const MAX_WORKGROUP_SUM_PARTITIONS: i32 = 1i << u32(MAX_QUBIT_COUNT - MAX_QUBITS_PER_WORKGROUP);

// Loss policy values. These are stamped onto a gate op's `q3` field by the host
// (see `LossPolicy::as_u32` on the Rust side) and tell the shader how to handle
// the gate when one of its operands is lost. `0` means "no policy stamped",
// which the shader treats the same as SKIP.
const LOSS_POLICY_SKIP              = 0u;
const LOSS_POLICY_PROPAGATE         = 1u;
const LOSS_POLICY_DEGRADE           = 2u;
const LOSS_POLICY_RESIDUAL_S_DAGGER = 3u;
const LOSS_POLICY_APPLY_ANYWAY      = 4u;

//#endregion

//#region Adaptive interpreter constants

const MAX_CLASSICAL_STEPS: u32 = 4096u;

// Status codes
const STATUS_RUNNING:          u32 = 0u;
const STATUS_QUANTUM_PENDING:  u32 = 1u;
const STATUS_TERMINATED:       u32 = 2u;
const STATUS_ERROR:            u32 = 3u;
const STATUS_YIELD:            u32 = 4u;

// pending_op_type values: 0 = gate, 1 = measure, 2 = reset, 3 = loss commit.
// A loss-commit pending op carries the lost qubit in pending_op_idx (not an
// ops-pool index) and is produced while draining pending_loss_mask. Its value
// must not collide with the gate/measure/reset types resolved in prepare_op.
const PENDING_OP_LOSS_COMMIT:  u32 = 3u;

// -----------------------------------------------------------------------------
// Adaptive interpreter — opcodes
// -----------------------------------------------------------------------------

// Shared opcode constants for the Adaptive Profile QIR bytecode interpreter.
//
// These constants define the bytecode encoding used by the Python AdaptiveProfilePass
// (emitter). Values must stay in sync with the Python ``_adaptive_opcodes.py`` file.
//
// Opcode word layout::
//
//     bits [7:0]   = primary opcode
//     bits [15:8]  = sub-opcode / condition code
//     bits [23:16] = flags
//
// Compose via bitwise OR: ``opcode | (sub << 8) | flag``
// Example: ``OP_ICMP | (ICMP_SLE << 8) | FLAG_SRC1_IMM``

// -- Flags (pre-shifted to bit 16+) ------------------------------------------
const FLAG_SRC0_IMM: u32 = 1 << 16;  // src0 field is an immediate value, not a register
const FLAG_SRC1_IMM: u32 = 1 << 17;  // src1 field is an immediate value, not a register
const FLAG_DST_IMM:  u32 = 1 << 18;  // dst  field is an immediate value, not a register
const FLAG_AUX0_IMM: u32 = 1 << 19;  // aux0 field is an immediate value, not a register
const FLAG_AUX1_IMM: u32 = 1 << 20;  // aux1 field is an immediate value, not a register
const FLAG_AUX2_IMM: u32 = 1 << 21;  // aux2 field is an immediate value, not a register
const FLAG_AUX3_IMM: u32 = 1 << 22;  // aux3 field is an immediate value, not a register

// -- Control Flow -------------------------------------------------------------
const OP_NOP:           u32 = 0x00;
const OP_RET:           u32 = 0x02;
const OP_JUMP:          u32 = 0x04;
const OP_BRANCH:        u32 = 0x05;
const OP_SWITCH:        u32 = 0x06;
const OP_CALL:          u32 = 0x07;
const OP_CALL_RETURN:   u32 = 0x08;

// -- Quantum ------------------------------------------------------------------
const OP_QUANTUM_GATE:  u32 = 0x10;
const OP_MEASURE:       u32 = 0x11;
const OP_RESET:         u32 = 0x12;
const OP_READ_RESULT:   u32 = 0x13;
const OP_RECORD_OUTPUT: u32 = 0x14;
const OP_READ_LOSS:     u32 = 0x15;

// -- Integer Arithmetic -------------------------------------------------------
const OP_ADD:           u32 = 0x20;
const OP_SUB:           u32 = 0x21;
const OP_MUL:           u32 = 0x22;
const OP_UDIV:          u32 = 0x23;
const OP_SDIV:          u32 = 0x24;
const OP_UREM:          u32 = 0x25;
const OP_SREM:          u32 = 0x26;

// -- Bitwise / Shift ---------------------------------------------------------
const OP_AND:           u32 = 0x28;
const OP_OR:            u32 = 0x29;
const OP_XOR:           u32 = 0x2A;
const OP_SHL:           u32 = 0x2B;
const OP_LSHR:          u32 = 0x2C;
const OP_ASHR:          u32 = 0x2D;

// -- Comparison ---------------------------------------------------------------
const OP_ICMP:          u32 = 0x30;
const OP_FCMP:          u32 = 0x31;

// -- Float Arithmetic ---------------------------------------------------------
const OP_FADD:          u32 = 0x38;
const OP_FSUB:          u32 = 0x39;
const OP_FMUL:          u32 = 0x3A;
const OP_FDIV:          u32 = 0x3B;
const OP_FREM:          u32 = 0x3C;

// -- Type Conversion ----------------------------------------------------------
const OP_ZEXT:          u32 = 0x40;
const OP_SEXT:          u32 = 0x41;
const OP_TRUNC:         u32 = 0x42;
const OP_FPEXT:         u32 = 0x43;
const OP_FPTRUNC:       u32 = 0x44;
const OP_INTTOPTR:      u32 = 0x45;
const OP_FPTOSI:        u32 = 0x46;
const OP_SITOFP:        u32 = 0x47;
const OP_FPTOUI:        u32 = 0x48;
const OP_UITOFP:        u32 = 0x49;

// -- SSA / Data Movement -----------------------------------------------------
const OP_PHI:           u32 = 0x50;
const OP_SELECT:        u32 = 0x51;
const OP_MOV:           u32 = 0x52;
const OP_CONST:         u32 = 0x53;

// -- Memory Operations --------------------------------------------------------
const OP_ALLOCA:        u32 = 0x60;
const OP_LOAD:          u32 = 0x61;
const OP_STORE:         u32 = 0x62;
const OP_GEP:           u32 = 0x63;

// -- ICmp condition codes (sub-opcode, placed in bits[15:8] via << 8) ---------
// Reference: https://llvm.org/docs/LangRef.html#icmp-instruction
const ICMP_EQ:          u32 = 0;
const ICMP_NE:          u32 = 1;
const ICMP_SLT:         u32 = 2;
const ICMP_SLE:         u32 = 3;
const ICMP_SGT:         u32 = 4;
const ICMP_SGE:         u32 = 5;
const ICMP_ULT:         u32 = 6;
const ICMP_ULE:         u32 = 7;
const ICMP_UGT:         u32 = 8;
const ICMP_UGE:         u32 = 9;

// -- FCmp condition codes -----------------------------------------------------
// Reference: https://llvm.org/docs/LangRef.html#fcmp-instruction
const FCMP_FALSE:       u32 = 0;
const FCMP_OEQ:         u32 = 1;
const FCMP_OGT:         u32 = 2;
const FCMP_OGE:         u32 = 3;
const FCMP_OLT:         u32 = 4;
const FCMP_OLE:         u32 = 5;
const FCMP_ONE:         u32 = 6;
const FCMP_ORD:         u32 = 7;
const FCMP_UNO:         u32 = 8;
const FCMP_UEQ:         u32 = 9;
const FCMP_UGT:         u32 = 10;
const FCMP_UGE:         u32 = 11;
const FCMP_ULT:         u32 = 12;
const FCMP_ULE:         u32 = 13;
const FCMP_UNE:         u32 = 14;
const FCMP_TRUE:        u32 = 15;

// -- Sentinel values ----------------------------------------------------------
const VOID_RETURN:        u32 = 0xFFFFFFFF;  // Function does not have a return value.

//#endregion

//#region Data structures

struct WorkgroupSums {
    qubits: array<vec2f, MAX_QUBIT_COUNT>, // Each vec2f holds (zero_probability, one_probability)
};

struct WorkgroupCollationBuffer {
    sums: array<WorkgroupSums, MAX_WORKGROUP_SUM_PARTITIONS>,
};

struct QubitState {
    zero_probability: f32,
    one_probability: f32,
    heat: f32, // -1.0 = lost
    idle_since: f32,
}

// Used to track state for the random number generator per shot. See `next_rand_f32` later for details.
struct xorwow_state {
    counter: u32,
    x: array<u32, 5>
}

/// GPU bytecode instruction.
///
/// Layout:
/// - `opcode`: packed word — bits\[7:0\]=primary, bits\[15:8\]=sub/condition, bits\[23:16\]=flags
/// - `dst`: destination register or branch target
/// - `src0`, `src1`: source registers or immediates
/// - `aux0`-`aux3`: auxiliary fields (gate index, block ids, side-table offsets, etc.)
struct Instruction {
    opcode: u32,
    dst: u32,
    src0: u32,
    src1: u32,
    aux0: u32,
    aux1: u32,
    aux2: u32,
    aux3: u32,
}

struct Block {
    instr_offset: u32,
    instr_count: u32,
}

struct Function {
    entry_block_id: u32,
    param_count: u32,
    param_base_reg: u32,
    reserved: u32,
}

struct PhiNodeEntry {
    block_id: u32,
    val_reg: u32,
}

struct SwitchCase {
    case_val: u32,
    target_block: u32,
}

struct Program {
    /// Bytecode instructions.
    instructions: array<Instruction, INSTRUCTIONS_SIZE>,
    /// Block table: indexed by block ID.
    block_table: array<Block, BLOCK_TABLE_SIZE>,
    /// Function table.
    function_table: array<Function, FUNCTION_TABLE_SIZE>,
    /// Phi entries table: `[predecessor_block_id, value_register]` entries.
    phi_table: array<PhiNodeEntry, PHI_TABLE_SIZE>,
    /// Switch cases table: `[match_value, target_block]` entries.
    switch_table: array<SwitchCase, SWITCH_CASES_SIZE>,
    /// Call argument register indices.
    call_arg_table: array<u32, CALL_ARGS_SIZE>,
    /// Constant data pool (flattened array constant values).
    constant_data: array<u32, CONSTANT_DATA_SIZE>,
}

struct CallStackFrame {
    /// Resume on this block on return.
    block_id: u32,
    /// Instruction after the call.
    return_pc: u32,
    /// Where to write the return value.
    return_reg: u32,
    /// This is for alignment.
    reserved: u32,
}

/// Per-shot interpreter state.
struct InterpreterState {
    /// Instruction index (absolute), PC stands for Program Counter.
    pc: u32,
    /// Current block ID.
    current_block_id: u32,
    ///Previous block ID (for phi resolution).
    previous_block_id: u32,
    /// 0=running, 1=quantum_pending, 2=terminated, 3=error, 4=yield.
    status: u32,
    /// Quantum op table index.
    pending_op_idx: u32,
    /// 0=gate, 1=measure, 2=reset.
    pending_op_type: u32,
    /// From ret instruction
    exit_code: u32,
    /// Call stack pointer.
    call_sp: u32,
    /// Call stack frames (4 u32 per frame × 14 frames = 56).
    call_stack_frames: array<CallStackFrame, 14>,
    /// Per-shot register file.
    registers: array<u32, MAX_REGISTERS>,
    /// Per-shot memory (constant_data + alloca'd values).
    memory: array<u32, MAX_MEMORY>,
}

// Buffer containing the state for each shot to execute per kernel dispatch
// An instance of this is tracked on the GPU for every active shot
struct ShotData {
    shot_id: u32,
    next_op_idx: u32,

    // The below random numbers will be initialized from the RNG per operation in the 'prepare_op' stage
    // Then the 'execute_op' stage will read these precomputed random numbers for noise modeling
    rng_state: xorwow_state, // 6 x u32
    rand_pauli: f32,
    rand_damping: f32,
    rand_dephase: f32,
    rand_measure: f32,
    // Bitmask of qubits the most recent noise sampler chose to lose. A following
    // loss-commit op consumes (and clears) its qubit's bit.
    pending_loss_mask: u32,

    // The type of the next operation to execute. This will be OPID_SHOT_BUFF_* if it should use the unitary from the op buffer
    op_type: u32,
    op_idx: u32,

    duration: f32, // Total duration of the shot so far, used for time-dependent noise modeling and shot estimations
    renormalize: f32, // Value to renormalize the state vector by on next execute (1.0 = no renormalization needed)

    // For quick testing during execution to enable skipping blocks of entries
    // TODO: Actually use these masks during execution to skip unneeded work
    qubit_is_0_mask: u32, // Bitmask for which qubits are currently in |0> state
    qubit_is_1_mask: u32, // Bitmask for which qubits are currently in |1> state

    // Track which qubit probabilities were updated in the last operation (to collate on next prepare_op)
    qubits_updated_last_op_mask: u32,
    // 20 x 4 bytes to this point = 80 bytes

    // Track the per-qubit probabilities for optimization of measurement sampling and noise modeling
    qubit_state: array<QubitState, MAX_QUBIT_COUNT>, // 27 x 16 bytes = 432 bytes
    // 512 bytes to this point

    // Map this to the Op structure for ease of use
    unitary: array<vec2f, 16>, // For MAT1Q and MAT2Q ops.

    // Adaptive interpreter state (embedded to reduce storage buffer count).
    // This is initialized by the host after the GPU init kernel runs.
    interp: InterpreterState,
}
// See https://www.w3.org/TR/WGSL/#structure-member-layout for alignment rules

// Buffer containing the list of operations (gates and noise) that make up the program to simulate
struct Op {
    id: u32,
    q1: u32,
    q2: u32,
    q3: u32,
    policy: u32,
    pad0: u32,
    pad1: u32,
    pad2: u32,
    // Entries in the unitary are: 00, 01, 02, 03, 10, 11, 12, 13, 20, ..., 32, 33
    // 1q matrix elements are stored in: 00, 01, 10, 11 (i.e., indices 0, 1, 4, and 5)
    unitary: array<vec2f, 16>,
} // Struct size: 4 * 4 + 16 * 8 = 160 bytes (which is aligned to 16 bytes)

struct ShotParams {
    shot_idx: i32,
    shot_state_vector_start: i32,
    workgroup_collation_idx: i32,
    workgroup_idx_in_shot: i32,
    thread_idx_in_shot: i32,
    total_threads_per_shot: i32,
    zero_entry_count: i32,
    op_iterations: i32,
}

struct NoiseTableMetadata {
    /// The total probability of any noise (i.e. sum of all noise entries) in `Q1.63` format
    noise_probability_lo: u32,
    noise_probability_hi: u32,
    /// The start offset of this table's entries in the global `NoiseTableEntry` array
    start_offset: u32,
    /// The number of entries in this noise table
    entry_count: u32,
}

struct NoiseTableEntry {
    /// The correlated pauli string as bits (2 bits per qubit). If bit 0 is set, then it has bit-flip
    /// noise, and if bit 1 is set then it has phase-flip noise. e.g., `110001 == "YIX"`
    paulis_lo: u32,
    paulis_hi: u32,
    /// The probability of the noise occurring in `Q1_63` format. This is a float format where the high
    /// order bit (bit 63) has the value 1.0 (`2^0 / 1`), bit 62 has the value 0.5 (`2^1 / 1`), etc.
    /// all the way to bit 63 with a value of approx 1.0842e-19 (`2^63 / 1`). This gives a range of
    /// values from [0..2) with equal spacing of 1.0842e-19 between values (unlike float or double),
    /// which makes it more suitable for random numbers used to select between a large number of small
    /// probability entries.
    probability_lo: u32,
    probability_hi: u32,
}

// BatchData holds all the read-only data shared across all shots in a batch.
struct BatchData {
    correlated_noise_tables: array<NoiseTableMetadata, NOISE_TABLE_COUNT>,
    correlated_noise_entries: array<NoiseTableEntry, NOISE_ENTRY_COUNT>,
    program: Program,
}

// Result of sampling which correlated noise entry (if any) to apply.
struct CorrelatedNoiseSample {
    should_apply: u32, // 0 = no noise, 1 = apply noise
    paulis_lo: u32,
    paulis_hi: u32,
}

// For every qubit, each 'execute' kernel thread will update its own workgroup storage location for accumulating probabilities
// The final probabilities will be reduced and written back to the shot state after the parallel execution completes.
struct QubitProbabilityPerThread {
    zero: array<f32, MAX_QUBIT_COUNT>,
    one: array<f32, MAX_QUBIT_COUNT>,
}; // size: 216 bytes

// When an error occurs, the below diagnostic data structure is used to store information about the error
struct DiagnosticData {
    error_code: atomic<u32>,
    termination_count: atomic<u32>,
    extra1: u32,
    extra2: f32,
    extra3: f32,
    _padding: u32,
    shot: ShotData, // 640 bytes
    op: Op,         // 144 bytes
    // Below is usually 6,912 bytes (size = THREADS_PER_WORKGROUP (32) * (8 * MAX_QUBIT_COUNT (27))
    workgroup_probabilities: array<QubitProbabilityPerThread, THREADS_PER_WORKGROUP>,
    // Below is usually 27,648 bytes (1 << u32(MAX_QUBIT_COUNT - MAX_QUBITS_PER_WORKGROUP)) * (8 * MAX_QUBIT_COUNT) bytes
    collation_buffer: WorkgroupCollationBuffer,
};

struct Uniforms {
    batch_start_shot_id: i32,
    rng_seed: u32,
}

//#endregion

//#region Buffers and workgroup memory

@group(0) @binding(0)
var<storage, read_write> workgroup_collation: WorkgroupCollationBuffer;
// Around 128 max partitions times 27 qubits times 8 bytes = 27 KB max size

@group(0) @binding(1)
var<storage, read_write> shots: array<ShotData>;

@group(0) @binding(2)
var<storage, read> ops: array<Op>;

// The one large buffer of state vector amplitudes. (Partitioned into multiple shots)
@group(0) @binding(3)
var<storage, read_write> stateVector: array<vec2f>;

// Buffer for storing measurement results per shot
@group(0) @binding(4)
var<storage, read_write> results: array<atomic<u32>>;

@group(0) @binding(5)
var<storage, read_write> diagnostics: DiagnosticData;

@group(0) @binding(6)
var<uniform> uniforms: Uniforms;

@group(0) @binding(7)
var<storage, read> batch_data: BatchData;

var<workgroup> qubitProbabilities: array<QubitProbabilityPerThread, THREADS_PER_WORKGROUP>;
// Workgroup memory size: THREADS_PER_WORKGROUP (32) * 216 = 6,912 bytes.

//#endregion

//#region Math utility functions

// Get the magnitude squared of a complex number
fn cplxMag2(a: vec2f) -> f32 {
    return (a.x * a.x + a.y * a.y);
}

// Complex multiplication
fn cplxMul(a: vec2f, b: vec2f) -> vec2f {
    return vec2f(
        a.x * b.x - a.y * b.y,
        a.x * b.y + a.y * b.x
    );
}

// Complex negation
fn cplxNeg(a: vec2f) -> vec2f {
    return vec2f(-a.x, -a.y);
}

// Negate all elements in a 4-element row of complex numbers
fn rowNeg(a: array<vec2f, 4>) -> array<vec2f, 4> {
    return array<vec2f, 4>(
        cplxNeg(a[0]),
        cplxNeg(a[1]),
        cplxNeg(a[2]),
        cplxNeg(a[3]));
}

// Compute the inner product of two 4-element rows of complex numbers
fn innerProduct(a: array<vec2f, 4>, b: array<vec2f, 4>) -> vec2f {
    var result: vec2f = vec2f(0.0, 0.0);
    for (var i: u32 = 0u; i < 4u; i++) {
        result += cplxMul(a[i], b[i]);
    }
    return result;
}

fn getOpRow(op_idx: u32, row: u32) -> array<vec2f, 4> {
    let op = &ops[op_idx];
    return array<vec2f, 4>(
        op.unitary[row * 4 + 0],
        op.unitary[row * 4 + 1],
        op.unitary[row * 4 + 2],
        op.unitary[row * 4 + 3]);
}

fn getUnitaryRow(shot_idx: i32, row: u32) -> array<vec2f, 4> {
    let shot = &shots[shot_idx];
    return array<vec2f, 4>(
        shot.unitary[row * 4 + 0],
        shot.unitary[row * 4 + 1],
        shot.unitary[row * 4 + 2],
        shot.unitary[row * 4 + 3]);
}

fn setUnitaryRow(shot_idx: u32, row: u32, newRow: array<vec2f, 4>) {
    let shot = &shots[shot_idx];
    shot.unitary[row * 4 + 0] = newRow[0];
    shot.unitary[row * 4 + 1] = newRow[1];
    shot.unitary[row * 4 + 2] = newRow[2];
    shot.unitary[row * 4 + 3] = newRow[3];
}

//#endregion

//#region Hash and random number generation

// See https://www.reedbeta.com/blog/hash-functions-for-gpu-rendering/
// Use PCG hash function to generate a well-distributed hash from a simple integer input (e.g., shot id)
fn hash_pcg(input: u32) -> u32 {
    var state = input * 747796405u + 2891336453u;
    var word = ((state >> ((state >> 28u) + 4u)) ^ state) * 277803737u;
    return (word >> 22u) ^ word;
}

// Returns a random u32 value based on the xorwow algorithm
fn next_rand_u32(shot_idx: u32) -> u32 {
    // Based on https://en.wikipedia.org/wiki/Xorshift
    let rng_state = &shots[shot_idx].rng_state;

    let t: u32 = rng_state.x[4];
    let s: u32 = rng_state.x[0];
    rng_state.x[4] = rng_state.x[3];
    rng_state.x[3] = rng_state.x[2];
    rng_state.x[2] = rng_state.x[1];
    rng_state.x[1] = s;

    // TODO: Simplify with a `var` once https://github.com/wgsl-analyzer/wgsl-analyzer/issues/1317 is fixed
    let t2 = t ^ (t >> 2u);
    let t3 = t2 ^ (t2 << 1u);
    let t4 = t3 ^ s ^ (s << 4u);
    rng_state.x[0] = t4;
    rng_state.counter = rng_state.counter + 362437u;
    return t4 + rng_state.counter;
}

fn next_rand_f32(shot_idx: u32) -> f32 {
    let rand_u32: u32 = next_rand_u32(shot_idx);

    // Convert the 32 random bits to a float in the [0.0, 1.0) range

    // Keep only the lower 23 bits (the fraction portion of a float) with a 0 exponent biased to 127
    let rand_f32_bits = (rand_u32 & 0x7FFFFF) | (127 << 23);
    // Bitcast to an f32 in the [1.0, 2.0) range
    let f: f32 = bitcast<f32>(rand_f32_bits);
    // And decrement by 1 to return values from [0..1)
    return f - 1.0;
}

//#endregion

//#region Operation classification helpers

fn is_1q_phase_gate(op_id: u32) -> bool {
    return (op_id == OPID_S || op_id == OPID_SAdj || op_id == OPID_T || op_id == OPID_TAdj || op_id == OPID_RZ);
}

fn is_1q_op(op_id: u32) -> bool {
    return ((op_id >= OPID_ID && op_id <= OPID_RZ) ||
        op_id == OPID_MZ || op_id == OPID_MRESETZ ||
        op_id == OPID_MAT1Q || op_id == OPID_SHOT_BUFF_1Q);
}

//#endregion

//#region Per-shot setup and reset

fn shot_init_per_op(shot_idx: u32) {
    let shot = &shots[shot_idx];

    // Default to 1.0 renormalization (i.e., no renormalization needed). MResetZ or noise affecting the
    // overall probability distribution (e.g. loss or amplitude damping) will update this if needed.
    shot.renormalize = 1.0;
    shot.qubits_updated_last_op_mask = 0u;

    // Generate the next set of random numbers to use for noise and measurement
    shot.rand_pauli = next_rand_f32(shot_idx);
    shot.rand_damping = next_rand_f32(shot_idx);
    shot.rand_dephase = next_rand_f32(shot_idx);
    shot.rand_measure = next_rand_f32(shot_idx);
    // Reserved draw: qubit loss is now sampled from the combined `rand_pauli`
    // distribution rather than its own value, but we still advance the RNG by
    // one draw here to keep the per-op random stream (and thus seeded results)
    // identical to the previous loss model.
    next_rand_f32(shot_idx);
}

// Resets the entire shot state, including RNG, probabilities, and per-qubit tracking.
fn reset_all(shot_idx: i32) {
    let shot = &shots[shot_idx];

    // One of the main goals of the shot_id is to seed the RNG state uniquely per shot
    let rng_seed = uniforms.rng_seed;
    let shot_id = u32(uniforms.batch_start_shot_id + shot_idx);

    // Due to DX12 backend issues, we can't just assign a zeroed struct, so manually reset all fields
    // DX12-start-strip
    *shot = ShotData();
    // DX12-end-strip
    shot.shot_id = shot_id;

    // After init, start execution from the first op
    shot.next_op_idx = 0u;

    shot.rng_state.x[0] = rng_seed ^ hash_pcg(shot_id);
    shot.rng_state.x[1] = rng_seed ^ hash_pcg(shot_id + 1);
    shot.rng_state.x[2] = rng_seed ^ hash_pcg(shot_id + 2);
    shot.rng_state.x[3] = rng_seed ^ hash_pcg(shot_id + 3);
    shot.rng_state.x[4] = rng_seed ^ hash_pcg(shot_id + 4);

    shot.op_type = 0;
    shot.op_idx = 0;

    // rand_* will be initialized in shot_init_per_op when preparing the first op
    shot.duration = 0.0;
    shot.renormalize = 1.0;

    shot.qubit_is_0_mask = (1u << u32(QUBIT_COUNT)) - 1u; // All qubits are |0>
    shot.qubit_is_1_mask = 0u;
    shot.qubits_updated_last_op_mask = 0;
    shot.pending_loss_mask = 0u;

    // Initialize all qubit probabilities to 100% |0>
    for (var i: i32 = 0; i < QUBIT_COUNT; i++) {
        shot.qubit_state[i].zero_probability = 1.0;
        shot.qubit_state[i].one_probability = 0.0;
        shot.qubit_state[i].heat = 0.0;
        shot.qubit_state[i].idle_since = 0.0;
    }

    // unitary will be set in prepare_op
}

//#endregion

//#region Qubit probability tracking

fn update_qubit_state(shot_idx: u32) {
    let shot = &shots[shot_idx];

    // If any qubits were updated in the last op, we may need to sum workgroup probabilities into the shot state
    // This is only needed if multiple workgroups were used for the shot execution. If not, then the
    // single workgroup for the shot would have written directly to the shot state already.

    // For each qubit that was updated in the last op
    for (var q: u32 = 0u; q < u32(QUBIT_COUNT); q++) {
        let qubit_mask: u32 = 1u << q;
        if ((shot.qubits_updated_last_op_mask & qubit_mask) != 0u) {
            // Sum the workgroup collation entries for this qubit into the shot state
            // Note: We ignore the fact a qubit may be 'lost' here. It should already be
            // in the |0> state if lost, so summing the probabilities is still valid.
            var total_zero: f32 = 0.0;
            var total_one: f32 = 0.0;

            if (WORKGROUPS_PER_SHOT > 1) {
                // Offset into workgroup collation buffer based on shot index
                let offset = shot_idx * u32(WORKGROUPS_PER_SHOT);
                for (var wkg_idx: u32 = 0u; wkg_idx < u32(WORKGROUPS_PER_SHOT); wkg_idx++) {
                    let sums = workgroup_collation.sums[wkg_idx + offset];
                    total_zero = total_zero + sums.qubits[q].x;
                    total_one = total_one + sums.qubits[q].y;
                }
            } else {
                // Single workgroup per shot case - just read directly from the shot
                total_zero = shot.qubit_state[q].zero_probability;
                total_one = shot.qubit_state[q].one_probability;
            }

            // Update the shot state with the summed probabilities
            // Round to 0 or 1 if extremely close to mitigate minor floating point errors
            // TODO: Use PROB_THRESHOLD constant here?
            if (total_zero < 0.000001) { total_zero = 0.0; }
            if (total_one < 0.000001) { total_one = 0.0; }
            if (total_zero > 0.999999) { total_zero = 1.0; }
            if (total_one > 0.999999) { total_one = 1.0; }

            shot.qubit_state[q].zero_probability = total_zero;
            shot.qubit_state[q].one_probability = total_one;

            // NOTE: Any kind of operation with a NaN float value results in a NaN, or false for logical comparisons
            // So beware of conditions that may not behave as expected if NaN values are possible.
            let within_threshold = abs(1.0 - (total_zero + total_one)) < PROB_THRESHOLD;
            if !within_threshold {
                // Populate the diagnostics buffer, if not already set
                let old_value = atomicCompareExchangeWeak(
                    &diagnostics.error_code,
                    0u,
                    ERR_INVALID_PROBS);
                if old_value.exchanged {
                    // This is the first error - fill in the details
                    diagnostics.extra1 = q;
                    diagnostics.extra2 = total_zero;
                    diagnostics.extra3 = total_one;
                    // DX12 backend has issues assigning structs. See https://github.com/gfx-rs/wgpu/issues/8552
                    // DX12-start-strip
                    diagnostics.shot = *shot;
                    diagnostics.op = ops[shot.op_idx];
                    // DX12-end-strip
                }
                // Store the error value (if none set already)
                let err_index = (shot_idx + 1) * RESULT_COUNT - 1;
                atomicCompareExchangeWeak(
                    &results[err_index],
                    0u,
                    ERR_INVALID_PROBS);
            }

            // Update the masks for definite states
            shot.qubit_is_0_mask = select(
                shot.qubit_is_0_mask & ~qubit_mask,
                shot.qubit_is_0_mask | qubit_mask,
                total_zero == 1.0);
            shot.qubit_is_1_mask = select(
                shot.qubit_is_1_mask & ~qubit_mask,
                shot.qubit_is_1_mask | qubit_mask,
                total_one == 1.0);
        }
    }
}

// For the state vector index and amplitude probability, update all the qubit probabilities for this thread
fn update_all_qubit_probs(stateVectorIndex: u32, amplitude: vec2f, tid: u32) {
    var mask: u32 = 1u;
    for (var q: u32 = 0u; q < u32(QUBIT_COUNT); q++) {
        let is_one: bool = (stateVectorIndex & mask) != 0u;
        let prob: f32 = cplxMag2(amplitude);
        if (is_one) {
            qubitProbabilities[tid].one[q] += prob;
        } else {
            qubitProbabilities[tid].zero[q] += prob;
        }
        mask = mask << 1u;
    }
}

fn sum_thread_totals_to_shot(q: u32, shot_idx: i32, wkg_collation_idx: i32) {
    var total_zero: f32 = 0.0;
    var total_one: f32 = 0.0;
    for (var j = 0; j < THREADS_PER_WORKGROUP; j++) {
        total_zero += qubitProbabilities[j].zero[q];
        total_one += qubitProbabilities[j].one[q];
    }
    if (wkg_collation_idx >= 0) {
        // Write to the workgroup collation buffer for later summation into the shot state
        workgroup_collation.sums[wkg_collation_idx].qubits[q] = vec2f(total_zero, total_one);
    } else {
        // Single workgroup per shot case - write directly to the shot state
        let within_threshold = abs(1.0 - (total_zero + total_one)) < PROB_THRESHOLD;
        if !within_threshold {
            // Populate the diagnostics buffer, if not already set
            let old_value = atomicCompareExchangeWeak(
                &diagnostics.error_code,
                0u,
                ERR_INVALID_THREAD_TOTAL);
            if old_value.exchanged {
                // This is the first error - fill in the details
                let shot = &shots[shot_idx];
                diagnostics.extra1 = q;
                diagnostics.extra2 = total_zero;
                diagnostics.extra3 = total_one;
                // DX12 backend has issues copying structs. See https://github.com/gfx-rs/wgpu/issues/8552
                // DX12-start-strip
                diagnostics.shot = *shot;
                diagnostics.op = ops[shot.op_idx];
                // DX12-end-strip
            }
            let err_index = (shot_idx + 1) * i32(RESULT_COUNT) - 1;
            atomicCompareExchangeWeak(
                    &results[err_index],
                    0u,
                    ERR_INVALID_THREAD_TOTAL);
        } else {
            shots[shot_idx].qubit_state[q].zero_probability = total_zero;
            shots[shot_idx].qubit_state[q].one_probability = total_one;
        }
    }
}

//#endregion

//#region Measurement and reset ops

// Build a measure-and-reset (or measure-only) instrument for `qubit` given a
// measured `result`, store it in the shot buffer, set up renormalization, and
// mark the qubit as no longer in a definite basis state so the execute stage
// recomputes its probabilities. Shared by `prep_measure_reset` and
// `prep_loss_commit`; the caller sets `shot.op_idx` and `shot.op_type`.
fn prep_measure_reset_instrument(shot_idx: u32, qubit: u32, result: u32, resets_to_zero: bool) {
    let shot = &shots[shot_idx];

    // Construct the measurement/reset instrument based on the measured result
    // Put the instrument into the shot buffer for the execute_op stage to apply
    if resets_to_zero {
        // Reset variants (MResetZ, ResetZ):
        // Result=0: [[1,0],[0,0]] - project onto |0⟩ (already there)
        // Result=1: [[0,1],[0,0]] - swap |1⟩ into |0⟩ slot (reset)
        shot.unitary[0] = select(vec2f(1.0, 0.0), vec2f(0.0, 0.0), result == 1u);
        shot.unitary[1] = select(vec2f(0.0, 0.0), vec2f(1.0, 0.0), result == 1u);
        shot.unitary[4] = vec2f();
        shot.unitary[5] = vec2f();
    } else {
        // Measure-only (MZ):
        // Result=0: [[1,0],[0,0]] - project onto |0⟩
        // Result=1: [[0,0],[0,1]] - project onto |1⟩ (keep in place)
        shot.unitary[0] = select(vec2f(1.0, 0.0), vec2f(0.0, 0.0), result == 1u);
        shot.unitary[1] = vec2f();
        shot.unitary[4] = vec2f();
        shot.unitary[5] = select(vec2f(0.0, 0.0), vec2f(1.0, 0.0), result == 1u);
    }

    shot.renormalize = select(
        1.0 / sqrt(shot.qubit_state[qubit].zero_probability),
        1.0 / sqrt(shot.qubit_state[qubit].one_probability),
        result == 1u);

    // We don't want the measurement pass to skip over this qubit, so ensure it's marked as not in a definite state
    shot.qubit_is_1_mask = shot.qubit_is_1_mask & ~(1u << qubit);
    shot.qubit_is_0_mask = shot.qubit_is_0_mask & ~(1u << qubit);

    // Set the qubits_updated_last_op_mask to all except those that were already in a definite
    // state (so we don't waste time updating probabilities that are already known). Note that
    // next 'prepare_op' should set the just measured qubit into a definite 0 or 1 state.
    shot.qubits_updated_last_op_mask =
        // A mask with all qubits set
        ((1u << u32(QUBIT_COUNT)) - 1u)
        // Exclude qubits already in definite states
            & ~(shot.qubit_is_0_mask | shot.qubit_is_1_mask);
}

// `qubit` and `result_id` are resolved by the caller: the base pipeline reads
// them from the ops pool (ops[op_idx].q1/.q2), while the adaptive interpreter
// resolves them from registers/immediates (resolve_q1/resolve_q2).
fn prep_measure_reset(shot_idx: u32, op_idx: u32, qubit: u32, result_id: u32, is_loss: bool, stores_result: bool, resets_to_zero: bool) {
    let shot = &shots[shot_idx];

    // Choose measurement result based on qubit probabilities and random number
    let result = select(1u, 0u, shot.rand_measure < shot.qubit_state[qubit].zero_probability);

    // If this is being called due to loss noise, we don't write the result back to the results buffer
    // Instead, mark the qubit as lost by setting the heat to -1.0
    if !is_loss {
        if stores_result {
            // If the qubit is already marked as lost, just report that and exit. It's already in the zero
            // state so nothing to update or renormalize. The execute op should be a no-op (ID)
            if shot.qubit_state[qubit].heat == -1.0 {
                atomicStore(&results[(shot_idx * RESULT_COUNT) + result_id], 2u);
                shot.op_type = OPID_ID;
                shot.op_idx = op_idx;
                // Qubit get reloaded after a Measurement, so set the heat back to 0.0
                shot.qubit_state[qubit].heat = 0.0;
                return;
            } else {
                atomicStore(&results[(shot_idx * RESULT_COUNT) + result_id], result);
            }
        } else {
            // No result to store (e.g. ResetZ). If the qubit is lost, it's already in the zero
            // state so nothing to update. Just set to ID and return.
            if shot.qubit_state[qubit].heat == -1.0 {
                shot.op_type = OPID_ID;
                shot.op_idx = op_idx;
                return;
            }
        }
    } else {
        shot.qubit_state[qubit].heat = -1.0;
    }

    prep_measure_reset_instrument(shot_idx, qubit, result, resets_to_zero);

    shot.op_idx = op_idx;
    // Use OPID_MRESETZ as the op_type for all three variants in execute stage
    // (they all use the same matrix-apply + update_all_qubit_probs path)
    shot.op_type = OPID_MRESETZ;
}

//#endregion

//#region Unitary construction helpers

// Builds a 4x4 (in shot.unitary) that applies the 1-qubit matrix `m` (given as
// m00,m01,m10,m11) to `target_is_q2 ? q2 : q1` and identity to the other qubit
// of the pair. The lost qubit is in the |0> state, so the identity factor keeps
// it there. The 2-qubit basis is |q1 q2>, so the row/col index is
// (2 * q1_bit + q2_bit).
fn set_1q_on_pair_unitary(shot_idx: u32, target_is_q2: bool,
                          m00: vec2f, m01: vec2f, m10: vec2f, m11: vec2f) {
    let shot = &shots[shot_idx];
    // Zero the whole 4x4 first.
    for (var i = 0u; i < 16u; i++) {
        shot.unitary[i] = vec2f(0.0, 0.0);
    }
    if target_is_q2 {
        // Acts on q2 (low bit): block-diagonal diag(M, M).
        // Top-left block (q1 = 0):
        shot.unitary[0]  = m00; shot.unitary[1]  = m01;
        shot.unitary[4]  = m10; shot.unitary[5]  = m11;
        // Bottom-right block (q1 = 1):
        shot.unitary[10] = m00; shot.unitary[11] = m01;
        shot.unitary[14] = m10; shot.unitary[15] = m11;
    } else {
        // Acts on q1 (high bit): M (x) I.
        shot.unitary[0]  = m00; shot.unitary[2]  = m01;
        shot.unitary[8]  = m10; shot.unitary[10] = m11;
        shot.unitary[5]  = m00; shot.unitary[7]  = m01;
        shot.unitary[13] = m10; shot.unitary[15] = m11;
    }
}

// Multiplies one row of the 4x4 pair unitary (in shot.unitary) by -i, in place.
// Folding a diag(1, -i) = S-dagger factor on one qubit into a 2-qubit matrix
// scales the rows whose target-qubit bit is 1 by -i. For a complex entry
// (x + y i), (x + y i) * -i = y - x i.
fn scale_pair_unitary_row_by_neg_i(shot_idx: u32, row: u32) {
    let shot = &shots[shot_idx];
    for (var c = 0u; c < 4u; c++) {
        let e = shot.unitary[row * 4u + c];
        shot.unitary[row * 4u + c] = vec2f(e.y, -e.x);
    }
}

// Sets up the shot to execute a 2-qubit shot-buffer op on the gate's operands.
fn finish_2q_shot_buffer(shot_idx: u32, op_idx: u32, q1: u32, q2: u32) {
    let shot = &shots[shot_idx];
    shot.op_idx = op_idx;
    shot.op_type = OPID_SHOT_BUFF_2Q;
    shot.qubits_updated_last_op_mask = (1u << q1) | (1u << q2);
}

//#endregion

//#region Qubit loss handling

// Returns true if the gate at `op_idx` touches at least one lost qubit.
// `q1`/`q2` are the (resolved) operands of the gate.
fn gate_has_lost_operand(shot_idx: u32, op_idx: u32, q1: u32, q2: u32) -> bool {
    let shot = &shots[shot_idx];
    let op = &ops[op_idx];
    if (shot.qubit_state[q1].heat == -1.0) {
        return true;
    }
    let is_2q = !is_1q_op(op.id);
    return is_2q && (shot.qubit_state[q2].heat == -1.0);
}

// Loses a single surviving `qubit` for the PROPAGATE policy: samples a
// measurement outcome, collapses the qubit to that outcome and resets it to
// |0>, and marks it lost (heat = -1.0). The collapse is expressed as a 2-qubit
// tensor on the gate's operands (reset on `qubit`, identity on the lost
// partner, which is already in |0>), reusing the standard shot-buffer execute
// path. `qubit` must be one of the gate's two operands `q1`/`q2`.
fn propagate_loss_to_qubit(shot_idx: u32, op_idx: u32, q1: u32, q2: u32, qubit: u32) {
    let shot = &shots[shot_idx];

    let result = select(1u, 0u, shot.rand_measure < shot.qubit_state[qubit].zero_probability);

    // Reset instrument (project + move |1> into |0> slot), same as MResetZ:
    //   result==0: [[1,0],[0,0]]
    //   result==1: [[0,1],[0,0]]
    let m00 = select(vec2f(1.0, 0.0), vec2f(0.0, 0.0), result == 1u);
    let m01 = select(vec2f(0.0, 0.0), vec2f(1.0, 0.0), result == 1u);
    let m10 = vec2f(0.0, 0.0);
    let m11 = vec2f(0.0, 0.0);

    let target_is_q2 = (qubit == q2);
    set_1q_on_pair_unitary(shot_idx, target_is_q2, m00, m01, m10, m11);

    // Renormalize by the measured branch probability.
    shot.renormalize = select(
        1.0 / sqrt(shot.qubit_state[qubit].zero_probability),
        1.0 / sqrt(shot.qubit_state[qubit].one_probability),
        result == 1u);

    // Mark the qubit lost and clear its definite-state bits so the probability
    // pass recomputes it.
    shot.qubit_state[qubit].heat = -1.0;
    shot.qubit_is_0_mask = shot.qubit_is_0_mask & ~(1u << qubit);
    shot.qubit_is_1_mask = shot.qubit_is_1_mask & ~(1u << qubit);

    finish_2q_shot_buffer(shot_idx, op_idx, q1, q2);
}

// Handles a gate whose operand(s) include at least one lost qubit, according to
// the loss policy stamped on the op's `policy` field. `q1`/`q2` are the
// (resolved) operands. The gate body is fully handled here (degraded unitary,
// loss propagation, or turned into Id); the caller must not run the original
// gate afterwards. Any attached Pauli noise is applied separately to the
// surviving operand via `apply_2q_pauli_noise_on_survivor`.
fn handle_lost_operand_policy(shot_idx: u32, op_idx: u32, q1: u32, q2: u32) {
    let shot = &shots[shot_idx];
    let op = &ops[op_idx];
    let is_1q = is_1q_op(op.id);
    let is_2q = !is_1q;
    let policy = op.policy;

    // Loss policies only make sense for multi-qubit gates.
    // If this is a single-qubit gate, skip it entirely.
    if (is_1q) {
        shot.op_type = OPID_ID;
        shot.op_idx = op_idx;
        return;
    }

    let q1_lost = shot.qubit_state[q1].heat == -1.0;
    let q2_lost = is_2q && (shot.qubit_state[q2].heat == -1.0);
    let has_survivor = is_2q && !(q1_lost && q2_lost);
    // The surviving operand (only meaningful when has_survivor is true).
    let survivor = select(q1, q2, q1_lost);
    let survivor_is_q2 = q1_lost;

    // SWAP is special: it physically relocates the two qubits, so their loss
    // state is always exchanged regardless of the policy (the policy only
    // governs whether the unitary runs). Handle it explicitly here.
    if (op.id == OPID_SWAP) {
        switch policy {
            case LOSS_POLICY_PROPAGATE {
                propagate_loss_to_qubit(shot_idx, op_idx, q1, q2, survivor);
                return;
            }
            case LOSS_POLICY_RESIDUAL_S_DAGGER {
                // Match the CPU/stabilizer SWAP + residual S-dagger semantics:
                //   1. Apply the full SWAP (shot.unitary already holds it).
                //   2. Apply S-dagger = diag(1, -i) to the (originally) lost
                //      operand's position, which after the SWAP holds the
                //      survivor's amplitudes.
                //   3. Exchange the per-qubit loss flag (heat) of the operands.

                // Fold the S-dagger into the SWAP matrix by scaling, by -i, the
                // two rows of the |q1 q2> pair matrix whose lost-qubit bit is 1.
                // q1 is the high bit (rows 2, 3); q2 is the low bit (rows 1, 3).
                let lost_row = select(1u, 2u, q1_lost);
                scale_pair_unitary_row_by_neg_i(shot_idx, lost_row);
                scale_pair_unitary_row_by_neg_i(shot_idx, 3u);
                // Exchange the per-qubit loss flag (heat) of the two operands.
                let heat1 = shot.qubit_state[q1].heat;
                shot.qubit_state[q1].heat = shot.qubit_state[q2].heat;
                shot.qubit_state[q2].heat = heat1;
                // The 2-qubit execute path skips amplitudes for qubits known to be
                // in a definite state, which would skip the amplitudes SWAP needs to move.
                // Clear those bits for both operands so the swap is actually applied.
                shot.qubit_is_0_mask = shot.qubit_is_0_mask & ~((1u << q1) | (1u << q2));
                shot.qubit_is_1_mask = shot.qubit_is_1_mask & ~((1u << q1) | (1u << q2));
                // shot.unitary now holds (S-dagger on lost) * SWAP.
                finish_2q_shot_buffer(shot_idx, op_idx, q1, q2);
                return;
            }
            case LOSS_POLICY_APPLY_ANYWAY {
                // Exchange the per-qubit loss flag (heat) of the two operands.
                let heat1 = shot.qubit_state[q1].heat;
                shot.qubit_state[q1].heat = shot.qubit_state[q2].heat;
                shot.qubit_state[q2].heat = heat1;
                // The 2-qubit execute path skips amplitudes for qubits known to be
                // in a definite state, which would skip the amplitudes SWAP needs to move.
                // Clear those bits for both operands so the swap is actually applied.
                shot.qubit_is_0_mask = shot.qubit_is_0_mask & ~((1u << q1) | (1u << q2));
                shot.qubit_is_1_mask = shot.qubit_is_1_mask & ~((1u << q1) | (1u << q2));
                // shot.unitary already holds the SWAP matrix (set by the caller).
                finish_2q_shot_buffer(shot_idx, op_idx, q1, q2);
                return;
            }
            case LOSS_POLICY_SKIP {
                shot.op_type = OPID_ID;
                shot.op_idx = op_idx;
                return;
            }
            default {
                // SWAP only supports SKIP, PROPAGATE, RESIDUAL_S_DAGGER, and
                // APPLY_ANYWAY. Any other policy (e.g. DEGRADE) is rejected by
                // the host, so reaching here indicates a bug.
                report_shot_error(shot_idx, ERR_UNSUPPORTED_LOSS_POLICY);
                shot.op_type = OPID_ID;
                shot.op_idx = op_idx;
                return;
            }
        }
    }

    // APPLY_ANYWAY is only valid for SWAP, which is handled above. Reaching here
    // with it on any other gate is rejected by the host, so it indicates a bug.
    if (policy == LOSS_POLICY_APPLY_ANYWAY) {
        report_shot_error(shot_idx, ERR_UNSUPPORTED_LOSS_POLICY);
        shot.op_type = OPID_ID;
        shot.op_idx = op_idx;
        return;
    }

    if (policy == LOSS_POLICY_PROPAGATE && has_survivor) {
        propagate_loss_to_qubit(shot_idx, op_idx, q1, q2, survivor);
        return;
    }

    if (policy == LOSS_POLICY_RESIDUAL_S_DAGGER && has_survivor) {
        // Apply S-dagger = diag(1, -i) to the surviving operand.
        set_1q_on_pair_unitary(shot_idx, survivor_is_q2,
            vec2f(1.0, 0.0), vec2f(0.0, 0.0),
            vec2f(0.0, 0.0), vec2f(0.0, -1.0));
        finish_2q_shot_buffer(shot_idx, op_idx, q1, q2);
        return;
    }

    // DEGRADE is only valid for the two-qubit rotations (Rxx/Ryy/Rzz), so the
    // op is guaranteed to be one of them when a survivor exists.
    if (policy == LOSS_POLICY_DEGRADE && has_survivor) {
        // Degrade the two-qubit rotation to its single-qubit version on the
        // survivor. The op's unitary[0] holds cos(θ/2) for Rxx/Ryy; we recover
        // the angle to build the 1-qubit rotation matrix.
        let cos_half = op.unitary[0].x;
        if (op.id == OPID_RXX) {
            // Rx(θ) = [[c, -i s], [-i s, c]], where s = sin(θ/2).
            let s = op.unitary[3].y * -1.0; // unitary[3] = (0, -sin(θ/2))
            set_1q_on_pair_unitary(shot_idx, survivor_is_q2,
                vec2f(cos_half, 0.0), vec2f(0.0, -s),
                vec2f(0.0, -s), vec2f(cos_half, 0.0));
        } else if (op.id == OPID_RYY) {
            // Ry(θ) = [[c, -s], [s, c]], where s = sin(θ/2).
            let s = op.unitary[3].y; // unitary[3] = (0, sin(θ/2)) for Ryy
            set_1q_on_pair_unitary(shot_idx, survivor_is_q2,
                vec2f(cos_half, 0.0), vec2f(-s, 0.0),
                vec2f(s, 0.0), vec2f(cos_half, 0.0));
        } else {
            // Rzz -> Rz(θ). The GPU Rz convention is [[1, 0], [0, e^{iθ}]],
            // and unitary[5] = e^{iθ} holds the full-angle phase.
            let phase = op.unitary[5];
            set_1q_on_pair_unitary(shot_idx, survivor_is_q2,
                vec2f(1.0, 0.0), vec2f(0.0, 0.0),
                vec2f(0.0, 0.0), phase);
        }
        finish_2q_shot_buffer(shot_idx, op_idx, q1, q2);
        return;
    }

    // SKIP, or any policy when both operands are lost (no survivor to act on):
    // skip the gate entirely.
    shot.op_type = OPID_ID;
    shot.op_idx = op_idx;
}

//#endregion

//#region Error reporting

// Records an error `code` for `shot_idx` in both the diagnostics buffer and the
// shot's result-code slot, mirroring the reporting done elsewhere in this file.
// Used for conditions the host guarantees never occur (e.g. a loss policy that
// is not valid for a given gate).
fn report_shot_error(shot_idx: u32, code: u32) {
    atomicCompareExchangeWeak(&diagnostics.error_code, 0u, code);
    let err_index = (shot_idx + 1u) * RESULT_COUNT - 1u;
    atomicCompareExchangeWeak(&results[err_index], 0u, code);
}

//#endregion

//#region Independent Pauli noise

// Starting from the given index, return the next index if pauli noise, else 0
fn get_pauli_noise_idx(op_idx: u32) -> u32 {
    if (arrayLength(&ops) > (op_idx + 1)) {
        let op = &ops[op_idx + 1];
        if (op.id == OPID_PAULI_NOISE_1Q || op.id == OPID_PAULI_NOISE_2Q) {
            return op_idx + 1u;
        }
    }
    return 0u;
}

fn apply_1q_pauli_noise(shot_idx: u32, op_idx: u32, noise_idx: u32, q1: u32) {
    // NOTE: Assumes that whatever prepared the program ensured that noise_op.q1 matches op.q1 and
    // that op is a 1-qubit gate. `q1` is the resolved target qubit (may be
    // dynamic for the adaptive interpreter, where op.q1 is only a placeholder).
    let shot = &shots[shot_idx];
    let op = &ops[op_idx];
    let noise_op = &ops[noise_idx];

    // Categorical outcome probabilities by 3-bit term (X=1, Z=2, Y=3, L=4),
    // stored at flat slot k = term in `unitary[k / 2][k % 2]`. The identity
    // outcome (slot 0) is implicit.
    let p_x = noise_op.unitary[0].y;
    let p_z = noise_op.unitary[1].x;
    let p_y = noise_op.unitary[1].y;
    let p_loss = noise_op.unitary[2].x;

    shot.op_type = OPID_SHOT_BUFF_1Q; // Indicate to use the matrix in the shot buffer

    let rand = shot.rand_pauli;
    if (rand < p_x) {
        // Apply the X permutation (basically swap the rows)
        shot.unitary[0] = op.unitary[4];
        shot.unitary[1] = op.unitary[5];
        shot.unitary[4] = op.unitary[0];
        shot.unitary[5] = op.unitary[1];
    } else if (rand < (p_x + p_y)) {
        // Apply the Y permutation (swap rows with negated |0> state)
        shot.unitary[0] = cplxNeg(op.unitary[4]);
        shot.unitary[1] = cplxNeg(op.unitary[5]);
        shot.unitary[4] = op.unitary[0];
        shot.unitary[5] = op.unitary[1];
    } else if (rand < (p_x + p_y + p_z)) {
        // Apply Z error (negate |1> state)
        shot.unitary[0] = op.unitary[0];
        shot.unitary[1] = op.unitary[1];
        shot.unitary[4] = cplxNeg(op.unitary[4]);
        shot.unitary[5] = cplxNeg(op.unitary[5]);
    } else {
        // Either loss or no noise: the gate executes unmodified. If loss was
        // sampled, schedule a loss commit for this qubit; a following
        // loss-commit op performs the measure + reset.
        if (rand < (p_x + p_z + p_y + p_loss)) {
            shot.pending_loss_mask |= (1u << q1);
        }
        // No noise. Set the op_type back to the op.id value if it's Id, MResetZ, MZ, or ResetZ, as they get handled specially in execute_op
        if (op.id == OPID_ID || op.id == OPID_MRESETZ || op.id == OPID_MZ || op.id == OPID_RESETZ) {
            shot.op_type = op.id;
        }
        if (is_1q_phase_gate(op.id)) {
            // For phase gates, treat everything as RZ for execution purposes
            shot.op_type = OPID_RZ;
        }
    }

    shot.op_idx = op_idx;
    if (shot.op_type == OPID_ID || shot.op_type == OPID_RZ) {
        shot.qubits_updated_last_op_mask = 0u;
    } else {
        shot.qubits_updated_last_op_mask = 1u << q1;
    };
}

fn apply_2q_pauli_noise(shot_idx: u32, op_idx: u32, noise_idx: u32, q1: u32, q2: u32) {
    let shot = &shots[shot_idx];
    let op = &ops[op_idx];
    let noise_op = &ops[noise_idx];

    // The categorical distribution over the 25 (q1_term, q2_term) outcomes is
    // stored at flat slot k = q1_term * 5 + q2_term in `unitary[k / 2][k % 2]`.
    // Terms use the 3-bit encoding: I=0, X=1, Z=2, Y=3, L=4. The II slot (0) is
    // implicit and carries the remaining probability.
    var rand = shot.rand_pauli;
    var q1_term = 0;
    var q2_term = 0;

    // Find the terms to apply based on the random number and the probabilities
    for (var a = 0; a < 5; a = a + 1) {
        for (var b = 0; b < 5; b = b + 1) {
            let k = a * 5 + b;
            if (k == 0) { continue; } // II carries no stored probability
            let slot = noise_op.unitary[k / 2];
            let p_ab = select(slot.x, slot.y, (k & 1) == 1);
            if (rand < p_ab) {
                q1_term = a;
                q2_term = b;
                // Break out of both loops
                a = 5;
                b = 5;
            } else {
                rand = rand - p_ab;
            }
        }
    }

    // Schedule loss commits for any qubit whose sampled term is loss (L = 4).
    // A following loss-commit op performs the measure + reset.
    if (q1_term == 4) { shot.pending_loss_mask |= (1u << q1); }
    if (q2_term == 4) { shot.pending_loss_mask |= (1u << q2); }

    // A Pauli fault (X, Z, Y = 1, 2, 3) is fused into the gate by permuting its
    // rows. Loss (4) and identity (0) leave the gate unmodified for that qubit.
    let q1_pauli = q1_term >= 1 && q1_term <= 3;
    let q2_pauli = q2_term >= 1 && q2_term <= 3;

    if (q1_pauli || q2_pauli) {
        // Get the rows of the 2 qubit unitary
        var op_row_0 = getOpRow(op_idx, 0);
        var op_row_1 = getOpRow(op_idx, 1);
        var op_row_2 = getOpRow(op_idx, 2);
        var op_row_3 = getOpRow(op_idx, 3);

        // Apply the Paulis to the matrices. Note this is just permuting the rows, and appliction
        // commutes, so we can apply them in any order. High order bit is q1. Low order bit is q2.
        //   X on q1 is rows  2<>0 and  3<>1, X on q2 is rows  1<>0 and  3<>2, etc.
        //   Y on q1 is rows -2<>0 and -3<>1, Y on q2 is rows -1<>0 and -3<>2
        //   Z on q1 is -2 and -3, Z on q2 is -1 and -3

        // Apply the q1 permutations as needed
        if (q1_term == 1) {
            // Apply the X permutation
            let old_row_0 = op_row_0;
            let old_row_1 = op_row_1;
            op_row_0 = op_row_2;
            op_row_1 = op_row_3;
            op_row_2 = old_row_0;
            op_row_3 = old_row_1;
        } else if (q1_term == 3) {
            // Apply the Y permutation
            let old_row_0 = op_row_0;
            let old_row_1 = op_row_1;
            op_row_0 = rowNeg(op_row_2);
            op_row_1 = rowNeg(op_row_3);
            op_row_2 = old_row_0;
            op_row_3 = old_row_1;
        } else if (q1_term == 2) {
            // Apply Z permutation
            op_row_2 = rowNeg(op_row_2);
            op_row_3 = rowNeg(op_row_3);
        }
        // Apply the q2 permutations as needed
        if (q2_term == 1) {
            // Apply the X permutation
            let old_row_0 = op_row_0;
            let old_row_2 = op_row_2;
            op_row_0 = op_row_1;
            op_row_2 = op_row_3;
            op_row_1 = old_row_0;
            op_row_3 = old_row_2;
        } else if (q2_term == 3) {
            // Apply the Y permutation
            let old_row_0 = op_row_0;
            let old_row_2 = op_row_2;
            op_row_0 = rowNeg(op_row_1);
            op_row_2 = rowNeg(op_row_3);
            op_row_1 = old_row_0;
            op_row_3 = old_row_2;
        } else if (q2_term == 2) {
            // Apply Z permutation
            op_row_1 = rowNeg(op_row_1);
            op_row_3 = rowNeg(op_row_3);
        }
        // Write the rows back to the shot buffer unitary
        setUnitaryRow(shot_idx, 0u, op_row_0);
        setUnitaryRow(shot_idx, 1u, op_row_1);
        setUnitaryRow(shot_idx, 2u, op_row_2);
        setUnitaryRow(shot_idx, 3u, op_row_3);
        shot.op_type = OPID_SHOT_BUFF_2Q;
    } else {
        // No Pauli fault to fuse (identity or loss only). Leave if CX, CY, CZ, or RZZ as they get handled specially in execute_op
        if (op.id == OPID_CX || op.id == OPID_CY || op.id == OPID_CZ || op.id == OPID_RZZ) {
            shot.op_type = op.id;
        } else {
            shot.op_type = OPID_SHOT_BUFF_2Q;
        }
    }
    shot.op_idx = op_idx;
    if (shot.op_type == OPID_CZ || shot.op_type == OPID_RZZ) {
        shot.qubits_updated_last_op_mask = 0u;
    } else  {
        shot.qubits_updated_last_op_mask = (1u << q1 ) | (1u << q2);
    }
}

// Left-multiplies the 4x4 pair unitary already in `shot.unitary` by a single
// Pauli (term: X=1, Z=2, Y=3) acting on `target_is_q2 ? q2 : q1`. This is the
// same row permutation/negation that `apply_2q_pauli_noise` fuses, just applied
// to the policy-degraded gate rather than the original op. Note the Y branch
// uses real signs (i.e. -i*Y), matching `apply_2q_pauli_noise`; the resulting
// global phase is unobservable for a Pauli noise channel.
fn fuse_1q_pauli_on_pair_unitary(shot_idx: u32, target_is_q2: bool, term: u32) {
    let si = i32(shot_idx);
    var row_0 = getUnitaryRow(si, 0u);
    var row_1 = getUnitaryRow(si, 1u);
    var row_2 = getUnitaryRow(si, 2u);
    var row_3 = getUnitaryRow(si, 3u);

    if (!target_is_q2) {
        // Acting on q1 (high bit): rows {0,1} <-> {2,3}.
        if (term == 1u) {            // X
            let o0 = row_0; let o1 = row_1;
            row_0 = row_2; row_1 = row_3;
            row_2 = o0;    row_3 = o1;
        } else if (term == 3u) {     // Y
            let o0 = row_0; let o1 = row_1;
            row_0 = rowNeg(row_2); row_1 = rowNeg(row_3);
            row_2 = o0;            row_3 = o1;
        } else {                     // Z
            row_2 = rowNeg(row_2); row_3 = rowNeg(row_3);
        }
    } else {
        // Acting on q2 (low bit): rows {0,2} <-> {1,3}.
        if (term == 1u) {            // X
            let o0 = row_0; let o2 = row_2;
            row_0 = row_1; row_2 = row_3;
            row_1 = o0;    row_3 = o2;
        } else if (term == 3u) {     // Y
            let o0 = row_0; let o2 = row_2;
            row_0 = rowNeg(row_1); row_2 = rowNeg(row_3);
            row_1 = o0;            row_3 = o2;
        } else {                     // Z
            row_1 = rowNeg(row_1); row_3 = rowNeg(row_3);
        }
    }

    setUnitaryRow(shot_idx, 0u, row_0);
    setUnitaryRow(shot_idx, 1u, row_1);
    setUnitaryRow(shot_idx, 2u, row_2);
    setUnitaryRow(shot_idx, 3u, row_3);
}

// Applies the Pauli noise attached to a 2-qubit gate that had a lost operand.
// The gate body itself was already handled by `handle_lost_operand_policy`
// (which may have left a degraded 4x4 in `shot.unitary`, or turned the gate
// into Id for SKIP). This mirrors the CPU `apply_fault`: the joint (q1, q2)
// term is sampled, but only the operand still alive *after* the policy ran
// receives its term; a lost operand gets nothing.
//
// Because this is only reached when the gate has at least one lost operand,
// there is at most one surviving operand, so at most one single-qubit Pauli is
// fused.
fn apply_2q_pauli_noise_on_survivor(shot_idx: u32, op_idx: u32, noise_idx: u32, q1: u32, q2: u32) {
    let shot = &shots[shot_idx];
    let noise_op = &ops[noise_idx];

    // Surviving operand(s) after the policy ran (alive => heat != -1.0).
    let q1_alive = shot.qubit_state[q1].heat != -1.0;
    let q2_alive = shot.qubit_state[q2].heat != -1.0;
    // Both lost (e.g. PROPAGATE collapsed the survivor): nothing to apply.
    if (!q1_alive && !q2_alive) {
        return;
    }

    // Sample the joint (q1_term, q2_term) outcome (same encoding/layout as
    // apply_2q_pauli_noise: I=0, X=1, Z=2, Y=3, L=4).
    var rand = shot.rand_pauli;
    var q1_term = 0;
    var q2_term = 0;
    for (var a = 0; a < 5; a = a + 1) {
        for (var b = 0; b < 5; b = b + 1) {
            let k = a * 5 + b;
            if (k == 0) { continue; }
            let slot = noise_op.unitary[k / 2];
            let p_ab = select(slot.x, slot.y, (k & 1) == 1);
            if (rand < p_ab) {
                q1_term = a;
                q2_term = b;
                a = 5;
                b = 5;
            } else {
                rand = rand - p_ab;
            }
        }
    }

    // The survivor's own term. (At most one operand is alive here.)
    let survivor_is_q2 = !q1_alive;
    let survivor = select(q1, q2, survivor_is_q2);
    let term = select(q1_term, q2_term, survivor_is_q2);

    // Loss (4): schedule a loss commit for the survivor; a later loss-commit op
    // performs the measure + reset. The gate set up by the policy still runs.
    if (term == 4) {
        shot.pending_loss_mask |= (1u << survivor);
        return;
    }

    // Identity (0): nothing to fuse; leave the policy's setup untouched.
    if (term == 0) {
        return;
    }

    // Pauli (X=1, Z=2, Y=3): fuse onto the survivor.
    if (shot.op_type == OPID_SHOT_BUFF_2Q) {
        // The policy left a degraded 4x4 in shot.unitary; left-multiply it by
        // the survivor Pauli.
        fuse_1q_pauli_on_pair_unitary(shot_idx, survivor_is_q2, u32(term));
    } else {
        // The policy turned the gate into Id (SKIP). Build a pair unitary that
        // applies just the Pauli to the survivor and identity to the lost
        // partner (which is in |0>). Real-sign Y matches the fuse path above.
        if (term == 1) {        // X
            set_1q_on_pair_unitary(shot_idx, survivor_is_q2,
                vec2f(0.0, 0.0), vec2f(1.0, 0.0),
                vec2f(1.0, 0.0), vec2f(0.0, 0.0));
        } else if (term == 3) { // Y (real-sign, i.e. -i*Y)
            set_1q_on_pair_unitary(shot_idx, survivor_is_q2,
                vec2f(0.0, 0.0), vec2f(-1.0, 0.0),
                vec2f(1.0, 0.0), vec2f(0.0, 0.0));
        } else {                // Z
            set_1q_on_pair_unitary(shot_idx, survivor_is_q2,
                vec2f(1.0, 0.0), vec2f(0.0, 0.0),
                vec2f(0.0, 0.0), vec2f(-1.0, 0.0));
        }
        finish_2q_shot_buffer(shot_idx, op_idx, q1, q2);
    }

    // The survivor's amplitudes may have been in a definite computational-basis
    // state; clear its definite-state bits so the execute pass recomputes them
    // after the Pauli (mirrors the SWAP handling in handle_lost_operand_policy).
    shot.qubit_is_0_mask = shot.qubit_is_0_mask & ~(1u << survivor);
    shot.qubit_is_1_mask = shot.qubit_is_1_mask & ~(1u << survivor);
}

//#endregion

//#region Shot and kernel params

fn get_shot_params(
        workgroupId: u32,
        tid: u32,
        op_qubit_count: i32) -> ShotParams {
    // Workgroups are per shot if 22 or less qubits, else 2 workgroups for 23 qubits, 4 for 24, etc..
    let shot_idx: i32 = i32(workgroupId) / WORKGROUPS_PER_SHOT;
    let shot_state_vector_start: i32 = shot_idx * (1i << u32(QUBIT_COUNT));
    let workgroup_idx_in_shot: i32 = i32(workgroupId) % WORKGROUPS_PER_SHOT;
    let thread_idx_in_shot: i32 = workgroup_idx_in_shot * THREADS_PER_WORKGROUP + i32(tid);
    let total_threads_per_shot: i32 = WORKGROUPS_PER_SHOT * THREADS_PER_WORKGROUP;

    // If using multiple workgroups per shot, each workgroup will write its partial sums to the collation
    // buffer for later summing by the prepare_op stage. If single workgroup per shot, no collation needed.
    // Use -1 as a marker for single workgroup per shot case (in which case we should write directly to the shot).
    let workgroup_collation_idx: i32 = select(-1, i32(workgroupId), WORKGROUPS_PER_SHOT > 1);

    let zero_entry_count: i32 = (1i << u32(QUBIT_COUNT)) >> u32(op_qubit_count);
    let op_iterations: i32 = zero_entry_count / total_threads_per_shot;

    return ShotParams(
        shot_idx,
        shot_state_vector_start,
        workgroup_collation_idx,
        workgroup_idx_in_shot,
        thread_idx_in_shot,
        total_threads_per_shot,
        zero_entry_count,
        op_iterations
    );
}

//#endregion

//#region Gate application (execute stage)

fn apply_1q_op(workgroupId: u32, tid: u32, q1: u32) {
    let params = get_shot_params(workgroupId, tid, 1 /* qubits per op */);
    let shot = &shots[params.shot_idx];
    let scale = shot.renormalize;
    let lowMask = (1 << q1) - 1;
    let highMask = (1 << u32(QUBIT_COUNT)) - 1 - lowMask;
    let qubit_is_0_mask = i32(shots[params.shot_idx].qubit_is_0_mask);
    let qubit_is_1_mask = i32(shots[params.shot_idx].qubit_is_1_mask);

    var summed_probs: vec4f = vec4f();

    /* This loop is where all the real work happens. Try to keep this tight and efficient.

    We want a 'structure of arrays' like access pattern here for efficiency, so we process the state vector
    in blocks where each thread in the workgroup(s) handle an adjacent entry to be processed.

    Each thread should start at the state vector shot start + 'thread_idx_in_shot', which is sequential across the workgroup threads
    Each next entry for the thread is WORKGROUPS_PER_SHOT * THREADS_PER_WORKGROUP away.
    */
    var entry_index = params.thread_idx_in_shot;

    for (var i = 0; i < params.op_iterations; i++) {
        let offset0: i32 = (entry_index & lowMask) | ((entry_index & highMask) << 1);
        let offset1: i32 = offset0 | (1 << q1);

        // See if we can skip doing any work for this pair, because the state vector entries to processes
        // are both definitely 0.0, as we know they are for states where other qubits are in definite opposite state.
        let skip_processing = ((offset0 & qubit_is_0_mask) != 0) || ((~offset1 & qubit_is_1_mask) != 0);

        if (!skip_processing) {
            if shot.op_type == OPID_RZ {
                // For RZ, we can skip reading/writing the |0> amplitude, as it is unchanged.
                // Just apply the phase to the |1> amplitude. Probabilities also don't change.
                let amp1: vec2f = stateVector[params.shot_state_vector_start + offset1];
                let new1 = cplxMul(amp1, shot.unitary[5]);
                stateVector[params.shot_state_vector_start + offset1] = new1;
            } else {
                let amp0: vec2f = stateVector[params.shot_state_vector_start + offset0];
                let amp1: vec2f = stateVector[params.shot_state_vector_start + offset1];

                let new0 = scale * (cplxMul(amp0, shot.unitary[0]) + cplxMul(amp1, shot.unitary[1]));
                let new1 = scale * (cplxMul(amp0, shot.unitary[4]) + cplxMul(amp1, shot.unitary[5]));

                stateVector[params.shot_state_vector_start + offset0] = new0;
                stateVector[params.shot_state_vector_start + offset1] = new1;

                if shot.op_type == OPID_MRESETZ || shot.op_type == OPID_LOSS_NOISE || scale != 1.0 {
                    // For MResetZ, loss-commit, or renormalization, update the probabilities for all qubits
                    update_all_qubit_probs(u32(offset0), new0, tid);
                    update_all_qubit_probs(u32(offset1), new1, tid);
                } else {
                    summed_probs[0] += cplxMag2(new0);
                    summed_probs[1] += cplxMag2(new1);
                }
            }
        }
        entry_index += params.total_threads_per_shot;
    }

    if scale == 1.0 && shot.op_type != OPID_RZ && shot.op_type != OPID_MRESETZ && shot.op_type != OPID_LOSS_NOISE {
        // Update this thread's totals for the two qubits in the workgroup storage
        qubitProbabilities[tid].zero[q1] = summed_probs[0];
        qubitProbabilities[tid].one[q1]  = summed_probs[1];
    }
}

fn apply_2q_op(workgroupId: u32, tid: u32, q1: u32, q2: u32) {
    let params = get_shot_params(workgroupId, tid, 2 /* qubits per op */);
    let shot = &shots[params.shot_idx];
    let update_probs = shot.op_type != OPID_CZ && shot.op_type != OPID_RZZ;

    // Sometimes a 2-qubit op may be converted to a no-op (ID) due to qubit loss etc., so skip processing in that case
    // Calculate masks to split the index into low, mid, and high bits around the two qubits
    let lowQubit = select(q1, q2, q1 > q2);
    let hiQubit = select(q1, q2, q1 < q2);

    // Number of bits in each section
    let lowBitCount = lowQubit;
    let midBitCount = hiQubit - lowQubit - 1;
    let hiBitCount = u32(QUBIT_COUNT) - hiQubit - 1;

    // The masks below help extract the low, mid, and high bits from the counter to use around the two qubits locations
    let lowMask = (1 << lowBitCount) - 1;
    let midMask = (1 << (lowBitCount + midBitCount)) - 1 - lowMask;
    let hiMask = (1 << u32(QUBIT_COUNT)) - 1 - midMask - lowMask;

    // Each iteration processes 4 amplitudes (the four affected by the 2-qubit gate), so quarter as many iterations as chunk size
    var entry_index = params.thread_idx_in_shot;
    var summed_probs: vec4f = vec4f();

    for (var i = 0; i < params.op_iterations; i++) {
        // q1 is the control, q2 is the target
        let offset00: i32 = (entry_index & lowMask) | ((entry_index & midMask) << 1) | ((entry_index & hiMask) << 2);
        let offset01: i32 = offset00 | (1 << q2);
        let offset10: i32 = offset00 | (1 << q1);
        let offset11: i32 = offset10 | (1 << q2);

        let can_skip_processing =
            (((u32(offset00) & shot.qubit_is_0_mask) != 0) ||
            ((~(u32(offset11)) & shot.qubit_is_1_mask) != 0));
        if !can_skip_processing {
            switch shot.op_type {
            case OPID_CZ {
                let amp11: vec2f = stateVector[params.shot_state_vector_start + offset11];
                stateVector[params.shot_state_vector_start + offset11] = cplxNeg(amp11);
                // CZ doesn't change any probabilities, so no need to update summed_probs
            }
            case OPID_RZZ {
                // Firt and last entries are unchanged, only need to update the middle two
                let amp01: vec2f = stateVector[params.shot_state_vector_start + offset01];
                let amp10: vec2f = stateVector[params.shot_state_vector_start + offset10];
                // Unitary matrix second entry in the second row is 5, third entry in the third row is 10
                stateVector[params.shot_state_vector_start + offset01] = cplxMul(amp01, shot.unitary[5]);
                stateVector[params.shot_state_vector_start + offset10] = cplxMul(amp10, shot.unitary[10]);
            }
            case OPID_CX {
                // Need to read all 4 to update the probabilities correctly, but only swap the |10> and |11> entries
                let amp00: vec2f = stateVector[params.shot_state_vector_start + offset00];
                let amp01: vec2f = stateVector[params.shot_state_vector_start + offset01];
                let amp10: vec2f = stateVector[params.shot_state_vector_start + offset10];
                let amp11: vec2f = stateVector[params.shot_state_vector_start + offset11];
                stateVector[params.shot_state_vector_start + offset10] = amp11;
                stateVector[params.shot_state_vector_start + offset11] = amp10;
                summed_probs[0] += (cplxMag2(amp00) + cplxMag2(amp01));
                summed_probs[1] += (cplxMag2(amp11) + cplxMag2(amp10));
                summed_probs[2] += (cplxMag2(amp00) + cplxMag2(amp11));
                summed_probs[3] += (cplxMag2(amp01) + cplxMag2(amp10));
            }
            case OPID_CY {
                // Like CX, but swap |10> and |11> with +/- i phases.
                let amp00: vec2f = stateVector[params.shot_state_vector_start + offset00];
                let amp01: vec2f = stateVector[params.shot_state_vector_start + offset01];
                let amp10: vec2f = stateVector[params.shot_state_vector_start + offset10];
                let amp11: vec2f = stateVector[params.shot_state_vector_start + offset11];
                stateVector[params.shot_state_vector_start + offset10] = vec2f(amp11.y, -amp11.x); // -i * |11>
                stateVector[params.shot_state_vector_start + offset11] = vec2f(-amp10.y, amp10.x); // i * |10>
                summed_probs[0] += (cplxMag2(amp00) + cplxMag2(amp01));
                summed_probs[1] += (cplxMag2(amp11) + cplxMag2(amp10));
                summed_probs[2] += (cplxMag2(amp00) + cplxMag2(amp11));
                summed_probs[3] += (cplxMag2(amp01) + cplxMag2(amp10));
            }
            default {
                // Assume OPID_SHOT_BUFF_2Q
                // Get the state vector entries
                let states = array<vec2f,4>(
                    stateVector[params.shot_state_vector_start + offset00],
                    stateVector[params.shot_state_vector_start + offset01],
                    stateVector[params.shot_state_vector_start + offset10],
                    stateVector[params.shot_state_vector_start + offset11]
                );
                // Apply the unitary from the shot buffer
                let result00 = innerProduct(getUnitaryRow(params.shot_idx, 0), states);
                let result01 = innerProduct(getUnitaryRow(params.shot_idx, 1), states);
                let result10 = innerProduct(getUnitaryRow(params.shot_idx, 2), states);
                let result11 = innerProduct(getUnitaryRow(params.shot_idx, 3), states);
                // Write back the results
                stateVector[params.shot_state_vector_start + offset00] = result00;
                stateVector[params.shot_state_vector_start + offset01] = result01;
                stateVector[params.shot_state_vector_start + offset10] = result10;
                stateVector[params.shot_state_vector_start + offset11] = result11;
                // Update the probabilities for the acted on qubits
                summed_probs[0] += (cplxMag2(result00) + cplxMag2(result01));
                summed_probs[1] += (cplxMag2(result10) + cplxMag2(result11));
                summed_probs[2] += (cplxMag2(result00) + cplxMag2(result10));
                summed_probs[3] += (cplxMag2(result01) + cplxMag2(result11));
            }
            }
        }

        entry_index += params.total_threads_per_shot;
    }

    // Update this thread's totals for the two qubits in the workgroup storage
    if (update_probs) {
        // Update all for other 2-qubit gates
        qubitProbabilities[tid].zero[q1] = summed_probs[0];
        qubitProbabilities[tid].one[q1]  = summed_probs[1];
        qubitProbabilities[tid].zero[q2] = summed_probs[2];
        qubitProbabilities[tid].one[q2]  = summed_probs[3];
    }
}

fn apply_correlated_noise(workgroupId: u32, tid: u32) {
    let params = get_shot_params(workgroupId, tid, 0 /* need to walk all entries */);
    // Probabilities are already updated in the prepare_op stage
    // Here we just need to apply the bit-flips and phase-flips to the state vector amplitudes

    let shot = &shots[params.shot_idx];

    // Get the bit-flip and phase-flip masks from the shot buffer (stored by prep_correlated_noise)
    let bit_flip_mask = bitcast<u32>(shot.unitary[0].x);
    let phase_flip_mask = bitcast<u32>(shot.unitary[0].y);

    // If no flips to apply, early exit
    if (bit_flip_mask == 0u && phase_flip_mask == 0u) {
        return;
    }

    var entry_index = params.thread_idx_in_shot;

    for (var i = 0; i < params.op_iterations; i++) {
        // Get the target index to swap the state with by flipping the bits as indicated in the bit_flip_mask
        let target_index = entry_index ^ i32(bit_flip_mask);

        // If there are an odd number of phase flips for the entry, we need to negate the amplitude
        let negate_index: f32 = select(1.0, -1.0, (countOneBits(entry_index & i32(phase_flip_mask)) & 1) != 0);

        if (bit_flip_mask == 0u && negate_index == -1.0) {
            // No bit flips to perform, but need to negate this entry (phase flip only)
            stateVector[params.shot_state_vector_start + entry_index] = cplxNeg(stateVector[params.shot_state_vector_start + entry_index]);
        } else if (entry_index < target_index) {
            // Bit flips are happening (as the indices are different), but to avoid double swapping only handle the swap
            // when entry_index < target_index (avoid reprocessing when later we encounter the target_index entry as the entry_index)

            let amp_entry: vec2f = stateVector[params.shot_state_vector_start + entry_index];
            let amp_target: vec2f = stateVector[params.shot_state_vector_start + target_index];

            // If there are an odd number of phase flips for the target, we need to negate that amplitude too
            let negate_target: f32 = select(1.0, -1.0, (countOneBits(target_index & i32(phase_flip_mask)) & 1) != 0);

            // Swap and apply any negations for phase flips.
            // Note this only applies -1 & 1 to the phase, not -i and i as the 'canonical' Y gate does.
            // However, this is sufficient for simulating noise, as the global phase doesn't matter.
            stateVector[params.shot_state_vector_start + entry_index] = cplxMul(amp_target, vec2f(negate_index, 0.0));
            stateVector[params.shot_state_vector_start + target_index] = cplxMul(amp_entry, vec2f(negate_target, 0.0));
        }

        // Jump ahead to the next entry to process
        entry_index += params.total_threads_per_shot;
    }
}

//#endregion

//#region Correlated noise

// Samples the correlated noise table to determine whether noise should be applied, and if so,
// which Pauli string was selected. If no noise is applied, the shot is set to ID and the caller
// can return early.
fn sample_correlated_noise(shot_idx: u32, op_idx: u32, noise_table_idx: u32) -> CorrelatedNoiseSample {
    let shot = &shots[shot_idx];
    let table = &batch_data.correlated_noise_tables[noise_table_idx];

    // Generate a Q1.63 random number (two u32 values for lo and hi 32 bits)
    // Mask off the high bit of rand_hi to ensure the value is in [0, 1) range
    let rand_lo = next_rand_u32(shot_idx);
    let rand_hi = next_rand_u32(shot_idx) & 0x7FFFFFFFu;

    // Get the total noise probability from the table metadata
    let noise_prob_lo = table.noise_probability_lo;
    let noise_prob_hi = table.noise_probability_hi;

    // Check if noise should be applied at all by comparing the random number against the total noise probability
    // If rand >= noise_probability, then no noise is applied
    if (rand_hi > noise_prob_hi || (rand_hi == noise_prob_hi && rand_lo >= noise_prob_lo)) {
        // No noise to apply - set the op to ID
        shot.op_type = OPID_ID;
        shot.op_idx = op_idx;
        shot.qubits_updated_last_op_mask = 0u;
        return CorrelatedNoiseSample(0u, 0u, 0u);
    }

    // Noise should be applied - binary search to find which Pauli string to apply
    let start = i32(table.start_offset);
    let count = i32(table.entry_count);
    let entry_idx = binary_search_noise_table(rand_lo, rand_hi, start, count);
    let entry = &batch_data.correlated_noise_entries[start + entry_idx];

    return CorrelatedNoiseSample(1u, entry.paulis_lo, entry.paulis_hi);
}

// Extracts the 3-bit term value for qubit position `i` from a Pauli + loss string.
// Terms use the encoding I=0, X=1, Z=2, Y=3, L=4. The low two bits double as the
// bit-flip (0x1) and phase-flip (0x2) indicators, and 0x4 marks loss.
// The Rust parsing stores terms with the rightmost (last) character at the lowest
// bits, so for position i we read the 3 bits at (qubit_count - 1 - i) * 3.
fn get_pauli_bits(paulis_lo: u32, paulis_hi: u32, qubit_count: u32, i: u32) -> u32 {
    let bit_position = (qubit_count - 1u - i) * 3u;
    if (bit_position + 3u <= 32u) {
        return (paulis_lo >> bit_position) & 0x7u;
    } else if (bit_position >= 32u) {
        return (paulis_hi >> (bit_position - 32u)) & 0x7u;
    } else {
        // The 3-bit term straddles the boundary between the lo and hi words.
        let low_part = paulis_lo >> bit_position;
        let high_part = paulis_hi << (32u - bit_position);
        return (low_part | high_part) & 0x7u;
    }
}

// Commits correlated noise masks into the shot state: stores the masks, swaps probabilities and
// tracking bits for bit-flipped qubits, records any loss, and sets the shot up for the correlated
// noise execute stage. Qubits in `loss_mask` are scheduled for loss; following loss-commit ops
// perform the measure + reset.
fn commit_correlated_noise(shot_idx: u32, op_idx: u32, bit_flip_mask: u32, phase_flip_mask: u32, loss_mask: u32) {
    let shot = &shots[shot_idx];

    // Schedule loss for any qubit whose sampled term was loss. The actual
    // measure + reset is performed by the loss-commit ops emitted after the
    // correlated-noise op.
    shot.pending_loss_mask |= loss_mask;

    // Store the masks in the shot buffer for the execute stage
    // We use the unitary entries to store these masks (reinterpreted as floats)
    shot.unitary[0] = vec2f(bitcast<f32>(bit_flip_mask), bitcast<f32>(phase_flip_mask));

    // For bit-flipped qubits, we need to swap the 0 and 1 probabilities and masks
    // This is done in prepare_op, not execute_op, since it's a simple swap
    for (var q: u32 = 0u; q < u32(QUBIT_COUNT); q++) {
        let qubit_mask = 1u << q;
        if ((bit_flip_mask & qubit_mask) != 0u) {
            // Swap the probabilities
            let temp = shot.qubit_state[q].zero_probability;
            shot.qubit_state[q].zero_probability = shot.qubit_state[q].one_probability;
            shot.qubit_state[q].one_probability = temp;

            // Swap the bits in qubit_is_0_mask and qubit_is_1_mask
            let was_0 = (shot.qubit_is_0_mask & qubit_mask) != 0u;
            let was_1 = (shot.qubit_is_1_mask & qubit_mask) != 0u;
            if (was_0) {
                shot.qubit_is_0_mask &= ~qubit_mask;
                shot.qubit_is_1_mask |= qubit_mask;
            } else if (was_1) {
                shot.qubit_is_1_mask &= ~qubit_mask;
                shot.qubit_is_0_mask |= qubit_mask;
            }
        }
    }

    // Set up the shot state for the correlated noise execution
    shot.op_type = OPID_CORRELATED_NOISE;
    shot.op_idx = op_idx;
    // No probabilities need to be recomputed in execute_op since we've already swapped them here
    shot.qubits_updated_last_op_mask = 0u;
}

// Performas a binary search on a correlated noise probability table
//
// Preconditions:
// - table is sorted ascending, with every entry higher than the prior
// - table entries are cumulative probabilities totaling <= 1.0
// - 'start' is the offset into the buffer array where this table's entries begin
// - 'count' is the number of entries in this table
// - 'rand_lo' and 'rand_hi' form a Q1.63 format random number in [0.0, 1.0) to use for the search
// - This will only called if a result should be found, i.e.,
//   - count > 0
//   - rand < table[start + count - 1].probability
//
// Returns the index of the found entry relative to 'start', which is the smallest index where "rand < table[start + index].probability"
fn binary_search_noise_table(rand_lo: u32, rand_hi: u32, start: i32, count: i32) -> i32 {
    var low: i32 = 0;
    var high: i32 = count;

    while (low < high) {
        let mid: i32 = low + (high - low) / 2;
        let p_lo = batch_data.correlated_noise_entries[start + mid].probability_lo;
        let p_hi = batch_data.correlated_noise_entries[start + mid].probability_hi;

        if (rand_hi < p_hi || (rand_hi == p_hi && rand_lo < p_lo)) {
            high = mid;
        } else {
            low = mid + 1;
        }
    }
    return low;
}

// Get the qubit id at the given index from the correlated noise op's qubit args
// Qubit args are stored in the unitary matrix elements as f32 values
fn get_correlated_noise_qubit(op_idx: u32, index: u32) -> u32 {
    // Qubit ids are stored in the unitary as f32 values, starting at unitary[0].x, unitary[0].y, etc.
    let vec_idx = index / 2u;
    let component = index % 2u;
    if (component == 0u) {
        return u32(ops[op_idx].unitary[vec_idx].x);
    } else {
        return u32(ops[op_idx].unitary[vec_idx].y);
    }
}

// Prepare the shot state for executing a correlated noise operation.
// Resolves qubit IDs from the op's unitary matrix, samples the noise table, builds masks, and applies.
fn prep_correlated_noise(shot_idx: u32, op_idx: u32) {
    let op = &ops[op_idx];
    let noise_table_idx = op.q1;
    let qubit_count = op.q2;

    let sample = sample_correlated_noise(shot_idx, op_idx, noise_table_idx);
    if (sample.should_apply == 0u) { return; }

    // Build bit-flip, phase-flip, and loss masks using qubit IDs from the op's unitary matrix
    var bit_flip_mask: u32 = 0u;
    var phase_flip_mask: u32 = 0u;
    var loss_mask: u32 = 0u;
    for (var i: u32 = 0u; i < qubit_count; i++) {
        let pauli_bits = get_pauli_bits(sample.paulis_lo, sample.paulis_hi, qubit_count, i);
        let qubit_mask = 1u << get_correlated_noise_qubit(op_idx, i);
        if ((pauli_bits & 0x4u) != 0u) {
            // Loss term (L = 4): the qubit is lost, no Pauli is applied to it.
            loss_mask |= qubit_mask;
        } else {
            if ((pauli_bits & 0x1u) != 0u) { bit_flip_mask |= qubit_mask; }
            if ((pauli_bits & 0x2u) != 0u) { phase_flip_mask |= qubit_mask; }
        }
    }

    commit_correlated_noise(shot_idx, op_idx, bit_flip_mask, phase_flip_mask, loss_mask);
}

//#endregion

//#region Adaptive QIR utility functions

// -----------------------------------------------------------------------------
// Adaptive interpreter — register file access
// -----------------------------------------------------------------------------

fn read_reg(shot_idx: u32, reg: u32) -> u32 {
    return shots[shot_idx].interp.registers[reg];
}

fn write_reg(shot_idx: u32, reg: u32, val: u32) {
    shots[shot_idx].interp.registers[reg] = val;
}

fn read_reg_i32(shot_idx: u32, reg: u32) -> i32 {
    return bitcast<i32>(read_reg(shot_idx, reg));
}

fn write_reg_i32(shot_idx: u32, reg: u32, val: i32) {
    write_reg(shot_idx, reg, bitcast<u32>(val));
}

fn read_reg_f32(shot_idx: u32, reg: u32) -> f32 {
    return bitcast<f32>(read_reg(shot_idx, reg));
}

fn write_reg_f32(shot_idx: u32, reg: u32, val: f32) {
    write_reg(shot_idx, reg, bitcast<u32>(val));
}

// -----------------------------------------------------------------------------
// Adaptive interpreter — instruction fetch and opcode extraction
// -----------------------------------------------------------------------------

fn fetch_instr(pc: u32) -> Instruction {
    return batch_data.program.instructions[pc];
}

fn get_opcode(packed: u32) -> u32   { return packed & 0xFFu; }
fn get_subcond(packed: u32) -> u32  { return (packed >> 8u) & 0xFFu; }
fn get_flags(packed: u32) -> u32    { return (packed >> 16u) & 0xFFu; }

fn resolve_i32(shot_idx: u32, operand: u32, flags: u32, operand_idx: u32) -> i32 {
    if (flags & (1u << operand_idx)) != 0u {
        return bitcast<i32>(operand);  // immediate
    }
    return read_reg_i32(shot_idx, operand);  // register
}

fn resolve_u32(shot_idx: u32, operand: u32, flags: u32, operand_idx: u32) -> u32 {
    if (flags & (1u << operand_idx)) != 0u {
        return operand;
    }
    return read_reg(shot_idx, operand);
}

fn resolve_f32(shot_idx: u32, operand: u32, flags: u32, operand_idx: u32) -> f32 {
    if (flags & (1u << operand_idx)) != 0u {
        return bitcast<f32>(operand);  // immediate (IEEE 754 bit pattern)
    }
    return read_reg_f32(shot_idx, operand);
}

// Resolves q1 for the current quantum instruction.
fn resolve_q1(shot_idx: u32) -> u32 {
    let state = shots[shot_idx].interp;
    let instr = fetch_instr(state.pc - 1);
    if (instr.opcode & FLAG_AUX1_IMM) != 0 {
        return instr.aux1;
    }
    return read_reg(shot_idx, instr.aux1);
}

// Resolves q2 for the current quantum instruction.
fn resolve_q2(shot_idx: u32) -> u32 {
    let state = shots[shot_idx].interp;
    let instr = fetch_instr(state.pc - 1);
    if (instr.opcode & FLAG_AUX2_IMM) != 0 {
        return instr.aux2;
    }
    return read_reg(shot_idx, instr.aux2);
}

// Resolves the rotation angle for the current quantum instruction.
// The angle is stored in the instruction's src0 field (register or immediate).
fn resolve_gate_angle(shot_idx: u32) -> f32 {
    let state = shots[shot_idx].interp;
    let instr = fetch_instr(state.pc - 1);
    let flags = get_flags(instr.opcode);
    return resolve_f32(shot_idx, instr.src0, flags, 0u);
}

// Read a measurement result from the existing results buffer.
// Results are stored as atomic<u32> at shot_idx * RESULT_COUNT + result_id.
fn read_measurement_result(shot_idx: u32, result_id: u32) -> bool {
    return atomicLoad(&results[shot_idx * RESULT_COUNT + result_id]) == 1u;
}

// Return true if the id corresponds to a rotation gate.
fn is_rotation_gate(id: u32) -> bool {
    return (12 <= id && id <= 14) || (17 <= id && id <= 19);
}

// Return true if the angle for the current rotation gate is dynamic.
fn is_dynamic_angle(shot_idx: u32) -> bool {
    let state = shots[shot_idx].interp;
    let instr = fetch_instr(state.pc - 1);
    return (instr.opcode | FLAG_SRC0_IMM) != 0;
}

// Commit a sampled qubit loss on an explicitly given qubit (measure + reset to
// |0> and mark the qubit lost). The lost qubit is carried to the execute stage
// in `op_idx`, and `op_type` is set to OPID_LOSS_NOISE so execute applies the
// reset matrix to that explicit qubit.
fn prep_loss_commit(shot_idx: u32, qubit: u32) {
    let shot = &shots[shot_idx];
    let result = select(1u, 0u, shot.rand_measure < shot.qubit_state[qubit].zero_probability);
    shot.qubit_state[qubit].heat = -1.0;
    prep_measure_reset_instrument(shot_idx, qubit, result, true /* resets_to_zero */);
    shot.op_idx = qubit; // execute reads the lost qubit from op_idx
    shot.op_type = OPID_LOSS_NOISE;
}

// Prepare correlated noise for the adaptive path.
// Qubit IDs are read from call_arg_table (register indices), following the same
// pattern as OP_CALL argument passing.
fn prep_correlated_noise_adaptive(shot_idx: u32, op_idx: u32, qubit_count: u32, arg_offset: u32) {
    let noise_table_idx = ops[op_idx].q1;

    let sample = sample_correlated_noise(shot_idx, op_idx, noise_table_idx);
    if (sample.should_apply == 0u) { return; }

    // Build bit-flip, phase-flip, and loss masks using qubit IDs from registers via call_arg_table
    var bit_flip_mask: u32 = 0u;
    var phase_flip_mask: u32 = 0u;
    var loss_mask: u32 = 0u;
    for (var i: u32 = 0u; i < qubit_count; i++) {
        let pauli_bits = get_pauli_bits(sample.paulis_lo, sample.paulis_hi, qubit_count, i);
        let arg_reg = batch_data.program.call_arg_table[arg_offset + i];
        let qubit_mask = 1u << read_reg(shot_idx, arg_reg);
        if ((pauli_bits & 0x4u) != 0u) {
            // Loss term (L = 4): the qubit is lost, no Pauli is applied to it.
            loss_mask |= qubit_mask;
        } else {
            if ((pauli_bits & 0x1u) != 0u) { bit_flip_mask |= qubit_mask; }
            if ((pauli_bits & 0x2u) != 0u) { phase_flip_mask |= qubit_mask; }
        }
    }

    commit_correlated_noise(shot_idx, op_idx, bit_flip_mask, phase_flip_mask, loss_mask);
}

//#endregion

//#region Kernels

//#region Shared kernel helpers

// Shared kernel helpers used by both the base and adaptive code paths.

// Zero this shot's slice of the state vector and set the |0...0> amplitude to 1,
// then reset the shot's tracking state. Shared by the initialize kernel.
fn init_state_vector(params: ShotParams) {
    // We want every thread to zero out its portion of the state vector for the shot
    // We also want threads executing in lockstep to update adjacent entries for better memory access patterns
    for (var i = 0; i < params.op_iterations; i++) {
        let entry_index: i32 = params.thread_idx_in_shot + i * params.total_threads_per_shot;
        stateVector[params.shot_state_vector_start + entry_index] = vec2f(0.0, 0.0);
    }

    // NOTE: No need to synchronize here, as each thread is writing to unique locations
    if (params.thread_idx_in_shot == 0) {
        // Set the |0...0> amplitude to 1.0 from the first workgroup & thread for the shot
        stateVector[params.shot_state_vector_start] = vec2f(1.0, 0.0);
        reset_all(params.shot_idx);
    }
}

// Finalize the setup of a plain (no-noise, no-loss) gate op for execution:
// translate the op id into the execute-stage op_type (shot-buffer conversions,
// phase gates as RZ) and record which qubit probabilities to update next round.
// `q1`/`q2` are the resolved operands (ops-pool values for base, register/
// immediate resolved for adaptive). Shared by both prepare_op paths.
fn finalize_gate_op(shot_idx: u32, op_idx: u32, q1: u32, q2: u32) {
    let shot = &shots[shot_idx];
    let op = &ops[op_idx];

    shot.op_idx = op_idx;
    shot.op_type = op.id;

    // Turn any Rxx, Ryy, or Rzz gates into a gate from the shot buffer
    // NOTE: Should probably just do this for all gates
    if (op.id == OPID_RXX || op.id == OPID_RYY || op.id == OPID_MAT2Q || op.id == OPID_SWAP) {
        shot.op_type = OPID_SHOT_BUFF_2Q; // Indicate to use the matrix in the shot buffer
    }

    if (op.id >= OPID_X && op.id < OPID_CX) {
        shot.op_type = OPID_SHOT_BUFF_1Q; // Indicate to use the matrix in the shot buffer
    }

    if (is_1q_phase_gate(op.id)) {
        // For phase gates, treat everything as RZ for execution purposes
        shot.op_type = OPID_RZ;
    }

    // Set this so the next prepare_op stage knows which qubits to update probabilities for
    switch shot.op_type {
      case OPID_ID, OPID_CZ, OPID_RZ, OPID_RZZ {
        shot.qubits_updated_last_op_mask = 0u;
      }
      case OPID_SHOT_BUFF_1Q {
        shot.qubits_updated_last_op_mask = 1u << q1;
      }
      case OPID_CX, OPID_CY, OPID_SHOT_BUFF_2Q {
        shot.qubits_updated_last_op_mask = (1u << q1) | (1u << q2);
      }
      default {
        // TODO: Set error/diagnostic info here
      }
    }
}

//#endregion

//#region Base prepare_op implementation

// *******************************
// PREPARE OP
// This stage prepares the shot state for the next operation to execute (and any updates needed from the prior op)
//
// Each op is prepared by one thread. This is how we deal with some of the challenges with synchronization
// when multiple workgroups with multiple threads are used for a shot in the EXECUTE stage. The 'execute_op'
// does work that is 'embarrassingly parallel' across the state vector amplitudes, but the PREPARE_OP stage
// deal with preparing for that work, and collating results back into the shot state afterwards.
//
// This allows us to use the GPU 'dispatch' mechanism to ensure consistencty across shots without complex,
// synchronization code, as the GPU guarantees that all threads in a dispatch complete before the next dispatch
// starts, and all buffer writes are visible to the next dispatch.
// *******************************

// NOTE: Run with workgroup size of 1 for now, as threads may diverge too much in prepare_op stage causing performance issues.
// TODO: Try to increase later if lack of parallelism is a bottleneck. (Update the dispatch call accordingly).
fn prepare_op_base_impl(shot_idx: u32) {
    // For the 'prepare_op' stage, each thread dispatched handles one shot, so the globalId.x is the shot index
    let shot = &shots[shot_idx];

    // WebGPU guarantees that buffers are zero-initialized, so next_op_idx will correctly be 0 on the first dispatch
    let op_idx = shot.next_op_idx;

    // If we've gone past the end, set the op type to id and exit, so the execute stage is a no-op
    if (op_idx >= u32(arrayLength(&ops))) {
        // TODO: Set error/diagnostic info here
        shot.op_type = OPID_ID;
        shot.renormalize = 1.0;
        shot.qubits_updated_last_op_mask = 0u;
        return;
    }

    let op = &ops[op_idx];

    // Update the shot state based on the results of the last executed op (if needed)
    if (shot.qubits_updated_last_op_mask != 0) {
        update_qubit_state(shot_idx);
    }

    shot_init_per_op(shot_idx);
    shot.unitary = op.unitary;

    // Handle MResetZ, MZ, and ResetZ operations. These have unique handling and no associated noise ops, so prep and exit
    if (op.id == OPID_MRESETZ) {
        prep_measure_reset(shot_idx, op_idx, op.q1, op.q2, false /* is_loss */, true /* stores_result */, true /* resets_to_zero */);
        shot.next_op_idx = op_idx + 1u; // No associated noise ops, so just advance by 1
        return;
    }
    if (op.id == OPID_MZ) {
        prep_measure_reset(shot_idx, op_idx, op.q1, op.q2, false /* is_loss */, true /* stores_result */, false /* resets_to_zero */);
        shot.next_op_idx = op_idx + 1u;
        return;
    }
    if (op.id == OPID_RESETZ) {
        prep_measure_reset(shot_idx, op_idx, op.q1, op.q2, false /* is_loss */, false /* stores_result */, true /* resets_to_zero */);
        shot.next_op_idx = op_idx + 1u;
        return;
    }

    // Loss-commit op: lose this qubit if and only if the preceding noise sampler
    // set its bit in pending_loss_mask; otherwise act as identity.
    if (op.id == OPID_LOSS_NOISE) {
        shot.next_op_idx = op_idx + 1u;
        let loss_bit = 1u << op.q1;
        if ((shot.pending_loss_mask & loss_bit) != 0u) {
            shot.pending_loss_mask &= ~loss_bit;
            prep_measure_reset(shot_idx, op_idx, op.q1, op.q2, true /* is_loss */, false /* stores_result */, true /* resets_to_zero */);
        } else {
            shot.op_type = OPID_ID;
            shot.op_idx = op_idx;
            shot.qubits_updated_last_op_mask = 0u;
        }
        return;
    }

    /* Handle noise:
       - For the 1-qubit op case, there could be pauli and loss noise after the op itself. We want to check for loss first and
         only apply pauli noise if the qubit wasn't lost. (If lost, the pauli noise and even the gate itself don't matter).
       - For the 2-qubit op case, there will only be optional pauli noise after the op itself. (Loss is applied via separate
         Id ops on each qubit after the 2-qubit op).
    */

    let pauli_op_idx = get_pauli_noise_idx(op_idx);
    // Advance past this gate and its (optional) inline Pauli/loss noise op. Any
    // loss-commit ops that follow are separate ops handled on later iterations.
    shot.next_op_idx = max(op_idx, pauli_op_idx) + 1u;

    // Handle correlated noise operations
    if (op.id == OPID_CORRELATED_NOISE) {
        prep_correlated_noise(shot_idx, op_idx);
        return;
    }

    // Before doing further work, if any qubit for the gate is lost, dispatch
    // the gate's configured loss policy (stamped on op.policy).
    let has_lost_operand = gate_has_lost_operand(shot_idx, op_idx, op.q1, op.q2);
    if (has_lost_operand) {
        handle_lost_operand_policy(shot_idx, op_idx, op.q1, op.q2);
    }

    if pauli_op_idx != 0 {
        if ops[pauli_op_idx].id == OPID_PAULI_NOISE_1Q {
            // A 1-qubit gate has a single operand; if it is lost there is no
            // surviving qubit to receive Pauli noise, so skip the noise.
            if (!has_lost_operand) {
                apply_1q_pauli_noise(shot_idx, op_idx, pauli_op_idx, op.q1);
            }
            return;
        } else {
            if (has_lost_operand) {
                // The gate body was handled by the loss policy above. Still apply
                // the attached Pauli noise to the surviving operand (if any).
                apply_2q_pauli_noise_on_survivor(shot_idx, op_idx, pauli_op_idx, op.q1, op.q2);
            } else {
                apply_2q_pauli_noise(shot_idx, op_idx, pauli_op_idx, op.q1, op.q2);
            }
            return;
        }
    }

    // If the gate has any lost operands (and no attached noise), the gate logic
    // was completely handled inside `handle_lost_operand_policy`.
    if (has_lost_operand) {
        return;
    }

    // No noise to apply, just set up the shot to execute the op as-is
    finalize_gate_op(shot_idx, op_idx, op.q1, op.q2);
}

//#endregion

//#region initialize kernel

@compute @workgroup_size(THREADS_PER_WORKGROUP)
fn initialize(
        @builtin(workgroup_id) workgroupId: vec3<u32>,
        @builtin(local_invocation_index) tid: u32) {
    // Get the params
    let params = get_shot_params(workgroupId.x, tid, 0 /* qubits per op */);

    // Zero the state vector and set the |0...0> amplitude to 1.0 for this shot.
    init_state_vector(params);

    // The adaptive interpreter needs additional per-shot state initialized.
    if (IS_ADAPTIVE && params.thread_idx_in_shot == 0) {
        // Zero the results buffer for this shot so stale exit codes from
        // prior runs do not leak via atomicCompareExchangeWeak in OP_RET.
        let results_base = u32(params.shot_idx) * RESULT_COUNT;
        for (var r = 0u; r < RESULT_COUNT; r++) {
            atomicStore(&results[results_base + r], 0u);
        }

        // Initialize memory from constant_data
        for (var m = 0u; m < CONSTANT_DATA_SIZE; m++) {
            shots[params.shot_idx].interp.memory[m] = batch_data.program.constant_data[m];
        }
        // Zero the alloca region for CPU-GPU parity
        for (var m = CONSTANT_DATA_SIZE; m < MAX_MEMORY; m++) {
            shots[params.shot_idx].interp.memory[m] = 0u;
        }
    }
}

//#endregion

//#region interpret_classical kernel

// -----------------------------------------------------------------------------
// Adaptive interpreter — interpret_classical entry point
// -----------------------------------------------------------------------------
//
// This is the main classical bytecode interpreter for the GPU-based adaptive
// quantum simulator. It implements a register-based virtual machine that
// executes classical (non-quantum) instructions on the GPU, one thread per
// shot. Each shot has its own independent interpreter state (program counter,
// registers, call stack) allowing many shots to run in parallel with
// potentially divergent control flow paths (e.g., after mid-circuit
// measurements).
//
// ## Execution Model
//
// The interpreter runs cooperatively with the quantum simulation pipeline:
//
//   1. The host dispatches `interpret_classical` for all shots.
//   2. Each shot executes classical instructions in a loop until one of:
//      (a) A quantum operation is encountered → status = QUANTUM_PENDING,
//          which tells the host to run the quantum simulation kernels
//          (prepare_op → execute) before re-entering this function.
//      (b) A `ret` instruction terminates the shot → status = TERMINATED.
//      (c) The step limit (MAX_CLASSICAL_STEPS) is hit → status = YIELD,
//          which prevents any single dispatch from running forever; the host
//          simply re-dispatches to continue.
//      (d) An unknown opcode is hit → status = ERROR.
//
// ## Instruction Encoding
//
// Each instruction occupies 2 × vec4<u32> (8 u32 words) in the `bytecode`
// buffer, fetched by `fetch_instr(pc)` into the `Instr` struct with fields:
//
//   opcode : packed opcode word (bits [7:0] = primary op, [15:8] = sub-
//            condition for comparisons, [23:16] = flags for immediates)
//   dst    : destination register index (or immediate for RET)
//   src0   : first source operand (register index or immediate)
//   src1   : second source operand (register index or immediate)
//   aux0–3 : auxiliary fields whose meaning varies per opcode (e.g., block
//            IDs, function IDs, qubit indices, phi-table offsets, etc.)
//
// The `resolve_u32` / `resolve_i32` helpers read an operand as either a
// register value or an inline immediate based on the FLAG_SRC0_IMM /
// FLAG_SRC1_IMM bits in the flags byte. This lets the compiler embed small
// constants directly in the instruction stream without extra CONST ops.


@compute @workgroup_size(1)
fn interpret_classical(@builtin(global_invocation_id) gid: vec3<u32>) {
    // Each GPU thread handles exactly one shot. The global invocation ID
    // maps directly to the shot index.
    let shot_idx = gid.x;
    let state = shots[shot_idx].interp;

    // -- Early-exit for shots that already finished or errored --
    let status = state.status;
    if status == STATUS_TERMINATED || status == STATUS_ERROR {
        return;
    }

    // -- Drain pending qubit losses before resuming classical execution --
    // The most recent noise op (per-gate Pauli/loss or correlated) may have
    // sampled one or more qubits as lost, recorded in pending_loss_mask. Commit
    // each as its own measure+reset quantum op (one per round) before running
    // any more bytecode, so loss is applied with the correct correlation.
    if shots[shot_idx].pending_loss_mask != 0u {
        let q = firstTrailingBit(shots[shot_idx].pending_loss_mask);
        shots[shot_idx].pending_loss_mask &= ~(1u << q);
        shots[shot_idx].interp.pending_op_idx = q;
        shots[shot_idx].interp.pending_op_type = PENDING_OP_LOSS_COMMIT;
        shots[shot_idx].interp.status = STATUS_QUANTUM_PENDING;
        return;
    }

    // If we were paused (QUANTUM_PENDING after a quantum op, or YIELD after
    // hitting the step limit), transition back to RUNNING so the main loop
    // resumes executing instructions from where it left off.
    if status != STATUS_RUNNING {
        shots[shot_idx].interp.status = STATUS_RUNNING;
    }

    // -- Load interpreter registers from GPU memory into local variables --
    // Using local vars for the hot-path state avoids repeated global memory
    // loads/stores on every instruction. They are written back at the end.
    var pc: u32 = state.pc;  // program counter
    var block_id: u32 = state.current_block_id;
    var prev_block: u32 = state.previous_block_id; // for PHI
    var steps: u32 = 0u;             // counts instructions executed this dispatch
    var should_break: bool = false;  // set to true to exit the main loop

    // -- Main interpreter loop --
    // Fetches and executes one instruction per iteration. Exits when the
    // shot terminates, yields for quantum work, hits the step limit, or
    // encounters an error.
    loop {
        // Guard against infinite loops in classical code: after executing
        // MAX_CLASSICAL_STEPS instructions, yield back to the host which
        // will re-dispatch this kernel to continue.
        if steps >= MAX_CLASSICAL_STEPS {
            // Only yield if the shot hasn't already errored (an error
            // status must not be overwritten by a yield).
            if state.status != STATUS_ERROR {
                shots[shot_idx].interp.status = STATUS_YIELD;
            }
            break;
        }

        // Fetch the instruction at the current PC. Each instruction is
        // 2 × vec4<u32> (8 words) in the bytecode buffer.
        let instr = fetch_instr(pc);

        // Unpack the opcode word into its three components:
        //   op      — primary opcode (bits 7:0), determines which case below runs
        //   subcond — sub-condition code (bits 15:8), used only by ICMP/FCMP to
        //             select the specific comparison predicate (eq, ne, slt, etc.)
        //   flags   — immediate-mode flags (bits 23:16), tells resolve_* whether
        //             src0/src1 are register indices or inline immediates
        let op = get_opcode(instr.opcode);
        let subcond = get_subcond(instr.opcode);
        let flags = get_flags(instr.opcode);

        // -- Opcode dispatch --
        // The switch below implements every bytecode instruction. Instructions
        // are grouped by category. Most follow a common pattern:
        //   1. Read operands via resolve_u32/i32 (register or immediate)
        //   2. Compute the result
        //   3. Write back to the destination register via write_reg*
        //   4. Advance pc++
        //
        // Control-flow ops (JUMP, BRANCH, SWITCH, CALL) modify pc and
        // block_id directly instead of incrementing pc.
        //
        // Quantum ops (QUANTUM_GATE, MEASURE, RESET) write pending-op
        // metadata to the interpreter state and set should_break=true to
        // pause execution and hand control back to the host for quantum
        // kernel dispatch.
        switch op {

            // -------------------------------------------------------------
            // CONTROL FLOW
            // -------------------------------------------------------------

            // NOP: No operation. Simply advances the program counter.
            case OP_NOP {
                pc++;
            }

            // RET: Terminates this shot's execution.
            // The exit code (from dst, which may be an immediate) is stored
            // both in the per-shot interpreter state and atomically into the
            // results buffer. The atomic-compare-exchange ensures only the
            // first non-zero exit code is recorded for this shot (useful for
            // error reporting). The termination count in the diagnostics
            // buffer is incremented so the host can detect when all shots
            // have finished.
            case OP_RET {
                let exit_code = resolve_u32(shot_idx, instr.dst, flags, 2u);
                shots[shot_idx].interp.exit_code = exit_code;
                // Atomically store exit code into the last slot of this shot's
                // result region, but only if it has not already been set.
                let err_index = (shot_idx + 1) * RESULT_COUNT - 1;
                atomicCompareExchangeWeak(&results[err_index], 0u, exit_code);
                shots[shot_idx].interp.status = STATUS_TERMINATED;
                atomicAdd(&diagnostics.termination_count, 1u);
                should_break = true;
            }

            // JUMP: Unconditional branch to a target block.
            // Encoding: dst = target block ID.
            // Updates prev_block (needed by subsequent PHI instructions in
            // the target block) and sets pc to the first instruction of the
            // target block via the block_table lookup.
            case OP_JUMP {
                prev_block = block_id;
                block_id = instr.dst;
                pc = batch_data.program.block_table[instr.dst].instr_offset;
            }

            // BRANCH: Conditional branch (if/else).
            // Encoding: src0 = condition (register or immediate),
            //           aux0 = true-branch block ID,
            //           aux1 = false-branch block ID.
            // Evaluates the condition: if non-zero, jumps to aux0; otherwise
            // jumps to aux1. Like JUMP, updates prev_block for PHI nodes.
            case OP_BRANCH {
                let cond = resolve_u32(shot_idx, instr.src0, flags, 0u) != 0u;
                prev_block = block_id;
                if cond {
                    block_id = instr.aux0;
                    pc = batch_data.program.block_table[instr.aux0].instr_offset;
                } else {
                    block_id = instr.aux1;
                    pc = batch_data.program.block_table[instr.aux1].instr_offset;
                }
            }

            // SWITCH: Multi-way branch (like a C switch statement).
            // Encoding: src0 = value to match,
            //           aux0 = default block ID,
            //           aux1 = offset into switch_table,
            //           aux2 = number of case entries.
            // Each switch_table entry is a vec2<u32>(match_value, target_block).
            // Linearly scans the case table; if a match is found, jumps to
            // that block. If no match, falls through to the default block.
            case OP_SWITCH {
                let val = resolve_u32(shot_idx, instr.src0, flags, 0u);
                let default_block = instr.aux0;
                let case_offset = instr.aux1;
                let case_count = instr.aux2;
                var target_block = default_block;
                for (var i = 0u; i < case_count; i++) {
                    let entry = batch_data.program.switch_table[case_offset + i];
                    if entry.case_val == val {
                        target_block = entry.target_block;
                        break;
                    }
                }
                prev_block = block_id;
                block_id = target_block;
                pc = batch_data.program.block_table[target_block].instr_offset;
            }

            // CALL: Invokes a function.
            // Encoding: dst = register to receive the return value,
            //           aux0 = function ID (index into function_table),
            //           aux1 = argument count,
            //           aux2 = offset into call_arg_table.
            //
            // The function_table entry is vec4(entry_block, param_count,
            // param_base_reg, reserved).
            //
            // Steps:
            //   1. Push a return frame onto the per-shot call stack. Each
            //      frame stores: (return_block, return_pc, return_reg,
            //      reserved) — 4 u32 words. The stack supports up to 8 frames.
            //   2. Copy each argument from caller registers (looked up via
            //      call_arg_table) into callee parameter registers starting
            //      at param_base_reg.
            //   3. Jump to the function's entry block.
            case OP_CALL {
                let func_id = instr.aux0;
                let arg_count = instr.aux1;
                let arg_offset = instr.aux2;
                let func = batch_data.program.function_table[func_id];
                // Push return info onto the call stack
                let sp = shots[shot_idx].interp.call_sp;
                // Guard: prevent call stack overflow (max 8 frames)
                if sp >= 8u {
                    shots[shot_idx].interp.exit_code = ERR_CALL_STACK_OVERFLOW;
                    let err_idx = (shot_idx + 1) * RESULT_COUNT - 1;
                    atomicCompareExchangeWeak(&results[err_idx], 0u, ERR_CALL_STACK_OVERFLOW);
                    shots[shot_idx].interp.status = STATUS_ERROR;
                    atomicAdd(&diagnostics.termination_count, 1u);
                    should_break = true;
                    break;
                }
                shots[shot_idx].interp.call_stack_frames[sp].block_id = block_id;    // return_block — resume here on return
                shots[shot_idx].interp.call_stack_frames[sp].return_pc = pc + 1u;    // return_pc — instruction after the CALL
                shots[shot_idx].interp.call_stack_frames[sp].return_reg = instr.dst; // return_reg — where to write result
                shots[shot_idx].interp.call_sp = sp + 1u;
                // Copy caller arguments into the callee's parameter registers
                let param_base = func.param_base_reg;
                for (var i = 0u; i < arg_count; i++) {
                    let arg_reg = batch_data.program.call_arg_table[arg_offset + i];
                    write_reg(shot_idx, param_base + i, read_reg(shot_idx, arg_reg));
                }
                // Transfer control to the function entry block
                block_id = func.entry_block_id;
                pc = batch_data.program.block_table[block_id].instr_offset;
            }

            // CALL_RETURN: Returns from a function call.
            // Encoding: src0 = register holding the return value.
            //
            // Pops the top frame from the call stack to restore block_id and
            // pc to the instruction after the CALL. If the caller specified a
            // return register (not 0xFFFFFFFF), copies the return value into
            // that register.
            case OP_CALL_RETURN {
                if shots[shot_idx].interp.call_sp == 0u {
                    shots[shot_idx].interp.exit_code = ERR_CALL_STACK_UNDERFLOW;
                    let err_idx = (shot_idx + 1) * RESULT_COUNT - 1;
                    atomicCompareExchangeWeak(&results[err_idx], 0u, ERR_CALL_STACK_UNDERFLOW);
                    shots[shot_idx].interp.status = STATUS_ERROR;
                    atomicAdd(&diagnostics.termination_count, 1u);
                    should_break = true;
                    break;
                }

                let sp = shots[shot_idx].interp.call_sp - 1;
                shots[shot_idx].interp.call_sp = sp;
                block_id = shots[shot_idx].interp.call_stack_frames[sp].block_id;
                pc = shots[shot_idx].interp.call_stack_frames[sp].return_pc;
                let return_reg = shots[shot_idx].interp.call_stack_frames[sp].return_reg;
                if return_reg != VOID_RETURN {
                    write_reg(shot_idx, return_reg, read_reg(shot_idx, instr.src0));
                }
            }

            // -------------------------------------------------------------
            // QUANTUM OPERATIONS — pause the interpreter, yield to the host
            // -------------------------------------------------------------
            // When the interpreter hits a quantum instruction, it cannot
            // execute it directly (quantum simulation runs in separate GPU
            // kernels with parallel state-vector processing). Instead, it
            // writes the pending operation details into the interpreter
            // state for the host to read, sets status = QUANTUM_PENDING,
            // advances pc past the instruction, and breaks out of the loop.
            //
            // The host then dispatches prepare_op (which reads the
            // pending op metadata and configures the shot for the quantum
            // kernel) followed by the execute kernel (which applies the
            // gate/measurement/reset to the state vector). After that, the
            // host re-dispatches interpret_classical to continue.
            //
            // Qubit IDs may be static (embedded in aux1/aux2 by the
            // compiler) or dynamic (computed at runtime and stored in
            // registers).

            // QUANTUM_GATE: Request a 1- or 2-qubit gate.
            // Encoding: aux0 = quantum op table index,
            //           aux1 = qubit 1 (or register if not sentinel),
            //           aux2 = qubit 2 (or register if not sentinel).
            case OP_QUANTUM_GATE {
                shots[shot_idx].interp.pending_op_idx = instr.aux0;
                shots[shot_idx].interp.pending_op_type = 0u; // type 0 = gate
                // Qubit IDs are resolved in prepare_op via resolve_q1/resolve_q2,
                // which use the FLAG_AUX1_IMM / FLAG_AUX2_IMM bits to decide
                // between immediate values and register lookups.
                shots[shot_idx].interp.status = STATUS_QUANTUM_PENDING;
                pc++;
                should_break = true;
            }

            // MEASURE: Request a qubit measurement.
            // Encoding: aux0 = quantum op table index,
            //           aux1 = qubit to measure (or register).
            // Only q1 is used; q2 is set to sentinel (unused).
            case OP_MEASURE {
                shots[shot_idx].interp.pending_op_idx = instr.aux0;
                shots[shot_idx].interp.pending_op_type = 1u; // type 1 = gate
                // Qubit and result IDs are resolved in prepare_op via
                // resolve_q1 (aux1) and resolve_q2 (aux2).
                shots[shot_idx].interp.status = STATUS_QUANTUM_PENDING;
                pc++;
                should_break = true;
            }

            // RESET: Request a qubit reset (measure + conditional X).
            // Encoding: aux0 = quantum op table index,
            //           aux1 = qubit to reset (or register).
            case OP_RESET {
                shots[shot_idx].interp.pending_op_idx = instr.aux0;
                shots[shot_idx].interp.pending_op_type = 2u; // type 2 = reset
                // Qubit ID is resolved in prepare_op via resolve_q1 (aux1).
                shots[shot_idx].interp.status = STATUS_QUANTUM_PENDING;
                pc++;
                should_break = true;
            }

            // -------------------------------------------------------------
            // QUANTUM RESULT ACCESS
            // -------------------------------------------------------------

            // READ_RESULT: Load a prior measurement outcome into a register.
            // Encoding: src0 = result ID (index into the results buffer),
            //           dst  = destination register.
            // The measurement result (0 or 1) was written by an earlier
            // MEASURE quantum op. This reads it atomically from the shared
            // results buffer and stores 0u or 1u into the destination
            // register, allowing classical code to branch on measurement
            // outcomes.
            case OP_READ_RESULT {
                let result_id = instr.src0;
                let result_val = read_measurement_result(shot_idx, result_id);
                write_reg(shot_idx, instr.dst, select(0u, 1u, result_val));
                pc++;
            }

            // RECORD_OUTPUT: Marker for output recording.
            // On the GPU this is a no-op — the host reads the results buffer
            // directly after all shots terminate. The instruction exists to
            // maintain compatibility with the QIR adaptive profile bytecode.
            case OP_RECORD_OUTPUT {
                pc++;
            }

            // READ_LOSS: Reports whether the measurement that produced a
            // result observed a lost qubit. The per-shot ``results`` buffer
            // encodes loss as the value 2u (0u = Zero, 1u = One, 2u = Loss),
            // so we compare against 2u and write 1u when the result was a loss,
            // else 0u.
            case OP_READ_LOSS {
                let result_id = instr.src0;
                let val = atomicLoad(&results[shot_idx * RESULT_COUNT + result_id]);
                write_reg(shot_idx, instr.dst, select(0u, 1u, val == 2u));
                pc++;
            }

            // -------------------------------------------------------------
            // INTEGER ARITHMETIC
            // -------------------------------------------------------------
            // All integer arithmetic ops follow the pattern:
            //   dst = src0 <op> src1
            // Operands are resolved via resolve_i32/u32, which checks the
            // FLAG_SRC0_IMM / FLAG_SRC1_IMM bits to determine if the field
            // is a register index or an inline immediate constant.

            // ADD: Signed integer addition. dst = src0 + src1.
            case OP_ADD {
                let a = resolve_i32(shot_idx, instr.src0, flags, 0u);
                let b = resolve_i32(shot_idx, instr.src1, flags, 1u);
                write_reg_i32(shot_idx, instr.dst, a + b);
                pc++;
            }

            // SUB: Signed integer subtraction. dst = src0 - src1.
            case OP_SUB {
                let a = resolve_i32(shot_idx, instr.src0, flags, 0u);
                let b = resolve_i32(shot_idx, instr.src1, flags, 1u);
                write_reg_i32(shot_idx, instr.dst, a - b);
                pc++;
            }

            // MUL: Signed integer multiplication. dst = src0 * src1.
            case OP_MUL {
                let a = resolve_i32(shot_idx, instr.src0, flags, 0u);
                let b = resolve_i32(shot_idx, instr.src1, flags, 1u);
                write_reg_i32(shot_idx, instr.dst, a * b);
                pc++;
            }

            // UDIV: Unsigned integer division. dst = src0 / src1.
            case OP_UDIV {
                let a = resolve_u32(shot_idx, instr.src0, flags, 0u);
                let b = resolve_u32(shot_idx, instr.src1, flags, 1u);
                write_reg(shot_idx, instr.dst, a / b);
                pc++;
            }

            // SDIV: Signed integer division (truncates toward zero). dst = src0 / src1.
            case OP_SDIV {
                let a = resolve_i32(shot_idx, instr.src0, flags, 0u);
                let b = resolve_i32(shot_idx, instr.src1, flags, 1u);
                write_reg_i32(shot_idx, instr.dst, a / b);
                pc++;
            }

            // UREM: Unsigned integer remainder. dst = src0 % src1.
            case OP_UREM {
                let a = resolve_u32(shot_idx, instr.src0, flags, 0u);
                let b = resolve_u32(shot_idx, instr.src1, flags, 1u);
                write_reg(shot_idx, instr.dst, a % b);
                pc++;
            }

            // SREM: Signed integer remainder.
            // Computes a - b * trunc(a/b) manually rather than using the %
            // operator, because WGSL i32 division truncates toward zero but
            // the built-in % may not preserve the sign of the dividend on
            // all GPU backends. This matches LLVM's srem semantics.
            case OP_SREM {
                let a = resolve_i32(shot_idx, instr.src0, flags, 0u);
                let b = resolve_i32(shot_idx, instr.src1, flags, 1u);
                write_reg_i32(shot_idx, instr.dst, a - b * (a / b));
                pc++;
            }

            // -------------------------------------------------------------
            // BITWISE / SHIFT OPERATIONS
            // -------------------------------------------------------------
            // Operate on the raw u32 bit pattern of the register values.

            // AND: Bitwise AND. dst = src0 & src1.
            case OP_AND {
                write_reg(shot_idx, instr.dst,
                    resolve_u32(shot_idx, instr.src0, flags, 0u) & resolve_u32(shot_idx, instr.src1, flags, 1u));
                pc++;
            }

            // OR: Bitwise OR. dst = src0 | src1.
            case OP_OR {
                write_reg(shot_idx, instr.dst,
                    resolve_u32(shot_idx, instr.src0, flags, 0u) | resolve_u32(shot_idx, instr.src1, flags, 1u));
                pc++;
            }

            // XOR: Bitwise exclusive OR. dst = src0 ^ src1.
            case OP_XOR {
                write_reg(shot_idx, instr.dst,
                    resolve_u32(shot_idx, instr.src0, flags, 0u) ^ resolve_u32(shot_idx, instr.src1, flags, 1u));
                pc++;
            }

            // SHL: Logical shift left. dst = src0 << src1.
            case OP_SHL {
                write_reg(shot_idx, instr.dst,
                    resolve_u32(shot_idx, instr.src0, flags, 0u) << resolve_u32(shot_idx, instr.src1, flags, 1u));
                pc++;
            }

            // LSHR: Logical shift right (zero-fill). dst = src0 >> src1.
            case OP_LSHR {
                write_reg(shot_idx, instr.dst,
                    resolve_u32(shot_idx, instr.src0, flags, 0u) >> resolve_u32(shot_idx, instr.src1, flags, 1u));
                pc++;
            }

            // ASHR: Arithmetic shift right (sign-extending). dst = src0 >> src1.
            // Uses i32 to preserve the sign bit during the shift.
            case OP_ASHR {
                let a = resolve_i32(shot_idx, instr.src0, flags, 0u);
                let b = resolve_u32(shot_idx, instr.src1, flags, 1u);
                write_reg_i32(shot_idx, instr.dst, a >> b);
                pc++;
            }

            // -------------------------------------------------------------
            // INTEGER COMPARISON (ICMP)
            // -------------------------------------------------------------
            // Compares two integer operands using the sub-condition code
            // encoded in bits [15:8] of the opcode word. The result is
            // written as 0u (false) or 1u (true) to the destination register.
            // Signed comparisons (SLT, SLE, SGT, SGE) use i32 directly;
            // unsigned comparisons (ULT, ULE, UGT, UGE) bitcast to u32.
            // These mirror LLVM icmp predicates.
            case OP_ICMP {
                let a = resolve_i32(shot_idx, instr.src0, flags, 0u);
                let b = resolve_i32(shot_idx, instr.src1, flags, 1u);
                var result: bool = false;
                switch subcond {
                    case ICMP_EQ  { result = (a == b); }
                    case ICMP_NE  { result = (a != b); }
                    case ICMP_SLT { result = (a < b); }
                    case ICMP_SLE { result = (a <= b); }
                    case ICMP_SGT { result = (a > b); }
                    case ICMP_SGE { result = (a >= b); }
                    case ICMP_ULT { result = (bitcast<u32>(a) < bitcast<u32>(b)); }
                    case ICMP_ULE { result = (bitcast<u32>(a) <= bitcast<u32>(b)); }
                    case ICMP_UGT { result = (bitcast<u32>(a) > bitcast<u32>(b)); }
                    case ICMP_UGE { result = (bitcast<u32>(a) >= bitcast<u32>(b)); }
                    default {
                        shots[shot_idx].interp.status = ERR_INVALID_INSTRUCTION;
                        shots[shot_idx].interp.exit_code = ERR_INVALID_INSTRUCTION;
                        let err_idx = (shot_idx + 1) * RESULT_COUNT - 1;
                        atomicCompareExchangeWeak(&results[err_idx], 0u, ERR_INVALID_INSTRUCTION);
                        shots[shot_idx].interp.status = STATUS_ERROR;
                        atomicAdd(&diagnostics.termination_count, 1u);
                        should_break = true;
                    }
                }
                write_reg(shot_idx, instr.dst, select(0u, 1u, result));
                pc++;
            }

            // -------------------------------------------------------------
            // FLOAT COMPARISON (FCMP)
            // -------------------------------------------------------------
            // Compares two f32 operands using the sub-condition code.
            // "O" prefix = ordered (both operands are not NaN). The result
            // is written as 0u/1u. Mirrors LLVM fcmp ordered predicates.
            case OP_FCMP {
                let a = resolve_f32(shot_idx, instr.src0, flags, 0u);
                let b = resolve_f32(shot_idx, instr.src1, flags, 1u);
                var result: bool = false;
                switch subcond {
                    case FCMP_OEQ { result = (a == b); }
                    case FCMP_ONE { result = (a != b); }
                    case FCMP_OLT { result = (a < b); }
                    case FCMP_OLE { result = (a <= b); }
                    case FCMP_OGT { result = (a > b); }
                    case FCMP_OGE { result = (a >= b); }
                    default {
                        shots[shot_idx].interp.exit_code = ERR_INVALID_INSTRUCTION;
                        let err_idx = (shot_idx + 1) * RESULT_COUNT - 1;
                        atomicCompareExchangeWeak(&results[err_idx], 0u, ERR_INVALID_INSTRUCTION);
                        shots[shot_idx].interp.status = STATUS_ERROR;
                        atomicAdd(&diagnostics.termination_count, 1u);
                        should_break = true;
                    }
                }
                write_reg(shot_idx, instr.dst, select(0u, 1u, result));
                pc++;
            }

            // -------------------------------------------------------------
            // FLOAT ARITHMETIC
            // -------------------------------------------------------------
            // These operate on f32 values stored in registers via bitcast.
            // Operands are always register-based (no immediate flags for
            // float ops).

            // FADD: Float addition. dst = src0 + src1.
            case OP_FADD {
                write_reg_f32(shot_idx, instr.dst,
                    resolve_f32(shot_idx, instr.src0, flags, 0u) + resolve_f32(shot_idx, instr.src1, flags, 1u));
                pc++;
            }

            // FSUB: Float subtraction. dst = src0 - src1.
            case OP_FSUB {
                write_reg_f32(shot_idx, instr.dst,
                    resolve_f32(shot_idx, instr.src0, flags, 0u) - resolve_f32(shot_idx, instr.src1, flags, 1u));
                pc++;
            }

            // FMUL: Float multiplication. dst = src0 * src1.
            case OP_FMUL {
                write_reg_f32(shot_idx, instr.dst,
                    resolve_f32(shot_idx, instr.src0, flags, 0u) * resolve_f32(shot_idx, instr.src1, flags, 1u));
                pc++;
            }

            // FDIV: Float division. dst = src0 / src1.
            case OP_FDIV {
                write_reg_f32(shot_idx, instr.dst,
                    resolve_f32(shot_idx, instr.src0, flags, 0u) / resolve_f32(shot_idx, instr.src1, flags, 1u));
                pc++;
            }

            // FREM: Float remainder. LLVM docs say this instruction has
            // the same semantics as C's fmod, which is implemented as:
            // dst = src0 - trunc(src0/src1) * src1
            case OP_FREM {
                let a = resolve_f32(shot_idx, instr.src0, flags, 0u);
                let b = resolve_f32(shot_idx, instr.src1, flags, 1u);
                write_reg_f32(shot_idx, instr.dst, a - trunc(a / b) * b);
                pc++;
            }

            // -------------------------------------------------------------
            // TYPE CONVERSIONS
            // -------------------------------------------------------------
            // Maps LLVM-style type conversion instructions. Many are
            // identity ops on the GPU since all integer registers are 32-bit
            // and all floats are f32. They exist to keep the bytecode in
            // 1:1 correspondence with the compiled QIR instructions.

            // ZEXT: Zero-extend — identity on 32-bit GPU (values already u32).
            case OP_ZEXT {
                write_reg(shot_idx, instr.dst, resolve_u32(shot_idx, instr.src0, flags, 0u));
                pc++;
            }

            // SEXT: Sign-extend from a narrower bit width to i32.
            // aux0 encodes the source bit width (e.g., 1 for i1→i32).
            // The shift-left then arithmetic-shift-right trick propagates
            // the sign bit from position (src_bits-1) into all higher bits.
            case OP_SEXT {
                let val = resolve_i32(shot_idx, instr.src0, flags, 0u);
                let src_bits = instr.aux0;  // source type bit width
                if src_bits > 0u && src_bits < 32u {
                    let shift = 32u - src_bits;
                    write_reg_i32(shot_idx, instr.dst, (val << shift) >> shift);
                } else {
                    write_reg_i32(shot_idx, instr.dst, val);
                }
                pc++;
            }

            // TRUNC: Truncate — identity on 32-bit GPU (already the target width).
            case OP_TRUNC {
                write_reg(shot_idx, instr.dst, resolve_u32(shot_idx, instr.src0, flags, 0u));
                pc++;
            }

            // FPEXT: Float widen (e.g., f32→f64) — identity since GPU only uses f32.
            case OP_FPEXT {
                write_reg_f32(shot_idx, instr.dst, resolve_f32(shot_idx, instr.src0, flags, 0u));
                pc++;
            }

            // FPTRUNC: Float narrow (e.g., f64→f32) — identity since GPU only uses f32.
            case OP_FPTRUNC {
                write_reg_f32(shot_idx, instr.dst, resolve_f32(shot_idx, instr.src0, flags, 0u));
                pc++;
            }

            // INTTOPTR: Integer to pointer cast — identity, pointers are u32 on GPU.
            case OP_INTTOPTR {
                write_reg(shot_idx, instr.dst, resolve_u32(shot_idx, instr.src0, flags, 0u));
                pc++;
            }

            // FPTOSI: Float to signed integer conversion. dst = i32(src0).
            case OP_FPTOSI {
                write_reg_i32(shot_idx, instr.dst, i32(resolve_f32(shot_idx, instr.src0, flags, 0u)));
                pc++;
            }

            // SITOFP: Signed integer to float conversion. dst = f32(src0).
            case OP_SITOFP {
                write_reg_f32(shot_idx, instr.dst, f32(resolve_i32(shot_idx, instr.src0, flags, 0u)));
                pc++;
            }

            // FPTOUI: Float to unsigned integer conversion. dst = u32(src0).
            case OP_FPTOUI {
                write_reg(shot_idx, instr.dst, u32(resolve_f32(shot_idx, instr.src0, flags, 0u)));
                pc++;
            }

            // UITOFP: Unsigned integer to float conversion. dst = f32(src0).
            case OP_UITOFP {
                write_reg_f32(shot_idx, instr.dst, f32(resolve_u32(shot_idx, instr.src0, flags, 0u)));
                pc++;
            }

            // -------------------------------------------------------------
            // PHI NODE (SSA resolution at runtime)
            // -------------------------------------------------------------
            // In SSA form, PHI nodes select a value based on which
            // predecessor block the control flow came from. The compiler
            // emits a phi_table with (predecessor_block_id, value_register)
            // pairs for each PHI instruction.
            //
            // Encoding: dst  = destination register,
            //           aux0 = offset into phi_table,
            //           aux1 = number of predecessor entries.
            //
            // At runtime, we scan the entries to find the one whose block
            // ID matches prev_block, then copy that register's value into
            // the destination. This is how the interpreter handles SSA
            // control-flow merges without explicit move instructions on
            // every edge.
            case OP_PHI {
                let offset = instr.aux0;
                let count = instr.aux1;
                for (var i = 0u; i < count; i++) {
                    let entry = batch_data.program.phi_table[offset + i];
                    if entry.block_id == prev_block {
                        write_reg(shot_idx, instr.dst, read_reg(shot_idx, entry.val_reg));
                        break;
                    }
                }
                pc++;
            }

            // -------------------------------------------------------------
            // DATA MOVEMENT
            // -------------------------------------------------------------

            // SELECT: Conditional move (ternary operator).
            // Encoding: src0 = condition, aux0 = true-value,
            //           aux1 = false-value, dst = destination.
            // dst = cond ? aux0 : aux1
            case OP_SELECT {
                let cond = resolve_u32(shot_idx, instr.src0, flags, 0u) != 0u;
                let true_val = resolve_u32(shot_idx, instr.aux0, flags, 3u);
                let false_val = resolve_u32(shot_idx, instr.aux1, flags, 4u);
                write_reg(shot_idx, instr.dst, select(false_val, true_val, cond));
                pc++;
            }

            // MOV: Register-to-register move (or immediate-to-register if flagged).
            // dst = src0 (resolved through flags for possible immediate).
            case OP_MOV {
                write_reg(shot_idx, instr.dst, resolve_u32(shot_idx, instr.src0, flags, 0u));
                pc++;
            }

            // CONST: Load an immediate constant into a register.
            // dst = src0 (always treated as a literal value, not a register).
            case OP_CONST {
                write_reg(shot_idx, instr.dst, instr.src0);
                pc++;
            }

            // -------------------------------------------------------------
            // MEMORY OPERATIONS
            // -------------------------------------------------------------

            // ALLOCA: Reserve memory and write the address to dst.
            // Encoding: src0 = number of words, src1 = compile-time assigned address.
            case OP_ALLOCA {
                let num_words = resolve_u32(shot_idx, instr.src0, flags, 0u);
                let addr = resolve_u32(shot_idx, instr.src1, flags, 1u);
                if addr + num_words > MAX_MEMORY {
                    shots[shot_idx].interp.exit_code = ERR_ALLOCA_OUT_OF_BOUNDS;
                    let err_idx = (shot_idx + 1) * RESULT_COUNT - 1;
                    atomicCompareExchangeWeak(&results[err_idx], 0u, ERR_ALLOCA_OUT_OF_BOUNDS);
                    shots[shot_idx].interp.status = STATUS_ERROR;
                    atomicAdd(&diagnostics.termination_count, 1u);
                    should_break = true;
                    break;
                }
                write_reg(shot_idx, instr.dst, addr);
                pc++;
            }

            // LOAD: Read a value from memory at the given address.
            // Encoding: src0 = memory address, dst = destination register.
            case OP_LOAD {
                let addr = resolve_u32(shot_idx, instr.src0, flags, 0u);
                if addr >= MAX_MEMORY {
                    shots[shot_idx].interp.exit_code = ERR_MEMORY_OUT_OF_BOUNDS;
                    let err_idx = (shot_idx + 1) * RESULT_COUNT - 1;
                    atomicCompareExchangeWeak(&results[err_idx], 0u, ERR_MEMORY_OUT_OF_BOUNDS);
                    shots[shot_idx].interp.status = STATUS_ERROR;
                    atomicAdd(&diagnostics.termination_count, 1u);
                    should_break = true;
                    break;
                }
                let val = shots[shot_idx].interp.memory[addr];
                write_reg(shot_idx, instr.dst, val);
                pc++;
            }

            // STORE: Write a value to memory at the given address.
            // Encoding: src0 = value to store, src1 = memory address.
            case OP_STORE {
                let val = resolve_u32(shot_idx, instr.src0, flags, 0u);
                let addr = resolve_u32(shot_idx, instr.src1, flags, 1u);
                if addr >= MAX_MEMORY {
                    shots[shot_idx].interp.exit_code = ERR_MEMORY_OUT_OF_BOUNDS;
                    let err_idx = (shot_idx + 1) * RESULT_COUNT - 1;
                    atomicCompareExchangeWeak(&results[err_idx], 0u, ERR_MEMORY_OUT_OF_BOUNDS);
                    shots[shot_idx].interp.status = STATUS_ERROR;
                    atomicAdd(&diagnostics.termination_count, 1u);
                    should_break = true;
                    break;
                }
                shots[shot_idx].interp.memory[addr] = val;
                pc++;
            }

            // GEP: Get element pointer — compute address from base + index * elem_size.
            // Encoding: src0 = base address, src1 = index, aux0 = element size.
            case OP_GEP {
                let base = resolve_u32(shot_idx, instr.src0, flags, 0u);
                let index = resolve_u32(shot_idx, instr.src1, flags, 1u);
                let elem_size = resolve_u32(shot_idx, instr.aux0, flags, 3u);
                let addr = base + index * elem_size;
                write_reg(shot_idx, instr.dst, addr);
                pc++;
            }

            // Unknown opcode — flag the shot as errored.
            default {
                shots[shot_idx].interp.status = STATUS_ERROR;
                atomicAdd(&diagnostics.termination_count, 1u);
                should_break = true;
            }
        }
        steps++;
        if should_break { break; }
    }

    // -- Persist interpreter state back to GPU memory --
    // Write the local variables back so the next dispatch (after quantum ops
    // or a yield) can resume exactly where this invocation left off.
    shots[shot_idx].interp.pc = pc;
    shots[shot_idx].interp.current_block_id = block_id;
    shots[shot_idx].interp.previous_block_id = prev_block;
}

//#endregion

//#region Adaptive prepare_op implementation

// -----------------------------------------------------------------------------
// Adaptive interpreter — prepare_op entry point
// -----------------------------------------------------------------------------
// Prepares a quantum operation for shots that have STATUS_QUANTUM_PENDING.
// Shots not in that state are set to OPID_ID so execute is a no-op.

fn prepare_op_adaptive_impl(shot_idx: u32) {
    let shot = &shots[shot_idx];
    let state = shots[shot_idx].interp;
    let status = state.status;

    // Only process shots that are quantum-pending
    if status != STATUS_QUANTUM_PENDING {
        // Set op_type to ID so execute is a no-op for this shot
        shot.op_type = OPID_ID;
        shot.renormalize = 1.0;
        shot.qubits_updated_last_op_mask = 0u;
        return;
    }

    // Update shot state from prior op execution
    if shot.qubits_updated_last_op_mask != 0 {
        update_qubit_state(shot_idx);
    }
    shot_init_per_op(shot_idx);

    let op_idx = state.pending_op_idx;
    let op_type = state.pending_op_type;

    // Loss commit: pending_op_idx holds the lost qubit (not an ops-pool index).
    // Measure + reset that qubit; the execute stage applies it via op_idx.
    if op_type == PENDING_OP_LOSS_COMMIT {
        prep_loss_commit(shot_idx, op_idx);
        return;
    }

    let op = &ops[op_idx];

    // Correlated noise: qubit IDs are stored as register indices in
    // call_arg_table; read aux1 (qubit count) and aux2 (arg offset)
    // from the instruction that triggered this quantum op.
    if op_type == 0u && op.id == OPID_CORRELATED_NOISE {
        let pc = state.pc;
        let noise_instr = fetch_instr(pc - 1u);
        let qubit_count = noise_instr.aux1;
        let arg_offset = noise_instr.aux2;
        shot.op_idx = op_idx;
        shot.op_type = op.id;
        prep_correlated_noise_adaptive(shot_idx, op_idx, qubit_count, arg_offset);
        shots[shot_idx].interp.status = STATUS_RUNNING;
        return;
    }

    let q1 = resolve_q1(shot_idx);
    let q2 = resolve_q2(shot_idx);

    shot.unitary = op.unitary;

    switch op_type {
        case 0u { // Gate
            // For rotation gates, recompute the unitary from the dynamic angle stored
            // in the instruction's src0 field if needed. The op pool unitary was built
            // at upload time and may not reflect a runtime-computed angle.
            if is_rotation_gate(op.id) && is_dynamic_angle(shot_idx) {
                if op.id == OPID_RX || op.id == OPID_RY || op.id == OPID_RZ {
                    let angle = resolve_gate_angle(shot_idx);
                    let half = angle * 0.5;
                    let c = cos(half);
                    let s = sin(half);
                    if op.id == OPID_RX {
                        // [[cos(θ/2), -i·sin(θ/2)], [-i·sin(θ/2), cos(θ/2)]]
                        shot.unitary[0] = vec2f(c, 0.0);
                        shot.unitary[1] = vec2f(0.0, -s);
                        shot.unitary[4] = vec2f(0.0, -s);
                        shot.unitary[5] = vec2f(c, 0.0);
                    } else if op.id == OPID_RY {
                        // [[cos(θ/2), -sin(θ/2)], [sin(θ/2), cos(θ/2)]]
                        shot.unitary[0] = vec2f(c, 0.0);
                        shot.unitary[1] = vec2f(-s, 0.0);
                        shot.unitary[4] = vec2f(s, 0.0);
                        shot.unitary[5] = vec2f(c, 0.0);
                    } else {
                        // RZ: [[1, 0], [0, e^(iθ)]]
                        shot.unitary[0] = vec2f(1.0, 0.0);
                        shot.unitary[1] = vec2f(0.0, 0.0);
                        shot.unitary[4] = vec2f(0.0, 0.0);
                        shot.unitary[5] = vec2f(cos(angle), sin(angle));
                    }
                } else if op.id == OPID_RXX || op.id == OPID_RYY || op.id == OPID_RZZ {
                    let angle = resolve_gate_angle(shot_idx);
                    let half = angle * 0.5;
                    let c = cos(half);
                    let s = sin(half);
                    if op.id == OPID_RXX {
                        // exp(-i·θ/2·X⊗X)
                        shot.unitary[0]  = vec2f(c, 0.0);
                        shot.unitary[3]  = vec2f(0.0, -s);
                        shot.unitary[5]  = vec2f(c, 0.0);
                        shot.unitary[6]  = vec2f(0.0, -s);
                        shot.unitary[9]  = vec2f(0.0, -s);
                        shot.unitary[10] = vec2f(c, 0.0);
                        shot.unitary[12] = vec2f(0.0, -s);
                        shot.unitary[15] = vec2f(c, 0.0);
                    } else if op.id == OPID_RYY {
                        // exp(-i·θ/2·Y⊗Y)
                        shot.unitary[0]  = vec2f(c, 0.0);
                        shot.unitary[3]  = vec2f(0.0, s);
                        shot.unitary[5]  = vec2f(c, 0.0);
                        shot.unitary[6]  = vec2f(0.0, -s);
                        shot.unitary[9]  = vec2f(0.0, -s);
                        shot.unitary[10] = vec2f(c, 0.0);
                        shot.unitary[12] = vec2f(0.0, s);
                        shot.unitary[15] = vec2f(c, 0.0);
                    } else {
                        // RZZ: diag(1, e^(iθ), e^(iθ), 1)
                        shot.unitary[0]  = vec2f(1.0, 0.0);
                        shot.unitary[5]  = vec2f(cos(angle), sin(angle));
                        shot.unitary[10] = vec2f(cos(angle), sin(angle));
                        shot.unitary[15] = vec2f(1.0, 0.0);
                    }
                }
            }

            shot.op_idx = op_idx;
            shot.op_type = op.id;

            // If any operand is lost, dispatch the gate's configured loss
            // policy (stamped on op.policy).
            let has_lost_operand = gate_has_lost_operand(shot_idx, op_idx, q1, q2);
            if (has_lost_operand) {
                handle_lost_operand_policy(shot_idx, op_idx, q1, q2);
            }

            // Check for noise ops after this gate in the ops pool
            let pauli_op_idx = get_pauli_noise_idx(op_idx);

            // Handle Pauli noise (loss, if sampled, is recorded in pending_loss_mask)
            if pauli_op_idx != 0u {
                if ops[pauli_op_idx].id == OPID_PAULI_NOISE_1Q {
                    // A 1-qubit gate has a single operand; if it is lost there
                    // is no surviving qubit to receive Pauli noise.
                    if (!has_lost_operand) {
                        apply_1q_pauli_noise(shot_idx, op_idx, pauli_op_idx, q1);
                    }
                } else {
                    if (has_lost_operand) {
                        // The gate body was handled by the loss policy above;
                        // apply the noise to the surviving operand (if any).
                        apply_2q_pauli_noise_on_survivor(shot_idx, op_idx, pauli_op_idx, q1, q2);
                    } else {
                        apply_2q_pauli_noise(shot_idx, op_idx, pauli_op_idx, q1, q2);
                    }
                }
                shots[shot_idx].interp.status = STATUS_RUNNING;
                return;
            }

            // If the gate has any lost operands (and no attached noise), the gate
            // logic was completely handled inside `handle_lost_operand_policy`.
            if (has_lost_operand) {
                shots[shot_idx].interp.status = STATUS_RUNNING;
                return;
            }

            // No noise — set up the op for execution
            finalize_gate_op(shot_idx, op_idx, q1, q2);
        }
        case 1u { // Measure
            // Check for noise ops before the measure op
            // (noise is applied as Id+noise, then original measure, matching non-adaptive pattern)
            let pauli_op_idx = get_pauli_noise_idx(op_idx);

            if pauli_op_idx != 0u {
                // Apply noise to the Id gate before measure, then the measure itself
                // The non-adaptive path inserts Id+noise before measure; here the Id
                // is at op_idx and the original measure op follows after noise ops
                if ops[pauli_op_idx].id == OPID_PAULI_NOISE_1Q {
                    apply_1q_pauli_noise(shot_idx, op_idx, pauli_op_idx, q1);
                } else {
                    apply_2q_pauli_noise(shot_idx, op_idx, pauli_op_idx, q1, q2);
                }
                shots[shot_idx].interp.status = STATUS_RUNNING;
                return;
            }

            // No noise — standard measure
            let resets = op.id == OPID_MRESETZ;
            prep_measure_reset(shot_idx, op_idx, q1, q2, false, true, resets);
        }
        case 2u { // Reset
            prep_measure_reset(shot_idx, op_idx, q1, q2, false, false, true);
        }
        default {
            shot.op_type = OPID_ID;
        }
    }

    // Mark shot as running so interpret_classical resumes next round
    shots[shot_idx].interp.status = STATUS_RUNNING;
}

//#endregion

//#region prepare_op and execute kernels

// Single prepare_op entry point. Dispatches to the base or adaptive
// implementation based on the compile-time IS_ADAPTIVE flag; the unused
// implementation is eliminated by the compiler.
@compute @workgroup_size(1)
fn prepare_op(@builtin(global_invocation_id) globalId: vec3<u32>) {
    if (IS_ADAPTIVE) {
        prepare_op_adaptive_impl(globalId.x);
    } else {
        prepare_op_base_impl(globalId.x);
    }
}

@compute @workgroup_size(THREADS_PER_WORKGROUP)
fn execute(
        @builtin(workgroup_id) workgroupId: vec3<u32>,
        @builtin(local_invocation_index) tid: u32) {
    let shot_idx: i32 = i32(workgroupId.x) / WORKGROUPS_PER_SHOT;
    let shot = &shots[shot_idx];

    // If it's an ID gate, or a pure phase gate (including CZ) then probabilities don't need updating
    // Correlated noise also updates probabilities in prepare_op, so can skip doing that here
    let update_probs = shot.op_type != OPID_ID && shot.op_type != OPID_CORRELATED_NOISE &&
            shot.op_type != OPID_RZ && shot.op_type != OPID_CZ && shot.op_type != OPID_RZZ;

    if (shot.op_type == OPID_ID) {
        // IGNORE
    } else if (shot.op_type == OPID_CORRELATED_NOISE) {
        apply_correlated_noise(workgroupId.x, tid);
    } else if (IS_ADAPTIVE && shot.op_type == OPID_LOSS_NOISE) {
        // Loss commit: the lost qubit is carried in op_idx (set by prep_loss_commit).
        apply_1q_op(workgroupId.x, tid, shot.op_idx);
    } else if (is_1q_op(shot.op_type)) {
        var q1: u32;
        if (IS_ADAPTIVE) {
            q1 = resolve_q1(u32(shot_idx));
        } else {
            q1 = ops[shot.op_idx].q1;
        }
        apply_1q_op(workgroupId.x, tid, q1);
    } else /* 2 qubit op */ {
        var q1: u32;
        var q2: u32;
        if (IS_ADAPTIVE) {
            q1 = resolve_q1(u32(shot_idx));
            q2 = resolve_q2(u32(shot_idx));
        } else {
            q1 = ops[shot.op_idx].q1;
            q2 = ops[shot.op_idx].q2;
        }
        apply_2q_op(workgroupId.x, tid, q1, q2);
    }

    // workgroupBarrier can't be conditional in DX12 backend, so we have to do an unconditional one here
    // outside of the skip_work conditional above.
    workgroupBarrier();

    // If the workgroup is done updating, have the first thread reduce the per-thread probabilities into the
    // totals for this workgroup. The subsequent 'prepare_op' will sum the workgroup entries into the shot state.
    // Skip for correlated noise since probabilities were already updated in prepare_op.
    if (tid == 0 && update_probs) {
        let workgroup_collation_idx: i32 = select(-1, i32(workgroupId.x), WORKGROUPS_PER_SHOT > 1);
        for (var q: u32 = 0u; q < u32(QUBIT_COUNT); q++) {
            if (shot.qubits_updated_last_op_mask & (1u << q)) != 0u {
                sum_thread_totals_to_shot(q, shot_idx, workgroup_collation_idx);
            }
        }
    }
}

//#endregion

//#endregion

"#,
        expect![[r#"
            429..440 'QUBIT_COUNT': i32
            448..449 '8': integer
            468..480 'RESULT_COUNT': u32
            488..489 '8': integer
            508..527 'WORKGR...R_SHOT': i32
            535..536 '1': integer
            555..573 'ENTRIE...THREAD': i32
            581..582 '5': integer
            601..622 'THREAD...KGROUP': i32
            630..632 '32': integer
            651..666 'MAX_QUBIT_COUNT': i32
            674..676 '27': integer
            695..719 'MAX_QU...KGROUP': i32
            727..728 '5': integer
            747..764 'NOISE_..._COUNT': u32
            772..773 '1': integer
            792..809 'NOISE_..._COUNT': u32
            817..818 '1': integer
            837..850 'MAX_REGISTERS': u32
            858..861 '256': integer
            880..890 'MAX_MEMORY': u32
            898..901 '256': integer
            920..937 'INSTRU...S_SIZE': u32
            945..946 '0': integer
            965..981 'BLOCK_...E_SIZE': u32
            989..990 '0': integer
            1009..1028 'FUNCTI...E_SIZE': u32
            1036..1037 '0': integer
            1056..1070 'PHI_TABLE_SIZE': u32
            1078..1079 '0': integer
            1098..1115 'SWITCH...S_SIZE': u32
            1123..1124 '0': integer
            1143..1157 'CALL_ARGS_SIZE': u32
            1165..1166 '0': integer
            1185..1203 'CONSTA...A_SIZE': u32
            1211..1212 '0': integer
            1544..1555 'IS_ADAPTIVE': bool
            1564..1569 'false': bool
            1626..1643 'ERR_IN..._PROBS': u32
            1646..1648 '1u': u32
            1656..1680 'ERR_IN..._TOTAL': u32
            1683..1685 '2u': u32
            1693..1716 'ERR_CA...ERFLOW': u32
            1719..1721 '3u': u32
            1729..1753 'ERR_CA...ERFLOW': u32
            1756..1758 '4u': u32
            1766..1789 'ERR_IN...UCTION': u32
            1792..1794 '5u': u32
            1802..1826 'ERR_AL...BOUNDS': u32
            1829..1831 '6u': u32
            1839..1863 'ERR_ME...BOUNDS': u32
            1866..1868 '7u': u32
            1876..1903 'ERR_UN...POLICY': u32
            1906..1909 '32u': u32
            1956..1963 'OPID_ID': u32
            1971..1973 '0u': u32
            1981..1992 'OPID_RESETZ': u32
            1996..1998 '1u': u32
            2006..2012 'OPID_X': u32
            2021..2023 '2u': u32
            2031..2037 'OPID_Y': u32
            2046..2048 '3u': u32
            2056..2062 'OPID_Z': u32
            2071..2073 '4u': u32
            2081..2087 'OPID_H': u32
            2096..2098 '5u': u32
            2106..2112 'OPID_S': u32
            2121..2123 '6u': u32
            2131..2140 'OPID_SAdj': u32
            2146..2148 '7u': u32
            2156..2162 'OPID_T': u32
            2171..2173 '8u': u32
            2181..2190 'OPID_TAdj': u32
            2196..2198 '9u': u32
            2206..2213 'OPID_RX': u32
            2221..2224 '12u': u32
            2232..2239 'OPID_RY': u32
            2247..2250 '13u': u32
            2258..2265 'OPID_RZ': u32
            2273..2276 '14u': u32
            2284..2291 'OPID_CX': u32
            2299..2302 '15u': u32
            2310..2317 'OPID_CZ': u32
            2325..2328 '16u': u32
            2336..2344 'OPID_RXX': u32
            2351..2354 '17u': u32
            2362..2370 'OPID_RYY': u32
            2377..2380 '18u': u32
            2388..2396 'OPID_RZZ': u32
            2403..2406 '19u': u32
            2414..2421 'OPID_MZ': u32
            2429..2432 '21u': u32
            2440..2452 'OPID_MRESETZ': u32
            2455..2458 '22u': u32
            2466..2475 'OPID_SWAP': u32
            2481..2484 '24u': u32
            2492..2502 'OPID_MAT1Q': u32
            2507..2510 '25u': u32
            2518..2528 'OPID_MAT2Q': u32
            2533..2536 '26u': u32
            2544..2551 'OPID_CY': u32
            2559..2562 '29u': u32
            2571..2590 'OPID_P...ISE_1Q': u32
            2593..2597 '128u': u32
            2605..2624 'OPID_P...ISE_2Q': u32
            2627..2631 '129u': u32
            2639..2654 'OPID_LOSS_NOISE': u32
            2657..2661 '130u': u32
            2669..2690 'OPID_C..._NOISE': u32
            2693..2697 '131u': u32
            2916..2933 'OPID_S...UFF_1Q': u32
            2936..2940 '256u': u32
            2948..2965 'OPID_S...UFF_2Q': u32
            2968..2972 '257u': u32
            3066..3080 'PROB_THRESHOLD': f32
            3088..3094 '0.0001': float
            3191..3219 'MAX_WO...ITIONS': i32
            3227..3229 '1i': i32
            3227..3280 '1i << ...GROUP)': i32
            3233..3280 'u32(MA...GROUP)': u32
            3237..3252 'MAX_QUBIT_COUNT': i32
            3237..3279 'MAX_QU...KGROUP': i32
            3255..3279 'MAX_QU...KGROUP': i32
            3573..3589 'LOSS_P...Y_SKIP': u32
            3605..3607 '0u': u32
            3615..3636 'LOSS_P...PAGATE': u32
            3647..3649 '1u': u32
            3657..3676 'LOSS_P...EGRADE': u32
            3689..3691 '2u': u32
            3699..3728 'LOSS_P...DAGGER': u32
            3731..3733 '3u': u32
            3741..3765 'LOSS_P...ANYWAY': u32
            3773..3775 '4u': u32
            3840..3859 'MAX_CL..._STEPS': u32
            3867..3872 '4096u': u32
            3897..3911 'STATUS_RUNNING': u32
            3928..3930 '0u': u32
            3938..3960 'STATUS...ENDING': u32
            3969..3971 '1u': u32
            3979..3996 'STATUS...INATED': u32
            4010..4012 '2u': u32
            4020..4032 'STATUS_ERROR': u32
            4051..4053 '3u': u32
            4061..4073 'STATUS_YIELD': u32
            4092..4094 '4u': u32
            4415..4437 'PENDIN...COMMIT': u32
            4446..4448 '3u': u32
            5255..5268 'FLAG_SRC0_IMM': u32
            5276..5277 '1': integer
            5276..5283 '1 << 16': integer
            5281..5283 '16': integer
            5344..5357 'FLAG_SRC1_IMM': u32
            5365..5366 '1': integer
            5365..5372 '1 << 17': integer
            5370..5372 '17': integer
            5433..5445 'FLAG_DST_IMM': u32
            5454..5455 '1': integer
            5454..5461 '1 << 18': integer
            5459..5461 '18': integer
            5522..5535 'FLAG_AUX0_IMM': u32
            5543..5544 '1': integer
            5543..5550 '1 << 19': integer
            5548..5550 '19': integer
            5611..5624 'FLAG_AUX1_IMM': u32
            5632..5633 '1': integer
            5632..5639 '1 << 20': integer
            5637..5639 '20': integer
            5700..5713 'FLAG_AUX2_IMM': u32
            5721..5722 '1': integer
            5721..5728 '1 << 21': integer
            5726..5728 '21': integer
            5789..5802 'FLAG_AUX3_IMM': u32
            5810..5811 '1': integer
            5810..5817 '1 << 22': integer
            5815..5817 '22': integer
            5960..5966 'OP_NOP': u32
            5984..5988 '0x00': integer
            5996..6002 'OP_RET': u32
            6020..6024 '0x02': integer
            6032..6039 'OP_JUMP': u32
            6056..6060 '0x04': integer
            6068..6077 'OP_BRANCH': u32
            6092..6096 '0x05': integer
            6104..6113 'OP_SWITCH': u32
            6128..6132 '0x06': integer
            6140..6147 'OP_CALL': u32
            6164..6168 '0x07': integer
            6176..6190 'OP_CALL_RETURN': u32
            6200..6204 '0x08': integer
            6294..6309 'OP_QUANTUM_GATE': u32
            6318..6322 '0x10': integer
            6330..6340 'OP_MEASURE': u32
            6354..6358 '0x11': integer
            6366..6374 'OP_RESET': u32
            6390..6394 '0x12': integer
            6402..6416 'OP_READ_RESULT': u32
            6426..6430 '0x13': integer
            6438..6454 'OP_REC...OUTPUT': u32
            6462..6466 '0x14': integer
            6474..6486 'OP_READ_LOSS': u32
            6498..6502 '0x15': integer
            6592..6598 'OP_ADD': u32
            6616..6620 '0x20': integer
            6628..6634 'OP_SUB': u32
            6652..6656 '0x21': integer
            6664..6670 'OP_MUL': u32
            6688..6692 '0x22': integer
            6700..6707 'OP_UDIV': u32
            6724..6728 '0x23': integer
            6736..6743 'OP_SDIV': u32
            6760..6764 '0x24': integer
            6772..6779 'OP_UREM': u32
            6796..6800 '0x25': integer
            6808..6815 'OP_SREM': u32
            6832..6836 '0x26': integer
            6925..6931 'OP_AND': u32
            6949..6953 '0x28': integer
            6961..6966 'OP_OR': u32
            6985..6989 '0x29': integer
            6997..7003 'OP_XOR': u32
            7021..7025 '0x2A': integer
            7033..7039 'OP_SHL': u32
            7057..7061 '0x2B': integer
            7069..7076 'OP_LSHR': u32
            7093..7097 '0x2C': integer
            7105..7112 'OP_ASHR': u32
            7129..7133 '0x2D': integer
            7223..7230 'OP_ICMP': u32
            7247..7251 '0x30': integer
            7259..7266 'OP_FCMP': u32
            7283..7287 '0x31': integer
            7377..7384 'OP_FADD': u32
            7401..7405 '0x38': integer
            7413..7420 'OP_FSUB': u32
            7437..7441 '0x39': integer
            7449..7456 'OP_FMUL': u32
            7473..7477 '0x3A': integer
            7485..7492 'OP_FDIV': u32
            7509..7513 '0x3B': integer
            7521..7528 'OP_FREM': u32
            7545..7549 '0x3C': integer
            7639..7646 'OP_ZEXT': u32
            7663..7667 '0x40': integer
            7675..7682 'OP_SEXT': u32
            7699..7703 '0x41': integer
            7711..7719 'OP_TRUNC': u32
            7735..7739 '0x42': integer
            7747..7755 'OP_FPEXT': u32
            7771..7775 '0x43': integer
            7783..7793 'OP_FPTRUNC': u32
            7807..7811 '0x44': integer
            7819..7830 'OP_INTTOPTR': u32
            7843..7847 '0x45': integer
            7855..7864 'OP_FPTOSI': u32
            7879..7883 '0x46': integer
            7891..7900 'OP_SITOFP': u32
            7915..7919 '0x47': integer
            7927..7936 'OP_FPTOUI': u32
            7951..7955 '0x48': integer
            7963..7972 'OP_UITOFP': u32
            7987..7991 '0x49': integer
            8080..8086 'OP_PHI': u32
            8104..8108 '0x50': integer
            8116..8125 'OP_SELECT': u32
            8140..8144 '0x51': integer
            8152..8158 'OP_MOV': u32
            8176..8180 '0x52': integer
            8188..8196 'OP_CONST': u32
            8212..8216 '0x53': integer
            8306..8315 'OP_ALLOCA': u32
            8330..8334 '0x60': integer
            8342..8349 'OP_LOAD': u32
            8366..8370 '0x61': integer
            8378..8386 'OP_STORE': u32
            8402..8406 '0x62': integer
            8414..8420 'OP_GEP': u32
            8438..8442 '0x63': integer
            8598..8605 'ICMP_EQ': u32
            8622..8623 '0': integer
            8631..8638 'ICMP_NE': u32
            8655..8656 '1': integer
            8664..8672 'ICMP_SLT': u32
            8688..8689 '2': integer
            8697..8705 'ICMP_SLE': u32
            8721..8722 '3': integer
            8730..8738 'ICMP_SGT': u32
            8754..8755 '4': integer
            8763..8771 'ICMP_SGE': u32
            8787..8788 '5': integer
            8796..8804 'ICMP_ULT': u32
            8820..8821 '6': integer
            8829..8837 'ICMP_ULE': u32
            8853..8854 '7': integer
            8862..8870 'ICMP_UGT': u32
            8886..8887 '8': integer
            8895..8903 'ICMP_UGE': u32
            8919..8920 '9': integer
            9076..9086 'FCMP_FALSE': u32
            9100..9101 '0': integer
            9109..9117 'FCMP_OEQ': u32
            9133..9134 '1': integer
            9142..9150 'FCMP_OGT': u32
            9166..9167 '2': integer
            9175..9183 'FCMP_OGE': u32
            9199..9200 '3': integer
            9208..9216 'FCMP_OLT': u32
            9232..9233 '4': integer
            9241..9249 'FCMP_OLE': u32
            9265..9266 '5': integer
            9274..9282 'FCMP_ONE': u32
            9298..9299 '6': integer
            9307..9315 'FCMP_ORD': u32
            9331..9332 '7': integer
            9340..9348 'FCMP_UNO': u32
            9364..9365 '8': integer
            9373..9381 'FCMP_UEQ': u32
            9397..9398 '9': integer
            9406..9414 'FCMP_UGT': u32
            9430..9432 '10': integer
            9440..9448 'FCMP_UGE': u32
            9464..9466 '11': integer
            9474..9482 'FCMP_ULT': u32
            9498..9500 '12': integer
            9508..9516 'FCMP_ULE': u32
            9532..9534 '13': integer
            9542..9550 'FCMP_UNE': u32
            9566..9568 '14': integer
            9576..9585 'FCMP_TRUE': u32
            9600..9602 '15': integer
            9692..9703 'VOID_RETURN': u32
            9718..9728 '0xFFFFFFFF': integer
            9863..9878 'MAX_QUBIT_COUNT': unexpected template argument, expected a `u32` or a `i32` greater than `0`
            10006..10034 'MAX_WO...ITIONS': unexpected template argument, expected a `u32` or a `i32` greater than `0`
            11207..11224 'INSTRU...S_SIZE': unexpected template argument, expected a `u32` or a `i32` greater than `0`
            11299..11315 'BLOCK_...E_SIZE': unexpected template argument, expected a `u32` or a `i32` greater than `0`
            11378..11397 'FUNCTI...E_SIZE': unexpected template argument, expected a `u32` or a `i32` greater than `0`
            11512..11526 'PHI_TABLE_SIZE': unexpected template argument, expected a `u32` or a `i32` greater than `0`
            11632..11649 'SWITCH...S_SIZE': unexpected template argument, expected a `u32` or a `i32` greater than `0`
            11723..11737 'CALL_ARGS_SIZE': unexpected template argument, expected a `u32` or a `i32` greater than `0`
            11832..11850 'CONSTA...A_SIZE': unexpected template argument, expected a `u32` or a `i32` greater than `0`
            12850..12863 'MAX_REGISTERS': unexpected template argument, expected a `u32` or a `i32` greater than `0`
            12948..12958 'MAX_MEMORY': unexpected template argument, expected a `u32` or a `i32` greater than `0`
            14713..14728 'MAX_QUBIT_COUNT': unexpected template argument, expected a `u32` or a `i32` greater than `0`
            17289..17306 'NOISE_..._COUNT': unexpected template argument, expected a `u32` or a `i32` greater than `0`
            17362..17379 'NOISE_..._COUNT': unexpected template argument, expected a `u32` or a `i32` greater than `0`
            17908..17923 'MAX_QUBIT_COUNT': unexpected template argument, expected a `u32` or a `i32` greater than `0`
            17946..17961 'MAX_QUBIT_COUNT': unexpected template argument, expected a `u32` or a `i32` greater than `0`
            18480..18501 'THREAD...KGROUP': unexpected template argument, expected a `u32` or a `i32` greater than `0`
            18849..18868 'workgr...lation': ref<storage, WorkgroupCollationBuffer, read_write>
            19020..19025 'shots': ref<storage, array<ShotData>, read_write>
            19086..19089 'ops': ref<storage, array<Op>, read>
            19236..19247 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            19362..19369 'results': ref<storage, array<atomic<u32>>, read_write>
            19439..19450 'diagnostics': ref<storage, DiagnosticData, read_write>
            19504..19512 'uniforms': ref<uniform, Uniforms, read>
            19566..19576 'batch_data': ref<storage, BatchData, read>
            19605..19623 'qubitP...lities': ref<workgroup, [error], read_write>
            19658..19679 'THREAD...KGROUP': unexpected template argument, expected a `u32` or a `i32` greater than `0`
            19866..19867 'a': vec2<f32>
            19897..19898 'a': vec2<f32>
            19897..19900 'a.x': f32
            19897..19906 'a.x * a.x': f32
            19897..19918 'a.x * ... * a.y': f32
            19903..19904 'a': vec2<f32>
            19903..19906 'a.x': f32
            19909..19910 'a': vec2<f32>
            19909..19912 'a.y': f32
            19909..19918 'a.y * a.y': f32
            19915..19916 'a': vec2<f32>
            19915..19918 'a.y': f32
            19961..19962 'a': vec2<f32>
            19971..19972 'b': vec2<f32>
            20003..20076 'vec2f(...     )': vec2<f32>
            20018..20019 'a': vec2<f32>
            20018..20021 'a.x': f32
            20018..20027 'a.x * b.x': f32
            20018..20039 'a.x * ... * b.y': f32
            20024..20025 'b': vec2<f32>
            20024..20027 'b.x': f32
            20030..20031 'a': vec2<f32>
            20030..20033 'a.y': f32
            20030..20039 'a.y * b.y': f32
            20036..20037 'b': vec2<f32>
            20036..20039 'b.y': f32
            20049..20050 'a': vec2<f32>
            20049..20052 'a.x': f32
            20049..20058 'a.x * b.y': f32
            20049..20070 'a.x * ... * b.x': f32
            20055..20056 'b': vec2<f32>
            20055..20058 'b.y': f32
            20061..20062 'a': vec2<f32>
            20061..20064 'a.y': f32
            20061..20070 'a.y * b.x': f32
            20067..20068 'b': vec2<f32>
            20067..20070 'b.x': f32
            20112..20113 'a': vec2<f32>
            20144..20161 'vec2f(... -a.y)': vec2<f32>
            20150..20154 '-a.x': f32
            20151..20152 'a': vec2<f32>
            20151..20154 'a.x': f32
            20156..20160 '-a.y': f32
            20157..20158 'a': vec2<f32>
            20157..20160 'a.y': f32
            20237..20238 'a': array<vec2<f32>, 4>
            20289..20397 'array<...a[3]))': array<vec2<f32>, 4>
            20314..20327 'cplxNeg(a[0])': vec2<f32>
            20322..20323 'a': array<vec2<f32>, 4>
            20322..20326 'a[0]': vec2<f32>
            20324..20325 '0': integer
            20337..20350 'cplxNeg(a[1])': vec2<f32>
            20345..20346 'a': array<vec2<f32>, 4>
            20345..20349 'a[1]': vec2<f32>
            20347..20348 '1': integer
            20360..20373 'cplxNeg(a[2])': vec2<f32>
            20368..20369 'a': array<vec2<f32>, 4>
            20368..20372 'a[2]': vec2<f32>
            20370..20371 '2': integer
            20383..20396 'cplxNeg(a[3])': vec2<f32>
            20391..20392 'a': array<vec2<f32>, 4>
            20391..20395 'a[3]': vec2<f32>
            20393..20394 '3': integer
            20488..20489 'a': array<vec2<f32>, 4>
            20508..20509 'b': array<vec2<f32>, 4>
            20547..20553 'result': ref<function, vec2<f32>, read_write>
            20563..20578 'vec2f(0.0, 0.0)': vec2<f32>
            20569..20572 '0.0': float
            20574..20577 '0.0': float
            20593..20594 'i': ref<function, u32, read_write>
            20602..20604 '0u': u32
            20606..20607 'i': ref<function, u32, read_write>
            20606..20612 'i < 4u': bool
            20610..20612 '4u': u32
            20614..20615 'i': ref<function, u32, read_write>
            20629..20635 'result': ref<function, vec2<f32>, read_write>
            20639..20658 'cplxMu... b[i])': vec2<f32>
            20647..20648 'a': array<vec2<f32>, 4>
            20647..20651 'a[i]': vec2<f32>
            20649..20650 'i': ref<function, u32, read_write>
            20653..20654 'b': array<vec2<f32>, 4>
            20653..20657 'b[i]': vec2<f32>
            20655..20656 'i': ref<function, u32, read_write>
            20677..20683 'result': ref<function, vec2<f32>, read_write>
            20700..20706 'op_idx': u32
            20713..20716 'row': u32
            20752..20754 'op': ptr<storage, Op, read>
            20757..20769 '&ops[op_idx]': ptr<storage, Op, read>
            20758..20761 'ops': ref<storage, array<Op>, read>
            20758..20769 'ops[op_idx]': ref<storage, Op, read>
            20762..20768 'op_idx': u32
            20782..20930 'array<... + 3])': array<vec2<f32>, 4>
            20807..20809 'op': ptr<storage, Op, read>
            20807..20817 'op.unitary': ref<storage, array<vec2<f32>, 16>, read>
            20807..20830 'op.uni...4 + 0]': ref<storage, vec2<f32>, read>
            20818..20821 'row': u32
            20818..20825 'row * 4': u32
            20818..20829 'row * 4 + 0': u32
            20824..20825 '4': integer
            20828..20829 '0': integer
            20840..20842 'op': ptr<storage, Op, read>
            20840..20850 'op.unitary': ref<storage, array<vec2<f32>, 16>, read>
            20840..20863 'op.uni...4 + 1]': ref<storage, vec2<f32>, read>
            20851..20854 'row': u32
            20851..20858 'row * 4': u32
            20851..20862 'row * 4 + 1': u32
            20857..20858 '4': integer
            20861..20862 '1': integer
            20873..20875 'op': ptr<storage, Op, read>
            20873..20883 'op.unitary': ref<storage, array<vec2<f32>, 16>, read>
            20873..20896 'op.uni...4 + 2]': ref<storage, vec2<f32>, read>
            20884..20887 'row': u32
            20884..20891 'row * 4': u32
            20884..20895 'row * 4 + 2': u32
            20890..20891 '4': integer
            20894..20895 '2': integer
            20906..20908 'op': ptr<storage, Op, read>
            20906..20916 'op.unitary': ref<storage, array<vec2<f32>, 16>, read>
            20906..20929 'op.uni...4 + 3]': ref<storage, vec2<f32>, read>
            20917..20920 'row': u32
            20917..20924 'row * 4': u32
            20917..20928 'row * 4 + 3': u32
            20923..20924 '4': integer
            20927..20928 '3': integer
            20952..20960 'shot_idx': i32
            20967..20970 'row': u32
            21006..21010 'shot': ptr<storage, ShotData, read_write>
            21013..21029 '&shots...t_idx]': ptr<storage, ShotData, read_write>
            21014..21019 'shots': ref<storage, array<ShotData>, read_write>
            21014..21029 'shots[shot_idx]': ref<storage, ShotData, read_write>
            21020..21028 'shot_idx': i32
            21042..21198 'array<... + 3])': array<vec2<f32>, 4>
            21067..21071 'shot': ptr<storage, ShotData, read_write>
            21067..21079 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            21067..21092 'shot.u...4 + 0]': ref<storage, vec2<f32>, read_write>
            21080..21083 'row': u32
            21080..21087 'row * 4': u32
            21080..21091 'row * 4 + 0': u32
            21086..21087 '4': integer
            21090..21091 '0': integer
            21102..21106 'shot': ptr<storage, ShotData, read_write>
            21102..21114 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            21102..21127 'shot.u...4 + 1]': ref<storage, vec2<f32>, read_write>
            21115..21118 'row': u32
            21115..21122 'row * 4': u32
            21115..21126 'row * 4 + 1': u32
            21121..21122 '4': integer
            21125..21126 '1': integer
            21137..21141 'shot': ptr<storage, ShotData, read_write>
            21137..21149 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            21137..21162 'shot.u...4 + 2]': ref<storage, vec2<f32>, read_write>
            21150..21153 'row': u32
            21150..21157 'row * 4': u32
            21150..21161 'row * 4 + 2': u32
            21156..21157 '4': integer
            21160..21161 '2': integer
            21172..21176 'shot': ptr<storage, ShotData, read_write>
            21172..21184 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            21172..21197 'shot.u...4 + 3]': ref<storage, vec2<f32>, read_write>
            21185..21188 'row': u32
            21185..21192 'row * 4': u32
            21185..21196 'row * 4 + 3': u32
            21191..21192 '4': integer
            21195..21196 '3': integer
            21220..21228 'shot_idx': u32
            21235..21238 'row': u32
            21245..21251 'newRow': array<vec2<f32>, 4>
            21280..21284 'shot': ptr<storage, ShotData, read_write>
            21287..21303 '&shots...t_idx]': ptr<storage, ShotData, read_write>
            21288..21293 'shots': ref<storage, array<ShotData>, read_write>
            21288..21303 'shots[shot_idx]': ref<storage, ShotData, read_write>
            21294..21302 'shot_idx': u32
            21309..21313 'shot': ptr<storage, ShotData, read_write>
            21309..21321 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            21309..21334 'shot.u...4 + 0]': ref<storage, vec2<f32>, read_write>
            21322..21325 'row': u32
            21322..21329 'row * 4': u32
            21322..21333 'row * 4 + 0': u32
            21328..21329 '4': integer
            21332..21333 '0': integer
            21337..21343 'newRow': array<vec2<f32>, 4>
            21337..21346 'newRow[0]': vec2<f32>
            21344..21345 '0': integer
            21352..21356 'shot': ptr<storage, ShotData, read_write>
            21352..21364 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            21352..21377 'shot.u...4 + 1]': ref<storage, vec2<f32>, read_write>
            21365..21368 'row': u32
            21365..21372 'row * 4': u32
            21365..21376 'row * 4 + 1': u32
            21371..21372 '4': integer
            21375..21376 '1': integer
            21380..21386 'newRow': array<vec2<f32>, 4>
            21380..21389 'newRow[1]': vec2<f32>
            21387..21388 '1': integer
            21395..21399 'shot': ptr<storage, ShotData, read_write>
            21395..21407 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            21395..21420 'shot.u...4 + 2]': ref<storage, vec2<f32>, read_write>
            21408..21411 'row': u32
            21408..21415 'row * 4': u32
            21408..21419 'row * 4 + 2': u32
            21414..21415 '4': integer
            21418..21419 '2': integer
            21423..21429 'newRow': array<vec2<f32>, 4>
            21423..21432 'newRow[2]': vec2<f32>
            21430..21431 '2': integer
            21438..21442 'shot': ptr<storage, ShotData, read_write>
            21438..21450 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            21438..21463 'shot.u...4 + 3]': ref<storage, vec2<f32>, read_write>
            21451..21454 'row': u32
            21451..21458 'row * 4': u32
            21451..21462 'row * 4 + 3': u32
            21457..21458 '4': integer
            21461..21462 '3': integer
            21466..21472 'newRow': array<vec2<f32>, 4>
            21466..21475 'newRow[3]': vec2<f32>
            21473..21474 '3': integer
            21727..21732 'input': u32
            21756..21761 'state': ref<function, u32, read_write>
            21764..21769 'input': u32
            21764..21782 'input ...96405u': u32
            21764..21796 'input ...36453u': u32
            21772..21782 '747796405u': u32
            21785..21796 '2891336453u': u32
            21806..21810 'word': ref<function, u32, read_write>
            21813..21868 '((stat...03737u': u32
            21814..21854 '(state... state': u32
            21815..21820 'state': ref<function, u32, read_write>
            21815..21845 'state ... + 4u)': u32
            21825..21844 '(state...) + 4u': u32
            21826..21831 'state': ref<function, u32, read_write>
            21826..21838 'state >> 28u': u32
            21835..21838 '28u': u32
            21842..21844 '4u': u32
            21849..21854 'state': ref<function, u32, read_write>
            21858..21868 '277803737u': u32
            21881..21901 '(word ...^ word': u32
            21882..21886 'word': ref<function, u32, read_write>
            21882..21893 'word >> 22u': u32
            21890..21893 '22u': u32
            21897..21901 'word': ref<function, u32, read_write>
            21983..21991 'shot_idx': u32
            22070..22079 'rng_state': ptr<storage, xorwow_state, read_write>
            22082..22108 '&shots..._state': ptr<storage, xorwow_state, read_write>
            22083..22088 'shots': ref<storage, array<ShotData>, read_write>
            22083..22098 'shots[shot_idx]': ref<storage, ShotData, read_write>
            22083..22108 'shots[..._state': ref<storage, xorwow_state, read_write>
            22089..22097 'shot_idx': u32
            22119..22120 't': u32
            22128..22137 'rng_state': ptr<storage, xorwow_state, read_write>
            22128..22139 'rng_state.x': ref<storage, array<u32, 5>, read_write>
            22128..22142 'rng_state.x[4]': ref<storage, u32, read_write>
            22140..22141 '4': integer
            22152..22153 's': u32
            22161..22170 'rng_state': ptr<storage, xorwow_state, read_write>
            22161..22172 'rng_state.x': ref<storage, array<u32, 5>, read_write>
            22161..22175 'rng_state.x[0]': ref<storage, u32, read_write>
            22173..22174 '0': integer
            22181..22190 'rng_state': ptr<storage, xorwow_state, read_write>
            22181..22192 'rng_state.x': ref<storage, array<u32, 5>, read_write>
            22181..22195 'rng_state.x[4]': ref<storage, u32, read_write>
            22193..22194 '4': integer
            22198..22207 'rng_state': ptr<storage, xorwow_state, read_write>
            22198..22209 'rng_state.x': ref<storage, array<u32, 5>, read_write>
            22198..22212 'rng_state.x[3]': ref<storage, u32, read_write>
            22210..22211 '3': integer
            22218..22227 'rng_state': ptr<storage, xorwow_state, read_write>
            22218..22229 'rng_state.x': ref<storage, array<u32, 5>, read_write>
            22218..22232 'rng_state.x[3]': ref<storage, u32, read_write>
            22230..22231 '3': integer
            22235..22244 'rng_state': ptr<storage, xorwow_state, read_write>
            22235..22246 'rng_state.x': ref<storage, array<u32, 5>, read_write>
            22235..22249 'rng_state.x[2]': ref<storage, u32, read_write>
            22247..22248 '2': integer
            22255..22264 'rng_state': ptr<storage, xorwow_state, read_write>
            22255..22266 'rng_state.x': ref<storage, array<u32, 5>, read_write>
            22255..22269 'rng_state.x[2]': ref<storage, u32, read_write>
            22267..22268 '2': integer
            22272..22281 'rng_state': ptr<storage, xorwow_state, read_write>
            22272..22283 'rng_state.x': ref<storage, array<u32, 5>, read_write>
            22272..22286 'rng_state.x[1]': ref<storage, u32, read_write>
            22284..22285 '1': integer
            22292..22301 'rng_state': ptr<storage, xorwow_state, read_write>
            22292..22303 'rng_state.x': ref<storage, array<u32, 5>, read_write>
            22292..22306 'rng_state.x[1]': ref<storage, u32, read_write>
            22304..22305 '1': integer
            22309..22310 's': u32
            22429..22431 't2': u32
            22434..22435 't': u32
            22434..22447 't ^ (t >> 2u)': u32
            22439..22440 't': u32
            22439..22446 't >> 2u': u32
            22444..22446 '2u': u32
            22457..22459 't3': u32
            22462..22464 't2': u32
            22462..22477 't2 ^ (t2 << 1u)': u32
            22468..22470 't2': u32
            22468..22476 't2 << 1u': u32
            22474..22476 '1u': u32
            22487..22489 't4': u32
            22492..22494 't3': u32
            22492..22498 't3 ^ s': u32
            22492..22510 't3 ^ s...<< 4u)': u32
            22497..22498 's': u32
            22502..22503 's': u32
            22502..22509 's << 4u': u32
            22507..22509 '4u': u32
            22516..22525 'rng_state': ptr<storage, xorwow_state, read_write>
            22516..22527 'rng_state.x': ref<storage, array<u32, 5>, read_write>
            22516..22530 'rng_state.x[0]': ref<storage, u32, read_write>
            22528..22529 '0': integer
            22533..22535 't4': u32
            22541..22550 'rng_state': ptr<storage, xorwow_state, read_write>
            22541..22558 'rng_st...ounter': ref<storage, u32, read_write>
            22561..22570 'rng_state': ptr<storage, xorwow_state, read_write>
            22561..22578 'rng_st...ounter': ref<storage, u32, read_write>
            22561..22588 'rng_st...62437u': u32
            22581..22588 '362437u': u32
            22601..22603 't4': u32
            22601..22623 't4 + r...ounter': u32
            22606..22615 'rng_state': ptr<storage, xorwow_state, read_write>
            22606..22623 'rng_st...ounter': ref<storage, u32, read_write>
            22645..22653 'shot_idx': u32
            22677..22685 'rand_u32': u32
            22693..22716 'next_r...t_idx)': u32
            22707..22715 'shot_idx': u32
            22898..22911 'rand_f32_bits': u32
            22914..22949 '(rand_...<< 23)': u32
            22915..22923 'rand_u32': u32
            22915..22934 'rand_u...7FFFFF': u32
            22926..22934 '0x7FFFFF': integer
            22939..22942 '127': integer
            22939..22948 '127 << 23': integer
            22946..22948 '23': integer
            23008..23009 'f': f32
            23017..23044 'bitcas..._bits)': f32
            23030..23043 'rand_f32_bits': u32
            23112..23113 'f': f32
            23112..23119 'f - 1.0': f32
            23116..23119 '1.0': float
            23202..23207 'op_id': u32
            23236..23241 'op_id': u32
            23236..23251 'op_id == OPID_S': bool
            23236..23273 'op_id ...D_SAdj': bool
            23236..23292 'op_id ...OPID_T': bool
            23236..23314 'op_id ...D_TAdj': bool
            23236..23334 'op_id ...PID_RZ': bool
            23245..23251 'OPID_S': u32
            23255..23260 'op_id': u32
            23255..23273 'op_id ...D_SAdj': bool
            23264..23273 'OPID_SAdj': u32
            23277..23282 'op_id': u32
            23277..23292 'op_id == OPID_T': bool
            23286..23292 'OPID_T': u32
            23296..23301 'op_id': u32
            23296..23314 'op_id ...D_TAdj': bool
            23305..23314 'OPID_TAdj': u32
            23318..23323 'op_id': u32
            23318..23334 'op_id ...PID_RZ': bool
            23327..23334 'OPID_RZ': u32
            23352..23357 'op_id': u32
            23386..23452 '(op_id...PID_MZ': bool
            23386..23477 '(op_id...RESETZ': bool
            23386..23508 '(op_id..._MAT1Q': bool
            23386..23538 '(op_id...UFF_1Q': bool
            23387..23392 'op_id': u32
            23387..23403 'op_id ...PID_ID': bool
            23387..23423 'op_id ...PID_RZ': bool
            23396..23403 'OPID_ID': u32
            23407..23412 'op_id': u32
            23407..23423 'op_id ...PID_RZ': bool
            23416..23423 'OPID_RZ': u32
            23436..23441 'op_id': u32
            23436..23452 'op_id ...PID_MZ': bool
            23445..23452 'OPID_MZ': u32
            23456..23461 'op_id': u32
            23456..23477 'op_id ...RESETZ': bool
            23465..23477 'OPID_MRESETZ': u32
            23489..23494 'op_id': u32
            23489..23508 'op_id ..._MAT1Q': bool
            23498..23508 'OPID_MAT1Q': u32
            23512..23517 'op_id': u32
            23512..23538 'op_id ...UFF_1Q': bool
            23521..23538 'OPID_S...UFF_1Q': u32
            23614..23622 'shot_idx': u32
            23639..23643 'shot': ptr<storage, ShotData, read_write>
            23646..23662 '&shots...t_idx]': ptr<storage, ShotData, read_write>
            23647..23652 'shots': ref<storage, array<ShotData>, read_write>
            23647..23662 'shots[shot_idx]': ref<storage, ShotData, read_write>
            23653..23661 'shot_idx': u32
            23874..23878 'shot': ptr<storage, ShotData, read_write>
            23874..23890 'shot.r...malize': ref<storage, f32, read_write>
            23893..23896 '1.0': float
            23902..23906 'shot': ptr<storage, ShotData, read_write>
            23902..23934 'shot.q...p_mask': ref<storage, u32, read_write>
            23937..23939 '0u': u32
            24026..24030 'shot': ptr<storage, ShotData, read_write>
            24026..24041 'shot.rand_pauli': ref<storage, f32, read_write>
            24044..24067 'next_r...t_idx)': f32
            24058..24066 'shot_idx': u32
            24073..24077 'shot': ptr<storage, ShotData, read_write>
            24073..24090 'shot.r...amping': ref<storage, f32, read_write>
            24093..24116 'next_r...t_idx)': f32
            24107..24115 'shot_idx': u32
            24122..24126 'shot': ptr<storage, ShotData, read_write>
            24122..24139 'shot.r...ephase': ref<storage, f32, read_write>
            24142..24165 'next_r...t_idx)': f32
            24156..24164 'shot_idx': u32
            24171..24175 'shot': ptr<storage, ShotData, read_write>
            24171..24188 'shot.r...easure': ref<storage, f32, read_write>
            24191..24214 'next_r...t_idx)': f32
            24205..24213 'shot_idx': u32
            24503..24526 'next_r...t_idx)': f32
            24517..24525 'shot_idx': u32
            24631..24639 'shot_idx': i32
            24656..24660 'shot': ptr<storage, ShotData, read_write>
            24663..24679 '&shots...t_idx]': ptr<storage, ShotData, read_write>
            24664..24669 'shots': ref<storage, array<ShotData>, read_write>
            24664..24679 'shots[shot_idx]': ref<storage, ShotData, read_write>
            24670..24678 'shot_idx': i32
            24777..24785 'rng_seed': u32
            24788..24796 'uniforms': ref<uniform, Uniforms, read>
            24788..24805 'unifor...g_seed': ref<uniform, u32, read>
            24815..24822 'shot_id': u32
            24825..24869 'u32(un...t_idx)': u32
            24829..24837 'uniforms': ref<uniform, Uniforms, read>
            24829..24857 'unifor...hot_id': ref<uniform, i32, read>
            24829..24868 'unifor...ot_idx': i32
            24860..24868 'shot_idx': i32
            25002..25007 '*shot': ref<storage, ShotData, read_write>
            25003..25007 'shot': ptr<storage, ShotData, read_write>
            25010..25020 'ShotData()': ShotData
            25048..25052 'shot': ptr<storage, ShotData, read_write>
            25048..25060 'shot.shot_id': ref<storage, u32, read_write>
            25063..25070 'shot_id': u32
            25130..25134 'shot': ptr<storage, ShotData, read_write>
            25130..25146 'shot.n...op_idx': ref<storage, u32, read_write>
            25149..25151 '0u': u32
            25158..25162 'shot': ptr<storage, ShotData, read_write>
            25158..25172 'shot.rng_state': ref<storage, xorwow_state, read_write>
            25158..25174 'shot.r...tate.x': ref<storage, array<u32, 5>, read_write>
            25158..25177 'shot.r...e.x[0]': ref<storage, u32, read_write>
            25175..25176 '0': integer
            25180..25188 'rng_seed': u32
            25180..25208 'rng_se...ot_id)': u32
            25191..25208 'hash_p...ot_id)': u32
            25200..25207 'shot_id': u32
            25214..25218 'shot': ptr<storage, ShotData, read_write>
            25214..25228 'shot.rng_state': ref<storage, xorwow_state, read_write>
            25214..25230 'shot.r...tate.x': ref<storage, array<u32, 5>, read_write>
            25214..25233 'shot.r...e.x[1]': ref<storage, u32, read_write>
            25231..25232 '1': integer
            25236..25244 'rng_seed': u32
            25236..25268 'rng_se...d + 1)': u32
            25247..25268 'hash_p...d + 1)': u32
            25256..25263 'shot_id': u32
            25256..25267 'shot_id + 1': u32
            25266..25267 '1': integer
            25274..25278 'shot': ptr<storage, ShotData, read_write>
            25274..25288 'shot.rng_state': ref<storage, xorwow_state, read_write>
            25274..25290 'shot.r...tate.x': ref<storage, array<u32, 5>, read_write>
            25274..25293 'shot.r...e.x[2]': ref<storage, u32, read_write>
            25291..25292 '2': integer
            25296..25304 'rng_seed': u32
            25296..25328 'rng_se...d + 2)': u32
            25307..25328 'hash_p...d + 2)': u32
            25316..25323 'shot_id': u32
            25316..25327 'shot_id + 2': u32
            25326..25327 '2': integer
            25334..25338 'shot': ptr<storage, ShotData, read_write>
            25334..25348 'shot.rng_state': ref<storage, xorwow_state, read_write>
            25334..25350 'shot.r...tate.x': ref<storage, array<u32, 5>, read_write>
            25334..25353 'shot.r...e.x[3]': ref<storage, u32, read_write>
            25351..25352 '3': integer
            25356..25364 'rng_seed': u32
            25356..25388 'rng_se...d + 3)': u32
            25367..25388 'hash_p...d + 3)': u32
            25376..25383 'shot_id': u32
            25376..25387 'shot_id + 3': u32
            25386..25387 '3': integer
            25394..25398 'shot': ptr<storage, ShotData, read_write>
            25394..25408 'shot.rng_state': ref<storage, xorwow_state, read_write>
            25394..25410 'shot.r...tate.x': ref<storage, array<u32, 5>, read_write>
            25394..25413 'shot.r...e.x[4]': ref<storage, u32, read_write>
            25411..25412 '4': integer
            25416..25424 'rng_seed': u32
            25416..25448 'rng_se...d + 4)': u32
            25427..25448 'hash_p...d + 4)': u32
            25436..25443 'shot_id': u32
            25436..25447 'shot_id + 4': u32
            25446..25447 '4': integer
            25455..25459 'shot': ptr<storage, ShotData, read_write>
            25455..25467 'shot.op_type': ref<storage, u32, read_write>
            25470..25471 '0': integer
            25477..25481 'shot': ptr<storage, ShotData, read_write>
            25477..25488 'shot.op_idx': ref<storage, u32, read_write>
            25491..25492 '0': integer
            25581..25585 'shot': ptr<storage, ShotData, read_write>
            25581..25594 'shot.duration': ref<storage, f32, read_write>
            25597..25600 '0.0': float
            25606..25610 'shot': ptr<storage, ShotData, read_write>
            25606..25622 'shot.r...malize': ref<storage, f32, read_write>
            25625..25628 '1.0': float
            25635..25639 'shot': ptr<storage, ShotData, read_write>
            25635..25655 'shot.q...0_mask': ref<storage, u32, read_write>
            25658..25687 '(1u <<...) - 1u': u32
            25659..25661 '1u': u32
            25659..25681 '1u << ...COUNT)': u32
            25665..25681 'u32(QU...COUNT)': u32
            25669..25680 'QUBIT_COUNT': i32
            25685..25687 '1u': u32
            25715..25719 'shot': ptr<storage, ShotData, read_write>
            25715..25735 'shot.q...1_mask': ref<storage, u32, read_write>
            25738..25740 '0u': u32
            25746..25750 'shot': ptr<storage, ShotData, read_write>
            25746..25778 'shot.q...p_mask': ref<storage, u32, read_write>
            25781..25782 '0': integer
            25788..25792 'shot': ptr<storage, ShotData, read_write>
            25788..25810 'shot.p...s_mask': ref<storage, u32, read_write>
            25813..25815 '0u': u32
            25885..25886 'i': ref<function, i32, read_write>
            25894..25895 '0': integer
            25897..25898 'i': ref<function, i32, read_write>
            25897..25912 'i < QUBIT_COUNT': bool
            25901..25912 'QUBIT_COUNT': i32
            25914..25915 'i': ref<function, i32, read_write>
            25929..25933 'shot': ptr<storage, ShotData, read_write>
            25929..25945 'shot.q..._state': ref<storage, [error], read_write>
            25929..25948 'shot.q...ate[i]': [error]
            25929..25965 'shot.q...bility': [error]
            25946..25947 'i': ref<function, i32, read_write>
            25968..25971 '1.0': float
            25981..25985 'shot': ptr<storage, ShotData, read_write>
            25981..25997 'shot.q..._state': ref<storage, [error], read_write>
            25981..26000 'shot.q...ate[i]': [error]
            25981..26016 'shot.q...bility': [error]
            25998..25999 'i': ref<function, i32, read_write>
            26019..26022 '0.0': float
            26032..26036 'shot': ptr<storage, ShotData, read_write>
            26032..26048 'shot.q..._state': ref<storage, [error], read_write>
            26032..26051 'shot.q...ate[i]': [error]
            26032..26056 'shot.q...].heat': [error]
            26049..26050 'i': ref<function, i32, read_write>
            26059..26062 '0.0': float
            26072..26076 'shot': ptr<storage, ShotData, read_write>
            26072..26088 'shot.q..._state': ref<storage, [error], read_write>
            26072..26091 'shot.q...ate[i]': [error]
            26072..26102 'shot.q..._since': [error]
            26089..26090 'i': ref<function, i32, read_write>
            26105..26108 '0.0': float
            25929..25948 'shot.q...ate[i]': cannot index into type ref<storage, [error], read_write>
            25929..25965 'shot.q...bility': cannot assign to non-reference `[error]`
            25981..26000 'shot.q...ate[i]': cannot index into type ref<storage, [error], read_write>
            25981..26016 'shot.q...bility': cannot assign to non-reference `[error]`
            26032..26051 'shot.q...ate[i]': cannot index into type ref<storage, [error], read_write>
            26032..26056 'shot.q...].heat': cannot assign to non-reference `[error]`
            26072..26091 'shot.q...ate[i]': cannot index into type ref<storage, [error], read_write>
            26072..26102 'shot.q..._since': cannot assign to non-reference `[error]`
            26235..26243 'shot_idx': u32
            26260..26264 'shot': ptr<storage, ShotData, read_write>
            26267..26283 '&shots...t_idx]': ptr<storage, ShotData, read_write>
            26268..26273 'shots': ref<storage, array<ShotData>, read_write>
            26268..26283 'shots[shot_idx]': ref<storage, ShotData, read_write>
            26274..26282 'shot_idx': u32
            26660..26661 'q': ref<function, u32, read_write>
            26669..26671 '0u': u32
            26673..26674 'q': ref<function, u32, read_write>
            26673..26693 'q < u3...COUNT)': bool
            26677..26693 'u32(QU...COUNT)': u32
            26681..26692 'QUBIT_COUNT': i32
            26695..26696 'q': ref<function, u32, read_write>
            26714..26724 'qubit_mask': u32
            26732..26734 '1u': u32
            26732..26739 '1u << q': u32
            26738..26739 'q': ref<function, u32, read_write>
            26753..26806 '(shot.... != 0u': bool
            26754..26758 'shot': ptr<storage, ShotData, read_write>
            26754..26786 'shot.q...p_mask': ref<storage, u32, read_write>
            26754..26799 'shot.q...t_mask': u32
            26789..26799 'qubit_mask': u32
            26804..26806 '0u': u32
            27087..27097 'total_zero': ref<function, f32, read_write>
            27105..27108 '0.0': float
            27126..27135 'total_one': ref<function, f32, read_write>
            27143..27146 '0.0': float
            27165..27184 'WORKGR...R_SHOT': i32
            27165..27188 'WORKGR...OT > 1': bool
            27187..27188 '1': integer
            27290..27296 'offset': u32
            27299..27307 'shot_idx': u32
            27299..27334 'shot_i..._SHOT)': u32
            27310..27334 'u32(WO..._SHOT)': u32
            27314..27333 'WORKGR...R_SHOT': i32
            27361..27368 'wkg_idx': ref<function, u32, read_write>
            27376..27378 '0u': u32
            27380..27387 'wkg_idx': ref<function, u32, read_write>
            27380..27414 'wkg_id..._SHOT)': bool
            27390..27414 'u32(WO..._SHOT)': u32
            27394..27413 'WORKGR...R_SHOT': i32
            27416..27423 'wkg_idx': ref<function, u32, read_write>
            27453..27457 'sums': [error]
            27460..27479 'workgr...lation': ref<storage, WorkgroupCollationBuffer, read_write>
            27460..27484 'workgr...n.sums': ref<storage, [error], read_write>
            27460..27502 'workgr...ffset]': [error]
            27485..27492 'wkg_idx': ref<function, u32, read_write>
            27485..27501 'wkg_id...offset': u32
            27495..27501 'offset': u32
            27524..27534 'total_zero': ref<function, f32, read_write>
            27537..27547 'total_zero': ref<function, f32, read_write>
            27537..27566 'total_...s[q].x': [error]
            27550..27554 'sums': [error]
            27550..27561 'sums.qubits': [error]
            27550..27564 'sums.qubits[q]': [error]
            27550..27566 'sums.q...s[q].x': [error]
            27562..27563 'q': ref<function, u32, read_write>
            27588..27597 'total_one': ref<function, f32, read_write>
            27600..27609 'total_one': ref<function, f32, read_write>
            27600..27628 'total_...s[q].y': [error]
            27612..27616 'sums': [error]
            27612..27623 'sums.qubits': [error]
            27612..27626 'sums.qubits[q]': [error]
            27612..27628 'sums.q...s[q].y': [error]
            27624..27625 'q': ref<function, u32, read_write>
            27770..27780 'total_zero': ref<function, f32, read_write>
            27783..27787 'shot': ptr<storage, ShotData, read_write>
            27783..27799 'shot.q..._state': ref<storage, [error], read_write>
            27783..27802 'shot.q...ate[q]': [error]
            27783..27819 'shot.q...bility': [error]
            27800..27801 'q': ref<function, u32, read_write>
            27837..27846 'total_one': ref<function, f32, read_write>
            27849..27853 'shot': ptr<storage, ShotData, read_write>
            27849..27865 'shot.q..._state': ref<storage, [error], read_write>
            27849..27868 'shot.q...ate[q]': [error]
            27849..27884 'shot.q...bility': [error]
            27866..27867 'q': ref<function, u32, read_write>
            28129..28139 'total_zero': ref<function, f32, read_write>
            28129..28150 'total_...000001': bool
            28142..28150 '0.000001': float
            28154..28164 'total_zero': ref<function, f32, read_write>
            28167..28170 '0.0': float
            28190..28199 'total_one': ref<function, f32, read_write>
            28190..28210 'total_...000001': bool
            28202..28210 '0.000001': float
            28214..28223 'total_one': ref<function, f32, read_write>
            28226..28229 '0.0': float
            28249..28259 'total_zero': ref<function, f32, read_write>
            28249..28270 'total_...999999': bool
            28262..28270 '0.999999': float
            28274..28284 'total_zero': ref<function, f32, read_write>
            28287..28290 '1.0': float
            28310..28319 'total_one': ref<function, f32, read_write>
            28310..28330 'total_...999999': bool
            28322..28330 '0.999999': float
            28334..28343 'total_one': ref<function, f32, read_write>
            28346..28349 '1.0': float
            28366..28370 'shot': ptr<storage, ShotData, read_write>
            28366..28382 'shot.q..._state': ref<storage, [error], read_write>
            28366..28385 'shot.q...ate[q]': [error]
            28366..28402 'shot.q...bility': [error]
            28383..28384 'q': ref<function, u32, read_write>
            28405..28415 'total_zero': ref<function, f32, read_write>
            28429..28433 'shot': ptr<storage, ShotData, read_write>
            28429..28445 'shot.q..._state': ref<storage, [error], read_write>
            28429..28448 'shot.q...ate[q]': [error]
            28429..28464 'shot.q...bility': [error]
            28446..28447 'q': ref<function, u32, read_write>
            28467..28476 'total_one': ref<function, f32, read_write>
            28711..28727 'within...eshold': bool
            28730..28765 'abs(1...._one))': f32
            28730..28782 'abs(1....ESHOLD': bool
            28734..28737 '1.0': float
            28734..28764 '1.0 - ...l_one)': f32
            28741..28751 'total_zero': ref<function, f32, read_write>
            28741..28763 'total_...al_one': f32
            28754..28763 'total_one': ref<function, f32, read_write>
            28768..28782 'PROB_THRESHOLD': f32
            28799..28816 '!withi...eshold': bool
            28800..28816 'within...eshold': bool
            28910..28919 'old_value': __atomic_compare_exchange_result
            28922..29056 'atomic...PROBS)': __atomic_compare_exchange_result
            28969..28992 '&diagn...r_code': ptr<storage, atomic<u32>, read_write>
            28970..28981 'diagnostics': ref<storage, DiagnosticData, read_write>
            28970..28992 'diagno...r_code': ref<storage, atomic<u32>, read_write>
            29014..29016 '0u': u32
            29038..29055 'ERR_IN..._PROBS': u32
            29077..29086 'old_value': __atomic_compare_exchange_result
            29077..29096 'old_va...hanged': bool
            29188..29199 'diagnostics': ref<storage, DiagnosticData, read_write>
            29188..29206 'diagno...extra1': ref<storage, u32, read_write>
            29209..29210 'q': ref<function, u32, read_write>
            29232..29243 'diagnostics': ref<storage, DiagnosticData, read_write>
            29232..29250 'diagno...extra2': ref<storage, f32, read_write>
            29253..29263 'total_zero': ref<function, f32, read_write>
            29285..29296 'diagnostics': ref<storage, DiagnosticData, read_write>
            29285..29303 'diagno...extra3': ref<storage, f32, read_write>
            29306..29315 'total_one': ref<function, f32, read_write>
            29490..29501 'diagnostics': ref<storage, DiagnosticData, read_write>
            29490..29506 'diagno...s.shot': ref<storage, ShotData, read_write>
            29509..29514 '*shot': ref<storage, ShotData, read_write>
            29510..29514 'shot': ptr<storage, ShotData, read_write>
            29536..29547 'diagnostics': ref<storage, DiagnosticData, read_write>
            29536..29550 'diagnostics.op': ref<storage, Op, read_write>
            29553..29556 'ops': ref<storage, array<Op>, read>
            29553..29569 'ops[sh...p_idx]': ref<storage, Op, read>
            29557..29561 'shot': ptr<storage, ShotData, read_write>
            29557..29568 'shot.op_idx': ref<storage, u32, read_write>
            29710..29719 'err_index': u32
            29722..29751 '(shot_..._COUNT': u32
            29722..29755 '(shot_...NT - 1': u32
            29723..29731 'shot_idx': u32
            29723..29735 'shot_idx + 1': u32
            29734..29735 '1': integer
            29739..29751 'RESULT_COUNT': u32
            29754..29755 '1': integer
            29773..29903 'atomic...PROBS)': __atomic_compare_exchange_result
            29820..29839 '&resul...index]': ptr<storage, atomic<u32>, read_write>
            29821..29828 'results': ref<storage, array<atomic<u32>>, read_write>
            29821..29839 'result...index]': ref<storage, atomic<u32>, read_write>
            29829..29838 'err_index': u32
            29861..29863 '0u': u32
            29885..29902 'ERR_IN..._PROBS': u32
            29984..29988 'shot': ptr<storage, ShotData, read_write>
            29984..30004 'shot.q...0_mask': ref<storage, u32, read_write>
            30007..30152 'select...= 1.0)': u32
            30031..30035 'shot': ptr<storage, ShotData, read_write>
            30031..30051 'shot.q...0_mask': ref<storage, u32, read_write>
            30031..30065 'shot.q...t_mask': u32
            30054..30065 '~qubit_mask': u32
            30055..30065 'qubit_mask': u32
            30083..30087 'shot': ptr<storage, ShotData, read_write>
            30083..30103 'shot.q...0_mask': ref<storage, u32, read_write>
            30083..30116 'shot.q...t_mask': u32
            30106..30116 'qubit_mask': u32
            30134..30144 'total_zero': ref<function, f32, read_write>
            30134..30151 'total_...== 1.0': bool
            30148..30151 '1.0': float
            30166..30170 'shot': ptr<storage, ShotData, read_write>
            30166..30186 'shot.q...1_mask': ref<storage, u32, read_write>
            30189..30333 'select...= 1.0)': u32
            30213..30217 'shot': ptr<storage, ShotData, read_write>
            30213..30233 'shot.q...1_mask': ref<storage, u32, read_write>
            30213..30247 'shot.q...t_mask': u32
            30236..30247 '~qubit_mask': u32
            30237..30247 'qubit_mask': u32
            30265..30269 'shot': ptr<storage, ShotData, read_write>
            30265..30285 'shot.q...1_mask': ref<storage, u32, read_write>
            30265..30298 'shot.q...t_mask': u32
            30288..30298 'qubit_mask': u32
            30316..30325 'total_one': ref<function, f32, read_write>
            30316..30332 'total_...== 1.0': bool
            30329..30332 '1.0': float
            27460..27502 'workgr...ffset]': cannot index into type ref<storage, [error], read_write>
            27783..27802 'shot.q...ate[q]': cannot index into type ref<storage, [error], read_write>
            27849..27868 'shot.q...ate[q]': cannot index into type ref<storage, [error], read_write>
            28366..28385 'shot.q...ate[q]': cannot index into type ref<storage, [error], read_write>
            28366..28402 'shot.q...bility': cannot assign to non-reference `[error]`
            28429..28448 'shot.q...ate[q]': cannot index into type ref<storage, [error], read_write>
            28429..28464 'shot.q...bility': cannot assign to non-reference `[error]`
            30488..30504 'stateV...rIndex': u32
            30511..30520 'amplitude': vec2<f32>
            30529..30532 'tid': u32
            30549..30553 'mask': ref<function, u32, read_write>
            30561..30563 '1u': u32
            30578..30579 'q': ref<function, u32, read_write>
            30587..30589 '0u': u32
            30591..30592 'q': ref<function, u32, read_write>
            30591..30611 'q < u3...COUNT)': bool
            30595..30611 'u32(QU...COUNT)': u32
            30599..30610 'QUBIT_COUNT': i32
            30613..30614 'q': ref<function, u32, read_write>
            30632..30638 'is_one': bool
            30647..30678 '(state... != 0u': bool
            30648..30664 'stateV...rIndex': u32
            30648..30671 'stateV...& mask': u32
            30667..30671 'mask': ref<function, u32, read_write>
            30676..30678 '0u': u32
            30692..30696 'prob': f32
            30704..30723 'cplxMa...itude)': f32
            30713..30722 'amplitude': vec2<f32>
            30737..30743 'is_one': bool
            30759..30777 'qubitP...lities': ref<workgroup, [error], read_write>
            30759..30782 'qubitP...s[tid]': [error]
            30759..30786 'qubitP...d].one': [error]
            30759..30789 'qubitP...one[q]': [error]
            30778..30781 'tid': u32
            30787..30788 'q': ref<function, u32, read_write>
            30793..30797 'prob': f32
            30828..30846 'qubitP...lities': ref<workgroup, [error], read_write>
            30828..30851 'qubitP...s[tid]': [error]
            30828..30856 'qubitP...].zero': [error]
            30828..30859 'qubitP...ero[q]': [error]
            30847..30850 'tid': u32
            30857..30858 'q': ref<function, u32, read_write>
            30863..30867 'prob': f32
            30887..30891 'mask': ref<function, u32, read_write>
            30894..30898 'mask': ref<function, u32, read_write>
            30894..30904 'mask << 1u': u32
            30902..30904 '1u': u32
            30759..30782 'qubitP...s[tid]': cannot index into type ref<workgroup, [error], read_write>
            30759..30789 'qubitP...one[q]': cannot assign to non-reference `[error]`
            30759..30782 'qubitP...s[tid]': cannot index into type ref<workgroup, [error], read_write>
            30828..30851 'qubitP...s[tid]': cannot index into type ref<workgroup, [error], read_write>
            30828..30859 'qubitP...ero[q]': cannot assign to non-reference `[error]`
            30828..30851 'qubitP...s[tid]': cannot index into type ref<workgroup, [error], read_write>
            30944..30945 'q': u32
            30952..30960 'shot_idx': i32
            30967..30984 'wkg_co...on_idx': i32
            31001..31011 'total_zero': ref<function, f32, read_write>
            31019..31022 '0.0': float
            31032..31041 'total_one': ref<function, f32, read_write>
            31049..31052 '0.0': float
            31067..31068 'j': ref<function, i32, read_write>
            31071..31072 '0': integer
            31074..31075 'j': ref<function, i32, read_write>
            31074..31099 'j < TH...KGROUP': bool
            31078..31099 'THREAD...KGROUP': i32
            31101..31102 'j': ref<function, i32, read_write>
            31116..31126 'total_zero': ref<function, f32, read_write>
            31130..31148 'qubitP...lities': ref<workgroup, [error], read_write>
            31130..31151 'qubitP...ies[j]': [error]
            31130..31156 'qubitP...].zero': [error]
            31130..31159 'qubitP...ero[q]': [error]
            31149..31150 'j': ref<function, i32, read_write>
            31157..31158 'q': u32
            31169..31178 'total_one': ref<function, f32, read_write>
            31182..31200 'qubitP...lities': ref<workgroup, [error], read_write>
            31182..31203 'qubitP...ies[j]': [error]
            31182..31207 'qubitP...j].one': [error]
            31182..31210 'qubitP...one[q]': [error]
            31201..31202 'j': ref<function, i32, read_write>
            31208..31209 'q': u32
            31226..31243 'wkg_co...on_idx': i32
            31226..31248 'wkg_co...x >= 0': bool
            31247..31248 '0': integer
            31351..31370 'workgr...lation': ref<storage, WorkgroupCollationBuffer, read_write>
            31351..31375 'workgr...n.sums': ref<storage, [error], read_write>
            31351..31394 'workgr...n_idx]': [error]
            31351..31401 'workgr...qubits': [error]
            31351..31404 'workgr...its[q]': [error]
            31376..31393 'wkg_co...on_idx': i32
            31402..31403 'q': u32
            31407..31435 'vec2f(...l_one)': vec2<f32>
            31413..31423 'total_zero': ref<function, f32, read_write>
            31425..31434 'total_one': ref<function, f32, read_write>
            31539..31555 'within...eshold': bool
            31558..31593 'abs(1...._one))': f32
            31558..31610 'abs(1....ESHOLD': bool
            31562..31565 '1.0': float
            31562..31592 '1.0 - ...l_one)': f32
            31569..31579 'total_zero': ref<function, f32, read_write>
            31569..31591 'total_...al_one': f32
            31582..31591 'total_one': ref<function, f32, read_write>
            31596..31610 'PROB_THRESHOLD': f32
            31623..31640 '!withi...eshold': bool
            31624..31640 'within...eshold': bool
            31726..31735 'old_value': __atomic_compare_exchange_result
            31738..31867 'atomic...TOTAL)': __atomic_compare_exchange_result
            31781..31804 '&diagn...r_code': ptr<storage, atomic<u32>, read_write>
            31782..31793 'diagnostics': ref<storage, DiagnosticData, read_write>
            31782..31804 'diagno...r_code': ref<storage, atomic<u32>, read_write>
            31822..31824 '0u': u32
            31842..31866 'ERR_IN..._TOTAL': u32
            31884..31893 'old_value': __atomic_compare_exchange_result
            31884..31903 'old_va...hanged': bool
            31991..31995 'shot': ptr<storage, ShotData, read_write>
            31998..32014 '&shots...t_idx]': ptr<storage, ShotData, read_write>
            31999..32004 'shots': ref<storage, array<ShotData>, read_write>
            31999..32014 'shots[shot_idx]': ref<storage, ShotData, read_write>
            32005..32013 'shot_idx': i32
            32032..32043 'diagnostics': ref<storage, DiagnosticData, read_write>
            32032..32050 'diagno...extra1': ref<storage, u32, read_write>
            32053..32054 'q': u32
            32072..32083 'diagnostics': ref<storage, DiagnosticData, read_write>
            32072..32090 'diagno...extra2': ref<storage, f32, read_write>
            32093..32103 'total_zero': ref<function, f32, read_write>
            32121..32132 'diagnostics': ref<storage, DiagnosticData, read_write>
            32121..32139 'diagno...extra3': ref<storage, f32, read_write>
            32142..32151 'total_one': ref<function, f32, read_write>
            32312..32323 'diagnostics': ref<storage, DiagnosticData, read_write>
            32312..32328 'diagno...s.shot': ref<storage, ShotData, read_write>
            32331..32336 '*shot': ref<storage, ShotData, read_write>
            32332..32336 'shot': ptr<storage, ShotData, read_write>
            32354..32365 'diagnostics': ref<storage, DiagnosticData, read_write>
            32354..32368 'diagnostics.op': ref<storage, Op, read_write>
            32371..32374 'ops': ref<storage, array<Op>, read>
            32371..32387 'ops[sh...p_idx]': ref<storage, Op, read>
            32375..32379 'shot': ptr<storage, ShotData, read_write>
            32375..32386 'shot.op_idx': ref<storage, u32, read_write>
            32453..32462 'err_index': i32
            32465..32499 '(shot_...COUNT)': i32
            32465..32503 '(shot_...T) - 1': i32
            32466..32474 'shot_idx': i32
            32466..32478 'shot_idx + 1': i32
            32477..32478 '1': integer
            32482..32499 'i32(RE...COUNT)': i32
            32486..32498 'RESULT_COUNT': u32
            32502..32503 '1': integer
            32517..32654 'atomic...TOTAL)': __atomic_compare_exchange_result
            32564..32583 '&resul...index]': ptr<storage, atomic<u32>, read_write>
            32565..32572 'results': ref<storage, array<atomic<u32>>, read_write>
            32565..32583 'result...index]': ref<storage, atomic<u32>, read_write>
            32573..32582 'err_index': i32
            32605..32607 '0u': u32
            32629..32653 'ERR_IN..._TOTAL': u32
            32685..32690 'shots': ref<storage, array<ShotData>, read_write>
            32685..32700 'shots[shot_idx]': ref<storage, ShotData, read_write>
            32685..32712 'shots[..._state': ref<storage, [error], read_write>
            32685..32715 'shots[...ate[q]': [error]
            32685..32732 'shots[...bility': [error]
            32691..32699 'shot_idx': i32
            32713..32714 'q': u32
            32735..32745 'total_zero': ref<function, f32, read_write>
            32759..32764 'shots': ref<storage, array<ShotData>, read_write>
            32759..32774 'shots[shot_idx]': ref<storage, ShotData, read_write>
            32759..32786 'shots[..._state': ref<storage, [error], read_write>
            32759..32789 'shots[...ate[q]': [error]
            32759..32805 'shots[...bility': [error]
            32765..32773 'shot_idx': i32
            32787..32788 'q': u32
            32808..32817 'total_one': ref<function, f32, read_write>
            31130..31151 'qubitP...ies[j]': cannot index into type ref<workgroup, [error], read_write>
            31130..31159 'qubitP...ero[q]': expected f32 but got [error]
            31182..31203 'qubitP...ies[j]': cannot index into type ref<workgroup, [error], read_write>
            31182..31210 'qubitP...one[q]': expected f32 but got [error]
            31351..31394 'workgr...n_idx]': cannot index into type ref<storage, [error], read_write>
            31351..31404 'workgr...its[q]': cannot assign to non-reference `[error]`
            32685..32715 'shots[...ate[q]': cannot index into type ref<storage, [error], read_write>
            32685..32732 'shots[...bility': cannot assign to non-reference `[error]`
            32759..32789 'shots[...ate[q]': cannot index into type ref<storage, [error], read_write>
            32759..32805 'shots[...bility': cannot assign to non-reference `[error]`
            33298..33306 'shot_idx': u32
            33313..33318 'qubit': u32
            33325..33331 'result': u32
            33338..33352 'resets_to_zero': bool
            33370..33374 'shot': ptr<storage, ShotData, read_write>
            33377..33393 '&shots...t_idx]': ptr<storage, ShotData, read_write>
            33378..33383 'shots': ref<storage, array<ShotData>, read_write>
            33378..33393 'shots[shot_idx]': ref<storage, ShotData, read_write>
            33384..33392 'shot_idx': u32
            33563..33577 'resets_to_zero': bool
            33777..33781 'shot': ptr<storage, ShotData, read_write>
            33777..33789 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            33777..33792 'shot.unitary[0]': ref<storage, vec2<f32>, read_write>
            33790..33791 '0': integer
            33795..33849 'select...== 1u)': vec2<f32>
            33802..33817 'vec2f(1.0, 0.0)': vec2<f32>
            33808..33811 '1.0': float
            33813..33816 '0.0': float
            33819..33834 'vec2f(0.0, 0.0)': vec2<f32>
            33825..33828 '0.0': float
            33830..33833 '0.0': float
            33836..33842 'result': u32
            33836..33848 'result == 1u': bool
            33846..33848 '1u': u32
            33859..33863 'shot': ptr<storage, ShotData, read_write>
            33859..33871 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            33859..33874 'shot.unitary[1]': ref<storage, vec2<f32>, read_write>
            33872..33873 '1': integer
            33877..33931 'select...== 1u)': vec2<f32>
            33884..33899 'vec2f(0.0, 0.0)': vec2<f32>
            33890..33893 '0.0': float
            33895..33898 '0.0': float
            33901..33916 'vec2f(1.0, 0.0)': vec2<f32>
            33907..33910 '1.0': float
            33912..33915 '0.0': float
            33918..33924 'result': u32
            33918..33930 'result == 1u': bool
            33928..33930 '1u': u32
            33941..33945 'shot': ptr<storage, ShotData, read_write>
            33941..33953 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            33941..33956 'shot.unitary[4]': ref<storage, vec2<f32>, read_write>
            33954..33955 '4': integer
            33959..33966 'vec2f()': vec2<f32>
            33976..33980 'shot': ptr<storage, ShotData, read_write>
            33976..33988 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            33976..33991 'shot.unitary[5]': ref<storage, vec2<f32>, read_write>
            33989..33990 '5': integer
            33994..34001 'vec2f()': vec2<f32>
            34182..34186 'shot': ptr<storage, ShotData, read_write>
            34182..34194 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            34182..34197 'shot.unitary[0]': ref<storage, vec2<f32>, read_write>
            34195..34196 '0': integer
            34200..34254 'select...== 1u)': vec2<f32>
            34207..34222 'vec2f(1.0, 0.0)': vec2<f32>
            34213..34216 '1.0': float
            34218..34221 '0.0': float
            34224..34239 'vec2f(0.0, 0.0)': vec2<f32>
            34230..34233 '0.0': float
            34235..34238 '0.0': float
            34241..34247 'result': u32
            34241..34253 'result == 1u': bool
            34251..34253 '1u': u32
            34264..34268 'shot': ptr<storage, ShotData, read_write>
            34264..34276 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            34264..34279 'shot.unitary[1]': ref<storage, vec2<f32>, read_write>
            34277..34278 '1': integer
            34282..34289 'vec2f()': vec2<f32>
            34299..34303 'shot': ptr<storage, ShotData, read_write>
            34299..34311 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            34299..34314 'shot.unitary[4]': ref<storage, vec2<f32>, read_write>
            34312..34313 '4': integer
            34317..34324 'vec2f()': vec2<f32>
            34334..34338 'shot': ptr<storage, ShotData, read_write>
            34334..34346 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            34334..34349 'shot.unitary[5]': ref<storage, vec2<f32>, read_write>
            34347..34348 '5': integer
            34352..34406 'select...== 1u)': vec2<f32>
            34359..34374 'vec2f(0.0, 0.0)': vec2<f32>
            34365..34368 '0.0': float
            34370..34373 '0.0': float
            34376..34391 'vec2f(1.0, 0.0)': vec2<f32>
            34382..34385 '1.0': float
            34387..34390 '0.0': float
            34393..34399 'result': u32
            34393..34405 'result == 1u': bool
            34403..34405 '1u': u32
            34419..34423 'shot': ptr<storage, ShotData, read_write>
            34419..34435 'shot.r...malize': ref<storage, f32, read_write>
            34438..34590 'select...== 1u)': [error]
            34454..34457 '1.0': float
            34454..34506 '1.0 / ...ility)': [error]
            34460..34506 'sqrt(s...ility)': [error]
            34465..34469 'shot': ptr<storage, ShotData, read_write>
            34465..34481 'shot.q..._state': ref<storage, [error], read_write>
            34465..34488 'shot.q...qubit]': [error]
            34465..34505 'shot.q...bility': [error]
            34482..34487 'qubit': u32
            34516..34519 '1.0': float
            34516..34567 '1.0 / ...ility)': [error]
            34522..34567 'sqrt(s...ility)': [error]
            34527..34531 'shot': ptr<storage, ShotData, read_write>
            34527..34543 'shot.q..._state': ref<storage, [error], read_write>
            34527..34550 'shot.q...qubit]': [error]
            34527..34566 'shot.q...bility': [error]
            34544..34549 'qubit': u32
            34577..34583 'result': u32
            34577..34589 'result == 1u': bool
            34587..34589 '1u': u32
            34713..34717 'shot': ptr<storage, ShotData, read_write>
            34713..34733 'shot.q...1_mask': ref<storage, u32, read_write>
            34736..34740 'shot': ptr<storage, ShotData, read_write>
            34736..34756 'shot.q...1_mask': ref<storage, u32, read_write>
            34736..34773 'shot.q...qubit)': u32
            34759..34773 '~(1u << qubit)': u32
            34761..34763 '1u': u32
            34761..34772 '1u << qubit': u32
            34767..34772 'qubit': u32
            34779..34783 'shot': ptr<storage, ShotData, read_write>
            34779..34799 'shot.q...0_mask': ref<storage, u32, read_write>
            34802..34806 'shot': ptr<storage, ShotData, read_write>
            34802..34822 'shot.q...0_mask': ref<storage, u32, read_write>
            34802..34839 'shot.q...qubit)': u32
            34825..34839 '~(1u << qubit)': u32
            34827..34829 '1u': u32
            34827..34838 '1u << qubit': u32
            34833..34838 'qubit': u32
            35126..35130 'shot': ptr<storage, ShotData, read_write>
            35126..35158 'shot.q...p_mask': ref<storage, u32, read_write>
            35207..35352 '((1u <..._mask)': u32
            35208..35237 '(1u <<...) - 1u': u32
            35209..35211 '1u': u32
            35209..35231 '1u << ...COUNT)': u32
            35215..35231 'u32(QU...COUNT)': u32
            35219..35230 'QUBIT_COUNT': i32
            35235..35237 '1u': u32
            35306..35352 '~(shot..._mask)': u32
            35308..35312 'shot': ptr<storage, ShotData, read_write>
            35308..35328 'shot.q...0_mask': ref<storage, u32, read_write>
            35308..35351 'shot.q...1_mask': u32
            35331..35335 'shot': ptr<storage, ShotData, read_write>
            35331..35351 'shot.q...1_mask': ref<storage, u32, read_write>
            34465..34488 'shot.q...qubit]': cannot index into type ref<storage, [error], read_write>
            34527..34550 'shot.q...qubit]': cannot index into type ref<storage, [error], read_write>
            35605..35613 'shot_idx': u32
            35620..35626 'op_idx': u32
            35633..35638 'qubit': u32
            35645..35654 'result_id': u32
            35661..35668 'is_loss': bool
            35676..35689 'stores_result': bool
            35697..35711 'resets_to_zero': bool
            35729..35733 'shot': ptr<storage, ShotData, read_write>
            35736..35752 '&shots...t_idx]': ptr<storage, ShotData, read_write>
            35737..35742 'shots': ref<storage, array<ShotData>, read_write>
            35737..35752 'shots[shot_idx]': ref<storage, ShotData, read_write>
            35743..35751 'shot_idx': u32
            35843..35849 'result': [error]
            35852..35928 'select...ility)': [error]
            35859..35861 '1u': u32
            35863..35865 '0u': u32
            35867..35871 'shot': ptr<storage, ShotData, read_write>
            35867..35884 'shot.r...easure': ref<storage, f32, read_write>
            35867..35927 'shot.r...bility': [error]
            35887..35891 'shot': ptr<storage, ShotData, read_write>
            35887..35903 'shot.q..._state': ref<storage, [error], read_write>
            35887..35910 'shot.q...qubit]': [error]
            35887..35927 'shot.q...bility': [error]
            35904..35909 'qubit': u32
            36108..36116 '!is_loss': bool
            36109..36116 'is_loss': bool
            36130..36143 'stores_result': bool
            36364..36368 'shot': ptr<storage, ShotData, read_write>
            36364..36380 'shot.q..._state': ref<storage, [error], read_write>
            36364..36387 'shot.q...qubit]': [error]
            36364..36392 'shot.q...].heat': [error]
            36364..36400 'shot.q...= -1.0': [error]
            36381..36386 'qubit': u32
            36396..36400 '-1.0': float
            36397..36400 '1.0': float
            36419..36483 'atomic...], 2u)': [error]
            36431..36478 '&resul...lt_id]': ptr<storage, atomic<u32>, read_write>
            36432..36439 'results': ref<storage, array<atomic<u32>>, read_write>
            36432..36478 'result...lt_id]': ref<storage, atomic<u32>, read_write>
            36440..36477 '(shot_...ult_id': u32
            36441..36449 'shot_idx': u32
            36441..36464 'shot_i..._COUNT': u32
            36452..36464 'RESULT_COUNT': u32
            36468..36477 'result_id': u32
            36480..36482 '2u': u32
            36501..36505 'shot': ptr<storage, ShotData, read_write>
            36501..36513 'shot.op_type': ref<storage, u32, read_write>
            36516..36523 'OPID_ID': u32
            36541..36545 'shot': ptr<storage, ShotData, read_write>
            36541..36552 'shot.op_idx': ref<storage, u32, read_write>
            36555..36561 'op_idx': u32
            36666..36670 'shot': ptr<storage, ShotData, read_write>
            36666..36682 'shot.q..._state': ref<storage, [error], read_write>
            36666..36689 'shot.q...qubit]': [error]
            36666..36694 'shot.q...].heat': [error]
            36683..36688 'qubit': u32
            36697..36700 '0.0': float
            36763..36831 'atomic...esult)': [error]
            36775..36822 '&resul...lt_id]': ptr<storage, atomic<u32>, read_write>
            36776..36783 'results': ref<storage, array<atomic<u32>>, read_write>
            36776..36822 'result...lt_id]': ref<storage, atomic<u32>, read_write>
            36784..36821 '(shot_...ult_id': u32
            36785..36793 'shot_idx': u32
            36785..36808 'shot_i..._COUNT': u32
            36796..36808 'RESULT_COUNT': u32
            36812..36821 'result_id': u32
            36824..36830 'result': [error]
            37045..37049 'shot': ptr<storage, ShotData, read_write>
            37045..37061 'shot.q..._state': ref<storage, [error], read_write>
            37045..37068 'shot.q...qubit]': [error]
            37045..37073 'shot.q...].heat': [error]
            37045..37081 'shot.q...= -1.0': [error]
            37062..37067 'qubit': u32
            37077..37081 '-1.0': float
            37078..37081 '1.0': float
            37100..37104 'shot': ptr<storage, ShotData, read_write>
            37100..37112 'shot.op_type': ref<storage, u32, read_write>
            37115..37122 'OPID_ID': u32
            37140..37144 'shot': ptr<storage, ShotData, read_write>
            37140..37151 'shot.op_idx': ref<storage, u32, read_write>
            37154..37160 'op_idx': u32
            37231..37235 'shot': ptr<storage, ShotData, read_write>
            37231..37247 'shot.q..._state': ref<storage, [error], read_write>
            37231..37254 'shot.q...qubit]': [error]
            37231..37259 'shot.q...].heat': [error]
            37248..37253 'qubit': u32
            37262..37266 '-1.0': float
            37263..37266 '1.0': float
            37279..37349 'prep_m..._zero)': [error]
            37309..37317 'shot_idx': u32
            37319..37324 'qubit': u32
            37326..37332 'result': [error]
            37334..37348 'resets_to_zero': bool
            37356..37360 'shot': ptr<storage, ShotData, read_write>
            37356..37367 'shot.op_idx': ref<storage, u32, read_write>
            37370..37376 'op_idx': u32
            37535..37539 'shot': ptr<storage, ShotData, read_write>
            37535..37547 'shot.op_type': ref<storage, u32, read_write>
            37550..37562 'OPID_MRESETZ': u32
            35887..35910 'shot.q...qubit]': cannot index into type ref<storage, [error], read_write>
            36666..36689 'shot.q...qubit]': cannot index into type ref<storage, [error], read_write>
            36666..36694 'shot.q...].heat': cannot assign to non-reference `[error]`
            36364..36387 'shot.q...qubit]': cannot index into type ref<storage, [error], read_write>
            37045..37068 'shot.q...qubit]': cannot index into type ref<storage, [error], read_write>
            37231..37254 'shot.q...qubit]': cannot index into type ref<storage, [error], read_write>
            37231..37259 'shot.q...].heat': cannot assign to non-reference `[error]`
            37326..37332 'result': expected u32 but got [error]
            37981..37989 'shot_idx': u32
            37996..38008 'target_is_q2': bool
            38042..38045 'm00': vec2<f32>
            38054..38057 'm01': vec2<f32>
            38066..38069 'm10': vec2<f32>
            38078..38081 'm11': vec2<f32>
            38100..38104 'shot': ptr<storage, ShotData, read_write>
            38107..38123 '&shots...t_idx]': ptr<storage, ShotData, read_write>
            38108..38113 'shots': ref<storage, array<ShotData>, read_write>
            38108..38123 'shots[shot_idx]': ref<storage, ShotData, read_write>
            38114..38122 'shot_idx': u32
            38171..38172 'i': ref<function, u32, read_write>
            38175..38177 '0u': u32
            38179..38180 'i': ref<function, u32, read_write>
            38179..38186 'i < 16u': bool
            38183..38186 '16u': u32
            38188..38189 'i': ref<function, u32, read_write>
            38203..38207 'shot': ptr<storage, ShotData, read_write>
            38203..38215 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            38203..38218 'shot.unitary[i]': ref<storage, vec2<f32>, read_write>
            38216..38217 'i': ref<function, u32, read_write>
            38221..38236 'vec2f(0.0, 0.0)': vec2<f32>
            38227..38230 '0.0': float
            38232..38235 '0.0': float
            38251..38263 'target_is_q2': bool
            38370..38374 'shot': ptr<storage, ShotData, read_write>
            38370..38382 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            38370..38385 'shot.unitary[0]': ref<storage, vec2<f32>, read_write>
            38383..38384 '0': integer
            38389..38392 'm00': vec2<f32>
            38394..38398 'shot': ptr<storage, ShotData, read_write>
            38394..38406 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            38394..38409 'shot.unitary[1]': ref<storage, vec2<f32>, read_write>
            38407..38408 '1': integer
            38413..38416 'm01': vec2<f32>
            38426..38430 'shot': ptr<storage, ShotData, read_write>
            38426..38438 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            38426..38441 'shot.unitary[4]': ref<storage, vec2<f32>, read_write>
            38439..38440 '4': integer
            38445..38448 'm10': vec2<f32>
            38450..38454 'shot': ptr<storage, ShotData, read_write>
            38450..38462 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            38450..38465 'shot.unitary[5]': ref<storage, vec2<f32>, read_write>
            38463..38464 '5': integer
            38469..38472 'm11': vec2<f32>
            38522..38526 'shot': ptr<storage, ShotData, read_write>
            38522..38534 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            38522..38538 'shot.u...ry[10]': ref<storage, vec2<f32>, read_write>
            38535..38537 '10': integer
            38541..38544 'm00': vec2<f32>
            38546..38550 'shot': ptr<storage, ShotData, read_write>
            38546..38558 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            38546..38562 'shot.u...ry[11]': ref<storage, vec2<f32>, read_write>
            38559..38561 '11': integer
            38565..38568 'm01': vec2<f32>
            38578..38582 'shot': ptr<storage, ShotData, read_write>
            38578..38590 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            38578..38594 'shot.u...ry[14]': ref<storage, vec2<f32>, read_write>
            38591..38593 '14': integer
            38597..38600 'm10': vec2<f32>
            38602..38606 'shot': ptr<storage, ShotData, read_write>
            38602..38614 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            38602..38618 'shot.u...ry[15]': ref<storage, vec2<f32>, read_write>
            38615..38617 '15': integer
            38621..38624 'm11': vec2<f32>
            38690..38694 'shot': ptr<storage, ShotData, read_write>
            38690..38702 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            38690..38705 'shot.unitary[0]': ref<storage, vec2<f32>, read_write>
            38703..38704 '0': integer
            38709..38712 'm00': vec2<f32>
            38714..38718 'shot': ptr<storage, ShotData, read_write>
            38714..38726 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            38714..38729 'shot.unitary[2]': ref<storage, vec2<f32>, read_write>
            38727..38728 '2': integer
            38733..38736 'm01': vec2<f32>
            38746..38750 'shot': ptr<storage, ShotData, read_write>
            38746..38758 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            38746..38761 'shot.unitary[8]': ref<storage, vec2<f32>, read_write>
            38759..38760 '8': integer
            38765..38768 'm10': vec2<f32>
            38770..38774 'shot': ptr<storage, ShotData, read_write>
            38770..38782 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            38770..38786 'shot.u...ry[10]': ref<storage, vec2<f32>, read_write>
            38783..38785 '10': integer
            38789..38792 'm11': vec2<f32>
            38802..38806 'shot': ptr<storage, ShotData, read_write>
            38802..38814 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            38802..38817 'shot.unitary[5]': ref<storage, vec2<f32>, read_write>
            38815..38816 '5': integer
            38821..38824 'm00': vec2<f32>
            38826..38830 'shot': ptr<storage, ShotData, read_write>
            38826..38838 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            38826..38841 'shot.unitary[7]': ref<storage, vec2<f32>, read_write>
            38839..38840 '7': integer
            38845..38848 'm01': vec2<f32>
            38858..38862 'shot': ptr<storage, ShotData, read_write>
            38858..38870 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            38858..38874 'shot.u...ry[13]': ref<storage, vec2<f32>, read_write>
            38871..38873 '13': integer
            38877..38880 'm10': vec2<f32>
            38882..38886 'shot': ptr<storage, ShotData, read_write>
            38882..38894 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            38882..38898 'shot.u...ry[15]': ref<storage, vec2<f32>, read_write>
            38895..38897 '15': integer
            38901..38904 'm11': vec2<f32>
            39223..39231 'shot_idx': u32
            39238..39241 'row': u32
            39258..39262 'shot': ptr<storage, ShotData, read_write>
            39265..39281 '&shots...t_idx]': ptr<storage, ShotData, read_write>
            39266..39271 'shots': ref<storage, array<ShotData>, read_write>
            39266..39281 'shots[shot_idx]': ref<storage, ShotData, read_write>
            39272..39280 'shot_idx': u32
            39296..39297 'c': ref<function, u32, read_write>
            39300..39302 '0u': u32
            39304..39305 'c': ref<function, u32, read_write>
            39304..39310 'c < 4u': bool
            39308..39310 '4u': u32
            39312..39313 'c': ref<function, u32, read_write>
            39331..39332 'e': vec2<f32>
            39335..39339 'shot': ptr<storage, ShotData, read_write>
            39335..39347 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            39335..39361 'shot.u...u + c]': ref<storage, vec2<f32>, read_write>
            39348..39351 'row': u32
            39348..39356 'row * 4u': u32
            39348..39360 'row * 4u + c': u32
            39354..39356 '4u': u32
            39359..39360 'c': ref<function, u32, read_write>
            39371..39375 'shot': ptr<storage, ShotData, read_write>
            39371..39383 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            39371..39397 'shot.u...u + c]': ref<storage, vec2<f32>, read_write>
            39384..39387 'row': u32
            39384..39392 'row * 4u': u32
            39384..39396 'row * 4u + c': u32
            39390..39392 '4u': u32
            39395..39396 'c': ref<function, u32, read_write>
            39400..39416 'vec2f(... -e.x)': vec2<f32>
            39406..39407 'e': vec2<f32>
            39406..39409 'e.y': f32
            39411..39415 '-e.x': f32
            39412..39413 'e': vec2<f32>
            39412..39415 'e.x': f32
            39532..39540 'shot_idx': u32
            39547..39553 'op_idx': u32
            39560..39562 'q1': u32
            39569..39571 'q2': u32
            39588..39592 'shot': ptr<storage, ShotData, read_write>
            39595..39611 '&shots...t_idx]': ptr<storage, ShotData, read_write>
            39596..39601 'shots': ref<storage, array<ShotData>, read_write>
            39596..39611 'shots[shot_idx]': ref<storage, ShotData, read_write>
            39602..39610 'shot_idx': u32
            39617..39621 'shot': ptr<storage, ShotData, read_write>
            39617..39628 'shot.op_idx': ref<storage, u32, read_write>
            39631..39637 'op_idx': u32
            39643..39647 'shot': ptr<storage, ShotData, read_write>
            39643..39655 'shot.op_type': ref<storage, u32, read_write>
            39658..39675 'OPID_S...UFF_2Q': u32
            39681..39685 'shot': ptr<storage, ShotData, read_write>
            39681..39713 'shot.q...p_mask': ref<storage, u32, read_write>
            39716..39739 '(1u <<...<< q2)': u32
            39717..39719 '1u': u32
            39717..39725 '1u << q1': u32
            39723..39725 'q1': u32
            39730..39732 '1u': u32
            39730..39738 '1u << q2': u32
            39736..39738 'q2': u32
            39941..39949 'shot_idx': u32
            39956..39962 'op_idx': u32
            39969..39971 'q1': u32
            39978..39980 'q2': u32
            40005..40009 'shot': ptr<storage, ShotData, read_write>
            40012..40028 '&shots...t_idx]': ptr<storage, ShotData, read_write>
            40013..40018 'shots': ref<storage, array<ShotData>, read_write>
            40013..40028 'shots[shot_idx]': ref<storage, ShotData, read_write>
            40019..40027 'shot_idx': u32
            40038..40040 'op': ptr<storage, Op, read>
            40043..40055 '&ops[op_idx]': ptr<storage, Op, read>
            40044..40047 'ops': ref<storage, array<Op>, read>
            40044..40055 'ops[op_idx]': ref<storage, Op, read>
            40048..40054 'op_idx': u32
            40065..40069 'shot': ptr<storage, ShotData, read_write>
            40065..40081 'shot.q..._state': ref<storage, [error], read_write>
            40065..40085 'shot.q...te[q1]': [error]
            40065..40090 'shot.q...].heat': [error]
            40065..40098 'shot.q...= -1.0': [error]
            40082..40084 'q1': u32
            40094..40098 '-1.0': float
            40095..40098 '1.0': float
            40117..40121 'true': bool
            40137..40142 'is_2q': bool
            40145..40161 '!is_1q...op.id)': bool
            40146..40161 'is_1q_op(op.id)': bool
            40155..40157 'op': ptr<storage, Op, read>
            40155..40160 'op.id': ref<storage, u32, read>
            40174..40179 'is_2q': bool
            40174..40218 'is_2q ... -1.0)': [error]
            40184..40188 'shot': ptr<storage, ShotData, read_write>
            40184..40200 'shot.q..._state': ref<storage, [error], read_write>
            40184..40204 'shot.q...te[q2]': [error]
            40184..40209 'shot.q...].heat': [error]
            40184..40217 'shot.q...= -1.0': [error]
            40201..40203 'q2': u32
            40213..40217 '-1.0': float
            40214..40217 '1.0': float
            40065..40085 'shot.q...te[q1]': cannot index into type ref<storage, [error], read_write>
            40184..40204 'shot.q...te[q2]': cannot index into type ref<storage, [error], read_write>
            40698..40706 'shot_idx': u32
            40713..40719 'op_idx': u32
            40726..40728 'q1': u32
            40735..40737 'q2': u32
            40744..40749 'qubit': u32
            40766..40770 'shot': ptr<storage, ShotData, read_write>
            40773..40789 '&shots...t_idx]': ptr<storage, ShotData, read_write>
            40774..40779 'shots': ref<storage, array<ShotData>, read_write>
            40774..40789 'shots[shot_idx]': ref<storage, ShotData, read_write>
            40780..40788 'shot_idx': u32
            40800..40806 'result': [error]
            40809..40885 'select...ility)': [error]
            40816..40818 '1u': u32
            40820..40822 '0u': u32
            40824..40828 'shot': ptr<storage, ShotData, read_write>
            40824..40841 'shot.r...easure': ref<storage, f32, read_write>
            40824..40884 'shot.r...bility': [error]
            40844..40848 'shot': ptr<storage, ShotData, read_write>
            40844..40860 'shot.q..._state': ref<storage, [error], read_write>
            40844..40867 'shot.q...qubit]': [error]
            40844..40884 'shot.q...bility': [error]
            40861..40866 'qubit': u32
            41041..41044 'm00': [error]
            41047..41101 'select...== 1u)': [error]
            41054..41069 'vec2f(1.0, 0.0)': vec2<f32>
            41060..41063 '1.0': float
            41065..41068 '0.0': float
            41071..41086 'vec2f(0.0, 0.0)': vec2<f32>
            41077..41080 '0.0': float
            41082..41085 '0.0': float
            41088..41094 'result': [error]
            41088..41100 'result == 1u': [error]
            41098..41100 '1u': u32
            41111..41114 'm01': [error]
            41117..41171 'select...== 1u)': [error]
            41124..41139 'vec2f(0.0, 0.0)': vec2<f32>
            41130..41133 '0.0': float
            41135..41138 '0.0': float
            41141..41156 'vec2f(1.0, 0.0)': vec2<f32>
            41147..41150 '1.0': float
            41152..41155 '0.0': float
            41158..41164 'result': [error]
            41158..41170 'result == 1u': [error]
            41168..41170 '1u': u32
            41181..41184 'm10': vec2<f32>
            41187..41202 'vec2f(0.0, 0.0)': vec2<f32>
            41193..41196 '0.0': float
            41198..41201 '0.0': float
            41212..41215 'm11': vec2<f32>
            41218..41233 'vec2f(0.0, 0.0)': vec2<f32>
            41224..41227 '0.0': float
            41229..41232 '0.0': float
            41244..41256 'target_is_q2': bool
            41260..41265 'qubit': u32
            41260..41271 'qubit == q2': bool
            41269..41271 'q2': u32
            41278..41344 'set_1q..., m11)': [error]
            41301..41309 'shot_idx': u32
            41311..41323 'target_is_q2': bool
            41325..41328 'm00': [error]
            41330..41333 'm01': [error]
            41335..41338 'm10': vec2<f32>
            41340..41343 'm11': vec2<f32>
            41406..41410 'shot': ptr<storage, ShotData, read_write>
            41406..41422 'shot.r...malize': ref<storage, f32, read_write>
            41425..41577 'select...== 1u)': [error]
            41441..41444 '1.0': float
            41441..41493 '1.0 / ...ility)': [error]
            41447..41493 'sqrt(s...ility)': [error]
            41452..41456 'shot': ptr<storage, ShotData, read_write>
            41452..41468 'shot.q..._state': ref<storage, [error], read_write>
            41452..41475 'shot.q...qubit]': [error]
            41452..41492 'shot.q...bility': [error]
            41469..41474 'qubit': u32
            41503..41506 '1.0': float
            41503..41554 '1.0 / ...ility)': [error]
            41509..41554 'sqrt(s...ility)': [error]
            41514..41518 'shot': ptr<storage, ShotData, read_write>
            41514..41530 'shot.q..._state': ref<storage, [error], read_write>
            41514..41537 'shot.q...qubit]': [error]
            41514..41553 'shot.q...bility': [error]
            41531..41536 'qubit': u32
            41564..41570 'result': [error]
            41564..41576 'result == 1u': [error]
            41574..41576 '1u': u32
            41691..41695 'shot': ptr<storage, ShotData, read_write>
            41691..41707 'shot.q..._state': ref<storage, [error], read_write>
            41691..41714 'shot.q...qubit]': [error]
            41691..41719 'shot.q...].heat': [error]
            41708..41713 'qubit': u32
            41722..41726 '-1.0': float
            41723..41726 '1.0': float
            41732..41736 'shot': ptr<storage, ShotData, read_write>
            41732..41752 'shot.q...0_mask': ref<storage, u32, read_write>
            41755..41759 'shot': ptr<storage, ShotData, read_write>
            41755..41775 'shot.q...0_mask': ref<storage, u32, read_write>
            41755..41792 'shot.q...qubit)': u32
            41778..41792 '~(1u << qubit)': u32
            41780..41782 '1u': u32
            41780..41791 '1u << qubit': u32
            41786..41791 'qubit': u32
            41798..41802 'shot': ptr<storage, ShotData, read_write>
            41798..41818 'shot.q...1_mask': ref<storage, u32, read_write>
            41821..41825 'shot': ptr<storage, ShotData, read_write>
            41821..41841 'shot.q...1_mask': ref<storage, u32, read_write>
            41821..41858 'shot.q...qubit)': u32
            41844..41858 '~(1u << qubit)': u32
            41846..41848 '1u': u32
            41846..41857 '1u << qubit': u32
            41852..41857 'qubit': u32
            41865..41912 'finish...1, q2)': [error]
            41887..41895 'shot_idx': u32
            41897..41903 'op_idx': u32
            41905..41907 'q1': u32
            41909..41911 'q2': u32
            40844..40867 'shot.q...qubit]': cannot index into type ref<storage, [error], read_write>
            41325..41328 'm00': expected vec2<f32> but got [error]
            41330..41333 'm01': expected vec2<f32> but got [error]
            41452..41475 'shot.q...qubit]': cannot index into type ref<storage, [error], read_write>
            41514..41537 'shot.q...qubit]': cannot index into type ref<storage, [error], read_write>
            41691..41714 'shot.q...qubit]': cannot index into type ref<storage, [error], read_write>
            41691..41719 'shot.q...].heat': cannot assign to non-reference `[error]`
            42393..42401 'shot_idx': u32
            42408..42414 'op_idx': u32
            42421..42423 'q1': u32
            42430..42432 'q2': u32
            42449..42453 'shot': ptr<storage, ShotData, read_write>
            42456..42472 '&shots...t_idx]': ptr<storage, ShotData, read_write>
            42457..42462 'shots': ref<storage, array<ShotData>, read_write>
            42457..42472 'shots[shot_idx]': ref<storage, ShotData, read_write>
            42463..42471 'shot_idx': u32
            42482..42484 'op': ptr<storage, Op, read>
            42487..42499 '&ops[op_idx]': ptr<storage, Op, read>
            42488..42491 'ops': ref<storage, array<Op>, read>
            42488..42499 'ops[op_idx]': ref<storage, Op, read>
            42492..42498 'op_idx': u32
            42509..42514 'is_1q': bool
            42517..42532 'is_1q_op(op.id)': bool
            42526..42528 'op': ptr<storage, Op, read>
            42526..42531 'op.id': ref<storage, u32, read>
            42542..42547 'is_2q': bool
            42550..42556 '!is_1q': bool
            42551..42556 'is_1q': bool
            42566..42572 'policy': u32
            42575..42577 'op': ptr<storage, Op, read>
            42575..42584 'op.policy': ref<storage, u32, read>
            42712..42717 'is_1q': bool
            42729..42733 'shot': ptr<storage, ShotData, read_write>
            42729..42741 'shot.op_type': ref<storage, u32, read_write>
            42744..42751 'OPID_ID': u32
            42761..42765 'shot': ptr<storage, ShotData, read_write>
            42761..42772 'shot.op_idx': ref<storage, u32, read_write>
            42775..42781 'op_idx': u32
            42814..42821 'q1_lost': [error]
            42824..42828 'shot': ptr<storage, ShotData, read_write>
            42824..42840 'shot.q..._state': ref<storage, [error], read_write>
            42824..42844 'shot.q...te[q1]': [error]
            42824..42849 'shot.q...].heat': [error]
            42824..42857 'shot.q...= -1.0': [error]
            42841..42843 'q1': u32
            42853..42857 '-1.0': float
            42854..42857 '1.0': float
            42867..42874 'q2_lost': [error]
            42877..42882 'is_2q': bool
            42877..42921 'is_2q ... -1.0)': [error]
            42887..42891 'shot': ptr<storage, ShotData, read_write>
            42887..42903 'shot.q..._state': ref<storage, [error], read_write>
            42887..42907 'shot.q...te[q2]': [error]
            42887..42912 'shot.q...].heat': [error]
            42887..42920 'shot.q...= -1.0': [error]
            42904..42906 'q2': u32
            42916..42920 '-1.0': float
            42917..42920 '1.0': float
            42931..42943 'has_survivor': [error]
            42946..42951 'is_2q': bool
            42946..42976 'is_2q ..._lost)': [error]
            42955..42976 '!(q1_l..._lost)': [error]
            42957..42964 'q1_lost': [error]
            42957..42975 'q1_los...2_lost': [error]
            42968..42975 'q2_lost': [error]
            43060..43068 'survivor': [error]
            43071..43094 'select..._lost)': [error]
            43078..43080 'q1': u32
            43082..43084 'q2': u32
            43086..43093 'q1_lost': [error]
            43104..43118 'survivor_is_q2': [error]
            43121..43128 'q1_lost': [error]
            43361..43363 'op': ptr<storage, Op, read>
            43361..43366 'op.id': ref<storage, u32, read>
            43361..43379 'op.id ...D_SWAP': bool
            43370..43379 'OPID_SWAP': u32
            43398..43404 'policy': u32
            43424..43445 'LOSS_P...PAGATE': u32
            43464..43523 'propag...vivor)': [error]
            43488..43496 'shot_idx': u32
            43498..43504 'op_idx': u32
            43506..43508 'q1': u32
            43510..43512 'q2': u32
            43514..43522 'survivor': [error]
            43580..43609 'LOSS_P...DAGGER': u32
            44316..44324 'lost_row': [error]
            44327..44350 'select..._lost)': [error]
            44334..44336 '1u': u32
            44338..44340 '2u': u32
            44342..44349 'q1_lost': [error]
            44368..44419 'scale_...t_row)': [error]
            44400..44408 'shot_idx': u32
            44410..44418 'lost_row': [error]
            44437..44482 'scale_...x, 3u)': [error]
            44469..44477 'shot_idx': u32
            44479..44481 '3u': u32
            44584..44589 'heat1': [error]
            44592..44596 'shot': ptr<storage, ShotData, read_write>
            44592..44608 'shot.q..._state': ref<storage, [error], read_write>
            44592..44612 'shot.q...te[q1]': [error]
            44592..44617 'shot.q...].heat': [error]
            44609..44611 'q1': u32
            44635..44639 'shot': ptr<storage, ShotData, read_write>
            44635..44651 'shot.q..._state': ref<storage, [error], read_write>
            44635..44655 'shot.q...te[q1]': [error]
            44635..44660 'shot.q...].heat': [error]
            44652..44654 'q1': u32
            44663..44667 'shot': ptr<storage, ShotData, read_write>
            44663..44679 'shot.q..._state': ref<storage, [error], read_write>
            44663..44683 'shot.q...te[q2]': [error]
            44663..44688 'shot.q...].heat': [error]
            44680..44682 'q2': u32
            44706..44710 'shot': ptr<storage, ShotData, read_write>
            44706..44722 'shot.q..._state': ref<storage, [error], read_write>
            44706..44726 'shot.q...te[q2]': [error]
            44706..44731 'shot.q...].heat': [error]
            44723..44725 'q2': u32
            44734..44739 'heat1': [error]
            45020..45024 'shot': ptr<storage, ShotData, read_write>
            45020..45040 'shot.q...0_mask': ref<storage, u32, read_write>
            45043..45047 'shot': ptr<storage, ShotData, read_write>
            45043..45063 'shot.q...0_mask': ref<storage, u32, read_write>
            45043..45092 'shot.q...< q2))': u32
            45066..45092 '~((1u ...< q2))': u32
            45068..45091 '(1u <<...<< q2)': u32
            45069..45071 '1u': u32
            45069..45077 '1u << q1': u32
            45075..45077 'q1': u32
            45082..45084 '1u': u32
            45082..45090 '1u << q2': u32
            45088..45090 'q2': u32
            45110..45114 'shot': ptr<storage, ShotData, read_write>
            45110..45130 'shot.q...1_mask': ref<storage, u32, read_write>
            45133..45137 'shot': ptr<storage, ShotData, read_write>
            45133..45153 'shot.q...1_mask': ref<storage, u32, read_write>
            45133..45182 'shot.q...< q2))': u32
            45156..45182 '~((1u ...< q2))': u32
            45158..45181 '(1u <<...<< q2)': u32
            45159..45161 '1u': u32
            45159..45167 '1u << q1': u32
            45165..45167 'q1': u32
            45172..45174 '1u': u32
            45172..45180 '1u << q2': u32
            45178..45180 'q2': u32
            45269..45316 'finish...1, q2)': [error]
            45291..45299 'shot_idx': u32
            45301..45307 'op_idx': u32
            45309..45311 'q1': u32
            45313..45315 'q2': u32
            45373..45397 'LOSS_P...ANYWAY': u32
            45500..45505 'heat1': [error]
            45508..45512 'shot': ptr<storage, ShotData, read_write>
            45508..45524 'shot.q..._state': ref<storage, [error], read_write>
            45508..45528 'shot.q...te[q1]': [error]
            45508..45533 'shot.q...].heat': [error]
            45525..45527 'q1': u32
            45551..45555 'shot': ptr<storage, ShotData, read_write>
            45551..45567 'shot.q..._state': ref<storage, [error], read_write>
            45551..45571 'shot.q...te[q1]': [error]
            45551..45576 'shot.q...].heat': [error]
            45568..45570 'q1': u32
            45579..45583 'shot': ptr<storage, ShotData, read_write>
            45579..45595 'shot.q..._state': ref<storage, [error], read_write>
            45579..45599 'shot.q...te[q2]': [error]
            45579..45604 'shot.q...].heat': [error]
            45596..45598 'q2': u32
            45622..45626 'shot': ptr<storage, ShotData, read_write>
            45622..45638 'shot.q..._state': ref<storage, [error], read_write>
            45622..45642 'shot.q...te[q2]': [error]
            45622..45647 'shot.q...].heat': [error]
            45639..45641 'q2': u32
            45650..45655 'heat1': [error]
            45936..45940 'shot': ptr<storage, ShotData, read_write>
            45936..45956 'shot.q...0_mask': ref<storage, u32, read_write>
            45959..45963 'shot': ptr<storage, ShotData, read_write>
            45959..45979 'shot.q...0_mask': ref<storage, u32, read_write>
            45959..46008 'shot.q...< q2))': u32
            45982..46008 '~((1u ...< q2))': u32
            45984..46007 '(1u <<...<< q2)': u32
            45985..45987 '1u': u32
            45985..45993 '1u << q1': u32
            45991..45993 'q1': u32
            45998..46000 '1u': u32
            45998..46006 '1u << q2': u32
            46004..46006 'q2': u32
            46026..46030 'shot': ptr<storage, ShotData, read_write>
            46026..46046 'shot.q...1_mask': ref<storage, u32, read_write>
            46049..46053 'shot': ptr<storage, ShotData, read_write>
            46049..46069 'shot.q...1_mask': ref<storage, u32, read_write>
            46049..46098 'shot.q...< q2))': u32
            46072..46098 '~((1u ...< q2))': u32
            46074..46097 '(1u <<...<< q2)': u32
            46075..46077 '1u': u32
            46075..46083 '1u << q1': u32
            46081..46083 'q1': u32
            46088..46090 '1u': u32
            46088..46096 '1u << q2': u32
            46094..46096 'q2': u32
            46199..46246 'finish...1, q2)': [error]
            46221..46229 'shot_idx': u32
            46231..46237 'op_idx': u32
            46239..46241 'q1': u32
            46243..46245 'q2': u32
            46303..46319 'LOSS_P...Y_SKIP': u32
            46338..46342 'shot': ptr<storage, ShotData, read_write>
            46338..46350 'shot.op_type': ref<storage, u32, read_write>
            46353..46360 'OPID_ID': u32
            46378..46382 'shot': ptr<storage, ShotData, read_write>
            46378..46389 'shot.op_idx': ref<storage, u32, read_write>
            46392..46398 'op_idx': u32
            46697..46753 'report...OLICY)': [error]
            46715..46723 'shot_idx': u32
            46725..46752 'ERR_UN...POLICY': u32
            46771..46775 'shot': ptr<storage, ShotData, read_write>
            46771..46783 'shot.op_type': ref<storage, u32, read_write>
            46786..46793 'OPID_ID': u32
            46811..46815 'shot': ptr<storage, ShotData, read_write>
            46811..46822 'shot.op_idx': ref<storage, u32, read_write>
            46825..46831 'op_idx': u32
            47059..47065 'policy': u32
            47059..47093 'policy...ANYWAY': bool
            47069..47093 'LOSS_P...ANYWAY': u32
            47105..47161 'report...OLICY)': [error]
            47123..47131 'shot_idx': u32
            47133..47160 'ERR_UN...POLICY': u32
            47171..47175 'shot': ptr<storage, ShotData, read_write>
            47171..47183 'shot.op_type': ref<storage, u32, read_write>
            47186..47193 'OPID_ID': u32
            47203..47207 'shot': ptr<storage, ShotData, read_write>
            47203..47214 'shot.op_idx': ref<storage, u32, read_write>
            47217..47223 'op_idx': u32
            47256..47262 'policy': u32
            47256..47287 'policy...PAGATE': bool
            47256..47303 'policy...rvivor': [error]
            47266..47287 'LOSS_P...PAGATE': u32
            47291..47303 'has_survivor': [error]
            47315..47374 'propag...vivor)': [error]
            47339..47347 'shot_idx': u32
            47349..47355 'op_idx': u32
            47357..47359 'q1': u32
            47361..47363 'q2': u32
            47365..47373 'survivor': [error]
            47407..47413 'policy': u32
            47407..47446 'policy...DAGGER': bool
            47407..47462 'policy...rvivor': [error]
            47417..47446 'LOSS_P...DAGGER': u32
            47450..47462 'has_survivor': [error]
            47540..47681 'set_1q...-1.0))': [error]
            47563..47571 'shot_idx': u32
            47573..47587 'survivor_is_q2': [error]
            47601..47616 'vec2f(1.0, 0.0)': vec2<f32>
            47607..47610 '1.0': float
            47612..47615 '0.0': float
            47618..47633 'vec2f(0.0, 0.0)': vec2<f32>
            47624..47627 '0.0': float
            47629..47632 '0.0': float
            47647..47662 'vec2f(0.0, 0.0)': vec2<f32>
            47653..47656 '0.0': float
            47658..47661 '0.0': float
            47664..47680 'vec2f(... -1.0)': vec2<f32>
            47670..47673 '0.0': float
            47675..47679 '-1.0': float
            47676..47679 '1.0': float
            47691..47738 'finish...1, q2)': [error]
            47713..47721 'shot_idx': u32
            47723..47729 'op_idx': u32
            47731..47733 'q1': u32
            47735..47737 'q2': u32
            47916..47922 'policy': u32
            47916..47945 'policy...EGRADE': bool
            47916..47961 'policy...rvivor': [error]
            47926..47945 'LOSS_P...EGRADE': u32
            47949..47961 'has_survivor': [error]
            48194..48202 'cos_half': f32
            48205..48207 'op': ptr<storage, Op, read>
            48205..48215 'op.unitary': ref<storage, array<vec2<f32>, 16>, read>
            48205..48218 'op.unitary[0]': ref<storage, vec2<f32>, read>
            48205..48220 'op.unitary[0].x': ref<storage, f32, read>
            48216..48217 '0': integer
            48234..48236 'op': ptr<storage, Op, read>
            48234..48239 'op.id': ref<storage, u32, read>
            48234..48251 'op.id ...ID_RXX': bool
            48243..48251 'OPID_RXX': u32
            48340..48341 's': f32
            48344..48346 'op': ptr<storage, Op, read>
            48344..48354 'op.unitary': ref<storage, array<vec2<f32>, 16>, read>
            48344..48357 'op.unitary[3]': ref<storage, vec2<f32>, read>
            48344..48359 'op.unitary[3].y': ref<storage, f32, read>
            48344..48366 'op.uni...* -1.0': f32
            48355..48356 '3': integer
            48362..48366 '-1.0': float
            48363..48366 '1.0': float
            48412..48568 'set_1q... 0.0))': [error]
            48435..48443 'shot_idx': u32
            48445..48459 'survivor_is_q2': [error]
            48477..48497 'vec2f(..., 0.0)': vec2<f32>
            48483..48491 'cos_half': f32
            48493..48496 '0.0': float
            48499..48513 'vec2f(0.0, -s)': vec2<f32>
            48505..48508 '0.0': float
            48510..48512 '-s': f32
            48511..48512 's': f32
            48531..48545 'vec2f(0.0, -s)': vec2<f32>
            48537..48540 '0.0': float
            48542..48544 '-s': f32
            48543..48544 's': f32
            48547..48567 'vec2f(..., 0.0)': vec2<f32>
            48553..48561 'cos_half': f32
            48563..48566 '0.0': float
            48690..48691 's': f32
            48694..48696 'op': ptr<storage, Op, read>
            48694..48704 'op.unitary': ref<storage, array<vec2<f32>, 16>, read>
            48694..48707 'op.unitary[3]': ref<storage, vec2<f32>, read>
            48694..48709 'op.unitary[3].y': ref<storage, f32, read>
            48705..48706 '3': integer
            48762..48917 'set_1q... 0.0))': [error]
            48785..48793 'shot_idx': u32
            48795..48809 'survivor_is_q2': [error]
            48827..48847 'vec2f(..., 0.0)': vec2<f32>
            48833..48841 'cos_half': f32
            48843..48846 '0.0': float
            48849..48863 'vec2f(-s, 0.0)': vec2<f32>
            48855..48857 '-s': f32
            48856..48857 's': f32
            48859..48862 '0.0': float
            48881..48894 'vec2f(s, 0.0)': vec2<f32>
            48887..48888 's': f32
            48890..48893 '0.0': float
            48896..48916 'vec2f(..., 0.0)': vec2<f32>
            48902..48910 'cos_half': f32
            48912..48915 '0.0': float
            49099..49104 'phase': vec2<f32>
            49107..49109 'op': ptr<storage, Op, read>
            49107..49117 'op.unitary': ref<storage, array<vec2<f32>, 16>, read>
            49107..49120 'op.unitary[5]': ref<storage, vec2<f32>, read>
            49118..49119 '5': integer
            49134..49272 'set_1q...phase)': [error]
            49157..49165 'shot_idx': u32
            49167..49181 'survivor_is_q2': [error]
            49199..49214 'vec2f(1.0, 0.0)': vec2<f32>
            49205..49208 '1.0': float
            49210..49213 '0.0': float
            49216..49231 'vec2f(0.0, 0.0)': vec2<f32>
            49222..49225 '0.0': float
            49227..49230 '0.0': float
            49249..49264 'vec2f(0.0, 0.0)': vec2<f32>
            49255..49258 '0.0': float
            49260..49263 '0.0': float
            49266..49271 'phase': vec2<f32>
            49292..49339 'finish...1, q2)': [error]
            49314..49322 'shot_idx': u32
            49324..49330 'op_idx': u32
            49332..49334 'q1': u32
            49336..49338 'q2': u32
            49479..49483 'shot': ptr<storage, ShotData, read_write>
            49479..49491 'shot.op_type': ref<storage, u32, read_write>
            49494..49501 'OPID_ID': u32
            49507..49511 'shot': ptr<storage, ShotData, read_write>
            49507..49518 'shot.op_idx': ref<storage, u32, read_write>
            49521..49527 'op_idx': u32
            42824..42844 'shot.q...te[q1]': cannot index into type ref<storage, [error], read_write>
            42887..42907 'shot.q...te[q2]': cannot index into type ref<storage, [error], read_write>
            43514..43522 'survivor': expected u32 but got [error]
            44410..44418 'lost_row': expected u32 but got [error]
            44592..44612 'shot.q...te[q1]': cannot index into type ref<storage, [error], read_write>
            44635..44655 'shot.q...te[q1]': cannot index into type ref<storage, [error], read_write>
            44635..44660 'shot.q...].heat': cannot assign to non-reference `[error]`
            44663..44683 'shot.q...te[q2]': cannot index into type ref<storage, [error], read_write>
            44706..44726 'shot.q...te[q2]': cannot index into type ref<storage, [error], read_write>
            44706..44731 'shot.q...].heat': cannot assign to non-reference `[error]`
            45508..45528 'shot.q...te[q1]': cannot index into type ref<storage, [error], read_write>
            45551..45571 'shot.q...te[q1]': cannot index into type ref<storage, [error], read_write>
            45551..45576 'shot.q...].heat': cannot assign to non-reference `[error]`
            45579..45599 'shot.q...te[q2]': cannot index into type ref<storage, [error], read_write>
            45622..45642 'shot.q...te[q2]': cannot index into type ref<storage, [error], read_write>
            45622..45647 'shot.q...].heat': cannot assign to non-reference `[error]`
            47365..47373 'survivor': expected u32 but got [error]
            47573..47587 'survivor_is_q2': expected bool but got [error]
            48445..48459 'survivor_is_q2': expected bool but got [error]
            48795..48809 'survivor_is_q2': expected bool but got [error]
            49167..49181 'survivor_is_q2': expected bool but got [error]
            49871..49879 'shot_idx': u32
            49886..49890 'code': u32
            49903..49963 'atomic... code)': __atomic_compare_exchange_result
            49929..49952 '&diagn...r_code': ptr<storage, atomic<u32>, read_write>
            49930..49941 'diagnostics': ref<storage, DiagnosticData, read_write>
            49930..49952 'diagno...r_code': ref<storage, atomic<u32>, read_write>
            49954..49956 '0u': u32
            49958..49962 'code': u32
            49973..49982 'err_index': u32
            49985..50015 '(shot_..._COUNT': u32
            49985..50020 '(shot_...T - 1u': u32
            49986..49994 'shot_idx': u32
            49986..49999 'shot_idx + 1u': u32
            49997..49999 '1u': u32
            50003..50015 'RESULT_COUNT': u32
            50018..50020 '1u': u32
            50026..50082 'atomic... code)': __atomic_compare_exchange_result
            50052..50071 '&resul...index]': ptr<storage, atomic<u32>, read_write>
            50053..50060 'results': ref<storage, array<atomic<u32>>, read_write>
            50053..50071 'result...index]': ref<storage, atomic<u32>, read_write>
            50061..50070 'err_index': u32
            50073..50075 '0u': u32
            50077..50081 'code': u32
            50238..50244 'op_idx': u32
            50268..50285 'arrayL...(&ops)': u32
            50268..50300 'arrayL...x + 1)': bool
            50280..50284 '&ops': ptr<storage, array<Op>, read>
            50281..50284 'ops': ref<storage, array<Op>, read>
            50289..50295 'op_idx': u32
            50289..50299 'op_idx + 1': u32
            50298..50299 '1': integer
            50316..50318 'op': ptr<storage, Op, read>
            50321..50337 '&ops[o...x + 1]': ptr<storage, Op, read>
            50322..50325 'ops': ref<storage, array<Op>, read>
            50322..50337 'ops[op_idx + 1]': ref<storage, Op, read>
            50326..50332 'op_idx': u32
            50326..50336 'op_idx + 1': u32
            50335..50336 '1': integer
            50351..50353 'op': ptr<storage, Op, read>
            50351..50356 'op.id': ref<storage, u32, read>
            50351..50379 'op.id ...ISE_1Q': bool
            50351..50411 'op.id ...ISE_2Q': bool
            50360..50379 'OPID_P...ISE_1Q': u32
            50383..50385 'op': ptr<storage, Op, read>
            50383..50388 'op.id': ref<storage, u32, read>
            50383..50411 'op.id ...ISE_2Q': bool
            50392..50411 'OPID_P...ISE_2Q': u32
            50434..50440 'op_idx': u32
            50434..50445 'op_idx + 1u': u32
            50443..50445 '1u': u32
            50474..50476 '0u': u32
            50505..50513 'shot_idx': u32
            50520..50526 'op_idx': u32
            50533..50542 'noise_idx': u32
            50549..50551 'q1': u32
            50824..50828 'shot': ptr<storage, ShotData, read_write>
            50831..50847 '&shots...t_idx]': ptr<storage, ShotData, read_write>
            50832..50837 'shots': ref<storage, array<ShotData>, read_write>
            50832..50847 'shots[shot_idx]': ref<storage, ShotData, read_write>
            50838..50846 'shot_idx': u32
            50857..50859 'op': ptr<storage, Op, read>
            50862..50874 '&ops[op_idx]': ptr<storage, Op, read>
            50863..50866 'ops': ref<storage, array<Op>, read>
            50863..50874 'ops[op_idx]': ref<storage, Op, read>
            50867..50873 'op_idx': u32
            50884..50892 'noise_op': ptr<storage, Op, read>
            50895..50910 '&ops[noise_idx]': ptr<storage, Op, read>
            50896..50899 'ops': ref<storage, array<Op>, read>
            50896..50910 'ops[noise_idx]': ref<storage, Op, read>
            50900..50909 'noise_idx': u32
            51112..51115 'p_x': f32
            51118..51126 'noise_op': ptr<storage, Op, read>
            51118..51134 'noise_...nitary': ref<storage, array<vec2<f32>, 16>, read>
            51118..51137 'noise_...ary[0]': ref<storage, vec2<f32>, read>
            51118..51139 'noise_...y[0].y': ref<storage, f32, read>
            51135..51136 '0': integer
            51149..51152 'p_z': f32
            51155..51163 'noise_op': ptr<storage, Op, read>
            51155..51171 'noise_...nitary': ref<storage, array<vec2<f32>, 16>, read>
            51155..51174 'noise_...ary[1]': ref<storage, vec2<f32>, read>
            51155..51176 'noise_...y[1].x': ref<storage, f32, read>
            51172..51173 '1': integer
            51186..51189 'p_y': f32
            51192..51200 'noise_op': ptr<storage, Op, read>
            51192..51208 'noise_...nitary': ref<storage, array<vec2<f32>, 16>, read>
            51192..51211 'noise_...ary[1]': ref<storage, vec2<f32>, read>
            51192..51213 'noise_...y[1].y': ref<storage, f32, read>
            51209..51210 '1': integer
            51223..51229 'p_loss': f32
            51232..51240 'noise_op': ptr<storage, Op, read>
            51232..51248 'noise_...nitary': ref<storage, array<vec2<f32>, 16>, read>
            51232..51251 'noise_...ary[2]': ref<storage, vec2<f32>, read>
            51232..51253 'noise_...y[2].x': ref<storage, f32, read>
            51249..51250 '2': integer
            51260..51264 'shot': ptr<storage, ShotData, read_write>
            51260..51272 'shot.op_type': ref<storage, u32, read_write>
            51275..51292 'OPID_S...UFF_1Q': u32
            51352..51356 'rand': f32
            51359..51363 'shot': ptr<storage, ShotData, read_write>
            51359..51374 'shot.rand_pauli': ref<storage, f32, read_write>
            51384..51388 'rand': f32
            51384..51394 'rand < p_x': bool
            51391..51394 'p_x': f32
            51467..51471 'shot': ptr<storage, ShotData, read_write>
            51467..51479 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            51467..51482 'shot.unitary[0]': ref<storage, vec2<f32>, read_write>
            51480..51481 '0': integer
            51485..51487 'op': ptr<storage, Op, read>
            51485..51495 'op.unitary': ref<storage, array<vec2<f32>, 16>, read>
            51485..51498 'op.unitary[4]': ref<storage, vec2<f32>, read>
            51496..51497 '4': integer
            51508..51512 'shot': ptr<storage, ShotData, read_write>
            51508..51520 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            51508..51523 'shot.unitary[1]': ref<storage, vec2<f32>, read_write>
            51521..51522 '1': integer
            51526..51528 'op': ptr<storage, Op, read>
            51526..51536 'op.unitary': ref<storage, array<vec2<f32>, 16>, read>
            51526..51539 'op.unitary[5]': ref<storage, vec2<f32>, read>
            51537..51538 '5': integer
            51549..51553 'shot': ptr<storage, ShotData, read_write>
            51549..51561 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            51549..51564 'shot.unitary[4]': ref<storage, vec2<f32>, read_write>
            51562..51563 '4': integer
            51567..51569 'op': ptr<storage, Op, read>
            51567..51577 'op.unitary': ref<storage, array<vec2<f32>, 16>, read>
            51567..51580 'op.unitary[0]': ref<storage, vec2<f32>, read>
            51578..51579 '0': integer
            51590..51594 'shot': ptr<storage, ShotData, read_write>
            51590..51602 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            51590..51605 'shot.unitary[5]': ref<storage, vec2<f32>, read_write>
            51603..51604 '5': integer
            51608..51610 'op': ptr<storage, Op, read>
            51608..51618 'op.unitary': ref<storage, array<vec2<f32>, 16>, read>
            51608..51621 'op.unitary[1]': ref<storage, vec2<f32>, read>
            51619..51620 '1': integer
            51738..51742 'shot': ptr<storage, ShotData, read_write>
            51738..51750 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            51738..51753 'shot.unitary[0]': ref<storage, vec2<f32>, read_write>
            51751..51752 '0': integer
            51756..51778 'cplxNe...ry[4])': vec2<f32>
            51764..51766 'op': ptr<storage, Op, read>
            51764..51774 'op.unitary': ref<storage, array<vec2<f32>, 16>, read>
            51764..51777 'op.unitary[4]': ref<storage, vec2<f32>, read>
            51775..51776 '4': integer
            51788..51792 'shot': ptr<storage, ShotData, read_write>
            51788..51800 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            51788..51803 'shot.unitary[1]': ref<storage, vec2<f32>, read_write>
            51801..51802 '1': integer
            51806..51828 'cplxNe...ry[5])': vec2<f32>
            51814..51816 'op': ptr<storage, Op, read>
            51814..51824 'op.unitary': ref<storage, array<vec2<f32>, 16>, read>
            51814..51827 'op.unitary[5]': ref<storage, vec2<f32>, read>
            51825..51826 '5': integer
            51838..51842 'shot': ptr<storage, ShotData, read_write>
            51838..51850 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            51838..51853 'shot.unitary[4]': ref<storage, vec2<f32>, read_write>
            51851..51852 '4': integer
            51856..51858 'op': ptr<storage, Op, read>
            51856..51866 'op.unitary': ref<storage, array<vec2<f32>, 16>, read>
            51856..51869 'op.unitary[0]': ref<storage, vec2<f32>, read>
            51867..51868 '0': integer
            51879..51883 'shot': ptr<storage, ShotData, read_write>
            51879..51891 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            51879..51894 'shot.unitary[5]': ref<storage, vec2<f32>, read_write>
            51892..51893 '5': integer
            51897..51899 'op': ptr<storage, Op, read>
            51897..51907 'op.unitary': ref<storage, array<vec2<f32>, 16>, read>
            51897..51910 'op.unitary[1]': ref<storage, vec2<f32>, read>
            51908..51909 '1': integer
            52007..52011 'shot': ptr<storage, ShotData, read_write>
            52007..52019 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            52007..52022 'shot.unitary[0]': ref<storage, vec2<f32>, read_write>
            52020..52021 '0': integer
            52025..52027 'op': ptr<storage, Op, read>
            52025..52035 'op.unitary': ref<storage, array<vec2<f32>, 16>, read>
            52025..52038 'op.unitary[0]': ref<storage, vec2<f32>, read>
            52036..52037 '0': integer
            52048..52052 'shot': ptr<storage, ShotData, read_write>
            52048..52060 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            52048..52063 'shot.unitary[1]': ref<storage, vec2<f32>, read_write>
            52061..52062 '1': integer
            52066..52068 'op': ptr<storage, Op, read>
            52066..52076 'op.unitary': ref<storage, array<vec2<f32>, 16>, read>
            52066..52079 'op.unitary[1]': ref<storage, vec2<f32>, read>
            52077..52078 '1': integer
            52089..52093 'shot': ptr<storage, ShotData, read_write>
            52089..52101 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            52089..52104 'shot.unitary[4]': ref<storage, vec2<f32>, read_write>
            52102..52103 '4': integer
            52107..52129 'cplxNe...ry[4])': vec2<f32>
            52115..52117 'op': ptr<storage, Op, read>
            52115..52125 'op.unitary': ref<storage, array<vec2<f32>, 16>, read>
            52115..52128 'op.unitary[4]': ref<storage, vec2<f32>, read>
            52126..52127 '4': integer
            52139..52143 'shot': ptr<storage, ShotData, read_write>
            52139..52151 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            52139..52154 'shot.unitary[5]': ref<storage, vec2<f32>, read_write>
            52152..52153 '5': integer
            52157..52179 'cplxNe...ry[5])': vec2<f32>
            52165..52167 'op': ptr<storage, Op, read>
            52165..52175 'op.unitary': ref<storage, array<vec2<f32>, 16>, read>
            52165..52178 'op.unitary[5]': ref<storage, vec2<f32>, read>
            52176..52177 '5': integer
            52411..52415 'rand': f32
            52411..52444 'rand <..._loss)': bool
            52419..52422 'p_x': f32
            52419..52428 'p_x + p_z': f32
            52419..52434 'p_x + p_z + p_y': f32
            52419..52443 'p_x + ...p_loss': f32
            52425..52428 'p_z': f32
            52431..52434 'p_y': f32
            52437..52443 'p_loss': f32
            52460..52464 'shot': ptr<storage, ShotData, read_write>
            52460..52482 'shot.p...s_mask': ref<storage, u32, read_write>
            52487..52489 '1u': u32
            52487..52495 '1u << q1': u32
            52493..52495 'q1': u32
            52661..52663 'op': ptr<storage, Op, read>
            52661..52666 'op.id': ref<storage, u32, read>
            52661..52677 'op.id ...PID_ID': bool
            52661..52702 'op.id ...RESETZ': bool
            52661..52722 'op.id ...PID_MZ': bool
            52661..52746 'op.id ...RESETZ': bool
            52670..52677 'OPID_ID': u32
            52681..52683 'op': ptr<storage, Op, read>
            52681..52686 'op.id': ref<storage, u32, read>
            52681..52702 'op.id ...RESETZ': bool
            52690..52702 'OPID_MRESETZ': u32
            52706..52708 'op': ptr<storage, Op, read>
            52706..52711 'op.id': ref<storage, u32, read>
            52706..52722 'op.id ...PID_MZ': bool
            52715..52722 'OPID_MZ': u32
            52726..52728 'op': ptr<storage, Op, read>
            52726..52731 'op.id': ref<storage, u32, read>
            52726..52746 'op.id ...RESETZ': bool
            52735..52746 'OPID_RESETZ': u32
            52762..52766 'shot': ptr<storage, ShotData, read_write>
            52762..52774 'shot.op_type': ref<storage, u32, read_write>
            52777..52779 'op': ptr<storage, Op, read>
            52777..52782 'op.id': ref<storage, u32, read>
            52806..52829 'is_1q_...op.id)': bool
            52823..52825 'op': ptr<storage, Op, read>
            52823..52828 'op.id': ref<storage, u32, read>
            52923..52927 'shot': ptr<storage, ShotData, read_write>
            52923..52935 'shot.op_type': ref<storage, u32, read_write>
            52938..52945 'OPID_RZ': u32
            52968..52972 'shot': ptr<storage, ShotData, read_write>
            52968..52979 'shot.op_idx': ref<storage, u32, read_write>
            52982..52988 'op_idx': u32
            52998..53002 'shot': ptr<storage, ShotData, read_write>
            52998..53010 'shot.op_type': ref<storage, u32, read_write>
            52998..53021 'shot.o...PID_ID': bool
            52998..53048 'shot.o...PID_RZ': bool
            53014..53021 'OPID_ID': u32
            53025..53029 'shot': ptr<storage, ShotData, read_write>
            53025..53037 'shot.op_type': ref<storage, u32, read_write>
            53025..53048 'shot.o...PID_RZ': bool
            53041..53048 'OPID_RZ': u32
            53060..53064 'shot': ptr<storage, ShotData, read_write>
            53060..53092 'shot.q...p_mask': ref<storage, u32, read_write>
            53095..53097 '0u': u32
            53120..53124 'shot': ptr<storage, ShotData, read_write>
            53120..53152 'shot.q...p_mask': ref<storage, u32, read_write>
            53155..53157 '1u': u32
            53155..53163 '1u << q1': u32
            53161..53163 'q1': u32
            53199..53207 'shot_idx': u32
            53214..53220 'op_idx': u32
            53227..53236 'noise_idx': u32
            53243..53245 'q1': u32
            53252..53254 'q2': u32
            53271..53275 'shot': ptr<storage, ShotData, read_write>
            53278..53294 '&shots...t_idx]': ptr<storage, ShotData, read_write>
            53279..53284 'shots': ref<storage, array<ShotData>, read_write>
            53279..53294 'shots[shot_idx]': ref<storage, ShotData, read_write>
            53285..53293 'shot_idx': u32
            53304..53306 'op': ptr<storage, Op, read>
            53309..53321 '&ops[op_idx]': ptr<storage, Op, read>
            53310..53313 'ops': ref<storage, array<Op>, read>
            53310..53321 'ops[op_idx]': ref<storage, Op, read>
            53314..53320 'op_idx': u32
            53331..53339 'noise_op': ptr<storage, Op, read>
            53342..53357 '&ops[noise_idx]': ptr<storage, Op, read>
            53343..53346 'ops': ref<storage, array<Op>, read>
            53343..53357 'ops[noise_idx]': ref<storage, Op, read>
            53347..53356 'noise_idx': u32
            53664..53668 'rand': ref<function, f32, read_write>
            53671..53675 'shot': ptr<storage, ShotData, read_write>
            53671..53686 'shot.rand_pauli': ref<storage, f32, read_write>
            53696..53703 'q1_term': ref<function, i32, read_write>
            53706..53707 '0': integer
            53717..53724 'q2_term': ref<function, i32, read_write>
            53727..53728 '0': integer
            53824..53825 'a': ref<function, i32, read_write>
            53828..53829 '0': integer
            53831..53832 'a': ref<function, i32, read_write>
            53831..53836 'a < 5': bool
            53835..53836 '5': integer
            53838..53839 'a': ref<function, i32, read_write>
            53842..53843 'a': ref<function, i32, read_write>
            53842..53847 'a + 1': i32
            53846..53847 '1': integer
            53868..53869 'b': ref<function, i32, read_write>
            53872..53873 '0': integer
            53875..53876 'b': ref<function, i32, read_write>
            53875..53880 'b < 5': bool
            53879..53880 '5': integer
            53882..53883 'b': ref<function, i32, read_write>
            53886..53887 'b': ref<function, i32, read_write>
            53886..53891 'b + 1': i32
            53890..53891 '1': integer
            53911..53912 'k': i32
            53915..53916 'a': ref<function, i32, read_write>
            53915..53920 'a * 5': i32
            53915..53924 'a * 5 + b': i32
            53919..53920 '5': integer
            53923..53924 'b': ref<function, i32, read_write>
            53942..53943 'k': i32
            53942..53948 'k == 0': bool
            53947..53948 '0': integer
            54016..54020 'slot': vec2<f32>
            54023..54031 'noise_op': ptr<storage, Op, read>
            54023..54039 'noise_...nitary': ref<storage, array<vec2<f32>, 16>, read>
            54023..54046 'noise_...k / 2]': ref<storage, vec2<f32>, read>
            54040..54041 'k': i32
            54040..54045 'k / 2': i32
            54044..54045 '2': integer
            54064..54068 'p_ab': f32
            54071..54107 'select... == 1)': f32
            54078..54082 'slot': vec2<f32>
            54078..54084 'slot.x': f32
            54086..54090 'slot': vec2<f32>
            54086..54092 'slot.y': f32
            54094..54106 '(k & 1) == 1': bool
            54095..54096 'k': i32
            54095..54100 'k & 1': i32
            54099..54100 '1': integer
            54105..54106 '1': integer
            54125..54129 'rand': ref<function, f32, read_write>
            54125..54136 'rand < p_ab': bool
            54132..54136 'p_ab': f32
            54156..54163 'q1_term': ref<function, i32, read_write>
            54166..54167 'a': ref<function, i32, read_write>
            54185..54192 'q2_term': ref<function, i32, read_write>
            54195..54196 'b': ref<function, i32, read_write>
            54257..54258 'a': ref<function, i32, read_write>
            54261..54262 '5': integer
            54280..54281 'b': ref<function, i32, read_write>
            54284..54285 '5': integer
            54324..54328 'rand': ref<function, f32, read_write>
            54331..54335 'rand': ref<function, f32, read_write>
            54331..54342 'rand - p_ab': f32
            54338..54342 'p_ab': f32
            54526..54533 'q1_term': ref<function, i32, read_write>
            54526..54538 'q1_term == 4': bool
            54537..54538 '4': integer
            54542..54546 'shot': ptr<storage, ShotData, read_write>
            54542..54564 'shot.p...s_mask': ref<storage, u32, read_write>
            54569..54571 '1u': u32
            54569..54577 '1u << q1': u32
            54575..54577 'q1': u32
            54590..54597 'q2_term': ref<function, i32, read_write>
            54590..54602 'q2_term == 4': bool
            54601..54602 '4': integer
            54606..54610 'shot': ptr<storage, ShotData, read_write>
            54606..54628 'shot.p...s_mask': ref<storage, u32, read_write>
            54633..54635 '1u': u32
            54633..54641 '1u << q2': u32
            54639..54641 'q2': u32
            54817..54825 'q1_pauli': bool
            54828..54835 'q1_term': ref<function, i32, read_write>
            54828..54840 'q1_term >= 1': bool
            54828..54856 'q1_ter...m <= 3': bool
            54839..54840 '1': integer
            54844..54851 'q1_term': ref<function, i32, read_write>
            54844..54856 'q1_term <= 3': bool
            54855..54856 '3': integer
            54866..54874 'q2_pauli': bool
            54877..54884 'q2_term': ref<function, i32, read_write>
            54877..54889 'q2_term >= 1': bool
            54877..54905 'q2_ter...m <= 3': bool
            54888..54889 '1': integer
            54893..54900 'q2_term': ref<function, i32, read_write>
            54893..54905 'q2_term <= 3': bool
            54904..54905 '3': integer
            54916..54924 'q1_pauli': bool
            54916..54936 'q1_pau..._pauli': bool
            54928..54936 'q2_pauli': bool
            54999..55007 'op_row_0': ref<function, array<vec2<f32>, 4>, read_write>
            55010..55029 'getOpR...dx, 0)': array<vec2<f32>, 4>
            55019..55025 'op_idx': u32
            55027..55028 '0': integer
            55043..55051 'op_row_1': ref<function, array<vec2<f32>, 4>, read_write>
            55054..55073 'getOpR...dx, 1)': array<vec2<f32>, 4>
            55063..55069 'op_idx': u32
            55071..55072 '1': integer
            55087..55095 'op_row_2': ref<function, array<vec2<f32>, 4>, read_write>
            55098..55117 'getOpR...dx, 2)': array<vec2<f32>, 4>
            55107..55113 'op_idx': u32
            55115..55116 '2': integer
            55131..55139 'op_row_3': ref<function, array<vec2<f32>, 4>, read_write>
            55142..55161 'getOpR...dx, 3)': array<vec2<f32>, 4>
            55151..55157 'op_idx': u32
            55159..55160 '3': integer
            55639..55646 'q1_term': ref<function, i32, read_write>
            55639..55651 'q1_term == 1': bool
            55650..55651 '1': integer
            55710..55719 'old_row_0': array<vec2<f32>, 4>
            55722..55730 'op_row_0': ref<function, array<vec2<f32>, 4>, read_write>
            55748..55757 'old_row_1': array<vec2<f32>, 4>
            55760..55768 'op_row_1': ref<function, array<vec2<f32>, 4>, read_write>
            55782..55790 'op_row_0': ref<function, array<vec2<f32>, 4>, read_write>
            55793..55801 'op_row_2': ref<function, array<vec2<f32>, 4>, read_write>
            55815..55823 'op_row_1': ref<function, array<vec2<f32>, 4>, read_write>
            55826..55834 'op_row_3': ref<function, array<vec2<f32>, 4>, read_write>
            55848..55856 'op_row_2': ref<function, array<vec2<f32>, 4>, read_write>
            55859..55868 'old_row_0': array<vec2<f32>, 4>
            55882..55890 'op_row_3': ref<function, array<vec2<f32>, 4>, read_write>
            55893..55902 'old_row_1': array<vec2<f32>, 4>
            55994..56003 'old_row_0': array<vec2<f32>, 4>
            56006..56014 'op_row_0': ref<function, array<vec2<f32>, 4>, read_write>
            56032..56041 'old_row_1': array<vec2<f32>, 4>
            56044..56052 'op_row_1': ref<function, array<vec2<f32>, 4>, read_write>
            56066..56074 'op_row_0': ref<function, array<vec2<f32>, 4>, read_write>
            56077..56093 'rowNeg...row_2)': array<vec2<f32>, 4>
            56084..56092 'op_row_2': ref<function, array<vec2<f32>, 4>, read_write>
            56107..56115 'op_row_1': ref<function, array<vec2<f32>, 4>, read_write>
            56118..56134 'rowNeg...row_3)': array<vec2<f32>, 4>
            56125..56133 'op_row_3': ref<function, array<vec2<f32>, 4>, read_write>
            56148..56156 'op_row_2': ref<function, array<vec2<f32>, 4>, read_write>
            56159..56168 'old_row_0': array<vec2<f32>, 4>
            56182..56190 'op_row_3': ref<function, array<vec2<f32>, 4>, read_write>
            56193..56202 'old_row_1': array<vec2<f32>, 4>
            56286..56294 'op_row_2': ref<function, array<vec2<f32>, 4>, read_write>
            56297..56313 'rowNeg...row_2)': array<vec2<f32>, 4>
            56304..56312 'op_row_2': ref<function, array<vec2<f32>, 4>, read_write>
            56327..56335 'op_row_3': ref<function, array<vec2<f32>, 4>, read_write>
            56338..56354 'rowNeg...row_3)': array<vec2<f32>, 4>
            56345..56353 'op_row_3': ref<function, array<vec2<f32>, 4>, read_write>
            56425..56432 'q2_term': ref<function, i32, read_write>
            56425..56437 'q2_term == 1': bool
            56436..56437 '1': integer
            56496..56505 'old_row_0': array<vec2<f32>, 4>
            56508..56516 'op_row_0': ref<function, array<vec2<f32>, 4>, read_write>
            56534..56543 'old_row_2': array<vec2<f32>, 4>
            56546..56554 'op_row_2': ref<function, array<vec2<f32>, 4>, read_write>
            56568..56576 'op_row_0': ref<function, array<vec2<f32>, 4>, read_write>
            56579..56587 'op_row_1': ref<function, array<vec2<f32>, 4>, read_write>
            56601..56609 'op_row_2': ref<function, array<vec2<f32>, 4>, read_write>
            56612..56620 'op_row_3': ref<function, array<vec2<f32>, 4>, read_write>
            56634..56642 'op_row_1': ref<function, array<vec2<f32>, 4>, read_write>
            56645..56654 'old_row_0': array<vec2<f32>, 4>
            56668..56676 'op_row_3': ref<function, array<vec2<f32>, 4>, read_write>
            56679..56688 'old_row_2': array<vec2<f32>, 4>
            56780..56789 'old_row_0': array<vec2<f32>, 4>
            56792..56800 'op_row_0': ref<function, array<vec2<f32>, 4>, read_write>
            56818..56827 'old_row_2': array<vec2<f32>, 4>
            56830..56838 'op_row_2': ref<function, array<vec2<f32>, 4>, read_write>
            56852..56860 'op_row_0': ref<function, array<vec2<f32>, 4>, read_write>
            56863..56879 'rowNeg...row_1)': array<vec2<f32>, 4>
            56870..56878 'op_row_1': ref<function, array<vec2<f32>, 4>, read_write>
            56893..56901 'op_row_2': ref<function, array<vec2<f32>, 4>, read_write>
            56904..56920 'rowNeg...row_3)': array<vec2<f32>, 4>
            56911..56919 'op_row_3': ref<function, array<vec2<f32>, 4>, read_write>
            56934..56942 'op_row_1': ref<function, array<vec2<f32>, 4>, read_write>
            56945..56954 'old_row_0': array<vec2<f32>, 4>
            56968..56976 'op_row_3': ref<function, array<vec2<f32>, 4>, read_write>
            56979..56988 'old_row_2': array<vec2<f32>, 4>
            57072..57080 'op_row_1': ref<function, array<vec2<f32>, 4>, read_write>
            57083..57099 'rowNeg...row_1)': array<vec2<f32>, 4>
            57090..57098 'op_row_1': ref<function, array<vec2<f32>, 4>, read_write>
            57113..57121 'op_row_3': ref<function, array<vec2<f32>, 4>, read_write>
            57124..57140 'rowNeg...row_3)': array<vec2<f32>, 4>
            57131..57139 'op_row_3': ref<function, array<vec2<f32>, 4>, read_write>
            57218..57255 'setUni...row_0)': [error]
            57232..57240 'shot_idx': u32
            57242..57244 '0u': u32
            57246..57254 'op_row_0': ref<function, array<vec2<f32>, 4>, read_write>
            57265..57302 'setUni...row_1)': [error]
            57279..57287 'shot_idx': u32
            57289..57291 '1u': u32
            57293..57301 'op_row_1': ref<function, array<vec2<f32>, 4>, read_write>
            57312..57349 'setUni...row_2)': [error]
            57326..57334 'shot_idx': u32
            57336..57338 '2u': u32
            57340..57348 'op_row_2': ref<function, array<vec2<f32>, 4>, read_write>
            57359..57396 'setUni...row_3)': [error]
            57373..57381 'shot_idx': u32
            57383..57385 '3u': u32
            57387..57395 'op_row_3': ref<function, array<vec2<f32>, 4>, read_write>
            57406..57410 'shot': ptr<storage, ShotData, read_write>
            57406..57418 'shot.op_type': ref<storage, u32, read_write>
            57421..57438 'OPID_S...UFF_2Q': u32
            57596..57598 'op': ptr<storage, Op, read>
            57596..57601 'op.id': ref<storage, u32, read>
            57596..57612 'op.id ...PID_CX': bool
            57596..57632 'op.id ...PID_CY': bool
            57596..57652 'op.id ...PID_CZ': bool
            57596..57673 'op.id ...ID_RZZ': bool
            57605..57612 'OPID_CX': u32
            57616..57618 'op': ptr<storage, Op, read>
            57616..57621 'op.id': ref<storage, u32, read>
            57616..57632 'op.id ...PID_CY': bool
            57625..57632 'OPID_CY': u32
            57636..57638 'op': ptr<storage, Op, read>
            57636..57641 'op.id': ref<storage, u32, read>
            57636..57652 'op.id ...PID_CZ': bool
            57645..57652 'OPID_CZ': u32
            57656..57658 'op': ptr<storage, Op, read>
            57656..57661 'op.id': ref<storage, u32, read>
            57656..57673 'op.id ...ID_RZZ': bool
            57665..57673 'OPID_RZZ': u32
            57689..57693 'shot': ptr<storage, ShotData, read_write>
            57689..57701 'shot.op_type': ref<storage, u32, read_write>
            57704..57706 'op': ptr<storage, Op, read>
            57704..57709 'op.id': ref<storage, u32, read>
            57740..57744 'shot': ptr<storage, ShotData, read_write>
            57740..57752 'shot.op_type': ref<storage, u32, read_write>
            57755..57772 'OPID_S...UFF_2Q': u32
            57794..57798 'shot': ptr<storage, ShotData, read_write>
            57794..57805 'shot.op_idx': ref<storage, u32, read_write>
            57808..57814 'op_idx': u32
            57824..57828 'shot': ptr<storage, ShotData, read_write>
            57824..57836 'shot.op_type': ref<storage, u32, read_write>
            57824..57847 'shot.o...PID_CZ': bool
            57824..57875 'shot.o...ID_RZZ': bool
            57840..57847 'OPID_CZ': u32
            57851..57855 'shot': ptr<storage, ShotData, read_write>
            57851..57863 'shot.op_type': ref<storage, u32, read_write>
            57851..57875 'shot.o...ID_RZZ': bool
            57867..57875 'OPID_RZZ': u32
            57887..57891 'shot': ptr<storage, ShotData, read_write>
            57887..57919 'shot.q...p_mask': ref<storage, u32, read_write>
            57922..57924 '0u': u32
            57948..57952 'shot': ptr<storage, ShotData, read_write>
            57948..57980 'shot.q...p_mask': ref<storage, u32, read_write>
            57983..58007 '(1u <<...<< q2)': u32
            57984..57986 '1u': u32
            57984..57992 '1u << q1': u32
            57990..57992 'q1': u32
            57998..58000 '1u': u32
            57998..58006 '1u << q2': u32
            58004..58006 'q2': u32
            58505..58513 'shot_idx': u32
            58520..58532 'target_is_q2': bool
            58540..58544 'term': u32
            58561..58563 'si': i32
            58566..58579 'i32(shot_idx)': i32
            58570..58578 'shot_idx': u32
            58589..58594 'row_0': ref<function, array<vec2<f32>, 4>, read_write>
            58597..58618 'getUni...i, 0u)': array<vec2<f32>, 4>
            58611..58613 'si': i32
            58615..58617 '0u': u32
            58628..58633 'row_1': ref<function, array<vec2<f32>, 4>, read_write>
            58636..58657 'getUni...i, 1u)': array<vec2<f32>, 4>
            58650..58652 'si': i32
            58654..58656 '1u': u32
            58667..58672 'row_2': ref<function, array<vec2<f32>, 4>, read_write>
            58675..58696 'getUni...i, 2u)': array<vec2<f32>, 4>
            58689..58691 'si': i32
            58693..58695 '2u': u32
            58706..58711 'row_3': ref<function, array<vec2<f32>, 4>, read_write>
            58714..58735 'getUni...i, 3u)': array<vec2<f32>, 4>
            58728..58730 'si': i32
            58732..58734 '3u': u32
            58746..58759 '!target_is_q2': bool
            58747..58759 'target_is_q2': bool
            58833..58837 'term': u32
            58833..58843 'term == 1u': bool
            58841..58843 '1u': u32
            58879..58881 'o0': array<vec2<f32>, 4>
            58884..58889 'row_0': ref<function, array<vec2<f32>, 4>, read_write>
            58895..58897 'o1': array<vec2<f32>, 4>
            58900..58905 'row_1': ref<function, array<vec2<f32>, 4>, read_write>
            58919..58924 'row_0': ref<function, array<vec2<f32>, 4>, read_write>
            58927..58932 'row_2': ref<function, array<vec2<f32>, 4>, read_write>
            58934..58939 'row_1': ref<function, array<vec2<f32>, 4>, read_write>
            58942..58947 'row_3': ref<function, array<vec2<f32>, 4>, read_write>
            58961..58966 'row_2': ref<function, array<vec2<f32>, 4>, read_write>
            58969..58971 'o0': array<vec2<f32>, 4>
            58976..58981 'row_3': ref<function, array<vec2<f32>, 4>, read_write>
            58984..58986 'o1': array<vec2<f32>, 4>
            59046..59048 'o0': array<vec2<f32>, 4>
            59051..59056 'row_0': ref<function, array<vec2<f32>, 4>, read_write>
            59062..59064 'o1': array<vec2<f32>, 4>
            59067..59072 'row_1': ref<function, array<vec2<f32>, 4>, read_write>
            59086..59091 'row_0': ref<function, array<vec2<f32>, 4>, read_write>
            59094..59107 'rowNeg(row_2)': array<vec2<f32>, 4>
            59101..59106 'row_2': ref<function, array<vec2<f32>, 4>, read_write>
            59109..59114 'row_1': ref<function, array<vec2<f32>, 4>, read_write>
            59117..59130 'rowNeg(row_3)': array<vec2<f32>, 4>
            59124..59129 'row_3': ref<function, array<vec2<f32>, 4>, read_write>
            59144..59149 'row_2': ref<function, array<vec2<f32>, 4>, read_write>
            59152..59154 'o0': array<vec2<f32>, 4>
            59167..59172 'row_3': ref<function, array<vec2<f32>, 4>, read_write>
            59175..59177 'o1': array<vec2<f32>, 4>
            59233..59238 'row_2': ref<function, array<vec2<f32>, 4>, read_write>
            59241..59254 'rowNeg(row_2)': array<vec2<f32>, 4>
            59248..59253 'row_2': ref<function, array<vec2<f32>, 4>, read_write>
            59256..59261 'row_3': ref<function, array<vec2<f32>, 4>, read_write>
            59264..59277 'rowNeg(row_3)': array<vec2<f32>, 4>
            59271..59276 'row_3': ref<function, array<vec2<f32>, 4>, read_write>
            59371..59375 'term': u32
            59371..59381 'term == 1u': bool
            59379..59381 '1u': u32
            59417..59419 'o0': array<vec2<f32>, 4>
            59422..59427 'row_0': ref<function, array<vec2<f32>, 4>, read_write>
            59433..59435 'o2': array<vec2<f32>, 4>
            59438..59443 'row_2': ref<function, array<vec2<f32>, 4>, read_write>
            59457..59462 'row_0': ref<function, array<vec2<f32>, 4>, read_write>
            59465..59470 'row_1': ref<function, array<vec2<f32>, 4>, read_write>
            59472..59477 'row_2': ref<function, array<vec2<f32>, 4>, read_write>
            59480..59485 'row_3': ref<function, array<vec2<f32>, 4>, read_write>
            59499..59504 'row_1': ref<function, array<vec2<f32>, 4>, read_write>
            59507..59509 'o0': array<vec2<f32>, 4>
            59514..59519 'row_3': ref<function, array<vec2<f32>, 4>, read_write>
            59522..59524 'o2': array<vec2<f32>, 4>
            59584..59586 'o0': array<vec2<f32>, 4>
            59589..59594 'row_0': ref<function, array<vec2<f32>, 4>, read_write>
            59600..59602 'o2': array<vec2<f32>, 4>
            59605..59610 'row_2': ref<function, array<vec2<f32>, 4>, read_write>
            59624..59629 'row_0': ref<function, array<vec2<f32>, 4>, read_write>
            59632..59645 'rowNeg(row_1)': array<vec2<f32>, 4>
            59639..59644 'row_1': ref<function, array<vec2<f32>, 4>, read_write>
            59647..59652 'row_2': ref<function, array<vec2<f32>, 4>, read_write>
            59655..59668 'rowNeg(row_3)': array<vec2<f32>, 4>
            59662..59667 'row_3': ref<function, array<vec2<f32>, 4>, read_write>
            59682..59687 'row_1': ref<function, array<vec2<f32>, 4>, read_write>
            59690..59692 'o0': array<vec2<f32>, 4>
            59705..59710 'row_3': ref<function, array<vec2<f32>, 4>, read_write>
            59713..59715 'o2': array<vec2<f32>, 4>
            59771..59776 'row_1': ref<function, array<vec2<f32>, 4>, read_write>
            59779..59792 'rowNeg(row_1)': array<vec2<f32>, 4>
            59786..59791 'row_1': ref<function, array<vec2<f32>, 4>, read_write>
            59794..59799 'row_3': ref<function, array<vec2<f32>, 4>, read_write>
            59802..59815 'rowNeg(row_3)': array<vec2<f32>, 4>
            59809..59814 'row_3': ref<function, array<vec2<f32>, 4>, read_write>
            59838..59872 'setUni...row_0)': [error]
            59852..59860 'shot_idx': u32
            59862..59864 '0u': u32
            59866..59871 'row_0': ref<function, array<vec2<f32>, 4>, read_write>
            59878..59912 'setUni...row_1)': [error]
            59892..59900 'shot_idx': u32
            59902..59904 '1u': u32
            59906..59911 'row_1': ref<function, array<vec2<f32>, 4>, read_write>
            59918..59952 'setUni...row_2)': [error]
            59932..59940 'shot_idx': u32
            59942..59944 '2u': u32
            59946..59951 'row_2': ref<function, array<vec2<f32>, 4>, read_write>
            59958..59992 'setUni...row_3)': [error]
            59972..59980 'shot_idx': u32
            59982..59984 '3u': u32
            59986..59991 'row_3': ref<function, array<vec2<f32>, 4>, read_write>
            60639..60647 'shot_idx': u32
            60654..60660 'op_idx': u32
            60667..60676 'noise_idx': u32
            60683..60685 'q1': u32
            60692..60694 'q2': u32
            60711..60715 'shot': ptr<storage, ShotData, read_write>
            60718..60734 '&shots...t_idx]': ptr<storage, ShotData, read_write>
            60719..60724 'shots': ref<storage, array<ShotData>, read_write>
            60719..60734 'shots[shot_idx]': ref<storage, ShotData, read_write>
            60725..60733 'shot_idx': u32
            60744..60752 'noise_op': ptr<storage, Op, read>
            60755..60770 '&ops[noise_idx]': ptr<storage, Op, read>
            60756..60759 'ops': ref<storage, array<Op>, read>
            60756..60770 'ops[noise_idx]': ref<storage, Op, read>
            60760..60769 'noise_idx': u32
            60855..60863 'q1_alive': [error]
            60866..60870 'shot': ptr<storage, ShotData, read_write>
            60866..60882 'shot.q..._state': ref<storage, [error], read_write>
            60866..60886 'shot.q...te[q1]': [error]
            60866..60891 'shot.q...].heat': [error]
            60866..60899 'shot.q...= -1.0': [error]
            60883..60885 'q1': u32
            60895..60899 '-1.0': float
            60896..60899 '1.0': float
            60909..60917 'q2_alive': [error]
            60920..60924 'shot': ptr<storage, ShotData, read_write>
            60920..60936 'shot.q..._state': ref<storage, [error], read_write>
            60920..60940 'shot.q...te[q2]': [error]
            60920..60945 'shot.q...].heat': [error]
            60920..60953 'shot.q...= -1.0': [error]
            60937..60939 'q2': u32
            60949..60953 '-1.0': float
            60950..60953 '1.0': float
            61039..61048 '!q1_alive': [error]
            61039..61061 '!q1_al..._alive': [error]
            61040..61048 'q1_alive': [error]
            61052..61061 '!q2_alive': [error]
            61053..61061 'q2_alive': [error]
            61227..61231 'rand': ref<function, f32, read_write>
            61234..61238 'shot': ptr<storage, ShotData, read_write>
            61234..61249 'shot.rand_pauli': ref<storage, f32, read_write>
            61259..61266 'q1_term': ref<function, i32, read_write>
            61269..61270 '0': integer
            61280..61287 'q2_term': ref<function, i32, read_write>
            61290..61291 '0': integer
            61306..61307 'a': ref<function, i32, read_write>
            61310..61311 '0': integer
            61313..61314 'a': ref<function, i32, read_write>
            61313..61318 'a < 5': bool
            61317..61318 '5': integer
            61320..61321 'a': ref<function, i32, read_write>
            61324..61325 'a': ref<function, i32, read_write>
            61324..61329 'a + 1': i32
            61328..61329 '1': integer
            61350..61351 'b': ref<function, i32, read_write>
            61354..61355 '0': integer
            61357..61358 'b': ref<function, i32, read_write>
            61357..61362 'b < 5': bool
            61361..61362 '5': integer
            61364..61365 'b': ref<function, i32, read_write>
            61368..61369 'b': ref<function, i32, read_write>
            61368..61373 'b + 1': i32
            61372..61373 '1': integer
            61393..61394 'k': i32
            61397..61398 'a': ref<function, i32, read_write>
            61397..61402 'a * 5': i32
            61397..61406 'a * 5 + b': i32
            61401..61402 '5': integer
            61405..61406 'b': ref<function, i32, read_write>
            61424..61425 'k': i32
            61424..61430 'k == 0': bool
            61429..61430 '0': integer
            61462..61466 'slot': vec2<f32>
            61469..61477 'noise_op': ptr<storage, Op, read>
            61469..61485 'noise_...nitary': ref<storage, array<vec2<f32>, 16>, read>
            61469..61492 'noise_...k / 2]': ref<storage, vec2<f32>, read>
            61486..61487 'k': i32
            61486..61491 'k / 2': i32
            61490..61491 '2': integer
            61510..61514 'p_ab': f32
            61517..61553 'select... == 1)': f32
            61524..61528 'slot': vec2<f32>
            61524..61530 'slot.x': f32
            61532..61536 'slot': vec2<f32>
            61532..61538 'slot.y': f32
            61540..61552 '(k & 1) == 1': bool
            61541..61542 'k': i32
            61541..61546 'k & 1': i32
            61545..61546 '1': integer
            61551..61552 '1': integer
            61571..61575 'rand': ref<function, f32, read_write>
            61571..61582 'rand < p_ab': bool
            61578..61582 'p_ab': f32
            61602..61609 'q1_term': ref<function, i32, read_write>
            61612..61613 'a': ref<function, i32, read_write>
            61631..61638 'q2_term': ref<function, i32, read_write>
            61641..61642 'b': ref<function, i32, read_write>
            61660..61661 'a': ref<function, i32, read_write>
            61664..61665 '5': integer
            61683..61684 'b': ref<function, i32, read_write>
            61687..61688 '5': integer
            61727..61731 'rand': ref<function, f32, read_write>
            61734..61738 'rand': ref<function, f32, read_write>
            61734..61745 'rand - p_ab': f32
            61741..61745 'p_ab': f32
            61855..61869 'survivor_is_q2': [error]
            61872..61881 '!q1_alive': [error]
            61873..61881 'q1_alive': [error]
            61891..61899 'survivor': [error]
            61902..61932 'select...is_q2)': [error]
            61909..61911 'q1': u32
            61913..61915 'q2': u32
            61917..61931 'survivor_is_q2': [error]
            61942..61946 'term': [error]
            61949..61989 'select...is_q2)': [error]
            61956..61963 'q1_term': ref<function, i32, read_write>
            61965..61972 'q2_term': ref<function, i32, read_write>
            61974..61988 'survivor_is_q2': [error]
            62160..62164 'term': [error]
            62160..62169 'term == 4': [error]
            62168..62169 '4': integer
            62181..62185 'shot': ptr<storage, ShotData, read_write>
            62181..62203 'shot.p...s_mask': ref<storage, u32, read_write>
            62208..62210 '1u': u32
            62208..62222 '1u << survivor': [error]
            62214..62222 'survivor': [error]
            62330..62334 'term': [error]
            62330..62339 'term == 0': [error]
            62338..62339 '0': integer
            62428..62432 'shot': ptr<storage, ShotData, read_write>
            62428..62440 'shot.op_type': ref<storage, u32, read_write>
            62428..62461 'shot.o...UFF_2Q': bool
            62444..62461 'OPID_S...UFF_2Q': u32
            62583..62649 'fuse_1...term))': [error]
            62613..62621 'shot_idx': u32
            62623..62637 'survivor_is_q2': [error]
            62639..62648 'u32(term)': [error]
            62643..62647 'term': [error]
            62910..62914 'term': [error]
            62910..62919 'term == 1': [error]
            62918..62919 '1': integer
            62947..63095 'set_1q... 0.0))': [error]
            62970..62978 'shot_idx': u32
            62980..62994 'survivor_is_q2': [error]
            63012..63027 'vec2f(0.0, 0.0)': vec2<f32>
            63018..63021 '0.0': float
            63023..63026 '0.0': float
            63029..63044 'vec2f(1.0, 0.0)': vec2<f32>
            63035..63038 '1.0': float
            63040..63043 '0.0': float
            63062..63077 'vec2f(1.0, 0.0)': vec2<f32>
            63068..63071 '1.0': float
            63073..63076 '0.0': float
            63079..63094 'vec2f(0.0, 0.0)': vec2<f32>
            63085..63088 '0.0': float
            63090..63093 '0.0': float
            63169..63318 'set_1q... 0.0))': [error]
            63192..63200 'shot_idx': u32
            63202..63216 'survivor_is_q2': [error]
            63234..63249 'vec2f(0.0, 0.0)': vec2<f32>
            63240..63243 '0.0': float
            63245..63248 '0.0': float
            63251..63267 'vec2f(..., 0.0)': vec2<f32>
            63257..63261 '-1.0': float
            63258..63261 '1.0': float
            63263..63266 '0.0': float
            63285..63300 'vec2f(1.0, 0.0)': vec2<f32>
            63291..63294 '1.0': float
            63296..63299 '0.0': float
            63302..63317 'vec2f(0.0, 0.0)': vec2<f32>
            63308..63311 '0.0': float
            63313..63316 '0.0': float
            63369..63518 'set_1q... 0.0))': [error]
            63392..63400 'shot_idx': u32
            63402..63416 'survivor_is_q2': [error]
            63434..63449 'vec2f(1.0, 0.0)': vec2<f32>
            63440..63443 '1.0': float
            63445..63448 '0.0': float
            63451..63466 'vec2f(0.0, 0.0)': vec2<f32>
            63457..63460 '0.0': float
            63462..63465 '0.0': float
            63484..63499 'vec2f(0.0, 0.0)': vec2<f32>
            63490..63493 '0.0': float
            63495..63498 '0.0': float
            63501..63517 'vec2f(..., 0.0)': vec2<f32>
            63507..63511 '-1.0': float
            63508..63511 '1.0': float
            63513..63516 '0.0': float
            63538..63585 'finish...1, q2)': [error]
            63560..63568 'shot_idx': u32
            63570..63576 'op_idx': u32
            63578..63580 'q1': u32
            63582..63584 'q2': u32
            63841..63845 'shot': ptr<storage, ShotData, read_write>
            63841..63861 'shot.q...0_mask': ref<storage, u32, read_write>
            63864..63868 'shot': ptr<storage, ShotData, read_write>
            63864..63884 'shot.q...0_mask': ref<storage, u32, read_write>
            63864..63904 'shot.q...vivor)': [error]
            63887..63904 '~(1u <...vivor)': [error]
            63889..63891 '1u': u32
            63889..63903 '1u << survivor': [error]
            63895..63903 'survivor': [error]
            63910..63914 'shot': ptr<storage, ShotData, read_write>
            63910..63930 'shot.q...1_mask': ref<storage, u32, read_write>
            63933..63937 'shot': ptr<storage, ShotData, read_write>
            63933..63953 'shot.q...1_mask': ref<storage, u32, read_write>
            63933..63973 'shot.q...vivor)': [error]
            63956..63973 '~(1u <...vivor)': [error]
            63958..63960 '1u': u32
            63958..63972 '1u << survivor': [error]
            63964..63972 'survivor': [error]
            60866..60886 'shot.q...te[q1]': cannot index into type ref<storage, [error], read_write>
            60920..60940 'shot.q...te[q2]': cannot index into type ref<storage, [error], read_write>
            62208..62222 '1u << survivor': expected u32 but got [error]
            62639..62648 'u32(term)': no constructor for builtin `op_u32_constructor` of type `u32` with parameters `[error]`
            62623..62637 'survivor_is_q2': expected bool but got [error]
            62639..62648 'u32(term)': expected u32 but got [error]
            62980..62994 'survivor_is_q2': expected bool but got [error]
            63202..63216 'survivor_is_q2': expected bool but got [error]
            63402..63416 'survivor_is_q2': expected bool but got [error]
            64054..64065 'workgroupId': u32
            64080..64083 'tid': u32
            64098..64112 'op_qubit_count': i32
            64245..64253 'shot_idx': i32
            64261..64277 'i32(wo...oupId)': i32
            64261..64299 'i32(wo...R_SHOT': i32
            64265..64276 'workgroupId': u32
            64280..64299 'WORKGR...R_SHOT': i32
            64309..64332 'shot_s..._start': i32
            64340..64348 'shot_idx': i32
            64340..64375 'shot_i...OUNT))': i32
            64352..64354 '1i': i32
            64352..64374 '1i << ...COUNT)': i32
            64358..64374 'u32(QU...COUNT)': u32
            64362..64373 'QUBIT_COUNT': i32
            64385..64406 'workgr...n_shot': i32
            64414..64430 'i32(wo...oupId)': i32
            64414..64452 'i32(wo...R_SHOT': i32
            64418..64429 'workgroupId': u32
            64433..64452 'WORKGR...R_SHOT': i32
            64462..64480 'thread...n_shot': i32
            64488..64509 'workgr...n_shot': i32
            64488..64533 'workgr...KGROUP': i32
            64488..64544 'workgr...2(tid)': i32
            64512..64533 'THREAD...KGROUP': i32
            64536..64544 'i32(tid)': i32
            64540..64543 'tid': u32
            64554..64576 'total_...r_shot': i32
            64584..64603 'WORKGR...R_SHOT': i32
            64584..64627 'WORKGR...KGROUP': i32
            64606..64627 'THREAD...KGROUP': i32
            64967..64990 'workgr...on_idx': i32
            64998..65051 'select...T > 1)': i32
            65005..65007 '-1': integer
            65006..65007 '1': integer
            65009..65025 'i32(wo...oupId)': i32
            65013..65024 'workgroupId': u32
            65027..65046 'WORKGR...R_SHOT': i32
            65027..65050 'WORKGR...OT > 1': bool
            65049..65050 '1': integer
            65062..65078 'zero_e..._count': i32
            65086..65133 '(1i <<...count)': i32
            65087..65089 '1i': i32
            65087..65109 '1i << ...COUNT)': i32
            65093..65109 'u32(QU...COUNT)': u32
            65097..65108 'QUBIT_COUNT': i32
            65114..65133 'u32(op...count)': u32
            65118..65132 'op_qubit_count': i32
            65143..65156 'op_iterations': i32
            65164..65180 'zero_e..._count': i32
            65164..65205 'zero_e...r_shot': i32
            65183..65205 'total_...r_shot': i32
            65219..65459 'ShotPa...     )': ShotParams
            65239..65247 'shot_idx': i32
            65257..65280 'shot_s..._start': i32
            65290..65313 'workgr...on_idx': i32
            65323..65344 'workgr...n_shot': i32
            65354..65372 'thread...n_shot': i32
            65382..65404 'total_...r_shot': i32
            65414..65430 'zero_e..._count': i32
            65440..65453 'op_iterations': i32
            65537..65548 'workgroupId': u32
            65555..65558 'tid': u32
            65565..65567 'q1': u32
            65584..65590 'params': ShotParams
            65593..65649 'get_sh...op */)': ShotParams
            65609..65620 'workgroupId': u32
            65622..65625 'tid': u32
            65627..65628 '1': integer
            65659..65663 'shot': ptr<storage, ShotData, read_write>
            65666..65689 '&shots...t_idx]': ptr<storage, ShotData, read_write>
            65667..65672 'shots': ref<storage, array<ShotData>, read_write>
            65667..65689 'shots[...t_idx]': ref<storage, ShotData, read_write>
            65673..65679 'params': ShotParams
            65673..65688 'params.shot_idx': i32
            65699..65704 'scale': f32
            65707..65711 'shot': ptr<storage, ShotData, read_write>
            65707..65723 'shot.r...malize': ref<storage, f32, read_write>
            65733..65740 'lowMask': i32
            65743..65756 '(1 << q1) - 1': integer
            65744..65745 '1': integer
            65744..65751 '1 << q1': integer
            65749..65751 'q1': u32
            65755..65756 '1': integer
            65766..65774 'highMask': i32
            65777..65804 '(1 << ...)) - 1': integer
            65777..65814 '(1 << ...owMask': i32
            65778..65779 '1': integer
            65778..65799 '1 << u...COUNT)': integer
            65783..65799 'u32(QU...COUNT)': u32
            65787..65798 'QUBIT_COUNT': i32
            65803..65804 '1': integer
            65807..65814 'lowMask': i32
            65824..65839 'qubit_is_0_mask': i32
            65842..65885 'i32(sh..._mask)': i32
            65846..65851 'shots': ref<storage, array<ShotData>, read_write>
            65846..65868 'shots[...t_idx]': ref<storage, ShotData, read_write>
            65846..65884 'shots[...0_mask': ref<storage, u32, read_write>
            65852..65858 'params': ShotParams
            65852..65867 'params.shot_idx': i32
            65895..65910 'qubit_is_1_mask': i32
            65913..65956 'i32(sh..._mask)': i32
            65917..65922 'shots': ref<storage, array<ShotData>, read_write>
            65917..65939 'shots[...t_idx]': ref<storage, ShotData, read_write>
            65917..65955 'shots[...1_mask': ref<storage, u32, read_write>
            65923..65929 'params': ShotParams
            65923..65938 'params.shot_idx': i32
            65967..65979 'summed_probs': ref<function, vec4<f32>, read_write>
            65989..65996 'vec4f()': vec4<f32>
            66530..66541 'entry_index': ref<function, i32, read_write>
            66544..66550 'params': ShotParams
            66544..66569 'params...n_shot': i32
            66585..66586 'i': ref<function, i32, read_write>
            66589..66590 '0': integer
            66592..66593 'i': ref<function, i32, read_write>
            66592..66616 'i < pa...ations': bool
            66596..66602 'params': ShotParams
            66596..66616 'params...ations': i32
            66618..66619 'i': ref<function, i32, read_write>
            66637..66644 'offset0': i32
            66652..66709 '(entry... << 1)': i32
            66653..66664 'entry_index': ref<function, i32, read_write>
            66653..66674 'entry_...owMask': i32
            66667..66674 'lowMask': i32
            66679..66708 '(entry...) << 1': i32
            66680..66691 'entry_index': ref<function, i32, read_write>
            66680..66702 'entry_...ghMask': i32
            66694..66702 'highMask': i32
            66707..66708 '1': integer
            66723..66730 'offset1': i32
            66738..66745 'offset0': i32
            66738..66757 'offset...<< q1)': i32
            66749..66750 '1': integer
            66749..66756 '1 << q1': integer
            66754..66756 'q1': u32
            66996..67011 'skip_processing': bool
            67014..67087 '((offs... != 0)': bool
            67015..67047 '(offse...) != 0': bool
            67016..67023 'offset0': i32
            67016..67041 'offset...0_mask': i32
            67026..67041 'qubit_is_0_mask': i32
            67046..67047 '0': integer
            67053..67086 '(~offs...) != 0': bool
            67054..67062 '~offset1': i32
            67054..67080 '~offse...1_mask': i32
            67055..67062 'offset1': i32
            67065..67080 'qubit_is_1_mask': i32
            67085..67086 '0': integer
            67102..67118 '!skip_...essing': bool
            67103..67118 'skip_processing': bool
            67137..67141 'shot': ptr<storage, ShotData, read_write>
            67137..67149 'shot.op_type': ref<storage, u32, read_write>
            67137..67160 'shot.o...PID_RZ': bool
            67153..67160 'OPID_RZ': u32
            67372..67376 'amp1': vec2<f32>
            67386..67397 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            67386..67439 'stateV...fset1]': ref<storage, vec2<f32>, read_write>
            67398..67404 'params': ShotParams
            67398..67428 'params..._start': i32
            67398..67438 'params...ffset1': i32
            67431..67438 'offset1': i32
            67461..67465 'new1': vec2<f32>
            67468..67498 'cplxMu...ry[5])': vec2<f32>
            67476..67480 'amp1': vec2<f32>
            67482..67486 'shot': ptr<storage, ShotData, read_write>
            67482..67494 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            67482..67497 'shot.unitary[5]': ref<storage, vec2<f32>, read_write>
            67495..67496 '5': integer
            67516..67527 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            67516..67569 'stateV...fset1]': ref<storage, vec2<f32>, read_write>
            67528..67534 'params': ShotParams
            67528..67558 'params..._start': i32
            67528..67568 'params...ffset1': i32
            67561..67568 'offset1': i32
            67572..67576 'new1': vec2<f32>
            67619..67623 'amp0': vec2<f32>
            67633..67644 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            67633..67686 'stateV...fset0]': ref<storage, vec2<f32>, read_write>
            67645..67651 'params': ShotParams
            67645..67675 'params..._start': i32
            67645..67685 'params...ffset0': i32
            67678..67685 'offset0': i32
            67708..67712 'amp1': vec2<f32>
            67722..67733 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            67722..67775 'stateV...fset1]': ref<storage, vec2<f32>, read_write>
            67734..67740 'params': ShotParams
            67734..67764 'params..._start': i32
            67734..67774 'params...ffset1': i32
            67767..67774 'offset1': i32
            67798..67802 'new0': vec2<f32>
            67805..67810 'scale': f32
            67805..67878 'scale ...y[1]))': vec2<f32>
            67814..67844 'cplxMu...ry[0])': vec2<f32>
            67814..67877 'cplxMu...ry[1])': vec2<f32>
            67822..67826 'amp0': vec2<f32>
            67828..67832 'shot': ptr<storage, ShotData, read_write>
            67828..67840 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            67828..67843 'shot.unitary[0]': ref<storage, vec2<f32>, read_write>
            67841..67842 '0': integer
            67847..67877 'cplxMu...ry[1])': vec2<f32>
            67855..67859 'amp1': vec2<f32>
            67861..67865 'shot': ptr<storage, ShotData, read_write>
            67861..67873 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            67861..67876 'shot.unitary[1]': ref<storage, vec2<f32>, read_write>
            67874..67875 '1': integer
            67900..67904 'new1': vec2<f32>
            67907..67912 'scale': f32
            67907..67980 'scale ...y[5]))': vec2<f32>
            67916..67946 'cplxMu...ry[4])': vec2<f32>
            67916..67979 'cplxMu...ry[5])': vec2<f32>
            67924..67928 'amp0': vec2<f32>
            67930..67934 'shot': ptr<storage, ShotData, read_write>
            67930..67942 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            67930..67945 'shot.unitary[4]': ref<storage, vec2<f32>, read_write>
            67943..67944 '4': integer
            67949..67979 'cplxMu...ry[5])': vec2<f32>
            67957..67961 'amp1': vec2<f32>
            67963..67967 'shot': ptr<storage, ShotData, read_write>
            67963..67975 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            67963..67978 'shot.unitary[5]': ref<storage, vec2<f32>, read_write>
            67976..67977 '5': integer
            67999..68010 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            67999..68052 'stateV...fset0]': ref<storage, vec2<f32>, read_write>
            68011..68017 'params': ShotParams
            68011..68041 'params..._start': i32
            68011..68051 'params...ffset0': i32
            68044..68051 'offset0': i32
            68055..68059 'new0': vec2<f32>
            68077..68088 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            68077..68130 'stateV...fset1]': ref<storage, vec2<f32>, read_write>
            68089..68095 'params': ShotParams
            68089..68119 'params..._start': i32
            68089..68129 'params...ffset1': i32
            68122..68129 'offset1': i32
            68133..68137 'new1': vec2<f32>
            68159..68163 'shot': ptr<storage, ShotData, read_write>
            68159..68171 'shot.op_type': ref<storage, u32, read_write>
            68159..68187 'shot.o...RESETZ': bool
            68159..68222 'shot.o..._NOISE': bool
            68159..68238 'shot.o...!= 1.0': bool
            68175..68187 'OPID_MRESETZ': u32
            68191..68195 'shot': ptr<storage, ShotData, read_write>
            68191..68203 'shot.op_type': ref<storage, u32, read_write>
            68191..68222 'shot.o..._NOISE': bool
            68207..68222 'OPID_LOSS_NOISE': u32
            68226..68231 'scale': f32
            68226..68238 'scale != 1.0': bool
            68235..68238 '1.0': float
            68370..68417 'update..., tid)': [error]
            68393..68405 'u32(offset0)': u32
            68397..68404 'offset0': i32
            68407..68411 'new0': vec2<f32>
            68413..68416 'tid': u32
            68439..68486 'update..., tid)': [error]
            68462..68474 'u32(offset1)': u32
            68466..68473 'offset1': i32
            68476..68480 'new1': vec2<f32>
            68482..68485 'tid': u32
            68533..68545 'summed_probs': ref<function, vec4<f32>, read_write>
            68533..68548 'summed_probs[0]': ref<function, f32, read_write>
            68546..68547 '0': integer
            68552..68566 'cplxMag2(new0)': f32
            68561..68565 'new0': vec2<f32>
            68588..68600 'summed_probs': ref<function, vec4<f32>, read_write>
            68588..68603 'summed_probs[1]': ref<function, f32, read_write>
            68601..68602 '1': integer
            68607..68621 'cplxMag2(new1)': f32
            68616..68620 'new1': vec2<f32>
            68673..68684 'entry_index': ref<function, i32, read_write>
            68688..68694 'params': ShotParams
            68688..68717 'params...r_shot': i32
            68733..68738 'scale': f32
            68733..68745 'scale == 1.0': bool
            68733..68772 'scale ...PID_RZ': bool
            68733..68804 'scale ...RESETZ': bool
            68733..68839 'scale ..._NOISE': bool
            68742..68745 '1.0': float
            68749..68753 'shot': ptr<storage, ShotData, read_write>
            68749..68761 'shot.op_type': ref<storage, u32, read_write>
            68749..68772 'shot.o...PID_RZ': bool
            68765..68772 'OPID_RZ': u32
            68776..68780 'shot': ptr<storage, ShotData, read_write>
            68776..68788 'shot.op_type': ref<storage, u32, read_write>
            68776..68804 'shot.o...RESETZ': bool
            68792..68804 'OPID_MRESETZ': u32
            68808..68812 'shot': ptr<storage, ShotData, read_write>
            68808..68820 'shot.op_type': ref<storage, u32, read_write>
            68808..68839 'shot.o..._NOISE': bool
            68824..68839 'OPID_LOSS_NOISE': u32
            68933..68951 'qubitP...lities': ref<workgroup, [error], read_write>
            68933..68956 'qubitP...s[tid]': [error]
            68933..68961 'qubitP...].zero': [error]
            68933..68965 'qubitP...ro[q1]': [error]
            68952..68955 'tid': u32
            68962..68964 'q1': u32
            68968..68980 'summed_probs': ref<function, vec4<f32>, read_write>
            68968..68983 'summed_probs[0]': ref<function, f32, read_write>
            68981..68982 '0': integer
            68993..69011 'qubitP...lities': ref<workgroup, [error], read_write>
            68993..69016 'qubitP...s[tid]': [error]
            68993..69020 'qubitP...d].one': [error]
            68993..69024 'qubitP...ne[q1]': [error]
            69012..69015 'tid': u32
            69021..69023 'q1': u32
            69028..69040 'summed_probs': ref<function, vec4<f32>, read_write>
            69028..69043 'summed_probs[1]': ref<function, f32, read_write>
            69041..69042 '1': integer
            68933..68956 'qubitP...s[tid]': cannot index into type ref<workgroup, [error], read_write>
            68933..68965 'qubitP...ro[q1]': cannot assign to non-reference `[error]`
            68993..69016 'qubitP...s[tid]': cannot index into type ref<workgroup, [error], read_write>
            68993..69024 'qubitP...ne[q1]': cannot assign to non-reference `[error]`
            69069..69080 'workgroupId': u32
            69087..69090 'tid': u32
            69097..69099 'q1': u32
            69106..69108 'q2': u32
            69125..69131 'params': ShotParams
            69134..69190 'get_sh...op */)': ShotParams
            69150..69161 'workgroupId': u32
            69163..69166 'tid': u32
            69168..69169 '2': integer
            69200..69204 'shot': ptr<storage, ShotData, read_write>
            69207..69230 '&shots...t_idx]': ptr<storage, ShotData, read_write>
            69208..69213 'shots': ref<storage, array<ShotData>, read_write>
            69208..69230 'shots[...t_idx]': ref<storage, ShotData, read_write>
            69214..69220 'params': ShotParams
            69214..69229 'params.shot_idx': i32
            69240..69252 'update_probs': bool
            69255..69259 'shot': ptr<storage, ShotData, read_write>
            69255..69267 'shot.op_type': ref<storage, u32, read_write>
            69255..69278 'shot.o...PID_CZ': bool
            69255..69306 'shot.o...ID_RZZ': bool
            69271..69278 'OPID_CZ': u32
            69282..69286 'shot': ptr<storage, ShotData, read_write>
            69282..69294 'shot.op_type': ref<storage, u32, read_write>
            69282..69306 'shot.o...ID_RZZ': bool
            69298..69306 'OPID_RZZ': u32
            69529..69537 'lowQubit': u32
            69540..69563 'select... > q2)': u32
            69547..69549 'q1': u32
            69551..69553 'q2': u32
            69555..69557 'q1': u32
            69555..69562 'q1 > q2': bool
            69560..69562 'q2': u32
            69573..69580 'hiQubit': u32
            69583..69606 'select... < q2)': u32
            69590..69592 'q1': u32
            69594..69596 'q2': u32
            69598..69600 'q1': u32
            69598..69605 'q1 < q2': bool
            69603..69605 'q2': u32
            69655..69666 'lowBitCount': u32
            69669..69677 'lowQubit': u32
            69687..69698 'midBitCount': u32
            69701..69708 'hiQubit': u32
            69701..69719 'hiQubi...wQubit': u32
            69701..69723 'hiQubi...it - 1': u32
            69711..69719 'lowQubit': u32
            69722..69723 '1': integer
            69733..69743 'hiBitCount': u32
            69746..69762 'u32(QU...COUNT)': u32
            69746..69772 'u32(QU...iQubit': u32
            69746..69776 'u32(QU...it - 1': u32
            69750..69761 'QUBIT_COUNT': i32
            69765..69772 'hiQubit': u32
            69775..69776 '1': integer
            69907..69914 'lowMask': i32
            69917..69939 '(1 << ...t) - 1': integer
            69918..69919 '1': integer
            69918..69934 '1 << l...tCount': integer
            69923..69934 'lowBitCount': u32
            69938..69939 '1': integer
            69949..69956 'midMask': i32
            69959..69997 '(1 << ...)) - 1': integer
            69959..70007 '(1 << ...owMask': i32
            69960..69961 '1': integer
            69960..69992 '1 << (...Count)': integer
            69966..69977 'lowBitCount': u32
            69966..69991 'lowBit...tCount': u32
            69980..69991 'midBitCount': u32
            69996..69997 '1': integer
            70000..70007 'lowMask': i32
            70017..70023 'hiMask': i32
            70026..70053 '(1 << ...)) - 1': integer
            70026..70063 '(1 << ...idMask': i32
            70026..70073 '(1 << ...owMask': i32
            70027..70028 '1': integer
            70027..70048 '1 << u...COUNT)': integer
            70032..70048 'u32(QU...COUNT)': u32
            70036..70047 'QUBIT_COUNT': i32
            70052..70053 '1': integer
            70056..70063 'midMask': i32
            70066..70073 'lowMask': i32
            70214..70225 'entry_index': ref<function, i32, read_write>
            70228..70234 'params': ShotParams
            70228..70253 'params...n_shot': i32
            70263..70275 'summed_probs': ref<function, vec4<f32>, read_write>
            70285..70292 'vec4f()': vec4<f32>
            70308..70309 'i': ref<function, i32, read_write>
            70312..70313 '0': integer
            70315..70316 'i': ref<function, i32, read_write>
            70315..70339 'i < pa...ations': bool
            70319..70325 'params': ShotParams
            70319..70339 'params...ations': i32
            70341..70342 'i': ref<function, i32, read_write>
            70407..70415 'offset00': i32
            70423..70479 '(entry... << 1)': i32
            70423..70511 '(entry... << 2)': i32
            70424..70435 'entry_index': ref<function, i32, read_write>
            70424..70445 'entry_...owMask': i32
            70438..70445 'lowMask': i32
            70450..70478 '(entry...) << 1': i32
            70451..70462 'entry_index': ref<function, i32, read_write>
            70451..70472 'entry_...idMask': i32
            70465..70472 'midMask': i32
            70477..70478 '1': integer
            70483..70510 '(entry...) << 2': i32
            70484..70495 'entry_index': ref<function, i32, read_write>
            70484..70504 'entry_...hiMask': i32
            70498..70504 'hiMask': i32
            70509..70510 '2': integer
            70525..70533 'offset01': i32
            70541..70549 'offset00': i32
            70541..70561 'offset...<< q2)': i32
            70553..70554 '1': integer
            70553..70560 '1 << q2': integer
            70558..70560 'q2': u32
            70575..70583 'offset10': i32
            70591..70599 'offset00': i32
            70591..70611 'offset...<< q1)': i32
            70603..70604 '1': integer
            70603..70610 '1 << q1': integer
            70608..70610 'q1': u32
            70625..70633 'offset11': i32
            70641..70649 'offset10': i32
            70641..70661 'offset...<< q2)': i32
            70653..70654 '1': integer
            70653..70660 '1 << q2': integer
            70658..70660 'q2': u32
            70676..70695 'can_sk...essing': bool
            70711..70820 '((u32(... != 0)': bool
            70712..70755 '(u32(o...) != 0': bool
            70713..70726 'u32(offset00)': u32
            70713..70749 'u32(of...0_mask': u32
            70717..70725 'offset00': i32
            70729..70733 'shot': ptr<storage, ShotData, read_write>
            70729..70749 'shot.q...0_mask': ref<storage, u32, read_write>
            70754..70755 '0': integer
            70773..70819 '(~(u32...) != 0': bool
            70774..70790 '~(u32(...et11))': u32
            70774..70813 '~(u32(...1_mask': u32
            70776..70789 'u32(offset11)': u32
            70780..70788 'offset11': i32
            70793..70797 'shot': ptr<storage, ShotData, read_write>
            70793..70813 'shot.q...1_mask': ref<storage, u32, read_write>
            70818..70819 '0': integer
            70834..70854 '!can_s...essing': bool
            70835..70854 'can_sk...essing': bool
            70876..70880 'shot': ptr<storage, ShotData, read_write>
            70876..70888 'shot.op_type': ref<storage, u32, read_write>
            70908..70915 'OPID_CZ': u32
            70938..70943 'amp11': vec2<f32>
            70953..70964 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            70953..71007 'stateV...set11]': ref<storage, vec2<f32>, read_write>
            70965..70971 'params': ShotParams
            70965..70995 'params..._start': i32
            70965..71006 'params...fset11': i32
            70998..71006 'offset11': i32
            71025..71036 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            71025..71079 'stateV...set11]': ref<storage, vec2<f32>, read_write>
            71037..71043 'params': ShotParams
            71037..71067 'params..._start': i32
            71037..71078 'params...fset11': i32
            71070..71078 'offset11': i32
            71082..71096 'cplxNeg(amp11)': vec2<f32>
            71090..71095 'amp11': vec2<f32>
            71219..71227 'OPID_RZZ': u32
            71341..71346 'amp01': vec2<f32>
            71356..71367 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            71356..71410 'stateV...set01]': ref<storage, vec2<f32>, read_write>
            71368..71374 'params': ShotParams
            71368..71398 'params..._start': i32
            71368..71409 'params...fset01': i32
            71401..71409 'offset01': i32
            71432..71437 'amp10': vec2<f32>
            71447..71458 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            71447..71501 'stateV...set10]': ref<storage, vec2<f32>, read_write>
            71459..71465 'params': ShotParams
            71459..71489 'params..._start': i32
            71459..71500 'params...fset10': i32
            71492..71500 'offset10': i32
            71625..71636 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            71625..71679 'stateV...set01]': ref<storage, vec2<f32>, read_write>
            71637..71643 'params': ShotParams
            71637..71667 'params..._start': i32
            71637..71678 'params...fset01': i32
            71670..71678 'offset01': i32
            71682..71713 'cplxMu...ry[5])': vec2<f32>
            71690..71695 'amp01': vec2<f32>
            71697..71701 'shot': ptr<storage, ShotData, read_write>
            71697..71709 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            71697..71712 'shot.unitary[5]': ref<storage, vec2<f32>, read_write>
            71710..71711 '5': integer
            71731..71742 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            71731..71785 'stateV...set10]': ref<storage, vec2<f32>, read_write>
            71743..71749 'params': ShotParams
            71743..71773 'params..._start': i32
            71743..71784 'params...fset10': i32
            71776..71784 'offset10': i32
            71788..71820 'cplxMu...y[10])': vec2<f32>
            71796..71801 'amp10': vec2<f32>
            71803..71807 'shot': ptr<storage, ShotData, read_write>
            71803..71815 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            71803..71819 'shot.u...ry[10]': ref<storage, vec2<f32>, read_write>
            71816..71818 '10': integer
            71853..71860 'OPID_CX': u32
            72000..72005 'amp00': vec2<f32>
            72015..72026 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            72015..72069 'stateV...set00]': ref<storage, vec2<f32>, read_write>
            72027..72033 'params': ShotParams
            72027..72057 'params..._start': i32
            72027..72068 'params...fset00': i32
            72060..72068 'offset00': i32
            72091..72096 'amp01': vec2<f32>
            72106..72117 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            72106..72160 'stateV...set01]': ref<storage, vec2<f32>, read_write>
            72118..72124 'params': ShotParams
            72118..72148 'params..._start': i32
            72118..72159 'params...fset01': i32
            72151..72159 'offset01': i32
            72182..72187 'amp10': vec2<f32>
            72197..72208 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            72197..72251 'stateV...set10]': ref<storage, vec2<f32>, read_write>
            72209..72215 'params': ShotParams
            72209..72239 'params..._start': i32
            72209..72250 'params...fset10': i32
            72242..72250 'offset10': i32
            72273..72278 'amp11': vec2<f32>
            72288..72299 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            72288..72342 'stateV...set11]': ref<storage, vec2<f32>, read_write>
            72300..72306 'params': ShotParams
            72300..72330 'params..._start': i32
            72300..72341 'params...fset11': i32
            72333..72341 'offset11': i32
            72360..72371 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            72360..72414 'stateV...set10]': ref<storage, vec2<f32>, read_write>
            72372..72378 'params': ShotParams
            72372..72402 'params..._start': i32
            72372..72413 'params...fset10': i32
            72405..72413 'offset10': i32
            72417..72422 'amp11': vec2<f32>
            72440..72451 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            72440..72494 'stateV...set11]': ref<storage, vec2<f32>, read_write>
            72452..72458 'params': ShotParams
            72452..72482 'params..._start': i32
            72452..72493 'params...fset11': i32
            72485..72493 'offset11': i32
            72497..72502 'amp10': vec2<f32>
            72520..72532 'summed_probs': ref<function, vec4<f32>, read_write>
            72520..72535 'summed_probs[0]': ref<function, f32, read_write>
            72533..72534 '0': integer
            72540..72555 'cplxMag2(amp00)': f32
            72540..72573 'cplxMa...amp01)': f32
            72549..72554 'amp00': vec2<f32>
            72558..72573 'cplxMag2(amp01)': f32
            72567..72572 'amp01': vec2<f32>
            72592..72604 'summed_probs': ref<function, vec4<f32>, read_write>
            72592..72607 'summed_probs[1]': ref<function, f32, read_write>
            72605..72606 '1': integer
            72612..72627 'cplxMag2(amp11)': f32
            72612..72645 'cplxMa...amp10)': f32
            72621..72626 'amp11': vec2<f32>
            72630..72645 'cplxMag2(amp10)': f32
            72639..72644 'amp10': vec2<f32>
            72664..72676 'summed_probs': ref<function, vec4<f32>, read_write>
            72664..72679 'summed_probs[2]': ref<function, f32, read_write>
            72677..72678 '2': integer
            72684..72699 'cplxMag2(amp00)': f32
            72684..72717 'cplxMa...amp11)': f32
            72693..72698 'amp00': vec2<f32>
            72702..72717 'cplxMag2(amp11)': f32
            72711..72716 'amp11': vec2<f32>
            72736..72748 'summed_probs': ref<function, vec4<f32>, read_write>
            72736..72751 'summed_probs[3]': ref<function, f32, read_write>
            72749..72750 '3': integer
            72756..72771 'cplxMag2(amp01)': f32
            72756..72789 'cplxMa...amp10)': f32
            72765..72770 'amp01': vec2<f32>
            72774..72789 'cplxMag2(amp10)': f32
            72783..72788 'amp10': vec2<f32>
            72823..72830 'OPID_CY': u32
            72923..72928 'amp00': vec2<f32>
            72938..72949 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            72938..72992 'stateV...set00]': ref<storage, vec2<f32>, read_write>
            72950..72956 'params': ShotParams
            72950..72980 'params..._start': i32
            72950..72991 'params...fset00': i32
            72983..72991 'offset00': i32
            73014..73019 'amp01': vec2<f32>
            73029..73040 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            73029..73083 'stateV...set01]': ref<storage, vec2<f32>, read_write>
            73041..73047 'params': ShotParams
            73041..73071 'params..._start': i32
            73041..73082 'params...fset01': i32
            73074..73082 'offset01': i32
            73105..73110 'amp10': vec2<f32>
            73120..73131 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            73120..73174 'stateV...set10]': ref<storage, vec2<f32>, read_write>
            73132..73138 'params': ShotParams
            73132..73162 'params..._start': i32
            73132..73173 'params...fset10': i32
            73165..73173 'offset10': i32
            73196..73201 'amp11': vec2<f32>
            73211..73222 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            73211..73265 'stateV...set11]': ref<storage, vec2<f32>, read_write>
            73223..73229 'params': ShotParams
            73223..73253 'params..._start': i32
            73223..73264 'params...fset11': i32
            73256..73264 'offset11': i32
            73283..73294 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            73283..73337 'stateV...set10]': ref<storage, vec2<f32>, read_write>
            73295..73301 'params': ShotParams
            73295..73325 'params..._start': i32
            73295..73336 'params...fset10': i32
            73328..73336 'offset10': i32
            73340..73364 'vec2f(...p11.x)': vec2<f32>
            73346..73351 'amp11': vec2<f32>
            73346..73353 'amp11.y': f32
            73355..73363 '-amp11.x': f32
            73356..73361 'amp11': vec2<f32>
            73356..73363 'amp11.x': f32
            73395..73406 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            73395..73449 'stateV...set11]': ref<storage, vec2<f32>, read_write>
            73407..73413 'params': ShotParams
            73407..73437 'params..._start': i32
            73407..73448 'params...fset11': i32
            73440..73448 'offset11': i32
            73452..73476 'vec2f(...p10.x)': vec2<f32>
            73458..73466 '-amp10.y': f32
            73459..73464 'amp10': vec2<f32>
            73459..73466 'amp10.y': f32
            73468..73473 'amp10': vec2<f32>
            73468..73475 'amp10.x': f32
            73506..73518 'summed_probs': ref<function, vec4<f32>, read_write>
            73506..73521 'summed_probs[0]': ref<function, f32, read_write>
            73519..73520 '0': integer
            73526..73541 'cplxMag2(amp00)': f32
            73526..73559 'cplxMa...amp01)': f32
            73535..73540 'amp00': vec2<f32>
            73544..73559 'cplxMag2(amp01)': f32
            73553..73558 'amp01': vec2<f32>
            73578..73590 'summed_probs': ref<function, vec4<f32>, read_write>
            73578..73593 'summed_probs[1]': ref<function, f32, read_write>
            73591..73592 '1': integer
            73598..73613 'cplxMag2(amp11)': f32
            73598..73631 'cplxMa...amp10)': f32
            73607..73612 'amp11': vec2<f32>
            73616..73631 'cplxMag2(amp10)': f32
            73625..73630 'amp10': vec2<f32>
            73650..73662 'summed_probs': ref<function, vec4<f32>, read_write>
            73650..73665 'summed_probs[2]': ref<function, f32, read_write>
            73663..73664 '2': integer
            73670..73685 'cplxMag2(amp00)': f32
            73670..73703 'cplxMa...amp11)': f32
            73679..73684 'amp00': vec2<f32>
            73688..73703 'cplxMag2(amp11)': f32
            73697..73702 'amp11': vec2<f32>
            73722..73734 'summed_probs': ref<function, vec4<f32>, read_write>
            73722..73737 'summed_probs[3]': ref<function, f32, read_write>
            73735..73736 '3': integer
            73742..73757 'cplxMag2(amp01)': f32
            73742..73775 'cplxMa...amp10)': f32
            73751..73756 'amp01': vec2<f32>
            73760..73775 'cplxMag2(amp10)': f32
            73769..73774 'amp10': vec2<f32>
            73926..73932 'states': array<vec2<f32>, 4>
            73935..74271 'array<...     )': array<vec2<f32>, 4>
            73971..73982 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            73971..74025 'stateV...set00]': ref<storage, vec2<f32>, read_write>
            73983..73989 'params': ShotParams
            73983..74013 'params..._start': i32
            73983..74024 'params...fset00': i32
            74016..74024 'offset00': i32
            74047..74058 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            74047..74101 'stateV...set01]': ref<storage, vec2<f32>, read_write>
            74059..74065 'params': ShotParams
            74059..74089 'params..._start': i32
            74059..74100 'params...fset01': i32
            74092..74100 'offset01': i32
            74123..74134 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            74123..74177 'stateV...set10]': ref<storage, vec2<f32>, read_write>
            74135..74141 'params': ShotParams
            74135..74165 'params..._start': i32
            74135..74176 'params...fset10': i32
            74168..74176 'offset10': i32
            74199..74210 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            74199..74253 'stateV...set11]': ref<storage, vec2<f32>, read_write>
            74211..74217 'params': ShotParams
            74211..74241 'params..._start': i32
            74211..74252 'params...fset11': i32
            74244..74252 'offset11': i32
            74351..74359 'result00': vec2<f32>
            74362..74417 'innerP...tates)': vec2<f32>
            74375..74408 'getUni...dx, 0)': array<vec2<f32>, 4>
            74389..74395 'params': ShotParams
            74389..74404 'params.shot_idx': i32
            74406..74407 '0': integer
            74410..74416 'states': array<vec2<f32>, 4>
            74439..74447 'result01': vec2<f32>
            74450..74505 'innerP...tates)': vec2<f32>
            74463..74496 'getUni...dx, 1)': array<vec2<f32>, 4>
            74477..74483 'params': ShotParams
            74477..74492 'params.shot_idx': i32
            74494..74495 '1': integer
            74498..74504 'states': array<vec2<f32>, 4>
            74527..74535 'result10': vec2<f32>
            74538..74593 'innerP...tates)': vec2<f32>
            74551..74584 'getUni...dx, 2)': array<vec2<f32>, 4>
            74565..74571 'params': ShotParams
            74565..74580 'params.shot_idx': i32
            74582..74583 '2': integer
            74586..74592 'states': array<vec2<f32>, 4>
            74615..74623 'result11': vec2<f32>
            74626..74681 'innerP...tates)': vec2<f32>
            74639..74672 'getUni...dx, 3)': array<vec2<f32>, 4>
            74653..74659 'params': ShotParams
            74653..74668 'params.shot_idx': i32
            74670..74671 '3': integer
            74674..74680 'states': array<vec2<f32>, 4>
            74741..74752 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            74741..74795 'stateV...set00]': ref<storage, vec2<f32>, read_write>
            74753..74759 'params': ShotParams
            74753..74783 'params..._start': i32
            74753..74794 'params...fset00': i32
            74786..74794 'offset00': i32
            74798..74806 'result00': vec2<f32>
            74824..74835 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            74824..74878 'stateV...set01]': ref<storage, vec2<f32>, read_write>
            74836..74842 'params': ShotParams
            74836..74866 'params..._start': i32
            74836..74877 'params...fset01': i32
            74869..74877 'offset01': i32
            74881..74889 'result01': vec2<f32>
            74907..74918 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            74907..74961 'stateV...set10]': ref<storage, vec2<f32>, read_write>
            74919..74925 'params': ShotParams
            74919..74949 'params..._start': i32
            74919..74960 'params...fset10': i32
            74952..74960 'offset10': i32
            74964..74972 'result10': vec2<f32>
            74990..75001 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            74990..75044 'stateV...set11]': ref<storage, vec2<f32>, read_write>
            75002..75008 'params': ShotParams
            75002..75032 'params..._start': i32
            75002..75043 'params...fset11': i32
            75035..75043 'offset11': i32
            75047..75055 'result11': vec2<f32>
            75141..75153 'summed_probs': ref<function, vec4<f32>, read_write>
            75141..75156 'summed_probs[0]': ref<function, f32, read_write>
            75154..75155 '0': integer
            75161..75179 'cplxMa...ult00)': f32
            75161..75200 'cplxMa...ult01)': f32
            75170..75178 'result00': vec2<f32>
            75182..75200 'cplxMa...ult01)': f32
            75191..75199 'result01': vec2<f32>
            75219..75231 'summed_probs': ref<function, vec4<f32>, read_write>
            75219..75234 'summed_probs[1]': ref<function, f32, read_write>
            75232..75233 '1': integer
            75239..75257 'cplxMa...ult10)': f32
            75239..75278 'cplxMa...ult11)': f32
            75248..75256 'result10': vec2<f32>
            75260..75278 'cplxMa...ult11)': f32
            75269..75277 'result11': vec2<f32>
            75297..75309 'summed_probs': ref<function, vec4<f32>, read_write>
            75297..75312 'summed_probs[2]': ref<function, f32, read_write>
            75310..75311 '2': integer
            75317..75335 'cplxMa...ult00)': f32
            75317..75356 'cplxMa...ult10)': f32
            75326..75334 'result00': vec2<f32>
            75338..75356 'cplxMa...ult10)': f32
            75347..75355 'result10': vec2<f32>
            75375..75387 'summed_probs': ref<function, vec4<f32>, read_write>
            75375..75390 'summed_probs[3]': ref<function, f32, read_write>
            75388..75389 '3': integer
            75395..75413 'cplxMa...ult01)': f32
            75395..75434 'cplxMa...ult11)': f32
            75404..75412 'result01': vec2<f32>
            75416..75434 'cplxMa...ult11)': f32
            75425..75433 'result11': vec2<f32>
            75484..75495 'entry_index': ref<function, i32, read_write>
            75499..75505 'params': ShotParams
            75499..75528 'params...r_shot': i32
            75624..75636 'update_probs': bool
            75694..75712 'qubitP...lities': ref<workgroup, [error], read_write>
            75694..75717 'qubitP...s[tid]': [error]
            75694..75722 'qubitP...].zero': [error]
            75694..75726 'qubitP...ro[q1]': [error]
            75713..75716 'tid': u32
            75723..75725 'q1': u32
            75729..75741 'summed_probs': ref<function, vec4<f32>, read_write>
            75729..75744 'summed_probs[0]': ref<function, f32, read_write>
            75742..75743 '0': integer
            75754..75772 'qubitP...lities': ref<workgroup, [error], read_write>
            75754..75777 'qubitP...s[tid]': [error]
            75754..75781 'qubitP...d].one': [error]
            75754..75785 'qubitP...ne[q1]': [error]
            75773..75776 'tid': u32
            75782..75784 'q1': u32
            75789..75801 'summed_probs': ref<function, vec4<f32>, read_write>
            75789..75804 'summed_probs[1]': ref<function, f32, read_write>
            75802..75803 '1': integer
            75814..75832 'qubitP...lities': ref<workgroup, [error], read_write>
            75814..75837 'qubitP...s[tid]': [error]
            75814..75842 'qubitP...].zero': [error]
            75814..75846 'qubitP...ro[q2]': [error]
            75833..75836 'tid': u32
            75843..75845 'q2': u32
            75849..75861 'summed_probs': ref<function, vec4<f32>, read_write>
            75849..75864 'summed_probs[2]': ref<function, f32, read_write>
            75862..75863 '2': integer
            75874..75892 'qubitP...lities': ref<workgroup, [error], read_write>
            75874..75897 'qubitP...s[tid]': [error]
            75874..75901 'qubitP...d].one': [error]
            75874..75905 'qubitP...ne[q2]': [error]
            75893..75896 'tid': u32
            75902..75904 'q2': u32
            75909..75921 'summed_probs': ref<function, vec4<f32>, read_write>
            75909..75924 'summed_probs[3]': ref<function, f32, read_write>
            75922..75923 '3': integer
            75694..75717 'qubitP...s[tid]': cannot index into type ref<workgroup, [error], read_write>
            75694..75726 'qubitP...ro[q1]': cannot assign to non-reference `[error]`
            75754..75777 'qubitP...s[tid]': cannot index into type ref<workgroup, [error], read_write>
            75754..75785 'qubitP...ne[q1]': cannot assign to non-reference `[error]`
            75814..75837 'qubitP...s[tid]': cannot index into type ref<workgroup, [error], read_write>
            75814..75846 'qubitP...ro[q2]': cannot assign to non-reference `[error]`
            75874..75897 'qubitP...s[tid]': cannot index into type ref<workgroup, [error], read_write>
            75874..75905 'qubitP...ne[q2]': cannot assign to non-reference `[error]`
            75961..75972 'workgroupId': u32
            75979..75982 'tid': u32
            75999..76005 'params': ShotParams
            76008..76075 'get_sh...es */)': ShotParams
            76024..76035 'workgroupId': u32
            76037..76040 'tid': u32
            76042..76043 '0': integer
            76246..76250 'shot': ptr<storage, ShotData, read_write>
            76253..76276 '&shots...t_idx]': ptr<storage, ShotData, read_write>
            76254..76259 'shots': ref<storage, array<ShotData>, read_write>
            76254..76276 'shots[...t_idx]': ref<storage, ShotData, read_write>
            76260..76266 'params': ShotParams
            76260..76275 'params.shot_idx': i32
            76387..76400 'bit_flip_mask': u32
            76403..76434 'bitcas...[0].x)': u32
            76416..76420 'shot': ptr<storage, ShotData, read_write>
            76416..76428 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            76416..76431 'shot.unitary[0]': ref<storage, vec2<f32>, read_write>
            76416..76433 'shot.u...y[0].x': ref<storage, f32, read_write>
            76429..76430 '0': integer
            76444..76459 'phase_flip_mask': u32
            76462..76493 'bitcas...[0].y)': u32
            76475..76479 'shot': ptr<storage, ShotData, read_write>
            76475..76487 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            76475..76490 'shot.unitary[0]': ref<storage, vec2<f32>, read_write>
            76475..76492 'shot.u...y[0].y': ref<storage, f32, read_write>
            76488..76489 '0': integer
            76544..76557 'bit_flip_mask': u32
            76544..76563 'bit_fl... == 0u': bool
            76544..76588 'bit_fl... == 0u': bool
            76561..76563 '0u': u32
            76567..76582 'phase_flip_mask': u32
            76567..76588 'phase_... == 0u': bool
            76586..76588 '0u': u32
            76623..76634 'entry_index': ref<function, i32, read_write>
            76637..76643 'params': ShotParams
            76637..76662 'params...n_shot': i32
            76678..76679 'i': ref<function, i32, read_write>
            76682..76683 '0': integer
            76685..76686 'i': ref<function, i32, read_write>
            76685..76709 'i < pa...ations': bool
            76689..76695 'params': ShotParams
            76689..76709 'params...ations': i32
            76711..76712 'i': ref<function, i32, read_write>
            76840..76852 'target_index': i32
            76855..76866 'entry_index': ref<function, i32, read_write>
            76855..76887 'entry_..._mask)': i32
            76869..76887 'i32(bi..._mask)': i32
            76873..76886 'bit_flip_mask': u32
            77002..77014 'negate_index': f32
            77022..77100 'select... != 0)': float
            77029..77032 '1.0': float
            77034..77038 '-1.0': float
            77035..77038 '1.0': float
            77040..77099 '(count...) != 0': bool
            77041..77089 'countO...mask))': i32
            77041..77093 'countO...)) & 1': i32
            77054..77065 'entry_index': ref<function, i32, read_write>
            77054..77088 'entry_..._mask)': i32
            77068..77088 'i32(ph..._mask)': i32
            77072..77087 'phase_flip_mask': u32
            77092..77093 '1': integer
            77098..77099 '0': integer
            77115..77128 'bit_flip_mask': u32
            77115..77134 'bit_fl... == 0u': bool
            77115..77158 'bit_fl...= -1.0': bool
            77132..77134 '0u': u32
            77138..77150 'negate_index': f32
            77138..77158 'negate...= -1.0': bool
            77154..77158 '-1.0': float
            77155..77158 '1.0': float
            77262..77273 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            77262..77319 'stateV...index]': ref<storage, vec2<f32>, read_write>
            77274..77280 'params': ShotParams
            77274..77304 'params..._start': i32
            77274..77318 'params..._index': i32
            77307..77318 'entry_index': ref<function, i32, read_write>
            77322..77388 'cplxNe...ndex])': vec2<f32>
            77330..77341 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            77330..77387 'stateV...index]': ref<storage, vec2<f32>, read_write>
            77342..77348 'params': ShotParams
            77342..77372 'params..._start': i32
            77342..77386 'params..._index': i32
            77375..77386 'entry_index': ref<function, i32, read_write>
            77711..77720 'amp_entry': vec2<f32>
            77730..77741 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            77730..77787 'stateV...index]': ref<storage, vec2<f32>, read_write>
            77742..77748 'params': ShotParams
            77742..77772 'params..._start': i32
            77742..77786 'params..._index': i32
            77775..77786 'entry_index': ref<function, i32, read_write>
            77805..77815 'amp_target': vec2<f32>
            77825..77836 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            77825..77883 'stateV...index]': ref<storage, vec2<f32>, read_write>
            77837..77843 'params': ShotParams
            77837..77867 'params..._start': i32
            77837..77882 'params..._index': i32
            77870..77882 'target_index': i32
            78012..78025 'negate_target': f32
            78033..78112 'select... != 0)': float
            78040..78043 '1.0': float
            78045..78049 '-1.0': float
            78046..78049 '1.0': float
            78051..78111 '(count...) != 0': bool
            78052..78101 'countO...mask))': i32
            78052..78105 'countO...)) & 1': i32
            78065..78077 'target_index': i32
            78065..78100 'target..._mask)': i32
            78080..78100 'i32(ph..._mask)': i32
            78084..78099 'phase_flip_mask': u32
            78104..78105 '1': integer
            78110..78111 '0': integer
            78393..78404 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            78393..78450 'stateV...index]': ref<storage, vec2<f32>, read_write>
            78405..78411 'params': ShotParams
            78405..78435 'params..._start': i32
            78405..78449 'params..._index': i32
            78438..78449 'entry_index': ref<function, i32, read_write>
            78453..78498 'cplxMu... 0.0))': vec2<f32>
            78461..78471 'amp_target': vec2<f32>
            78473..78497 'vec2f(..., 0.0)': vec2<f32>
            78479..78491 'negate_index': f32
            78493..78496 '0.0': float
            78512..78523 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            78512..78570 'stateV...index]': ref<storage, vec2<f32>, read_write>
            78524..78530 'params': ShotParams
            78524..78554 'params..._start': i32
            78524..78569 'params..._index': i32
            78557..78569 'target_index': i32
            78573..78618 'cplxMu... 0.0))': vec2<f32>
            78581..78590 'amp_entry': vec2<f32>
            78592..78617 'vec2f(..., 0.0)': vec2<f32>
            78598..78611 'negate_target': f32
            78613..78616 '0.0': float
            78690..78701 'entry_index': ref<function, i32, read_write>
            78705..78711 'params': ShotParams
            78705..78734 'params...r_shot': i32
            79027..79035 'shot_idx': u32
            79042..79048 'op_idx': u32
            79055..79070 'noise_table_idx': u32
            79112..79116 'shot': ptr<storage, ShotData, read_write>
            79119..79135 '&shots...t_idx]': ptr<storage, ShotData, read_write>
            79120..79125 'shots': ref<storage, array<ShotData>, read_write>
            79120..79135 'shots[shot_idx]': ref<storage, ShotData, read_write>
            79126..79134 'shot_idx': u32
            79145..79150 'table': [error]
            79153..79205 '&batch...e_idx]': [error]
            79154..79164 'batch_data': ref<storage, BatchData, read>
            79154..79188 'batch_...tables': ref<storage, [error], read>
            79154..79205 'batch_...e_idx]': [error]
            79189..79204 'noise_table_idx': u32
            79372..79379 'rand_lo': u32
            79382..79405 'next_r...t_idx)': u32
            79396..79404 'shot_idx': u32
            79415..79422 'rand_hi': u32
            79425..79448 'next_r...t_idx)': u32
            79425..79462 'next_r...FFFFFu': u32
            79439..79447 'shot_idx': u32
            79451..79462 '0x7FFFFFFFu': u32
            79536..79549 'noise_prob_lo': [error]
            79552..79557 'table': [error]
            79552..79578 'table....ity_lo': [error]
            79588..79601 'noise_prob_hi': [error]
            79604..79609 'table': [error]
            79604..79630 'table....ity_hi': [error]
            79817..79824 'rand_hi': u32
            79817..79840 'rand_h...rob_hi': [error]
            79817..79898 'rand_h...ob_lo)': [error]
            79827..79840 'noise_prob_hi': [error]
            79845..79852 'rand_hi': u32
            79845..79869 'rand_h...rob_hi': [error]
            79845..79897 'rand_h...rob_lo': [error]
            79856..79869 'noise_prob_hi': [error]
            79873..79880 'rand_lo': u32
            79873..79897 'rand_l...rob_lo': [error]
            79884..79897 'noise_prob_lo': [error]
            79958..79962 'shot': ptr<storage, ShotData, read_write>
            79958..79970 'shot.op_type': ref<storage, u32, read_write>
            79973..79980 'OPID_ID': u32
            79990..79994 'shot': ptr<storage, ShotData, read_write>
            79990..80001 'shot.op_idx': ref<storage, u32, read_write>
            80004..80010 'op_idx': u32
            80020..80024 'shot': ptr<storage, ShotData, read_write>
            80020..80052 'shot.q...p_mask': ref<storage, u32, read_write>
            80055..80057 '0u': u32
            80074..80107 'Correl...u, 0u)': CorrelatedNoiseSample
            80096..80098 '0u': u32
            80100..80102 '0u': u32
            80104..80106 '0u': u32
            80207..80212 'start': [error]
            80215..80238 'i32(ta...ffset)': [error]
            80219..80224 'table': [error]
            80219..80237 'table....offset': [error]
            80248..80253 'count': [error]
            80256..80278 'i32(ta...count)': [error]
            80260..80265 'table': [error]
            80260..80277 'table...._count': [error]
            80288..80297 'entry_idx': i32
            80300..80357 'binary...count)': i32
            80326..80333 'rand_lo': u32
            80335..80342 'rand_hi': u32
            80344..80349 'start': [error]
            80351..80356 'count': [error]
            80367..80372 'entry': [error]
            80375..80430 '&batch...y_idx]': [error]
            80376..80386 'batch_data': ref<storage, BatchData, read>
            80376..80411 'batch_...ntries': ref<storage, [error], read>
            80376..80430 'batch_...y_idx]': [error]
            80412..80417 'start': [error]
            80412..80429 'start ...ry_idx': [error]
            80420..80429 'entry_idx': i32
            80444..80503 'Correl...is_hi)': [error]
            80466..80468 '1u': u32
            80470..80475 'entry': [error]
            80470..80485 'entry.paulis_lo': [error]
            80487..80492 'entry': [error]
            80487..80502 'entry.paulis_hi': [error]
            79154..79205 'batch_...e_idx]': cannot index into type ref<storage, [error], read>
            80215..80238 'i32(ta...ffset)': no constructor for builtin `op_i32_constructor` of type `i32` with parameters `[error]`
            80256..80278 'i32(ta...count)': no constructor for builtin `op_i32_constructor` of type `i32` with parameters `[error]`
            80344..80349 'start': expected i32 but got [error]
            80351..80356 'count': expected i32 but got [error]
            80412..80429 'start ...ry_idx': expected i32 or u32 but got [error]
            80376..80430 'batch_...y_idx]': cannot index into type ref<storage, [error], read>
            80470..80485 'entry.paulis_lo': expected u32 but got [error]
            80487..80502 'entry.paulis_hi': expected u32 but got [error]
            80922..80931 'paulis_lo': u32
            80938..80947 'paulis_hi': u32
            80954..80965 'qubit_count': u32
            80972..80973 'i': u32
            80997..81009 'bit_position': u32
            81012..81039 '(qubit...) * 3u': u32
            81013..81024 'qubit_count': u32
            81013..81029 'qubit_...t - 1u': u32
            81013..81033 'qubit_...1u - i': u32
            81027..81029 '1u': u32
            81032..81033 'i': u32
            81037..81039 '3u': u32
            81049..81061 'bit_position': u32
            81049..81066 'bit_po...n + 3u': u32
            81049..81073 'bit_po...<= 32u': bool
            81064..81066 '3u': u32
            81070..81073 '32u': u32
            81092..81126 '(pauli...& 0x7u': u32
            81093..81102 'paulis_lo': u32
            81093..81118 'paulis...sition': u32
            81106..81118 'bit_position': u32
            81122..81126 '0x7u': u32
            81181..81223 '(pauli...& 0x7u': u32
            81182..81191 'paulis_hi': u32
            81182..81215 'paulis...- 32u)': u32
            81196..81208 'bit_position': u32
            81196..81214 'bit_po... - 32u': u32
            81211..81214 '32u': u32
            81219..81223 '0x7u': u32
            81328..81336 'low_part': u32
            81339..81348 'paulis_lo': u32
            81339..81364 'paulis...sition': u32
            81352..81364 'bit_position': u32
            81378..81387 'high_part': u32
            81390..81399 'paulis_hi': u32
            81390..81423 'paulis...ition)': u32
            81404..81407 '32u': u32
            81404..81422 '32u - ...sition': u32
            81410..81422 'bit_position': u32
            81440..81469 '(low_p...& 0x7u': u32
            81441..81449 'low_part': u32
            81441..81461 'low_pa...h_part': u32
            81452..81461 'high_part': u32
            81465..81469 '0x7u': u32
            81831..81839 'shot_idx': u32
            81846..81852 'op_idx': u32
            81859..81872 'bit_flip_mask': u32
            81879..81894 'phase_flip_mask': u32
            81901..81910 'loss_mask': u32
            81927..81931 'shot': ptr<storage, ShotData, read_write>
            81934..81950 '&shots...t_idx]': ptr<storage, ShotData, read_write>
            81935..81940 'shots': ref<storage, array<ShotData>, read_write>
            81935..81950 'shots[shot_idx]': ref<storage, ShotData, read_write>
            81941..81949 'shot_idx': u32
            82137..82141 'shot': ptr<storage, ShotData, read_write>
            82137..82159 'shot.p...s_mask': ref<storage, u32, read_write>
            82163..82172 'loss_mask': u32
            82324..82328 'shot': ptr<storage, ShotData, read_write>
            82324..82336 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            82324..82339 'shot.unitary[0]': ref<storage, vec2<f32>, read_write>
            82337..82338 '0': integer
            82342..82407 'vec2f(...mask))': vec2<f32>
            82348..82375 'bitcas..._mask)': f32
            82361..82374 'bit_flip_mask': u32
            82377..82406 'bitcas..._mask)': f32
            82390..82405 'phase_flip_mask': u32
            82582..82583 'q': ref<function, u32, read_write>
            82591..82593 '0u': u32
            82595..82596 'q': ref<function, u32, read_write>
            82595..82615 'q < u3...COUNT)': bool
            82599..82615 'u32(QU...COUNT)': u32
            82603..82614 'QUBIT_COUNT': i32
            82617..82618 'q': ref<function, u32, read_write>
            82636..82646 'qubit_mask': u32
            82649..82651 '1u': u32
            82649..82656 '1u << q': u32
            82655..82656 'q': ref<function, u32, read_write>
            82670..82704 '(bit_f... != 0u': bool
            82671..82684 'bit_flip_mask': u32
            82671..82697 'bit_fl...t_mask': u32
            82687..82697 'qubit_mask': u32
            82702..82704 '0u': u32
            82762..82766 'temp': [error]
            82769..82773 'shot': ptr<storage, ShotData, read_write>
            82769..82785 'shot.q..._state': ref<storage, [error], read_write>
            82769..82788 'shot.q...ate[q]': [error]
            82769..82805 'shot.q...bility': [error]
            82786..82787 'q': ref<function, u32, read_write>
            82819..82823 'shot': ptr<storage, ShotData, read_write>
            82819..82835 'shot.q..._state': ref<storage, [error], read_write>
            82819..82838 'shot.q...ate[q]': [error]
            82819..82855 'shot.q...bility': [error]
            82836..82837 'q': ref<function, u32, read_write>
            82858..82862 'shot': ptr<storage, ShotData, read_write>
            82858..82874 'shot.q..._state': ref<storage, [error], read_write>
            82858..82877 'shot.q...ate[q]': [error]
            82858..82893 'shot.q...bility': [error]
            82875..82876 'q': ref<function, u32, read_write>
            82907..82911 'shot': ptr<storage, ShotData, read_write>
            82907..82923 'shot.q..._state': ref<storage, [error], read_write>
            82907..82926 'shot.q...ate[q]': [error]
            82907..82942 'shot.q...bility': [error]
            82924..82925 'q': ref<function, u32, read_write>
            82945..82949 'temp': [error]
            83036..83041 'was_0': bool
            83044..83085 '(shot.... != 0u': bool
            83045..83049 'shot': ptr<storage, ShotData, read_write>
            83045..83065 'shot.q...0_mask': ref<storage, u32, read_write>
            83045..83078 'shot.q...t_mask': u32
            83068..83078 'qubit_mask': u32
            83083..83085 '0u': u32
            83103..83108 'was_1': bool
            83111..83152 '(shot.... != 0u': bool
            83112..83116 'shot': ptr<storage, ShotData, read_write>
            83112..83132 'shot.q...1_mask': ref<storage, u32, read_write>
            83112..83145 'shot.q...t_mask': u32
            83135..83145 'qubit_mask': u32
            83150..83152 '0u': u32
            83170..83175 'was_0': bool
            83195..83199 'shot': ptr<storage, ShotData, read_write>
            83195..83215 'shot.q...0_mask': ref<storage, u32, read_write>
            83219..83230 '~qubit_mask': u32
            83220..83230 'qubit_mask': u32
            83248..83252 'shot': ptr<storage, ShotData, read_write>
            83248..83268 'shot.q...1_mask': ref<storage, u32, read_write>
            83272..83282 'qubit_mask': u32
            83332..83336 'shot': ptr<storage, ShotData, read_write>
            83332..83352 'shot.q...1_mask': ref<storage, u32, read_write>
            83356..83367 '~qubit_mask': u32
            83357..83367 'qubit_mask': u32
            83385..83389 'shot': ptr<storage, ShotData, read_write>
            83385..83405 'shot.q...0_mask': ref<storage, u32, read_write>
            83409..83419 'qubit_mask': u32
            83520..83524 'shot': ptr<storage, ShotData, read_write>
            83520..83532 'shot.op_type': ref<storage, u32, read_write>
            83535..83556 'OPID_C..._NOISE': u32
            83562..83566 'shot': ptr<storage, ShotData, read_write>
            83562..83573 'shot.op_idx': ref<storage, u32, read_write>
            83576..83582 'op_idx': u32
            83686..83690 'shot': ptr<storage, ShotData, read_write>
            83686..83718 'shot.q...p_mask': ref<storage, u32, read_write>
            83721..83723 '0u': u32
            82769..82788 'shot.q...ate[q]': cannot index into type ref<storage, [error], read_write>
            82819..82838 'shot.q...ate[q]': cannot index into type ref<storage, [error], read_write>
            82819..82855 'shot.q...bility': cannot assign to non-reference `[error]`
            82858..82877 'shot.q...ate[q]': cannot index into type ref<storage, [error], read_write>
            82907..82926 'shot.q...ate[q]': cannot index into type ref<storage, [error], read_write>
            82907..82942 'shot.q...bility': cannot assign to non-reference `[error]`
            84485..84492 'rand_lo': u32
            84499..84506 'rand_hi': u32
            84513..84518 'start': i32
            84525..84530 'count': i32
            84554..84557 'low': ref<function, i32, read_write>
            84565..84566 '0': integer
            84576..84580 'high': ref<function, i32, read_write>
            84588..84593 'count': i32
            84607..84610 'low': ref<function, i32, read_write>
            84607..84617 'low < high': bool
            84613..84617 'high': ref<function, i32, read_write>
            84633..84636 'mid': i32
            84644..84647 'low': ref<function, i32, read_write>
            84644..84666 'low + ...w) / 2': i32
            84650..84666 '(high ...w) / 2': i32
            84651..84655 'high': ref<function, i32, read_write>
            84651..84661 'high - low': i32
            84658..84661 'low': ref<function, i32, read_write>
            84665..84666 '2': integer
            84680..84684 'p_lo': [error]
            84687..84697 'batch_data': ref<storage, BatchData, read>
            84687..84722 'batch_...ntries': ref<storage, [error], read>
            84687..84735 'batch_...+ mid]': [error]
            84687..84750 'batch_...ity_lo': [error]
            84723..84728 'start': i32
            84723..84734 'start + mid': i32
            84731..84734 'mid': i32
            84764..84768 'p_hi': [error]
            84771..84781 'batch_data': ref<storage, BatchData, read>
            84771..84806 'batch_...ntries': ref<storage, [error], read>
            84771..84819 'batch_...+ mid]': [error]
            84771..84834 'batch_...ity_hi': [error]
            84807..84812 'start': i32
            84807..84818 'start + mid': i32
            84815..84818 'mid': i32
            84849..84856 'rand_hi': u32
            84849..84863 'rand_hi < p_hi': [error]
            84849..84902 'rand_h... p_lo)': [error]
            84859..84863 'p_hi': [error]
            84868..84875 'rand_hi': u32
            84868..84883 'rand_hi == p_hi': [error]
            84868..84901 'rand_h...< p_lo': [error]
            84879..84883 'p_hi': [error]
            84887..84894 'rand_lo': u32
            84887..84901 'rand_lo < p_lo': [error]
            84897..84901 'p_lo': [error]
            84918..84922 'high': ref<function, i32, read_write>
            84925..84928 'mid': i32
            84959..84962 'low': ref<function, i32, read_write>
            84965..84968 'mid': i32
            84965..84972 'mid + 1': i32
            84971..84972 '1': integer
            85001..85004 'low': ref<function, i32, read_write>
            84687..84735 'batch_...+ mid]': cannot index into type ref<storage, [error], read>
            84771..84819 'batch_...+ mid]': cannot index into type ref<storage, [error], read>
            85190..85196 'op_idx': u32
            85203..85208 'index': u32
            85335..85342 'vec_idx': u32
            85345..85350 'index': u32
            85345..85355 'index / 2u': u32
            85353..85355 '2u': u32
            85365..85374 'component': u32
            85377..85382 'index': u32
            85377..85387 'index % 2u': u32
            85385..85387 '2u': u32
            85397..85406 'component': u32
            85397..85412 'component == 0u': bool
            85410..85412 '0u': u32
            85431..85466 'u32(op...dx].x)': u32
            85435..85438 'ops': ref<storage, array<Op>, read>
            85435..85446 'ops[op_idx]': ref<storage, Op, read>
            85435..85454 'ops[op...nitary': ref<storage, array<vec2<f32>, 16>, read>
            85435..85463 'ops[op...c_idx]': ref<storage, vec2<f32>, read>
            85435..85465 'ops[op...idx].x': ref<storage, f32, read>
            85439..85445 'op_idx': u32
            85455..85462 'vec_idx': u32
            85496..85531 'u32(op...dx].y)': u32
            85500..85503 'ops': ref<storage, array<Op>, read>
            85500..85511 'ops[op_idx]': ref<storage, Op, read>
            85500..85519 'ops[op...nitary': ref<storage, array<vec2<f32>, 16>, read>
            85500..85528 'ops[op...c_idx]': ref<storage, vec2<f32>, read>
            85500..85530 'ops[op...idx].y': ref<storage, f32, read>
            85504..85510 'op_idx': u32
            85520..85527 'vec_idx': u32
            85741..85749 'shot_idx': u32
            85756..85762 'op_idx': u32
            85779..85781 'op': ptr<storage, Op, read>
            85784..85796 '&ops[op_idx]': ptr<storage, Op, read>
            85785..85788 'ops': ref<storage, array<Op>, read>
            85785..85796 'ops[op_idx]': ref<storage, Op, read>
            85789..85795 'op_idx': u32
            85806..85821 'noise_table_idx': u32
            85824..85826 'op': ptr<storage, Op, read>
            85824..85829 'op.q1': ref<storage, u32, read>
            85839..85850 'qubit_count': u32
            85853..85855 'op': ptr<storage, Op, read>
            85853..85858 'op.q2': ref<storage, u32, read>
            85869..85875 'sample': CorrelatedNoiseSample
            85878..85936 'sample...e_idx)': CorrelatedNoiseSample
            85902..85910 'shot_idx': u32
            85912..85918 'op_idx': u32
            85920..85935 'noise_table_idx': u32
            85946..85952 'sample': CorrelatedNoiseSample
            85946..85965 'sample..._apply': u32
            85946..85971 'sample... == 0u': bool
            85969..85971 '0u': u32
            86089..86102 'bit_flip_mask': ref<function, u32, read_write>
            86110..86112 '0u': u32
            86122..86137 'phase_flip_mask': ref<function, u32, read_write>
            86145..86147 '0u': u32
            86157..86166 'loss_mask': ref<function, u32, read_write>
            86174..86176 '0u': u32
            86191..86192 'i': ref<function, u32, read_write>
            86200..86202 '0u': u32
            86204..86205 'i': ref<function, u32, read_write>
            86204..86219 'i < qubit_count': bool
            86208..86219 'qubit_count': u32
            86221..86222 'i': ref<function, u32, read_write>
            86240..86250 'pauli_bits': u32
            86253..86319 'get_pa...nt, i)': u32
            86268..86274 'sample': CorrelatedNoiseSample
            86268..86284 'sample...lis_lo': u32
            86286..86292 'sample': CorrelatedNoiseSample
            86286..86302 'sample...lis_hi': u32
            86304..86315 'qubit_count': u32
            86317..86318 'i': ref<function, u32, read_write>
            86333..86343 'qubit_mask': u32
            86346..86348 '1u': u32
            86346..86389 '1u << ...dx, i)': u32
            86352..86389 'get_co...dx, i)': u32
            86379..86385 'op_idx': u32
            86387..86388 'i': ref<function, u32, read_write>
            86403..86428 '(pauli... != 0u': bool
            86404..86414 'pauli_bits': u32
            86404..86421 'pauli_...& 0x4u': u32
            86417..86421 '0x4u': u32
            86426..86428 '0u': u32
            86524..86533 'loss_mask': ref<function, u32, read_write>
            86537..86547 'qubit_mask': u32
            86582..86607 '(pauli... != 0u': bool
            86583..86593 'pauli_bits': u32
            86583..86600 'pauli_...& 0x1u': u32
            86596..86600 '0x1u': u32
            86605..86607 '0u': u32
            86611..86624 'bit_flip_mask': ref<function, u32, read_write>
            86628..86638 'qubit_mask': u32
            86658..86683 '(pauli... != 0u': bool
            86659..86669 'pauli_bits': u32
            86659..86676 'pauli_...& 0x2u': u32
            86672..86676 '0x2u': u32
            86681..86683 '0u': u32
            86687..86702 'phase_flip_mask': ref<function, u32, read_write>
            86706..86716 'qubit_mask': u32
            86741..86825 'commit..._mask)': [error]
            86765..86773 'shot_idx': u32
            86775..86781 'op_idx': u32
            86783..86796 'bit_flip_mask': ref<function, u32, read_write>
            86798..86813 'phase_flip_mask': ref<function, u32, read_write>
            86815..86824 'loss_mask': ref<function, u32, read_write>
            87110..87118 'shot_idx': u32
            87125..87128 'reg': u32
            87155..87160 'shots': ref<storage, array<ShotData>, read_write>
            87155..87170 'shots[shot_idx]': ref<storage, ShotData, read_write>
            87155..87177 'shots[...interp': ref<storage, InterpreterState, read_write>
            87155..87187 'shots[...isters': ref<storage, [error], read_write>
            87155..87192 'shots[...s[reg]': [error]
            87161..87169 'shot_idx': u32
            87188..87191 'reg': u32
            87155..87192 'shots[...s[reg]': cannot index into type ref<storage, [error], read_write>
            87210..87218 'shot_idx': u32
            87225..87228 'reg': u32
            87235..87238 'val': u32
            87251..87256 'shots': ref<storage, array<ShotData>, read_write>
            87251..87266 'shots[shot_idx]': ref<storage, ShotData, read_write>
            87251..87273 'shots[...interp': ref<storage, InterpreterState, read_write>
            87251..87283 'shots[...isters': ref<storage, [error], read_write>
            87251..87288 'shots[...s[reg]': [error]
            87257..87265 'shot_idx': u32
            87284..87287 'reg': u32
            87291..87294 'val': u32
            87251..87288 'shots[...s[reg]': cannot index into type ref<storage, [error], read_write>
            87251..87288 'shots[...s[reg]': cannot assign to non-reference `[error]`
            87315..87323 'shot_idx': u32
            87330..87333 'reg': u32
            87360..87397 'bitcas... reg))': i32
            87373..87396 'read_r..., reg)': u32
            87382..87390 'shot_idx': u32
            87392..87395 'reg': u32
            87419..87427 'shot_idx': u32
            87434..87437 'reg': u32
            87444..87447 'val': i32
            87460..87503 'write_...(val))': [error]
            87470..87478 'shot_idx': u32
            87480..87483 'reg': u32
            87485..87502 'bitcas...>(val)': u32
            87498..87501 'val': i32
            87524..87532 'shot_idx': u32
            87539..87542 'reg': u32
            87569..87606 'bitcas... reg))': f32
            87582..87605 'read_r..., reg)': u32
            87591..87599 'shot_idx': u32
            87601..87604 'reg': u32
            87628..87636 'shot_idx': u32
            87643..87646 'reg': u32
            87653..87656 'val': f32
            87669..87712 'write_...(val))': [error]
            87679..87687 'shot_idx': u32
            87689..87692 'reg': u32
            87694..87711 'bitcas...>(val)': u32
            87707..87710 'val': f32
            87963..87965 'pc': u32
            88000..88010 'batch_data': ref<storage, BatchData, read>
            88000..88018 'batch_...rogram': ref<storage, Program, read>
            88000..88031 'batch_...ctions': ref<storage, [error], read>
            88000..88035 'batch_...ns[pc]': [error]
            88032..88034 'pc': u32
            88000..88035 'batch_...ns[pc]': cannot index into type ref<storage, [error], read>
            88054..88060 'packed': u32
            88085..88091 'packed': u32
            88085..88099 'packed & 0xFFu': u32
            88094..88099 '0xFFu': u32
            88118..88124 'packed': u32
            88148..88170 '(packe... 0xFFu': u32
            88149..88155 'packed': u32
            88149..88161 'packed >> 8u': u32
            88159..88161 '8u': u32
            88165..88170 '0xFFu': u32
            88187..88193 'packed': u32
            88219..88242 '(packe... 0xFFu': u32
            88220..88226 'packed': u32
            88220..88233 'packed >> 16u': u32
            88230..88233 '16u': u32
            88237..88242 '0xFFu': u32
            88262..88270 'shot_idx': u32
            88277..88284 'operand': u32
            88291..88296 'flags': u32
            88303..88314 'operand_idx': u32
            88337..88372 '(flags... != 0u': bool
            88338..88343 'flags': u32
            88338..88365 'flags ...d_idx)': u32
            88347..88349 '1u': u32
            88347..88364 '1u << ...nd_idx': u32
            88353..88364 'operand_idx': u32
            88370..88372 '0u': u32
            88390..88411 'bitcas...erand)': i32
            88403..88410 'operand': u32
            88444..88475 'read_r...erand)': i32
            88457..88465 'shot_idx': u32
            88467..88474 'operand': u32
            88508..88516 'shot_idx': u32
            88523..88530 'operand': u32
            88537..88542 'flags': u32
            88549..88560 'operand_idx': u32
            88583..88618 '(flags... != 0u': bool
            88584..88589 'flags': u32
            88584..88611 'flags ...d_idx)': u32
            88593..88595 '1u': u32
            88593..88610 '1u << ...nd_idx': u32
            88599..88610 'operand_idx': u32
            88616..88618 '0u': u32
            88636..88643 'operand': u32
            88662..88689 'read_r...erand)': u32
            88671..88679 'shot_idx': u32
            88681..88688 'operand': u32
            88709..88717 'shot_idx': u32
            88724..88731 'operand': u32
            88738..88743 'flags': u32
            88750..88761 'operand_idx': u32
            88784..88819 '(flags... != 0u': bool
            88785..88790 'flags': u32
            88785..88812 'flags ...d_idx)': u32
            88794..88796 '1u': u32
            88794..88811 '1u << ...nd_idx': u32
            88800..88811 'operand_idx': u32
            88817..88819 '0u': u32
            88837..88858 'bitcas...erand)': f32
            88850..88857 'operand': u32
            88914..88945 'read_r...erand)': f32
            88927..88935 'shot_idx': u32
            88937..88944 'operand': u32
            89016..89024 'shot_idx': u32
            89048..89053 'state': InterpreterState
            89056..89061 'shots': ref<storage, array<ShotData>, read_write>
            89056..89071 'shots[shot_idx]': ref<storage, ShotData, read_write>
            89056..89078 'shots[...interp': ref<storage, InterpreterState, read_write>
            89062..89070 'shot_idx': u32
            89088..89093 'instr': Instruction
            89096..89121 'fetch_...c - 1)': Instruction
            89108..89113 'state': InterpreterState
            89108..89116 'state.pc': u32
            89108..89120 'state.pc - 1': u32
            89119..89120 '1': integer
            89130..89165 '(instr...) != 0': bool
            89131..89136 'instr': Instruction
            89131..89143 'instr.opcode': u32
            89131..89159 'instr....X1_IMM': u32
            89146..89159 'FLAG_AUX1_IMM': u32
            89164..89165 '0': integer
            89183..89188 'instr': Instruction
            89183..89193 'instr.aux1': u32
            89212..89242 'read_r....aux1)': u32
            89221..89229 'shot_idx': u32
            89231..89236 'instr': Instruction
            89231..89241 'instr.aux1': u32
            89313..89321 'shot_idx': u32
            89345..89350 'state': InterpreterState
            89353..89358 'shots': ref<storage, array<ShotData>, read_write>
            89353..89368 'shots[shot_idx]': ref<storage, ShotData, read_write>
            89353..89375 'shots[...interp': ref<storage, InterpreterState, read_write>
            89359..89367 'shot_idx': u32
            89385..89390 'instr': Instruction
            89393..89418 'fetch_...c - 1)': Instruction
            89405..89410 'state': InterpreterState
            89405..89413 'state.pc': u32
            89405..89417 'state.pc - 1': u32
            89416..89417 '1': integer
            89427..89462 '(instr...) != 0': bool
            89428..89433 'instr': Instruction
            89428..89440 'instr.opcode': u32
            89428..89456 'instr....X2_IMM': u32
            89443..89456 'FLAG_AUX2_IMM': u32
            89461..89462 '0': integer
            89480..89485 'instr': Instruction
            89480..89490 'instr.aux2': u32
            89509..89539 'read_r....aux2)': u32
            89518..89526 'shot_idx': u32
            89528..89533 'instr': Instruction
            89528..89538 'instr.aux2': u32
            89714..89722 'shot_idx': u32
            89746..89751 'state': InterpreterState
            89754..89759 'shots': ref<storage, array<ShotData>, read_write>
            89754..89769 'shots[shot_idx]': ref<storage, ShotData, read_write>
            89754..89776 'shots[...interp': ref<storage, InterpreterState, read_write>
            89760..89768 'shot_idx': u32
            89786..89791 'instr': Instruction
            89794..89819 'fetch_...c - 1)': Instruction
            89806..89811 'state': InterpreterState
            89806..89814 'state.pc': u32
            89806..89818 'state.pc - 1': u32
            89817..89818 '1': integer
            89829..89834 'flags': u32
            89837..89860 'get_fl...pcode)': u32
            89847..89852 'instr': Instruction
            89847..89859 'instr.opcode': u32
            89873..89917 'resolv...s, 0u)': f32
            89885..89893 'shot_idx': u32
            89895..89900 'instr': Instruction
            89895..89905 'instr.src0': u32
            89907..89912 'flags': u32
            89914..89916 '0u': u32
            90089..90097 'shot_idx': u32
            90104..90113 'result_id': u32
            90141..90198 'atomic...t_id])': u32
            90141..90204 'atomic... == 1u': bool
            90152..90197 '&resul...lt_id]': ptr<storage, atomic<u32>, read_write>
            90153..90160 'results': ref<storage, array<atomic<u32>>, read_write>
            90153..90197 'result...lt_id]': ref<storage, atomic<u32>, read_write>
            90161..90169 'shot_idx': u32
            90161..90184 'shot_i..._COUNT': u32
            90161..90196 'shot_i...ult_id': u32
            90172..90184 'RESULT_COUNT': u32
            90187..90196 'result_id': u32
            90202..90204 '1u': u32
            90286..90288 'id': u32
            90316..90364 '(12 <=...<= 19)': bool
            90317..90319 '12': integer
            90317..90325 '12 <= id': bool
            90317..90337 '12 <= ... <= 14': bool
            90323..90325 'id': u32
            90329..90331 'id': u32
            90329..90337 'id <= 14': bool
            90335..90337 '14': integer
            90343..90345 '17': integer
            90343..90351 '17 <= id': bool
            90343..90363 '17 <= ... <= 19': bool
            90349..90351 'id': u32
            90355..90357 'id': u32
            90355..90363 'id <= 19': bool
            90361..90363 '19': integer
            90459..90467 'shot_idx': u32
            90492..90497 'state': InterpreterState
            90500..90505 'shots': ref<storage, array<ShotData>, read_write>
            90500..90515 'shots[shot_idx]': ref<storage, ShotData, read_write>
            90500..90522 'shots[...interp': ref<storage, InterpreterState, read_write>
            90506..90514 'shot_idx': u32
            90532..90537 'instr': Instruction
            90540..90565 'fetch_...c - 1)': Instruction
            90552..90557 'state': InterpreterState
            90552..90560 'state.pc': u32
            90552..90564 'state.pc - 1': u32
            90563..90564 '1': integer
            90578..90613 '(instr...) != 0': bool
            90579..90584 'instr': Instruction
            90579..90591 'instr.opcode': u32
            90579..90607 'instr....C0_IMM': u32
            90594..90607 'FLAG_SRC0_IMM': u32
            90612..90613 '0': integer
            90917..90925 'shot_idx': u32
            90932..90937 'qubit': u32
            90954..90958 'shot': ptr<storage, ShotData, read_write>
            90961..90977 '&shots...t_idx]': ptr<storage, ShotData, read_write>
            90962..90967 'shots': ref<storage, array<ShotData>, read_write>
            90962..90977 'shots[shot_idx]': ref<storage, ShotData, read_write>
            90968..90976 'shot_idx': u32
            90987..90993 'result': [error]
            90996..91072 'select...ility)': [error]
            91003..91005 '1u': u32
            91007..91009 '0u': u32
            91011..91015 'shot': ptr<storage, ShotData, read_write>
            91011..91028 'shot.r...easure': ref<storage, f32, read_write>
            91011..91071 'shot.r...bility': [error]
            91031..91035 'shot': ptr<storage, ShotData, read_write>
            91031..91047 'shot.q..._state': ref<storage, [error], read_write>
            91031..91054 'shot.q...qubit]': [error]
            91031..91071 'shot.q...bility': [error]
            91048..91053 'qubit': u32
            91078..91082 'shot': ptr<storage, ShotData, read_write>
            91078..91094 'shot.q..._state': ref<storage, [error], read_write>
            91078..91101 'shot.q...qubit]': [error]
            91078..91106 'shot.q...].heat': [error]
            91095..91100 'qubit': u32
            91109..91113 '-1.0': float
            91110..91113 '1.0': float
            91119..91200 'prep_m...ro */)': [error]
            91149..91157 'shot_idx': u32
            91159..91164 'qubit': u32
            91166..91172 'result': [error]
            91174..91178 'true': bool
            91206..91210 'shot': ptr<storage, ShotData, read_write>
            91206..91217 'shot.op_idx': ref<storage, u32, read_write>
            91220..91225 'qubit': u32
            91275..91279 'shot': ptr<storage, ShotData, read_write>
            91275..91287 'shot.op_type': ref<storage, u32, read_write>
            91290..91305 'OPID_LOSS_NOISE': u32
            91031..91054 'shot.q...qubit]': cannot index into type ref<storage, [error], read_write>
            91078..91101 'shot.q...qubit]': cannot index into type ref<storage, [error], read_write>
            91078..91106 'shot.q...].heat': cannot assign to non-reference `[error]`
            91166..91172 'result': expected u32 but got [error]
            91516..91524 'shot_idx': u32
            91531..91537 'op_idx': u32
            91544..91555 'qubit_count': u32
            91562..91572 'arg_offset': u32
            91589..91604 'noise_table_idx': u32
            91607..91610 'ops': ref<storage, array<Op>, read>
            91607..91618 'ops[op_idx]': ref<storage, Op, read>
            91607..91621 'ops[op_idx].q1': ref<storage, u32, read>
            91611..91617 'op_idx': u32
            91632..91638 'sample': CorrelatedNoiseSample
            91641..91699 'sample...e_idx)': CorrelatedNoiseSample
            91665..91673 'shot_idx': u32
            91675..91681 'op_idx': u32
            91683..91698 'noise_table_idx': u32
            91709..91715 'sample': CorrelatedNoiseSample
            91709..91728 'sample..._apply': u32
            91709..91734 'sample... == 0u': bool
            91732..91734 '0u': u32
            91857..91870 'bit_flip_mask': ref<function, u32, read_write>
            91878..91880 '0u': u32
            91890..91905 'phase_flip_mask': ref<function, u32, read_write>
            91913..91915 '0u': u32
            91925..91934 'loss_mask': ref<function, u32, read_write>
            91942..91944 '0u': u32
            91959..91960 'i': ref<function, u32, read_write>
            91968..91970 '0u': u32
            91972..91973 'i': ref<function, u32, read_write>
            91972..91987 'i < qubit_count': bool
            91976..91987 'qubit_count': u32
            91989..91990 'i': ref<function, u32, read_write>
            92008..92018 'pauli_bits': u32
            92021..92087 'get_pa...nt, i)': u32
            92036..92042 'sample': CorrelatedNoiseSample
            92036..92052 'sample...lis_lo': u32
            92054..92060 'sample': CorrelatedNoiseSample
            92054..92070 'sample...lis_hi': u32
            92072..92083 'qubit_count': u32
            92085..92086 'i': ref<function, u32, read_write>
            92101..92108 'arg_reg': [error]
            92111..92121 'batch_data': ref<storage, BatchData, read>
            92111..92129 'batch_...rogram': ref<storage, Program, read>
            92111..92144 'batch_..._table': ref<storage, [error], read>
            92111..92160 'batch_...t + i]': [error]
            92145..92155 'arg_offset': u32
            92145..92159 'arg_offset + i': u32
            92158..92159 'i': ref<function, u32, read_write>
            92174..92184 'qubit_mask': u32
            92187..92189 '1u': u32
            92187..92220 '1u << ...g_reg)': u32
            92193..92220 'read_r...g_reg)': u32
            92202..92210 'shot_idx': u32
            92212..92219 'arg_reg': [error]
            92234..92259 '(pauli... != 0u': bool
            92235..92245 'pauli_bits': u32
            92235..92252 'pauli_...& 0x4u': u32
            92248..92252 '0x4u': u32
            92257..92259 '0u': u32
            92355..92364 'loss_mask': ref<function, u32, read_write>
            92368..92378 'qubit_mask': u32
            92413..92438 '(pauli... != 0u': bool
            92414..92424 'pauli_bits': u32
            92414..92431 'pauli_...& 0x1u': u32
            92427..92431 '0x1u': u32
            92436..92438 '0u': u32
            92442..92455 'bit_flip_mask': ref<function, u32, read_write>
            92459..92469 'qubit_mask': u32
            92489..92514 '(pauli... != 0u': bool
            92490..92500 'pauli_bits': u32
            92490..92507 'pauli_...& 0x2u': u32
            92503..92507 '0x2u': u32
            92512..92514 '0u': u32
            92518..92533 'phase_flip_mask': ref<function, u32, read_write>
            92537..92547 'qubit_mask': u32
            92572..92656 'commit..._mask)': [error]
            92596..92604 'shot_idx': u32
            92606..92612 'op_idx': u32
            92614..92627 'bit_flip_mask': ref<function, u32, read_write>
            92629..92644 'phase_flip_mask': ref<function, u32, read_write>
            92646..92655 'loss_mask': ref<function, u32, read_write>
            92111..92160 'batch_...t + i]': cannot index into type ref<storage, [error], read>
            92212..92219 'arg_reg': expected u32 but got [error]
            92977..92983 'params': ShotParams
            93208..93209 'i': ref<function, i32, read_write>
            93212..93213 '0': integer
            93215..93216 'i': ref<function, i32, read_write>
            93215..93239 'i < pa...ations': bool
            93219..93225 'params': ShotParams
            93219..93239 'params...ations': i32
            93241..93242 'i': ref<function, i32, read_write>
            93260..93271 'entry_index': i32
            93279..93285 'params': ShotParams
            93279..93304 'params...n_shot': i32
            93279..93340 'params...r_shot': i32
            93307..93308 'i': ref<function, i32, read_write>
            93307..93340 'i * pa...r_shot': i32
            93311..93317 'params': ShotParams
            93311..93340 'params...r_shot': i32
            93350..93361 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            93350..93407 'stateV...index]': ref<storage, vec2<f32>, read_write>
            93362..93368 'params': ShotParams
            93362..93392 'params..._start': i32
            93362..93406 'params..._index': i32
            93395..93406 'entry_index': i32
            93410..93425 'vec2f(0.0, 0.0)': vec2<f32>
            93416..93419 '0.0': float
            93421..93424 '0.0': float
            93530..93536 'params': ShotParams
            93530..93555 'params...n_shot': i32
            93530..93560 'params...t == 0': bool
            93559..93560 '0': integer
            93663..93674 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            93663..93706 'stateV...start]': ref<storage, vec2<f32>, read_write>
            93675..93681 'params': ShotParams
            93675..93705 'params..._start': i32
            93709..93724 'vec2f(1.0, 0.0)': vec2<f32>
            93715..93718 '1.0': float
            93720..93723 '0.0': float
            93734..93760 'reset_...t_idx)': [error]
            93744..93750 'params': ShotParams
            93744..93759 'params.shot_idx': i32
            94174..94182 'shot_idx': u32
            94189..94195 'op_idx': u32
            94202..94204 'q1': u32
            94211..94213 'q2': u32
            94230..94234 'shot': ptr<storage, ShotData, read_write>
            94237..94253 '&shots...t_idx]': ptr<storage, ShotData, read_write>
            94238..94243 'shots': ref<storage, array<ShotData>, read_write>
            94238..94253 'shots[shot_idx]': ref<storage, ShotData, read_write>
            94244..94252 'shot_idx': u32
            94263..94265 'op': ptr<storage, Op, read>
            94268..94280 '&ops[op_idx]': ptr<storage, Op, read>
            94269..94272 'ops': ref<storage, array<Op>, read>
            94269..94280 'ops[op_idx]': ref<storage, Op, read>
            94273..94279 'op_idx': u32
            94287..94291 'shot': ptr<storage, ShotData, read_write>
            94287..94298 'shot.op_idx': ref<storage, u32, read_write>
            94301..94307 'op_idx': u32
            94313..94317 'shot': ptr<storage, ShotData, read_write>
            94313..94325 'shot.op_type': ref<storage, u32, read_write>
            94328..94330 'op': ptr<storage, Op, read>
            94328..94333 'op.id': ref<storage, u32, read>
            94472..94474 'op': ptr<storage, Op, read>
            94472..94477 'op.id': ref<storage, u32, read>
            94472..94489 'op.id ...ID_RXX': bool
            94472..94510 'op.id ...ID_RYY': bool
            94472..94533 'op.id ..._MAT2Q': bool
            94472..94555 'op.id ...D_SWAP': bool
            94481..94489 'OPID_RXX': u32
            94493..94495 'op': ptr<storage, Op, read>
            94493..94498 'op.id': ref<storage, u32, read>
            94493..94510 'op.id ...ID_RYY': bool
            94502..94510 'OPID_RYY': u32
            94514..94516 'op': ptr<storage, Op, read>
            94514..94519 'op.id': ref<storage, u32, read>
            94514..94533 'op.id ..._MAT2Q': bool
            94523..94533 'OPID_MAT2Q': u32
            94537..94539 'op': ptr<storage, Op, read>
            94537..94542 'op.id': ref<storage, u32, read>
            94537..94555 'op.id ...D_SWAP': bool
            94546..94555 'OPID_SWAP': u32
            94567..94571 'shot': ptr<storage, ShotData, read_write>
            94567..94579 'shot.op_type': ref<storage, u32, read_write>
            94582..94599 'OPID_S...UFF_2Q': u32
            94665..94667 'op': ptr<storage, Op, read>
            94665..94670 'op.id': ref<storage, u32, read>
            94665..94680 'op.id >= OPID_X': bool
            94665..94699 'op.id ...PID_CX': bool
            94674..94680 'OPID_X': u32
            94684..94686 'op': ptr<storage, Op, read>
            94684..94689 'op.id': ref<storage, u32, read>
            94684..94699 'op.id < OPID_CX': bool
            94692..94699 'OPID_CX': u32
            94711..94715 'shot': ptr<storage, ShotData, read_write>
            94711..94723 'shot.op_type': ref<storage, u32, read_write>
            94726..94743 'OPID_S...UFF_1Q': u32
            94809..94832 'is_1q_...op.id)': bool
            94826..94828 'op': ptr<storage, Op, read>
            94826..94831 'op.id': ref<storage, u32, read>
            94918..94922 'shot': ptr<storage, ShotData, read_write>
            94918..94930 'shot.op_type': ref<storage, u32, read_write>
            94933..94940 'OPID_RZ': u32
            95052..95056 'shot': ptr<storage, ShotData, read_write>
            95052..95064 'shot.op_type': ref<storage, u32, read_write>
            95078..95085 'OPID_ID': u32
            95087..95094 'OPID_CZ': u32
            95096..95103 'OPID_RZ': u32
            95105..95113 'OPID_RZZ': u32
            95124..95128 'shot': ptr<storage, ShotData, read_write>
            95124..95156 'shot.q...p_mask': ref<storage, u32, read_write>
            95159..95161 '0u': u32
            95182..95199 'OPID_S...UFF_1Q': u32
            95210..95214 'shot': ptr<storage, ShotData, read_write>
            95210..95242 'shot.q...p_mask': ref<storage, u32, read_write>
            95245..95247 '1u': u32
            95245..95253 '1u << q1': u32
            95251..95253 'q1': u32
            95274..95281 'OPID_CX': u32
            95283..95290 'OPID_CY': u32
            95292..95309 'OPID_S...UFF_2Q': u32
            95320..95324 'shot': ptr<storage, ShotData, read_write>
            95320..95352 'shot.q...p_mask': ref<storage, u32, read_write>
            95355..95378 '(1u <<...<< q2)': u32
            95356..95358 '1u': u32
            95356..95364 '1u << q1': u32
            95362..95364 'q1': u32
            95369..95371 '1u': u32
            95369..95377 '1u << q2': u32
            95375..95377 'q2': u32
            96700..96708 'shot_idx': u32
            96837..96841 'shot': ptr<storage, ShotData, read_write>
            96844..96860 '&shots...t_idx]': ptr<storage, ShotData, read_write>
            96845..96850 'shots': ref<storage, array<ShotData>, read_write>
            96845..96860 'shots[shot_idx]': ref<storage, ShotData, read_write>
            96851..96859 'shot_idx': u32
            96988..96994 'op_idx': u32
            96997..97001 'shot': ptr<storage, ShotData, read_write>
            96997..97013 'shot.n...op_idx': ref<storage, u32, read_write>
            97123..97129 'op_idx': u32
            97123..97155 'op_idx...&ops))': bool
            97133..97155 'u32(ar...&ops))': u32
            97137..97154 'arrayL...(&ops)': u32
            97149..97153 '&ops': ptr<storage, array<Op>, read>
            97150..97153 'ops': ref<storage, array<Op>, read>
            97215..97219 'shot': ptr<storage, ShotData, read_write>
            97215..97227 'shot.op_type': ref<storage, u32, read_write>
            97230..97237 'OPID_ID': u32
            97247..97251 'shot': ptr<storage, ShotData, read_write>
            97247..97263 'shot.r...malize': ref<storage, f32, read_write>
            97266..97269 '1.0': float
            97279..97283 'shot': ptr<storage, ShotData, read_write>
            97279..97311 'shot.q...p_mask': ref<storage, u32, read_write>
            97314..97316 '0u': u32
            97349..97351 'op': ptr<storage, Op, read>
            97354..97366 '&ops[op_idx]': ptr<storage, Op, read>
            97355..97358 'ops': ref<storage, array<Op>, read>
            97355..97366 'ops[op_idx]': ref<storage, Op, read>
            97359..97365 'op_idx': u32
            97463..97467 'shot': ptr<storage, ShotData, read_write>
            97463..97495 'shot.q...p_mask': ref<storage, u32, read_write>
            97463..97500 'shot.q...k != 0': bool
            97499..97500 '0': integer
            97512..97540 'update...t_idx)': [error]
            97531..97539 'shot_idx': u32
            97553..97579 'shot_i...t_idx)': [error]
            97570..97578 'shot_idx': u32
            97585..97589 'shot': ptr<storage, ShotData, read_write>
            97585..97597 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            97600..97602 'op': ptr<storage, Op, read>
            97600..97610 'op.unitary': ref<storage, array<vec2<f32>, 16>, read>
            97744..97746 'op': ptr<storage, Op, read>
            97744..97749 'op.id': ref<storage, u32, read>
            97744..97765 'op.id ...RESETZ': bool
            97753..97765 'OPID_MRESETZ': u32
            97777..97901 'prep_m...ro */)': [error]
            97796..97804 'shot_idx': u32
            97806..97812 'op_idx': u32
            97814..97816 'op': ptr<storage, Op, read>
            97814..97819 'op.q1': ref<storage, u32, read>
            97821..97823 'op': ptr<storage, Op, read>
            97821..97826 'op.q2': ref<storage, u32, read>
            97828..97833 'false': bool
            97849..97853 'true': bool
            97875..97879 'true': bool
            97911..97915 'shot': ptr<storage, ShotData, read_write>
            97911..97927 'shot.n...op_idx': ref<storage, u32, read_write>
            97930..97936 'op_idx': u32
            97930..97941 'op_idx + 1u': u32
            97939..97941 '1u': u32
            98022..98024 'op': ptr<storage, Op, read>
            98022..98027 'op.id': ref<storage, u32, read>
            98022..98038 'op.id ...PID_MZ': bool
            98031..98038 'OPID_MZ': u32
            98050..98175 'prep_m...ro */)': [error]
            98069..98077 'shot_idx': u32
            98079..98085 'op_idx': u32
            98087..98089 'op': ptr<storage, Op, read>
            98087..98092 'op.q1': ref<storage, u32, read>
            98094..98096 'op': ptr<storage, Op, read>
            98094..98099 'op.q2': ref<storage, u32, read>
            98101..98106 'false': bool
            98122..98126 'true': bool
            98148..98153 'false': bool
            98185..98189 'shot': ptr<storage, ShotData, read_write>
            98185..98201 'shot.n...op_idx': ref<storage, u32, read_write>
            98204..98210 'op_idx': u32
            98204..98215 'op_idx + 1u': u32
            98213..98215 '1u': u32
            98247..98249 'op': ptr<storage, Op, read>
            98247..98252 'op.id': ref<storage, u32, read>
            98247..98267 'op.id ...RESETZ': bool
            98256..98267 'OPID_RESETZ': u32
            98279..98404 'prep_m...ro */)': [error]
            98298..98306 'shot_idx': u32
            98308..98314 'op_idx': u32
            98316..98318 'op': ptr<storage, Op, read>
            98316..98321 'op.q1': ref<storage, u32, read>
            98323..98325 'op': ptr<storage, Op, read>
            98323..98328 'op.q2': ref<storage, u32, read>
            98330..98335 'false': bool
            98351..98356 'false': bool
            98378..98382 'true': bool
            98414..98418 'shot': ptr<storage, ShotData, read_write>
            98414..98430 'shot.n...op_idx': ref<storage, u32, read_write>
            98433..98439 'op_idx': u32
            98433..98444 'op_idx + 1u': u32
            98442..98444 '1u': u32
            98627..98629 'op': ptr<storage, Op, read>
            98627..98632 'op.id': ref<storage, u32, read>
            98627..98651 'op.id ..._NOISE': bool
            98636..98651 'OPID_LOSS_NOISE': u32
            98663..98667 'shot': ptr<storage, ShotData, read_write>
            98663..98679 'shot.n...op_idx': ref<storage, u32, read_write>
            98682..98688 'op_idx': u32
            98682..98693 'op_idx + 1u': u32
            98691..98693 '1u': u32
            98707..98715 'loss_bit': u32
            98718..98720 '1u': u32
            98718..98729 '1u << op.q1': u32
            98724..98726 'op': ptr<storage, Op, read>
            98724..98729 'op.q1': ref<storage, u32, read>
            98743..98784 '(shot.... != 0u': bool
            98744..98748 'shot': ptr<storage, ShotData, read_write>
            98744..98766 'shot.p...s_mask': ref<storage, u32, read_write>
            98744..98777 'shot.p...ss_bit': u32
            98769..98777 'loss_bit': u32
            98782..98784 '0u': u32
            98800..98804 'shot': ptr<storage, ShotData, read_write>
            98800..98822 'shot.p...s_mask': ref<storage, u32, read_write>
            98826..98835 '~loss_bit': u32
            98827..98835 'loss_bit': u32
            98849..98973 'prep_m...ro */)': [error]
            98868..98876 'shot_idx': u32
            98878..98884 'op_idx': u32
            98886..98888 'op': ptr<storage, Op, read>
            98886..98891 'op.q1': ref<storage, u32, read>
            98893..98895 'op': ptr<storage, Op, read>
            98893..98898 'op.q2': ref<storage, u32, read>
            98900..98904 'true': bool
            98920..98925 'false': bool
            98947..98951 'true': bool
            99004..99008 'shot': ptr<storage, ShotData, read_write>
            99004..99016 'shot.op_type': ref<storage, u32, read_write>
            99019..99026 'OPID_ID': u32
            99040..99044 'shot': ptr<storage, ShotData, read_write>
            99040..99051 'shot.op_idx': ref<storage, u32, read_write>
            99054..99060 'op_idx': u32
            99074..99078 'shot': ptr<storage, ShotData, read_write>
            99074..99106 'shot.q...p_mask': ref<storage, u32, read_write>
            99109..99111 '0u': u32
            99612..99624 'pauli_op_idx': u32
            99627..99654 'get_pa...p_idx)': u32
            99647..99653 'op_idx': u32
            99822..99826 'shot': ptr<storage, ShotData, read_write>
            99822..99838 'shot.n...op_idx': ref<storage, u32, read_write>
            99841..99866 'max(op...p_idx)': u32
            99841..99871 'max(op...) + 1u': u32
            99845..99851 'op_idx': u32
            99853..99865 'pauli_op_idx': u32
            99869..99871 '1u': u32
            99924..99926 'op': ptr<storage, Op, read>
            99924..99929 'op.id': ref<storage, u32, read>
            99924..99954 'op.id ..._NOISE': bool
            99933..99954 'OPID_C..._NOISE': u32
            99966..100005 'prep_c...p_idx)': [error]
            99988..99996 'shot_idx': u32
            99998..100004 'op_idx': u32
            100181..100197 'has_lo...perand': bool
            100200..100253 'gate_h...op.q2)': bool
            100222..100230 'shot_idx': u32
            100232..100238 'op_idx': u32
            100240..100242 'op': ptr<storage, Op, read>
            100240..100245 'op.q1': ref<storage, u32, read>
            100247..100249 'op': ptr<storage, Op, read>
            100247..100252 'op.q2': ref<storage, u32, read>
            100263..100279 'has_lo...perand': bool
            100291..100349 'handle...op.q2)': [error]
            100318..100326 'shot_idx': u32
            100328..100334 'op_idx': u32
            100336..100338 'op': ptr<storage, Op, read>
            100336..100341 'op.q1': ref<storage, u32, read>
            100343..100345 'op': ptr<storage, Op, read>
            100343..100348 'op.q2': ref<storage, u32, read>
            100365..100377 'pauli_op_idx': u32
            100365..100382 'pauli_...x != 0': bool
            100381..100382 '0': integer
            100396..100399 'ops': ref<storage, array<Op>, read>
            100396..100413 'ops[pa...p_idx]': ref<storage, Op, read>
            100396..100416 'ops[pa...dx].id': ref<storage, u32, read>
            100396..100439 'ops[pa...ISE_1Q': bool
            100400..100412 'pauli_op_idx': u32
            100420..100439 'OPID_P...ISE_1Q': u32
            100610..100627 '!has_l...perand': bool
            100611..100627 'has_lo...perand': bool
            100647..100706 'apply_...op.q1)': [error]
            100668..100676 'shot_idx': u32
            100678..100684 'op_idx': u32
            100686..100698 'pauli_op_idx': u32
            100700..100702 'op': ptr<storage, Op, read>
            100700..100705 'op.q1': ref<storage, u32, read>
            100775..100791 'has_lo...perand': bool
            100973..101051 'apply_...op.q2)': [error]
            101006..101014 'shot_idx': u32
            101016..101022 'op_idx': u32
            101024..101036 'pauli_op_idx': u32
            101038..101040 'op': ptr<storage, Op, read>
            101038..101043 'op.q1': ref<storage, u32, read>
            101045..101047 'op': ptr<storage, Op, read>
            101045..101050 'op.q2': ref<storage, u32, read>
            101090..101156 'apply_...op.q2)': [error]
            101111..101119 'shot_idx': u32
            101121..101127 'op_idx': u32
            101129..101141 'pauli_op_idx': u32
            101143..101145 'op': ptr<storage, Op, read>
            101143..101148 'op.q1': ref<storage, u32, read>
            101150..101152 'op': ptr<storage, Op, read>
            101150..101155 'op.q2': ref<storage, u32, read>
            101365..101381 'has_lo...perand': bool
            101483..101531 'finali...op.q2)': [error]
            101500..101508 'shot_idx': u32
            101510..101516 'op_idx': u32
            101518..101520 'op': ptr<storage, Op, read>
            101518..101523 'op.q1': ref<storage, u32, read>
            101525..101527 'op': ptr<storage, Op, read>
            101525..101530 'op.q2': ref<storage, u32, read>
            101673..101684 'workgroupId': vec3<u32>
            101738..101741 'tid': u32
            101780..101786 'params': ShotParams
            101789..101847 'get_sh...op */)': ShotParams
            101805..101816 'workgroupId': vec3<u32>
            101805..101818 'workgroupId.x': u32
            101820..101823 'tid': u32
            101825..101826 '0': integer
            101935..101960 'init_s...arams)': [error]
            101953..101959 'params': ShotParams
            102048..102059 'IS_ADAPTIVE': bool
            102048..102093 'IS_ADA...t == 0': bool
            102063..102069 'params': ShotParams
            102063..102088 'params...n_shot': i32
            102063..102093 'params...t == 0': bool
            102092..102093 '0': integer
            102258..102270 'results_base': u32
            102273..102293 'u32(pa...t_idx)': u32
            102273..102308 'u32(pa..._COUNT': u32
            102277..102283 'params': ShotParams
            102277..102292 'params.shot_idx': i32
            102296..102308 'RESULT_COUNT': u32
            102327..102328 'r': ref<function, u32, read_write>
            102331..102333 '0u': u32
            102335..102336 'r': ref<function, u32, read_write>
            102335..102351 'r < RE..._COUNT': bool
            102339..102351 'RESULT_COUNT': u32
            102353..102354 'r': ref<function, u32, read_write>
            102372..102415 'atomic...], 0u)': [error]
            102384..102410 '&resul...e + r]': ptr<storage, atomic<u32>, read_write>
            102385..102392 'results': ref<storage, array<atomic<u32>>, read_write>
            102385..102410 'result...e + r]': ref<storage, atomic<u32>, read_write>
            102393..102405 'results_base': u32
            102393..102409 'result...se + r': u32
            102408..102409 'r': ref<function, u32, read_write>
            102412..102414 '0u': u32
            102493..102494 'm': ref<function, u32, read_write>
            102497..102499 '0u': u32
            102501..102502 'm': ref<function, u32, read_write>
            102501..102523 'm < CO...A_SIZE': bool
            102505..102523 'CONSTA...A_SIZE': u32
            102525..102526 'm': ref<function, u32, read_write>
            102544..102549 'shots': ref<storage, array<ShotData>, read_write>
            102544..102566 'shots[...t_idx]': ref<storage, ShotData, read_write>
            102544..102573 'shots[...interp': ref<storage, InterpreterState, read_write>
            102544..102580 'shots[...memory': ref<storage, [error], read_write>
            102544..102583 'shots[...ory[m]': [error]
            102550..102556 'params': ShotParams
            102550..102565 'params.shot_idx': i32
            102581..102582 'm': ref<function, u32, read_write>
            102586..102596 'batch_data': ref<storage, BatchData, read>
            102586..102604 'batch_...rogram': ref<storage, Program, read>
            102586..102618 'batch_...t_data': ref<storage, [error], read>
            102586..102621 'batch_...ata[m]': [error]
            102619..102620 'm': ref<function, u32, read_write>
            102703..102704 'm': ref<function, u32, read_write>
            102707..102725 'CONSTA...A_SIZE': u32
            102727..102728 'm': ref<function, u32, read_write>
            102727..102741 'm < MAX_MEMORY': bool
            102731..102741 'MAX_MEMORY': u32
            102743..102744 'm': ref<function, u32, read_write>
            102762..102767 'shots': ref<storage, array<ShotData>, read_write>
            102762..102784 'shots[...t_idx]': ref<storage, ShotData, read_write>
            102762..102791 'shots[...interp': ref<storage, InterpreterState, read_write>
            102762..102798 'shots[...memory': ref<storage, [error], read_write>
            102762..102801 'shots[...ory[m]': [error]
            102768..102774 'params': ShotParams
            102768..102783 'params.shot_idx': i32
            102799..102800 'm': ref<function, u32, read_write>
            102804..102806 '0u': u32
            102544..102583 'shots[...ory[m]': cannot index into type ref<storage, [error], read_write>
            102544..102583 'shots[...ory[m]': cannot assign to non-reference `[error]`
            102586..102621 'batch_...ata[m]': cannot index into type ref<storage, [error], read>
            102762..102801 'shots[...ory[m]': cannot index into type ref<storage, [error], read_write>
            102762..102801 'shots[...ory[m]': cannot assign to non-reference `[error]`
            105427..105430 'gid': vec3<u32>
            105567..105575 'shot_idx': u32
            105578..105581 'gid': vec3<u32>
            105578..105583 'gid.x': u32
            105593..105598 'state': InterpreterState
            105601..105606 'shots': ref<storage, array<ShotData>, read_write>
            105601..105616 'shots[shot_idx]': ref<storage, ShotData, read_write>
            105601..105623 'shots[...interp': ref<storage, InterpreterState, read_write>
            105607..105615 'shot_idx': u32
            105701..105707 'status': u32
            105710..105715 'state': InterpreterState
            105710..105722 'state.status': u32
            105731..105737 'status': u32
            105731..105758 'status...INATED': bool
            105731..105784 'status..._ERROR': bool
            105741..105758 'STATUS...INATED': u32
            105762..105768 'status': u32
            105762..105784 'status..._ERROR': bool
            105772..105784 'STATUS_ERROR': u32
            106205..106210 'shots': ref<storage, array<ShotData>, read_write>
            106205..106220 'shots[shot_idx]': ref<storage, ShotData, read_write>
            106205..106238 'shots[...s_mask': ref<storage, u32, read_write>
            106205..106244 'shots[... != 0u': bool
            106211..106219 'shot_idx': u32
            106242..106244 '0u': u32
            106259..106260 'q': u32
            106263..106314 'firstT..._mask)': u32
            106280..106285 'shots': ref<storage, array<ShotData>, read_write>
            106280..106295 'shots[shot_idx]': ref<storage, ShotData, read_write>
            106280..106313 'shots[...s_mask': ref<storage, u32, read_write>
            106286..106294 'shot_idx': u32
            106324..106329 'shots': ref<storage, array<ShotData>, read_write>
            106324..106339 'shots[shot_idx]': ref<storage, ShotData, read_write>
            106324..106357 'shots[...s_mask': ref<storage, u32, read_write>
            106330..106338 'shot_idx': u32
            106361..106371 '~(1u << q)': u32
            106363..106365 '1u': u32
            106363..106370 '1u << q': u32
            106369..106370 'q': u32
            106381..106386 'shots': ref<storage, array<ShotData>, read_write>
            106381..106396 'shots[shot_idx]': ref<storage, ShotData, read_write>
            106381..106403 'shots[...interp': ref<storage, InterpreterState, read_write>
            106381..106418 'shots[...op_idx': ref<storage, u32, read_write>
            106387..106395 'shot_idx': u32
            106421..106422 'q': u32
            106432..106437 'shots': ref<storage, array<ShotData>, read_write>
            106432..106447 'shots[shot_idx]': ref<storage, ShotData, read_write>
            106432..106454 'shots[...interp': ref<storage, InterpreterState, read_write>
            106432..106470 'shots[...p_type': ref<storage, u32, read_write>
            106438..106446 'shot_idx': u32
            106473..106495 'PENDIN...COMMIT': u32
            106505..106510 'shots': ref<storage, array<ShotData>, read_write>
            106505..106520 'shots[shot_idx]': ref<storage, ShotData, read_write>
            106505..106527 'shots[...interp': ref<storage, InterpreterState, read_write>
            106505..106534 'shots[...status': ref<storage, u32, read_write>
            106511..106519 'shot_idx': u32
            106537..106559 'STATUS...ENDING': u32
            106806..106812 'status': u32
            106806..106830 'status...UNNING': bool
            106816..106830 'STATUS_RUNNING': u32
            106841..106846 'shots': ref<storage, array<ShotData>, read_write>
            106841..106856 'shots[shot_idx]': ref<storage, ShotData, read_write>
            106841..106863 'shots[...interp': ref<storage, InterpreterState, read_write>
            106841..106870 'shots[...status': ref<storage, u32, read_write>
            106847..106855 'shot_idx': u32
            106873..106887 'STATUS_RUNNING': u32
            107134..107136 'pc': ref<function, u32, read_write>
            107144..107149 'state': InterpreterState
            107144..107152 'state.pc': u32
            107182..107190 'block_id': ref<function, u32, read_write>
            107198..107203 'state': InterpreterState
            107198..107220 'state....ock_id': u32
            107230..107240 'prev_block': ref<function, u32, read_write>
            107248..107253 'state': InterpreterState
            107248..107271 'state....ock_id': u32
            107292..107297 'steps': ref<function, u32, read_write>
            107305..107307 '0u': u32
            107375..107387 'should_break': ref<function, bool, read_write>
            107396..107401 'false': bool
            107876..107881 'steps': ref<function, u32, read_write>
            107876..107904 'steps ..._STEPS': bool
            107885..107904 'MAX_CL..._STEPS': u32
            108052..108057 'state': InterpreterState
            108052..108064 'state.status': u32
            108052..108080 'state...._ERROR': bool
            108068..108080 'STATUS_ERROR': u32
            108099..108104 'shots': ref<storage, array<ShotData>, read_write>
            108099..108114 'shots[shot_idx]': ref<storage, ShotData, read_write>
            108099..108121 'shots[...interp': ref<storage, InterpreterState, read_write>
            108099..108128 'shots[...status': ref<storage, u32, read_write>
            108105..108113 'shot_idx': u32
            108131..108143 'STATUS_YIELD': u32
            108333..108338 'instr': Instruction
            108341..108356 'fetch_instr(pc)': Instruction
            108353..108355 'pc': ref<function, u32, read_write>
            108843..108845 'op': u32
            108848..108872 'get_op...pcode)': u32
            108859..108864 'instr': Instruction
            108859..108871 'instr.opcode': u32
            108886..108893 'subcond': u32
            108896..108921 'get_su...pcode)': u32
            108908..108913 'instr': Instruction
            108908..108920 'instr.opcode': u32
            108935..108940 'flags': u32
            108943..108966 'get_fl...pcode)': u32
            108953..108958 'instr': Instruction
            108953..108965 'instr.opcode': u32
            109767..109769 'op': u32
            110044..110050 'OP_NOP': u32
            110069..110071 'pc': ref<function, u32, read_write>
            110649..110655 'OP_RET': u32
            110678..110687 'exit_code': u32
            110690..110733 'resolv...s, 2u)': u32
            110702..110710 'shot_idx': u32
            110712..110717 'instr': Instruction
            110712..110721 'instr.dst': u32
            110723..110728 'flags': u32
            110730..110732 '2u': u32
            110751..110756 'shots': ref<storage, array<ShotData>, read_write>
            110751..110766 'shots[shot_idx]': ref<storage, ShotData, read_write>
            110751..110773 'shots[...interp': ref<storage, InterpreterState, read_write>
            110751..110783 'shots[...t_code': ref<storage, u32, read_write>
            110757..110765 'shot_idx': u32
            110786..110795 'exit_code': u32
            110972..110981 'err_index': u32
            110984..111013 '(shot_..._COUNT': u32
            110984..111017 '(shot_...NT - 1': u32
            110985..110993 'shot_idx': u32
            110985..110997 'shot_idx + 1': u32
            110996..110997 '1': integer
            111001..111013 'RESULT_COUNT': u32
            111016..111017 '1': integer
            111035..111096 'atomic..._code)': __atomic_compare_exchange_result
            111061..111080 '&resul...index]': ptr<storage, atomic<u32>, read_write>
            111062..111069 'results': ref<storage, array<atomic<u32>>, read_write>
            111062..111080 'result...index]': ref<storage, atomic<u32>, read_write>
            111070..111079 'err_index': u32
            111082..111084 '0u': u32
            111086..111095 'exit_code': u32
            111114..111119 'shots': ref<storage, array<ShotData>, read_write>
            111114..111129 'shots[shot_idx]': ref<storage, ShotData, read_write>
            111114..111136 'shots[...interp': ref<storage, InterpreterState, read_write>
            111114..111143 'shots[...status': ref<storage, u32, read_write>
            111120..111128 'shot_idx': u32
            111146..111163 'STATUS...INATED': u32
            111181..111226 'atomic...t, 1u)': u32
            111191..111221 '&diagn..._count': ptr<storage, atomic<u32>, read_write>
            111192..111203 'diagnostics': ref<storage, DiagnosticData, read_write>
            111192..111221 'diagno..._count': ref<storage, atomic<u32>, read_write>
            111223..111225 '1u': u32
            111244..111256 'should_break': ref<function, bool, read_write>
            111259..111263 'true': bool
            111615..111622 'OP_JUMP': u32
            111641..111651 'prev_block': ref<function, u32, read_write>
            111654..111662 'block_id': ref<function, u32, read_write>
            111680..111688 'block_id': ref<function, u32, read_write>
            111691..111696 'instr': Instruction
            111691..111700 'instr.dst': u32
            111718..111720 'pc': ref<function, u32, read_write>
            111723..111733 'batch_data': ref<storage, BatchData, read>
            111723..111741 'batch_...rogram': ref<storage, Program, read>
            111723..111753 'batch_..._table': ref<storage, [error], read>
            111723..111764 'batch_...r.dst]': [error]
            111723..111777 'batch_...offset': [error]
            111754..111759 'instr': Instruction
            111754..111763 'instr.dst': u32
            112193..112202 'OP_BRANCH': u32
            112225..112229 'cond': bool
            112232..112276 'resolv...s, 0u)': u32
            112232..112282 'resolv... != 0u': bool
            112244..112252 'shot_idx': u32
            112254..112259 'instr': Instruction
            112254..112264 'instr.src0': u32
            112266..112271 'flags': u32
            112273..112275 '0u': u32
            112280..112282 '0u': u32
            112300..112310 'prev_block': ref<function, u32, read_write>
            112313..112321 'block_id': ref<function, u32, read_write>
            112342..112346 'cond': bool
            112369..112377 'block_id': ref<function, u32, read_write>
            112380..112385 'instr': Instruction
            112380..112390 'instr.aux0': u32
            112412..112414 'pc': ref<function, u32, read_write>
            112417..112427 'batch_data': ref<storage, BatchData, read>
            112417..112435 'batch_...rogram': ref<storage, Program, read>
            112417..112447 'batch_..._table': ref<storage, [error], read>
            112417..112459 'batch_....aux0]': [error]
            112417..112472 'batch_...offset': [error]
            112448..112453 'instr': Instruction
            112448..112458 'instr.aux0': u32
            112519..112527 'block_id': ref<function, u32, read_write>
            112530..112535 'instr': Instruction
            112530..112540 'instr.aux1': u32
            112562..112564 'pc': ref<function, u32, read_write>
            112567..112577 'batch_data': ref<storage, BatchData, read>
            112567..112585 'batch_...rogram': ref<storage, Program, read>
            112567..112597 'batch_..._table': ref<storage, [error], read>
            112567..112609 'batch_....aux1]': [error]
            112567..112622 'batch_...offset': [error]
            112598..112603 'instr': Instruction
            112598..112608 'instr.aux1': u32
            113189..113198 'OP_SWITCH': u32
            113221..113224 'val': u32
            113227..113271 'resolv...s, 0u)': u32
            113239..113247 'shot_idx': u32
            113249..113254 'instr': Instruction
            113249..113259 'instr.src0': u32
            113261..113266 'flags': u32
            113268..113270 '0u': u32
            113293..113306 'default_block': u32
            113309..113314 'instr': Instruction
            113309..113319 'instr.aux0': u32
            113341..113352 'case_offset': u32
            113355..113360 'instr': Instruction
            113355..113365 'instr.aux1': u32
            113387..113397 'case_count': u32
            113400..113405 'instr': Instruction
            113400..113410 'instr.aux2': u32
            113432..113444 'target_block': ref<function, u32, read_write>
            113447..113460 'default_block': u32
            113487..113488 'i': ref<function, u32, read_write>
            113491..113493 '0u': u32
            113495..113496 'i': ref<function, u32, read_write>
            113495..113509 'i < case_count': bool
            113499..113509 'case_count': u32
            113511..113512 'i': ref<function, u32, read_write>
            113542..113547 'entry': [error]
            113550..113560 'batch_data': ref<storage, BatchData, read>
            113550..113568 'batch_...rogram': ref<storage, Program, read>
            113550..113581 'batch_..._table': ref<storage, [error], read>
            113550..113598 'batch_...t + i]': [error]
            113582..113593 'case_offset': u32
            113582..113597 'case_offset + i': u32
            113596..113597 'i': ref<function, u32, read_write>
            113623..113628 'entry': [error]
            113623..113637 'entry.case_val': [error]
            113623..113644 'entry....== val': [error]
            113641..113644 'val': u32
            113671..113683 'target_block': ref<function, u32, read_write>
            113686..113691 'entry': [error]
            113686..113704 'entry...._block': [error]
            113793..113803 'prev_block': ref<function, u32, read_write>
            113806..113814 'block_id': ref<function, u32, read_write>
            113832..113840 'block_id': ref<function, u32, read_write>
            113843..113855 'target_block': ref<function, u32, read_write>
            113873..113875 'pc': ref<function, u32, read_write>
            113878..113888 'batch_data': ref<storage, BatchData, read>
            113878..113896 'batch_...rogram': ref<storage, Program, read>
            113878..113908 'batch_..._table': ref<storage, [error], read>
            113878..113922 'batch_...block]': [error]
            113878..113935 'batch_...offset': [error]
            113909..113921 'target_block': ref<function, u32, read_write>
            114905..114912 'OP_CALL': u32
            114935..114942 'func_id': u32
            114945..114950 'instr': Instruction
            114945..114955 'instr.aux0': u32
            114977..114986 'arg_count': u32
            114989..114994 'instr': Instruction
            114989..114999 'instr.aux1': u32
            115021..115031 'arg_offset': u32
            115034..115039 'instr': Instruction
            115034..115044 'instr.aux2': u32
            115066..115070 'func': [error]
            115073..115083 'batch_data': ref<storage, BatchData, read>
            115073..115091 'batch_...rogram': ref<storage, Program, read>
            115073..115106 'batch_..._table': ref<storage, [error], read>
            115073..115115 'batch_...nc_id]': [error]
            115107..115114 'func_id': u32
            115193..115195 'sp': u32
            115198..115203 'shots': ref<storage, array<ShotData>, read_write>
            115198..115213 'shots[shot_idx]': ref<storage, ShotData, read_write>
            115198..115220 'shots[...interp': ref<storage, InterpreterState, read_write>
            115198..115228 'shots[...all_sp': ref<storage, u32, read_write>
            115204..115212 'shot_idx': u32
            115318..115320 'sp': u32
            115318..115326 'sp >= 8u': bool
            115324..115326 '8u': u32
            115349..115354 'shots': ref<storage, array<ShotData>, read_write>
            115349..115364 'shots[shot_idx]': ref<storage, ShotData, read_write>
            115349..115371 'shots[...interp': ref<storage, InterpreterState, read_write>
            115349..115381 'shots[...t_code': ref<storage, u32, read_write>
            115355..115363 'shot_idx': u32
            115384..115407 'ERR_CA...ERFLOW': u32
            115433..115440 'err_idx': u32
            115443..115472 '(shot_..._COUNT': u32
            115443..115476 '(shot_...NT - 1': u32
            115444..115452 'shot_idx': u32
            115444..115456 'shot_idx + 1': u32
            115455..115456 '1': integer
            115460..115472 'RESULT_COUNT': u32
            115475..115476 '1': integer
            115498..115571 'atomic...RFLOW)': __atomic_compare_exchange_result
            115524..115541 '&resul...r_idx]': ptr<storage, atomic<u32>, read_write>
            115525..115532 'results': ref<storage, array<atomic<u32>>, read_write>
            115525..115541 'result...r_idx]': ref<storage, atomic<u32>, read_write>
            115533..115540 'err_idx': u32
            115543..115545 '0u': u32
            115547..115570 'ERR_CA...ERFLOW': u32
            115593..115598 'shots': ref<storage, array<ShotData>, read_write>
            115593..115608 'shots[shot_idx]': ref<storage, ShotData, read_write>
            115593..115615 'shots[...interp': ref<storage, InterpreterState, read_write>
            115593..115622 'shots[...status': ref<storage, u32, read_write>
            115599..115607 'shot_idx': u32
            115625..115637 'STATUS_ERROR': u32
            115659..115704 'atomic...t, 1u)': u32
            115669..115699 '&diagn..._count': ptr<storage, atomic<u32>, read_write>
            115670..115681 'diagnostics': ref<storage, DiagnosticData, read_write>
            115670..115699 'diagno..._count': ref<storage, atomic<u32>, read_write>
            115701..115703 '1u': u32
            115726..115738 'should_break': ref<function, bool, read_write>
            115741..115745 'true': bool
            115808..115813 'shots': ref<storage, array<ShotData>, read_write>
            115808..115823 'shots[shot_idx]': ref<storage, ShotData, read_write>
            115808..115830 'shots[...interp': ref<storage, InterpreterState, read_write>
            115808..115848 'shots[...frames': ref<storage, array<CallStackFrame, 14>, read_write>
            115808..115852 'shots[...es[sp]': ref<storage, CallStackFrame, read_write>
            115808..115861 'shots[...ock_id': ref<storage, u32, read_write>
            115814..115822 'shot_idx': u32
            115849..115851 'sp': u32
            115864..115872 'block_id': ref<function, u32, read_write>
            115935..115940 'shots': ref<storage, array<ShotData>, read_write>
            115935..115950 'shots[shot_idx]': ref<storage, ShotData, read_write>
            115935..115957 'shots[...interp': ref<storage, InterpreterState, read_write>
            115935..115975 'shots[...frames': ref<storage, array<CallStackFrame, 14>, read_write>
            115935..115979 'shots[...es[sp]': ref<storage, CallStackFrame, read_write>
            115935..115989 'shots[...urn_pc': ref<storage, u32, read_write>
            115941..115949 'shot_idx': u32
            115976..115978 'sp': u32
            115992..115994 'pc': ref<function, u32, read_write>
            115992..115999 'pc + 1u': u32
            115997..115999 '1u': u32
            116064..116069 'shots': ref<storage, array<ShotData>, read_write>
            116064..116079 'shots[shot_idx]': ref<storage, ShotData, read_write>
            116064..116086 'shots[...interp': ref<storage, InterpreterState, read_write>
            116064..116104 'shots[...frames': ref<storage, array<CallStackFrame, 14>, read_write>
            116064..116108 'shots[...es[sp]': ref<storage, CallStackFrame, read_write>
            116064..116119 'shots[...rn_reg': ref<storage, u32, read_write>
            116070..116078 'shot_idx': u32
            116105..116107 'sp': u32
            116122..116127 'instr': Instruction
            116122..116131 'instr.dst': u32
            116189..116194 'shots': ref<storage, array<ShotData>, read_write>
            116189..116204 'shots[shot_idx]': ref<storage, ShotData, read_write>
            116189..116211 'shots[...interp': ref<storage, InterpreterState, read_write>
            116189..116219 'shots[...all_sp': ref<storage, u32, read_write>
            116195..116203 'shot_idx': u32
            116222..116224 'sp': u32
            116222..116229 'sp + 1u': u32
            116227..116229 '1u': u32
            116330..116340 'param_base': [error]
            116343..116347 'func': [error]
            116343..116362 'func.p...se_reg': [error]
            116389..116390 'i': ref<function, u32, read_write>
            116393..116395 '0u': u32
            116397..116398 'i': ref<function, u32, read_write>
            116397..116410 'i < arg_count': bool
            116401..116410 'arg_count': u32
            116412..116413 'i': ref<function, u32, read_write>
            116443..116450 'arg_reg': [error]
            116453..116463 'batch_data': ref<storage, BatchData, read>
            116453..116471 'batch_...rogram': ref<storage, Program, read>
            116453..116486 'batch_..._table': ref<storage, [error], read>
            116453..116502 'batch_...t + i]': [error]
            116487..116497 'arg_offset': u32
            116487..116501 'arg_offset + i': u32
            116500..116501 'i': ref<function, u32, read_write>
            116524..116588 'write_..._reg))': [error]
            116534..116542 'shot_idx': u32
            116544..116554 'param_base': [error]
            116544..116558 'param_base + i': [error]
            116557..116558 'i': ref<function, u32, read_write>
            116560..116587 'read_r...g_reg)': u32
            116569..116577 'shot_idx': u32
            116579..116586 'arg_reg': [error]
            116688..116696 'block_id': ref<function, u32, read_write>
            116699..116703 'func': [error]
            116699..116718 'func.e...ock_id': [error]
            116736..116738 'pc': ref<function, u32, read_write>
            116741..116751 'batch_data': ref<storage, BatchData, read>
            116741..116759 'batch_...rogram': ref<storage, Program, read>
            116741..116771 'batch_..._table': ref<storage, [error], read>
            116741..116781 'batch_...ck_id]': [error]
            116741..116794 'batch_...offset': [error]
            116772..116780 'block_id': ref<function, u32, read_write>
            117233..117247 'OP_CALL_RETURN': u32
            117269..117274 'shots': ref<storage, array<ShotData>, read_write>
            117269..117284 'shots[shot_idx]': ref<storage, ShotData, read_write>
            117269..117291 'shots[...interp': ref<storage, InterpreterState, read_write>
            117269..117299 'shots[...all_sp': ref<storage, u32, read_write>
            117269..117305 'shots[... == 0u': bool
            117275..117283 'shot_idx': u32
            117303..117305 '0u': u32
            117328..117333 'shots': ref<storage, array<ShotData>, read_write>
            117328..117343 'shots[shot_idx]': ref<storage, ShotData, read_write>
            117328..117350 'shots[...interp': ref<storage, InterpreterState, read_write>
            117328..117360 'shots[...t_code': ref<storage, u32, read_write>
            117334..117342 'shot_idx': u32
            117363..117387 'ERR_CA...ERFLOW': u32
            117413..117420 'err_idx': u32
            117423..117452 '(shot_..._COUNT': u32
            117423..117456 '(shot_...NT - 1': u32
            117424..117432 'shot_idx': u32
            117424..117436 'shot_idx + 1': u32
            117435..117436 '1': integer
            117440..117452 'RESULT_COUNT': u32
            117455..117456 '1': integer
            117478..117552 'atomic...RFLOW)': __atomic_compare_exchange_result
            117504..117521 '&resul...r_idx]': ptr<storage, atomic<u32>, read_write>
            117505..117512 'results': ref<storage, array<atomic<u32>>, read_write>
            117505..117521 'result...r_idx]': ref<storage, atomic<u32>, read_write>
            117513..117520 'err_idx': u32
            117523..117525 '0u': u32
            117527..117551 'ERR_CA...ERFLOW': u32
            117574..117579 'shots': ref<storage, array<ShotData>, read_write>
            117574..117589 'shots[shot_idx]': ref<storage, ShotData, read_write>
            117574..117596 'shots[...interp': ref<storage, InterpreterState, read_write>
            117574..117603 'shots[...status': ref<storage, u32, read_write>
            117580..117588 'shot_idx': u32
            117606..117618 'STATUS_ERROR': u32
            117640..117685 'atomic...t, 1u)': u32
            117650..117680 '&diagn..._count': ptr<storage, atomic<u32>, read_write>
            117651..117662 'diagnostics': ref<storage, DiagnosticData, read_write>
            117651..117680 'diagno..._count': ref<storage, atomic<u32>, read_write>
            117682..117684 '1u': u32
            117707..117719 'should_break': ref<function, bool, read_write>
            117722..117726 'true': bool
            117794..117796 'sp': u32
            117799..117804 'shots': ref<storage, array<ShotData>, read_write>
            117799..117814 'shots[shot_idx]': ref<storage, ShotData, read_write>
            117799..117821 'shots[...interp': ref<storage, InterpreterState, read_write>
            117799..117829 'shots[...all_sp': ref<storage, u32, read_write>
            117799..117833 'shots[...sp - 1': u32
            117805..117813 'shot_idx': u32
            117832..117833 '1': integer
            117851..117856 'shots': ref<storage, array<ShotData>, read_write>
            117851..117866 'shots[shot_idx]': ref<storage, ShotData, read_write>
            117851..117873 'shots[...interp': ref<storage, InterpreterState, read_write>
            117851..117881 'shots[...all_sp': ref<storage, u32, read_write>
            117857..117865 'shot_idx': u32
            117884..117886 'sp': u32
            117904..117912 'block_id': ref<function, u32, read_write>
            117915..117920 'shots': ref<storage, array<ShotData>, read_write>
            117915..117930 'shots[shot_idx]': ref<storage, ShotData, read_write>
            117915..117937 'shots[...interp': ref<storage, InterpreterState, read_write>
            117915..117955 'shots[...frames': ref<storage, array<CallStackFrame, 14>, read_write>
            117915..117959 'shots[...es[sp]': ref<storage, CallStackFrame, read_write>
            117915..117968 'shots[...ock_id': ref<storage, u32, read_write>
            117921..117929 'shot_idx': u32
            117956..117958 'sp': u32
            117986..117988 'pc': ref<function, u32, read_write>
            117991..117996 'shots': ref<storage, array<ShotData>, read_write>
            117991..118006 'shots[shot_idx]': ref<storage, ShotData, read_write>
            117991..118013 'shots[...interp': ref<storage, InterpreterState, read_write>
            117991..118031 'shots[...frames': ref<storage, array<CallStackFrame, 14>, read_write>
            117991..118035 'shots[...es[sp]': ref<storage, CallStackFrame, read_write>
            117991..118045 'shots[...urn_pc': ref<storage, u32, read_write>
            117997..118005 'shot_idx': u32
            118032..118034 'sp': u32
            118067..118077 'return_reg': u32
            118080..118085 'shots': ref<storage, array<ShotData>, read_write>
            118080..118095 'shots[shot_idx]': ref<storage, ShotData, read_write>
            118080..118102 'shots[...interp': ref<storage, InterpreterState, read_write>
            118080..118120 'shots[...frames': ref<storage, array<CallStackFrame, 14>, read_write>
            118080..118124 'shots[...es[sp]': ref<storage, CallStackFrame, read_write>
            118080..118135 'shots[...rn_reg': ref<storage, u32, read_write>
            118086..118094 'shot_idx': u32
            118121..118123 'sp': u32
            118156..118166 'return_reg': u32
            118156..118181 'return...RETURN': bool
            118170..118181 'VOID_RETURN': u32
            118204..118267 'write_...src0))': [error]
            118214..118222 'shot_idx': u32
            118224..118234 'return_reg': u32
            118236..118266 'read_r....src0)': u32
            118245..118253 'shot_idx': u32
            118255..118260 'instr': Instruction
            118255..118265 'instr.src0': u32
            119815..119830 'OP_QUANTUM_GATE': u32
            119849..119854 'shots': ref<storage, array<ShotData>, read_write>
            119849..119864 'shots[shot_idx]': ref<storage, ShotData, read_write>
            119849..119871 'shots[...interp': ref<storage, InterpreterState, read_write>
            119849..119886 'shots[...op_idx': ref<storage, u32, read_write>
            119855..119863 'shot_idx': u32
            119889..119894 'instr': Instruction
            119889..119899 'instr.aux0': u32
            119917..119922 'shots': ref<storage, array<ShotData>, read_write>
            119917..119932 'shots[shot_idx]': ref<storage, ShotData, read_write>
            119917..119939 'shots[...interp': ref<storage, InterpreterState, read_write>
            119917..119955 'shots[...p_type': ref<storage, u32, read_write>
            119923..119931 'shot_idx': u32
            119958..119960 '0u': u32
            120222..120227 'shots': ref<storage, array<ShotData>, read_write>
            120222..120237 'shots[shot_idx]': ref<storage, ShotData, read_write>
            120222..120244 'shots[...interp': ref<storage, InterpreterState, read_write>
            120222..120251 'shots[...status': ref<storage, u32, read_write>
            120228..120236 'shot_idx': u32
            120254..120276 'STATUS...ENDING': u32
            120294..120296 'pc': ref<function, u32, read_write>
            120316..120328 'should_break': ref<function, bool, read_write>
            120331..120335 'true': bool
            120606..120616 'OP_MEASURE': u32
            120635..120640 'shots': ref<storage, array<ShotData>, read_write>
            120635..120650 'shots[shot_idx]': ref<storage, ShotData, read_write>
            120635..120657 'shots[...interp': ref<storage, InterpreterState, read_write>
            120635..120672 'shots[...op_idx': ref<storage, u32, read_write>
            120641..120649 'shot_idx': u32
            120675..120680 'instr': Instruction
            120675..120685 'instr.aux0': u32
            120703..120708 'shots': ref<storage, array<ShotData>, read_write>
            120703..120718 'shots[shot_idx]': ref<storage, ShotData, read_write>
            120703..120725 'shots[...interp': ref<storage, InterpreterState, read_write>
            120703..120741 'shots[...p_type': ref<storage, u32, read_write>
            120709..120717 'shot_idx': u32
            120744..120746 '1u': u32
            120912..120917 'shots': ref<storage, array<ShotData>, read_write>
            120912..120927 'shots[shot_idx]': ref<storage, ShotData, read_write>
            120912..120934 'shots[...interp': ref<storage, InterpreterState, read_write>
            120912..120941 'shots[...status': ref<storage, u32, read_write>
            120918..120926 'shot_idx': u32
            120944..120966 'STATUS...ENDING': u32
            120984..120986 'pc': ref<function, u32, read_write>
            121006..121018 'should_break': ref<function, bool, read_write>
            121021..121025 'true': bool
            121248..121256 'OP_RESET': u32
            121275..121280 'shots': ref<storage, array<ShotData>, read_write>
            121275..121290 'shots[shot_idx]': ref<storage, ShotData, read_write>
            121275..121297 'shots[...interp': ref<storage, InterpreterState, read_write>
            121275..121312 'shots[...op_idx': ref<storage, u32, read_write>
            121281..121289 'shot_idx': u32
            121315..121320 'instr': Instruction
            121315..121325 'instr.aux0': u32
            121343..121348 'shots': ref<storage, array<ShotData>, read_write>
            121343..121358 'shots[shot_idx]': ref<storage, ShotData, read_write>
            121343..121365 'shots[...interp': ref<storage, InterpreterState, read_write>
            121343..121381 'shots[...p_type': ref<storage, u32, read_write>
            121349..121357 'shot_idx': u32
            121384..121386 '2u': u32
            121499..121504 'shots': ref<storage, array<ShotData>, read_write>
            121499..121514 'shots[shot_idx]': ref<storage, ShotData, read_write>
            121499..121521 'shots[...interp': ref<storage, InterpreterState, read_write>
            121499..121528 'shots[...status': ref<storage, u32, read_write>
            121505..121513 'shot_idx': u32
            121531..121553 'STATUS...ENDING': u32
            121571..121573 'pc': ref<function, u32, read_write>
            121593..121605 'should_break': ref<function, bool, read_write>
            121608..121612 'true': bool
            122364..122378 'OP_READ_RESULT': u32
            122401..122410 'result_id': u32
            122413..122418 'instr': Instruction
            122413..122423 'instr.src0': u32
            122445..122455 'result_val': bool
            122458..122502 'read_m...lt_id)': bool
            122482..122490 'shot_idx': u32
            122492..122501 'result_id': u32
            122520..122578 'write_..._val))': [error]
            122530..122538 'shot_idx': u32
            122540..122545 'instr': Instruction
            122540..122549 'instr.dst': u32
            122551..122577 'select...t_val)': u32
            122558..122560 '0u': u32
            122562..122564 '1u': u32
            122566..122576 'result_val': bool
            122596..122598 'pc': ref<function, u32, read_write>
            122928..122944 'OP_REC...OUTPUT': u32
            122963..122965 'pc': ref<function, u32, read_write>
            123335..123347 'OP_READ_LOSS': u32
            123370..123379 'result_id': u32
            123382..123387 'instr': Instruction
            123382..123392 'instr.src0': u32
            123414..123417 'val': u32
            123420..123477 'atomic...t_id])': u32
            123431..123476 '&resul...lt_id]': ptr<storage, atomic<u32>, read_write>
            123432..123439 'results': ref<storage, array<atomic<u32>>, read_write>
            123432..123476 'result...lt_id]': ref<storage, atomic<u32>, read_write>
            123440..123448 'shot_idx': u32
            123440..123463 'shot_i..._COUNT': u32
            123440..123475 'shot_i...ult_id': u32
            123451..123463 'RESULT_COUNT': u32
            123466..123475 'result_id': u32
            123495..123552 'write_...= 2u))': [error]
            123505..123513 'shot_idx': u32
            123515..123520 'instr': Instruction
            123515..123524 'instr.dst': u32
            123526..123551 'select...== 2u)': u32
            123533..123535 '0u': u32
            123537..123539 '1u': u32
            123541..123544 'val': u32
            123541..123550 'val == 2u': bool
            123548..123550 '2u': u32
            123570..123572 'pc': ref<function, u32, read_write>
            124180..124186 'OP_ADD': u32
            124209..124210 'a': i32
            124213..124257 'resolv...s, 0u)': i32
            124225..124233 'shot_idx': u32
            124235..124240 'instr': Instruction
            124235..124245 'instr.src0': u32
            124247..124252 'flags': u32
            124254..124256 '0u': u32
            124279..124280 'b': i32
            124283..124327 'resolv...s, 1u)': i32
            124295..124303 'shot_idx': u32
            124305..124310 'instr': Instruction
            124305..124315 'instr.src1': u32
            124317..124322 'flags': u32
            124324..124326 '1u': u32
            124345..124386 'write_...a + b)': [error]
            124359..124367 'shot_idx': u32
            124369..124374 'instr': Instruction
            124369..124378 'instr.dst': u32
            124380..124381 'a': i32
            124380..124385 'a + b': i32
            124384..124385 'b': i32
            124404..124406 'pc': ref<function, u32, read_write>
            124509..124515 'OP_SUB': u32
            124538..124539 'a': i32
            124542..124586 'resolv...s, 0u)': i32
            124554..124562 'shot_idx': u32
            124564..124569 'instr': Instruction
            124564..124574 'instr.src0': u32
            124576..124581 'flags': u32
            124583..124585 '0u': u32
            124608..124609 'b': i32
            124612..124656 'resolv...s, 1u)': i32
            124624..124632 'shot_idx': u32
            124634..124639 'instr': Instruction
            124634..124644 'instr.src1': u32
            124646..124651 'flags': u32
            124653..124655 '1u': u32
            124674..124715 'write_...a - b)': [error]
            124688..124696 'shot_idx': u32
            124698..124703 'instr': Instruction
            124698..124707 'instr.dst': u32
            124709..124710 'a': i32
            124709..124714 'a - b': i32
            124713..124714 'b': i32
            124733..124735 'pc': ref<function, u32, read_write>
            124841..124847 'OP_MUL': u32
            124870..124871 'a': i32
            124874..124918 'resolv...s, 0u)': i32
            124886..124894 'shot_idx': u32
            124896..124901 'instr': Instruction
            124896..124906 'instr.src0': u32
            124908..124913 'flags': u32
            124915..124917 '0u': u32
            124940..124941 'b': i32
            124944..124988 'resolv...s, 1u)': i32
            124956..124964 'shot_idx': u32
            124966..124971 'instr': Instruction
            124966..124976 'instr.src1': u32
            124978..124983 'flags': u32
            124985..124987 '1u': u32
            125006..125047 'write_...a * b)': [error]
            125020..125028 'shot_idx': u32
            125030..125035 'instr': Instruction
            125030..125039 'instr.dst': u32
            125041..125042 'a': i32
            125041..125046 'a * b': i32
            125045..125046 'b': i32
            125065..125067 'pc': ref<function, u32, read_write>
            125170..125177 'OP_UDIV': u32
            125200..125201 'a': u32
            125204..125248 'resolv...s, 0u)': u32
            125216..125224 'shot_idx': u32
            125226..125231 'instr': Instruction
            125226..125236 'instr.src0': u32
            125238..125243 'flags': u32
            125245..125247 '0u': u32
            125270..125271 'b': u32
            125274..125318 'resolv...s, 1u)': u32
            125286..125294 'shot_idx': u32
            125296..125301 'instr': Instruction
            125296..125306 'instr.src1': u32
            125308..125313 'flags': u32
            125315..125317 '1u': u32
            125336..125373 'write_...a / b)': [error]
            125346..125354 'shot_idx': u32
            125356..125361 'instr': Instruction
            125356..125365 'instr.dst': u32
            125367..125368 'a': u32
            125367..125372 'a / b': u32
            125371..125372 'b': u32
            125391..125393 'pc': ref<function, u32, read_write>
            125518..125525 'OP_SDIV': u32
            125548..125549 'a': i32
            125552..125596 'resolv...s, 0u)': i32
            125564..125572 'shot_idx': u32
            125574..125579 'instr': Instruction
            125574..125584 'instr.src0': u32
            125586..125591 'flags': u32
            125593..125595 '0u': u32
            125618..125619 'b': i32
            125622..125666 'resolv...s, 1u)': i32
            125634..125642 'shot_idx': u32
            125644..125649 'instr': Instruction
            125644..125654 'instr.src1': u32
            125656..125661 'flags': u32
            125663..125665 '1u': u32
            125684..125725 'write_...a / b)': [error]
            125698..125706 'shot_idx': u32
            125708..125713 'instr': Instruction
            125708..125717 'instr.dst': u32
            125719..125720 'a': i32
            125719..125724 'a / b': i32
            125723..125724 'b': i32
            125743..125745 'pc': ref<function, u32, read_write>
            125849..125856 'OP_UREM': u32
            125879..125880 'a': u32
            125883..125927 'resolv...s, 0u)': u32
            125895..125903 'shot_idx': u32
            125905..125910 'instr': Instruction
            125905..125915 'instr.src0': u32
            125917..125922 'flags': u32
            125924..125926 '0u': u32
            125949..125950 'b': u32
            125953..125997 'resolv...s, 1u)': u32
            125965..125973 'shot_idx': u32
            125975..125980 'instr': Instruction
            125975..125985 'instr.src1': u32
            125987..125992 'flags': u32
            125994..125996 '1u': u32
            126015..126052 'write_...a % b)': [error]
            126025..126033 'shot_idx': u32
            126035..126040 'instr': Instruction
            126035..126044 'instr.dst': u32
            126046..126047 'a': u32
            126046..126051 'a % b': u32
            126050..126051 'b': u32
            126070..126072 'pc': ref<function, u32, read_write>
            126452..126459 'OP_SREM': u32
            126482..126483 'a': i32
            126486..126530 'resolv...s, 0u)': i32
            126498..126506 'shot_idx': u32
            126508..126513 'instr': Instruction
            126508..126518 'instr.src0': u32
            126520..126525 'flags': u32
            126527..126529 '0u': u32
            126552..126553 'b': i32
            126556..126600 'resolv...s, 1u)': i32
            126568..126576 'shot_idx': u32
            126578..126583 'instr': Instruction
            126578..126588 'instr.src1': u32
            126590..126595 'flags': u32
            126597..126599 '1u': u32
            126618..126669 'write_... / b))': [error]
            126632..126640 'shot_idx': u32
            126642..126647 'instr': Instruction
            126642..126651 'instr.dst': u32
            126653..126654 'a': i32
            126653..126668 'a - b * (a / b)': i32
            126657..126658 'b': i32
            126657..126668 'b * (a / b)': i32
            126662..126663 'a': i32
            126662..126667 'a / b': i32
            126666..126667 'b': i32
            126687..126689 'pc': ref<function, u32, read_write>
            127048..127054 'OP_AND': u32
            127073..127216 'write_..., 1u))': [error]
            127083..127091 'shot_idx': u32
            127093..127098 'instr': Instruction
            127093..127102 'instr.dst': u32
            127124..127168 'resolv...s, 0u)': u32
            127124..127215 'resolv...s, 1u)': u32
            127136..127144 'shot_idx': u32
            127146..127151 'instr': Instruction
            127146..127156 'instr.src0': u32
            127158..127163 'flags': u32
            127165..127167 '0u': u32
            127171..127215 'resolv...s, 1u)': u32
            127183..127191 'shot_idx': u32
            127193..127198 'instr': Instruction
            127193..127203 'instr.src1': u32
            127205..127210 'flags': u32
            127212..127214 '1u': u32
            127234..127236 'pc': ref<function, u32, read_write>
            127322..127327 'OP_OR': u32
            127346..127489 'write_..., 1u))': [error]
            127356..127364 'shot_idx': u32
            127366..127371 'instr': Instruction
            127366..127375 'instr.dst': u32
            127397..127441 'resolv...s, 0u)': u32
            127397..127488 'resolv...s, 1u)': u32
            127409..127417 'shot_idx': u32
            127419..127424 'instr': Instruction
            127419..127429 'instr.src0': u32
            127431..127436 'flags': u32
            127438..127440 '0u': u32
            127444..127488 'resolv...s, 1u)': u32
            127456..127464 'shot_idx': u32
            127466..127471 'instr': Instruction
            127466..127476 'instr.src1': u32
            127478..127483 'flags': u32
            127485..127487 '1u': u32
            127507..127509 'pc': ref<function, u32, read_write>
            127606..127612 'OP_XOR': u32
            127631..127774 'write_..., 1u))': [error]
            127641..127649 'shot_idx': u32
            127651..127656 'instr': Instruction
            127651..127660 'instr.dst': u32
            127682..127726 'resolv...s, 0u)': u32
            127682..127773 'resolv...s, 1u)': u32
            127694..127702 'shot_idx': u32
            127704..127709 'instr': Instruction
            127704..127714 'instr.src0': u32
            127716..127721 'flags': u32
            127723..127725 '0u': u32
            127729..127773 'resolv...s, 1u)': u32
            127741..127749 'shot_idx': u32
            127751..127756 'instr': Instruction
            127751..127761 'instr.src1': u32
            127763..127768 'flags': u32
            127770..127772 '1u': u32
            127792..127794 'pc': ref<function, u32, read_write>
            127890..127896 'OP_SHL': u32
            127915..128059 'write_..., 1u))': [error]
            127925..127933 'shot_idx': u32
            127935..127940 'instr': Instruction
            127935..127944 'instr.dst': u32
            127966..128010 'resolv...s, 0u)': u32
            127966..128058 'resolv...s, 1u)': u32
            127978..127986 'shot_idx': u32
            127988..127993 'instr': Instruction
            127988..127998 'instr.src0': u32
            128000..128005 'flags': u32
            128007..128009 '0u': u32
            128014..128058 'resolv...s, 1u)': u32
            128026..128034 'shot_idx': u32
            128036..128041 'instr': Instruction
            128036..128046 'instr.src1': u32
            128048..128053 'flags': u32
            128055..128057 '1u': u32
            128077..128079 'pc': ref<function, u32, read_write>
            128189..128196 'OP_LSHR': u32
            128215..128359 'write_..., 1u))': [error]
            128225..128233 'shot_idx': u32
            128235..128240 'instr': Instruction
            128235..128244 'instr.dst': u32
            128266..128310 'resolv...s, 0u)': u32
            128266..128358 'resolv...s, 1u)': u32
            128278..128286 'shot_idx': u32
            128288..128293 'instr': Instruction
            128288..128298 'instr.src0': u32
            128300..128305 'flags': u32
            128307..128309 '0u': u32
            128314..128358 'resolv...s, 1u)': u32
            128326..128334 'shot_idx': u32
            128336..128341 'instr': Instruction
            128336..128346 'instr.src1': u32
            128348..128353 'flags': u32
            128355..128357 '1u': u32
            128377..128379 'pc': ref<function, u32, read_write>
            128564..128571 'OP_ASHR': u32
            128594..128595 'a': i32
            128598..128642 'resolv...s, 0u)': i32
            128610..128618 'shot_idx': u32
            128620..128625 'instr': Instruction
            128620..128630 'instr.src0': u32
            128632..128637 'flags': u32
            128639..128641 '0u': u32
            128664..128665 'b': u32
            128668..128712 'resolv...s, 1u)': u32
            128680..128688 'shot_idx': u32
            128690..128695 'instr': Instruction
            128690..128700 'instr.src1': u32
            128702..128707 'flags': u32
            128709..128711 '1u': u32
            128730..128772 'write_... >> b)': [error]
            128744..128752 'shot_idx': u32
            128754..128759 'instr': Instruction
            128754..128763 'instr.dst': u32
            128765..128766 'a': i32
            128765..128771 'a >> b': i32
            128770..128771 'b': u32
            128790..128792 'pc': ref<function, u32, read_write>
            129444..129451 'OP_ICMP': u32
            129474..129475 'a': i32
            129478..129522 'resolv...s, 0u)': i32
            129490..129498 'shot_idx': u32
            129500..129505 'instr': Instruction
            129500..129510 'instr.src0': u32
            129512..129517 'flags': u32
            129519..129521 '0u': u32
            129544..129545 'b': i32
            129548..129592 'resolv...s, 1u)': i32
            129560..129568 'shot_idx': u32
            129570..129575 'instr': Instruction
            129570..129580 'instr.src1': u32
            129582..129587 'flags': u32
            129589..129591 '1u': u32
            129614..129620 'result': ref<function, bool, read_write>
            129629..129634 'false': bool
            129659..129666 'subcond': u32
            129694..129701 'ICMP_EQ': u32
            129705..129711 'result': ref<function, bool, read_write>
            129715..129716 'a': i32
            129715..129721 'a == b': bool
            129720..129721 'b': i32
            129751..129758 'ICMP_NE': u32
            129762..129768 'result': ref<function, bool, read_write>
            129772..129773 'a': i32
            129772..129778 'a != b': bool
            129777..129778 'b': i32
            129808..129816 'ICMP_SLT': u32
            129819..129825 'result': ref<function, bool, read_write>
            129829..129830 'a': i32
            129829..129834 'a < b': bool
            129833..129834 'b': i32
            129864..129872 'ICMP_SLE': u32
            129875..129881 'result': ref<function, bool, read_write>
            129885..129886 'a': i32
            129885..129891 'a <= b': bool
            129890..129891 'b': i32
            129921..129929 'ICMP_SGT': u32
            129932..129938 'result': ref<function, bool, read_write>
            129942..129943 'a': i32
            129942..129947 'a > b': bool
            129946..129947 'b': i32
            129977..129985 'ICMP_SGE': u32
            129988..129994 'result': ref<function, bool, read_write>
            129998..129999 'a': i32
            129998..130004 'a >= b': bool
            130003..130004 'b': i32
            130034..130042 'ICMP_ULT': u32
            130045..130051 'result': ref<function, bool, read_write>
            130055..130070 'bitcast<u32>(a)': u32
            130055..130088 'bitcas...32>(b)': bool
            130068..130069 'a': i32
            130073..130088 'bitcast<u32>(b)': u32
            130086..130087 'b': i32
            130118..130126 'ICMP_ULE': u32
            130129..130135 'result': ref<function, bool, read_write>
            130139..130154 'bitcast<u32>(a)': u32
            130139..130173 'bitcas...32>(b)': bool
            130152..130153 'a': i32
            130158..130173 'bitcast<u32>(b)': u32
            130171..130172 'b': i32
            130203..130211 'ICMP_UGT': u32
            130214..130220 'result': ref<function, bool, read_write>
            130224..130239 'bitcast<u32>(a)': u32
            130224..130257 'bitcas...32>(b)': bool
            130237..130238 'a': i32
            130242..130257 'bitcast<u32>(b)': u32
            130255..130256 'b': i32
            130287..130295 'ICMP_UGE': u32
            130298..130304 'result': ref<function, bool, read_write>
            130308..130323 'bitcast<u32>(a)': u32
            130308..130342 'bitcas...32>(b)': bool
            130321..130322 'a': i32
            130327..130342 'bitcast<u32>(b)': u32
            130340..130341 'b': i32
            130401..130406 'shots': ref<storage, array<ShotData>, read_write>
            130401..130416 'shots[shot_idx]': ref<storage, ShotData, read_write>
            130401..130423 'shots[...interp': ref<storage, InterpreterState, read_write>
            130401..130430 'shots[...status': ref<storage, u32, read_write>
            130407..130415 'shot_idx': u32
            130433..130456 'ERR_IN...UCTION': u32
            130482..130487 'shots': ref<storage, array<ShotData>, read_write>
            130482..130497 'shots[shot_idx]': ref<storage, ShotData, read_write>
            130482..130504 'shots[...interp': ref<storage, InterpreterState, read_write>
            130482..130514 'shots[...t_code': ref<storage, u32, read_write>
            130488..130496 'shot_idx': u32
            130517..130540 'ERR_IN...UCTION': u32
            130570..130577 'err_idx': u32
            130580..130609 '(shot_..._COUNT': u32
            130580..130613 '(shot_...NT - 1': u32
            130581..130589 'shot_idx': u32
            130581..130593 'shot_idx + 1': u32
            130592..130593 '1': integer
            130597..130609 'RESULT_COUNT': u32
            130612..130613 '1': integer
            130639..130712 'atomic...CTION)': __atomic_compare_exchange_result
            130665..130682 '&resul...r_idx]': ptr<storage, atomic<u32>, read_write>
            130666..130673 'results': ref<storage, array<atomic<u32>>, read_write>
            130666..130682 'result...r_idx]': ref<storage, atomic<u32>, read_write>
            130674..130681 'err_idx': u32
            130684..130686 '0u': u32
            130688..130711 'ERR_IN...UCTION': u32
            130738..130743 'shots': ref<storage, array<ShotData>, read_write>
            130738..130753 'shots[shot_idx]': ref<storage, ShotData, read_write>
            130738..130760 'shots[...interp': ref<storage, InterpreterState, read_write>
            130738..130767 'shots[...status': ref<storage, u32, read_write>
            130744..130752 'shot_idx': u32
            130770..130782 'STATUS_ERROR': u32
            130808..130853 'atomic...t, 1u)': u32
            130818..130848 '&diagn..._count': ptr<storage, atomic<u32>, read_write>
            130819..130830 'diagnostics': ref<storage, DiagnosticData, read_write>
            130819..130848 'diagno..._count': ref<storage, atomic<u32>, read_write>
            130850..130852 '1u': u32
            130879..130891 'should_break': ref<function, bool, read_write>
            130894..130898 'true': bool
            130956..131010 'write_...sult))': [error]
            130966..130974 'shot_idx': u32
            130976..130981 'instr': Instruction
            130976..130985 'instr.dst': u32
            130987..131009 'select...esult)': u32
            130994..130996 '0u': u32
            130998..131000 '1u': u32
            131002..131008 'result': ref<function, bool, read_write>
            131028..131030 'pc': ref<function, u32, read_write>
            131480..131487 'OP_FCMP': u32
            131510..131511 'a': f32
            131514..131558 'resolv...s, 0u)': f32
            131526..131534 'shot_idx': u32
            131536..131541 'instr': Instruction
            131536..131546 'instr.src0': u32
            131548..131553 'flags': u32
            131555..131557 '0u': u32
            131580..131581 'b': f32
            131584..131628 'resolv...s, 1u)': f32
            131596..131604 'shot_idx': u32
            131606..131611 'instr': Instruction
            131606..131616 'instr.src1': u32
            131618..131623 'flags': u32
            131625..131627 '1u': u32
            131650..131656 'result': ref<function, bool, read_write>
            131665..131670 'false': bool
            131695..131702 'subcond': u32
            131730..131738 'FCMP_OEQ': u32
            131741..131747 'result': ref<function, bool, read_write>
            131751..131752 'a': f32
            131751..131757 'a == b': bool
            131756..131757 'b': f32
            131787..131795 'FCMP_ONE': u32
            131798..131804 'result': ref<function, bool, read_write>
            131808..131809 'a': f32
            131808..131814 'a != b': bool
            131813..131814 'b': f32
            131844..131852 'FCMP_OLT': u32
            131855..131861 'result': ref<function, bool, read_write>
            131865..131866 'a': f32
            131865..131870 'a < b': bool
            131869..131870 'b': f32
            131900..131908 'FCMP_OLE': u32
            131911..131917 'result': ref<function, bool, read_write>
            131921..131922 'a': f32
            131921..131927 'a <= b': bool
            131926..131927 'b': f32
            131957..131965 'FCMP_OGT': u32
            131968..131974 'result': ref<function, bool, read_write>
            131978..131979 'a': f32
            131978..131983 'a > b': bool
            131982..131983 'b': f32
            132013..132021 'FCMP_OGE': u32
            132024..132030 'result': ref<function, bool, read_write>
            132034..132035 'a': f32
            132034..132040 'a >= b': bool
            132039..132040 'b': f32
            132099..132104 'shots': ref<storage, array<ShotData>, read_write>
            132099..132114 'shots[shot_idx]': ref<storage, ShotData, read_write>
            132099..132121 'shots[...interp': ref<storage, InterpreterState, read_write>
            132099..132131 'shots[...t_code': ref<storage, u32, read_write>
            132105..132113 'shot_idx': u32
            132134..132157 'ERR_IN...UCTION': u32
            132187..132194 'err_idx': u32
            132197..132226 '(shot_..._COUNT': u32
            132197..132230 '(shot_...NT - 1': u32
            132198..132206 'shot_idx': u32
            132198..132210 'shot_idx + 1': u32
            132209..132210 '1': integer
            132214..132226 'RESULT_COUNT': u32
            132229..132230 '1': integer
            132256..132329 'atomic...CTION)': __atomic_compare_exchange_result
            132282..132299 '&resul...r_idx]': ptr<storage, atomic<u32>, read_write>
            132283..132290 'results': ref<storage, array<atomic<u32>>, read_write>
            132283..132299 'result...r_idx]': ref<storage, atomic<u32>, read_write>
            132291..132298 'err_idx': u32
            132301..132303 '0u': u32
            132305..132328 'ERR_IN...UCTION': u32
            132355..132360 'shots': ref<storage, array<ShotData>, read_write>
            132355..132370 'shots[shot_idx]': ref<storage, ShotData, read_write>
            132355..132377 'shots[...interp': ref<storage, InterpreterState, read_write>
            132355..132384 'shots[...status': ref<storage, u32, read_write>
            132361..132369 'shot_idx': u32
            132387..132399 'STATUS_ERROR': u32
            132425..132470 'atomic...t, 1u)': u32
            132435..132465 '&diagn..._count': ptr<storage, atomic<u32>, read_write>
            132436..132447 'diagnostics': ref<storage, DiagnosticData, read_write>
            132436..132465 'diagno..._count': ref<storage, atomic<u32>, read_write>
            132467..132469 '1u': u32
            132496..132508 'should_break': ref<function, bool, read_write>
            132511..132515 'true': bool
            132573..132627 'write_...sult))': [error]
            132583..132591 'shot_idx': u32
            132593..132598 'instr': Instruction
            132593..132602 'instr.dst': u32
            132604..132626 'select...esult)': u32
            132611..132613 '0u': u32
            132615..132617 '1u': u32
            132619..132625 'result': ref<function, bool, read_write>
            132645..132647 'pc': ref<function, u32, read_write>
            133103..133110 'OP_FADD': u32
            133129..133276 'write_..., 1u))': [error]
            133143..133151 'shot_idx': u32
            133153..133158 'instr': Instruction
            133153..133162 'instr.dst': u32
            133184..133228 'resolv...s, 0u)': f32
            133184..133275 'resolv...s, 1u)': f32
            133196..133204 'shot_idx': u32
            133206..133211 'instr': Instruction
            133206..133216 'instr.src0': u32
            133218..133223 'flags': u32
            133225..133227 '0u': u32
            133231..133275 'resolv...s, 1u)': f32
            133243..133251 'shot_idx': u32
            133253..133258 'instr': Instruction
            133253..133263 'instr.src1': u32
            133265..133270 'flags': u32
            133272..133274 '1u': u32
            133294..133296 'pc': ref<function, u32, read_write>
            133391..133398 'OP_FSUB': u32
            133417..133564 'write_..., 1u))': [error]
            133431..133439 'shot_idx': u32
            133441..133446 'instr': Instruction
            133441..133450 'instr.dst': u32
            133472..133516 'resolv...s, 0u)': f32
            133472..133563 'resolv...s, 1u)': f32
            133484..133492 'shot_idx': u32
            133494..133499 'instr': Instruction
            133494..133504 'instr.src0': u32
            133506..133511 'flags': u32
            133513..133515 '0u': u32
            133519..133563 'resolv...s, 1u)': f32
            133531..133539 'shot_idx': u32
            133541..133546 'instr': Instruction
            133541..133551 'instr.src1': u32
            133553..133558 'flags': u32
            133560..133562 '1u': u32
            133582..133584 'pc': ref<function, u32, read_write>
            133682..133689 'OP_FMUL': u32
            133708..133855 'write_..., 1u))': [error]
            133722..133730 'shot_idx': u32
            133732..133737 'instr': Instruction
            133732..133741 'instr.dst': u32
            133763..133807 'resolv...s, 0u)': f32
            133763..133854 'resolv...s, 1u)': f32
            133775..133783 'shot_idx': u32
            133785..133790 'instr': Instruction
            133785..133795 'instr.src0': u32
            133797..133802 'flags': u32
            133804..133806 '0u': u32
            133810..133854 'resolv...s, 1u)': f32
            133822..133830 'shot_idx': u32
            133832..133837 'instr': Instruction
            133832..133842 'instr.src1': u32
            133844..133849 'flags': u32
            133851..133853 '1u': u32
            133873..133875 'pc': ref<function, u32, read_write>
            133967..133974 'OP_FDIV': u32
            133993..134140 'write_..., 1u))': [error]
            134007..134015 'shot_idx': u32
            134017..134022 'instr': Instruction
            134017..134026 'instr.dst': u32
            134048..134092 'resolv...s, 0u)': f32
            134048..134139 'resolv...s, 1u)': f32
            134060..134068 'shot_idx': u32
            134070..134075 'instr': Instruction
            134070..134080 'instr.src0': u32
            134082..134087 'flags': u32
            134089..134091 '0u': u32
            134095..134139 'resolv...s, 1u)': f32
            134107..134115 'shot_idx': u32
            134117..134122 'instr': Instruction
            134117..134127 'instr.src1': u32
            134129..134134 'flags': u32
            134136..134138 '1u': u32
            134158..134160 'pc': ref<function, u32, read_write>
            134393..134400 'OP_FREM': u32
            134423..134424 'a': f32
            134427..134471 'resolv...s, 0u)': f32
            134439..134447 'shot_idx': u32
            134449..134454 'instr': Instruction
            134449..134459 'instr.src0': u32
            134461..134466 'flags': u32
            134468..134470 '0u': u32
            134493..134494 'b': f32
            134497..134541 'resolv...s, 1u)': f32
            134509..134517 'shot_idx': u32
            134519..134524 'instr': Instruction
            134519..134529 'instr.src1': u32
            134531..134536 'flags': u32
            134538..134540 '1u': u32
            134559..134615 'write_...) * b)': [error]
            134573..134581 'shot_idx': u32
            134583..134588 'instr': Instruction
            134583..134592 'instr.dst': u32
            134594..134595 'a': f32
            134594..134614 'a - tr...b) * b': f32
            134598..134610 'trunc(a / b)': f32
            134598..134614 'trunc(...b) * b': f32
            134604..134605 'a': f32
            134604..134609 'a / b': f32
            134608..134609 'b': f32
            134613..134614 'b': f32
            134633..134635 'pc': ref<function, u32, read_write>
            135232..135239 'OP_ZEXT': u32
            135258..135334 'write_..., 0u))': [error]
            135268..135276 'shot_idx': u32
            135278..135283 'instr': Instruction
            135278..135287 'instr.dst': u32
            135289..135333 'resolv...s, 0u)': u32
            135301..135309 'shot_idx': u32
            135311..135316 'instr': Instruction
            135311..135321 'instr.src0': u32
            135323..135328 'flags': u32
            135330..135332 '0u': u32
            135352..135354 'pc': ref<function, u32, read_write>
            135682..135689 'OP_SEXT': u32
            135712..135715 'val': i32
            135718..135762 'resolv...s, 0u)': i32
            135730..135738 'shot_idx': u32
            135740..135745 'instr': Instruction
            135740..135750 'instr.src0': u32
            135752..135757 'flags': u32
            135759..135761 '0u': u32
            135784..135792 'src_bits': u32
            135795..135800 'instr': Instruction
            135795..135805 'instr.aux0': u32
            135852..135860 'src_bits': u32
            135852..135865 'src_bits > 0u': bool
            135852..135883 'src_bi... < 32u': bool
            135863..135865 '0u': u32
            135869..135877 'src_bits': u32
            135869..135883 'src_bits < 32u': bool
            135880..135883 '32u': u32
            135910..135915 'shift': u32
            135918..135921 '32u': u32
            135918..135932 '32u - src_bits': u32
            135924..135932 'src_bits': u32
            135954..136013 'write_...shift)': [error]
            135968..135976 'shot_idx': u32
            135978..135983 'instr': Instruction
            135978..135987 'instr.dst': u32
            135989..136012 '(val <... shift': i32
            135990..135993 'val': i32
            135990..136002 'val << shift': i32
            135997..136002 'shift': u32
            136007..136012 'shift': u32
            136060..136099 'write_..., val)': [error]
            136074..136082 'shot_idx': u32
            136084..136089 'instr': Instruction
            136084..136093 'instr.dst': u32
            136095..136098 'val': i32
            136135..136137 'pc': ref<function, u32, read_write>
            136259..136267 'OP_TRUNC': u32
            136286..136362 'write_..., 0u))': [error]
            136296..136304 'shot_idx': u32
            136306..136311 'instr': Instruction
            136306..136315 'instr.dst': u32
            136317..136361 'resolv...s, 0u)': u32
            136329..136337 'shot_idx': u32
            136339..136344 'instr': Instruction
            136339..136349 'instr.src0': u32
            136351..136356 'flags': u32
            136358..136360 '0u': u32
            136380..136382 'pc': ref<function, u32, read_write>
            136508..136516 'OP_FPEXT': u32
            136535..136615 'write_..., 0u))': [error]
            136549..136557 'shot_idx': u32
            136559..136564 'instr': Instruction
            136559..136568 'instr.dst': u32
            136570..136614 'resolv...s, 0u)': f32
            136582..136590 'shot_idx': u32
            136592..136597 'instr': Instruction
            136592..136602 'instr.src0': u32
            136604..136609 'flags': u32
            136611..136613 '0u': u32
            136633..136635 'pc': ref<function, u32, read_write>
            136764..136774 'OP_FPTRUNC': u32
            136793..136873 'write_..., 0u))': [error]
            136807..136815 'shot_idx': u32
            136817..136822 'instr': Instruction
            136817..136826 'instr.dst': u32
            136828..136872 'resolv...s, 0u)': f32
            136840..136848 'shot_idx': u32
            136850..136855 'instr': Instruction
            136850..136860 'instr.src0': u32
            136862..136867 'flags': u32
            136869..136871 '0u': u32
            136891..136893 'pc': ref<function, u32, read_write>
            137017..137028 'OP_INTTOPTR': u32
            137047..137123 'write_..., 0u))': [error]
            137057..137065 'shot_idx': u32
            137067..137072 'instr': Instruction
            137067..137076 'instr.dst': u32
            137078..137122 'resolv...s, 0u)': u32
            137090..137098 'shot_idx': u32
            137100..137105 'instr': Instruction
            137100..137110 'instr.src0': u32
            137112..137117 'flags': u32
            137119..137121 '0u': u32
            137141..137143 'pc': ref<function, u32, read_write>
            137255..137264 'OP_FPTOSI': u32
            137283..137368 'write_... 0u)))': [error]
            137297..137305 'shot_idx': u32
            137307..137312 'instr': Instruction
            137307..137316 'instr.dst': u32
            137318..137367 'i32(re..., 0u))': i32
            137322..137366 'resolv...s, 0u)': f32
            137334..137342 'shot_idx': u32
            137344..137349 'instr': Instruction
            137344..137354 'instr.src0': u32
            137356..137361 'flags': u32
            137363..137365 '0u': u32
            137386..137388 'pc': ref<function, u32, read_write>
            137500..137509 'OP_SITOFP': u32
            137528..137613 'write_... 0u)))': [error]
            137542..137550 'shot_idx': u32
            137552..137557 'instr': Instruction
            137552..137561 'instr.dst': u32
            137563..137612 'f32(re..., 0u))': f32
            137567..137611 'resolv...s, 0u)': i32
            137579..137587 'shot_idx': u32
            137589..137594 'instr': Instruction
            137589..137599 'instr.src0': u32
            137601..137606 'flags': u32
            137608..137610 '0u': u32
            137631..137633 'pc': ref<function, u32, read_write>
            137747..137756 'OP_FPTOUI': u32
            137775..137856 'write_... 0u)))': [error]
            137785..137793 'shot_idx': u32
            137795..137800 'instr': Instruction
            137795..137804 'instr.dst': u32
            137806..137855 'u32(re..., 0u))': u32
            137810..137854 'resolv...s, 0u)': f32
            137822..137830 'shot_idx': u32
            137832..137837 'instr': Instruction
            137832..137842 'instr.src0': u32
            137844..137849 'flags': u32
            137851..137853 '0u': u32
            137874..137876 'pc': ref<function, u32, read_write>
            137990..137999 'OP_UITOFP': u32
            138018..138103 'write_... 0u)))': [error]
            138032..138040 'shot_idx': u32
            138042..138047 'instr': Instruction
            138042..138051 'instr.dst': u32
            138053..138102 'f32(re..., 0u))': f32
            138057..138101 'resolv...s, 0u)': u32
            138069..138077 'shot_idx': u32
            138079..138084 'instr': Instruction
            138079..138089 'instr.src0': u32
            138091..138096 'flags': u32
            138098..138100 '0u': u32
            138121..138123 'pc': ref<function, u32, read_write>
            139155..139161 'OP_PHI': u32
            139184..139190 'offset': u32
            139193..139198 'instr': Instruction
            139193..139203 'instr.aux0': u32
            139225..139230 'count': u32
            139233..139238 'instr': Instruction
            139233..139243 'instr.aux1': u32
            139270..139271 'i': ref<function, u32, read_write>
            139274..139276 '0u': u32
            139278..139279 'i': ref<function, u32, read_write>
            139278..139287 'i < count': bool
            139282..139287 'count': u32
            139289..139290 'i': ref<function, u32, read_write>
            139320..139325 'entry': [error]
            139328..139338 'batch_data': ref<storage, BatchData, read>
            139328..139346 'batch_...rogram': ref<storage, Program, read>
            139328..139356 'batch_..._table': ref<storage, [error], read>
            139328..139368 'batch_...t + i]': [error]
            139357..139363 'offset': u32
            139357..139367 'offset + i': u32
            139366..139367 'i': ref<function, u32, read_write>
            139393..139398 'entry': [error]
            139393..139407 'entry.block_id': [error]
            139393..139421 'entry...._block': [error]
            139411..139421 'prev_block': ref<function, u32, read_write>
            139448..139513 'write_..._reg))': [error]
            139458..139466 'shot_idx': u32
            139468..139473 'instr': Instruction
            139468..139477 'instr.dst': u32
            139479..139512 'read_r...l_reg)': u32
            139488..139496 'shot_idx': u32
            139498..139503 'entry': [error]
            139498..139511 'entry.val_reg': [error]
            139602..139604 'pc': ref<function, u32, read_write>
            140050..140059 'OP_SELECT': u32
            140082..140086 'cond': bool
            140089..140133 'resolv...s, 0u)': u32
            140089..140139 'resolv... != 0u': bool
            140101..140109 'shot_idx': u32
            140111..140116 'instr': Instruction
            140111..140121 'instr.src0': u32
            140123..140128 'flags': u32
            140130..140132 '0u': u32
            140137..140139 '0u': u32
            140161..140169 'true_val': u32
            140172..140216 'resolv...s, 3u)': u32
            140184..140192 'shot_idx': u32
            140194..140199 'instr': Instruction
            140194..140204 'instr.aux0': u32
            140206..140211 'flags': u32
            140213..140215 '3u': u32
            140238..140247 'false_val': u32
            140250..140294 'resolv...s, 4u)': u32
            140262..140270 'shot_idx': u32
            140272..140277 'instr': Instruction
            140272..140282 'instr.aux1': u32
            140284..140289 'flags': u32
            140291..140293 '4u': u32
            140312..140377 'write_...cond))': [error]
            140322..140330 'shot_idx': u32
            140332..140337 'instr': Instruction
            140332..140341 'instr.dst': u32
            140343..140376 'select... cond)': u32
            140350..140359 'false_val': u32
            140361..140369 'true_val': u32
            140371..140375 'cond': bool
            140395..140397 'pc': ref<function, u32, read_write>
            140593..140599 'OP_MOV': u32
            140618..140694 'write_..., 0u))': [error]
            140628..140636 'shot_idx': u32
            140638..140643 'instr': Instruction
            140638..140647 'instr.dst': u32
            140649..140693 'resolv...s, 0u)': u32
            140661..140669 'shot_idx': u32
            140671..140676 'instr': Instruction
            140671..140681 'instr.src0': u32
            140683..140688 'flags': u32
            140690..140692 '0u': u32
            140712..140714 'pc': ref<function, u32, read_write>
            140895..140903 'OP_CONST': u32
            140922..140964 'write_....src0)': [error]
            140932..140940 'shot_idx': u32
            140942..140947 'instr': Instruction
            140942..140951 'instr.dst': u32
            140953..140958 'instr': Instruction
            140953..140963 'instr.src0': u32
            140982..140984 'pc': ref<function, u32, read_write>
            141363..141372 'OP_ALLOCA': u32
            141395..141404 'num_words': u32
            141407..141451 'resolv...s, 0u)': u32
            141419..141427 'shot_idx': u32
            141429..141434 'instr': Instruction
            141429..141439 'instr.src0': u32
            141441..141446 'flags': u32
            141448..141450 '0u': u32
            141473..141477 'addr': u32
            141480..141524 'resolv...s, 1u)': u32
            141492..141500 'shot_idx': u32
            141502..141507 'instr': Instruction
            141502..141512 'instr.src1': u32
            141514..141519 'flags': u32
            141521..141523 '1u': u32
            141545..141549 'addr': u32
            141545..141561 'addr +..._words': u32
            141545..141574 'addr +...MEMORY': bool
            141552..141561 'num_words': u32
            141564..141574 'MAX_MEMORY': u32
            141597..141602 'shots': ref<storage, array<ShotData>, read_write>
            141597..141612 'shots[shot_idx]': ref<storage, ShotData, read_write>
            141597..141619 'shots[...interp': ref<storage, InterpreterState, read_write>
            141597..141629 'shots[...t_code': ref<storage, u32, read_write>
            141603..141611 'shot_idx': u32
            141632..141656 'ERR_AL...BOUNDS': u32
            141682..141689 'err_idx': u32
            141692..141721 '(shot_..._COUNT': u32
            141692..141725 '(shot_...NT - 1': u32
            141693..141701 'shot_idx': u32
            141693..141705 'shot_idx + 1': u32
            141704..141705 '1': integer
            141709..141721 'RESULT_COUNT': u32
            141724..141725 '1': integer
            141747..141821 'atomic...OUNDS)': __atomic_compare_exchange_result
            141773..141790 '&resul...r_idx]': ptr<storage, atomic<u32>, read_write>
            141774..141781 'results': ref<storage, array<atomic<u32>>, read_write>
            141774..141790 'result...r_idx]': ref<storage, atomic<u32>, read_write>
            141782..141789 'err_idx': u32
            141792..141794 '0u': u32
            141796..141820 'ERR_AL...BOUNDS': u32
            141843..141848 'shots': ref<storage, array<ShotData>, read_write>
            141843..141858 'shots[shot_idx]': ref<storage, ShotData, read_write>
            141843..141865 'shots[...interp': ref<storage, InterpreterState, read_write>
            141843..141872 'shots[...status': ref<storage, u32, read_write>
            141849..141857 'shot_idx': u32
            141875..141887 'STATUS_ERROR': u32
            141909..141954 'atomic...t, 1u)': u32
            141919..141949 '&diagn..._count': ptr<storage, atomic<u32>, read_write>
            141920..141931 'diagnostics': ref<storage, DiagnosticData, read_write>
            141920..141949 'diagno..._count': ref<storage, atomic<u32>, read_write>
            141951..141953 '1u': u32
            141976..141988 'should_break': ref<function, bool, read_write>
            141991..141995 'true': bool
            142058..142094 'write_... addr)': [error]
            142068..142076 'shot_idx': u32
            142078..142083 'instr': Instruction
            142078..142087 'instr.dst': u32
            142089..142093 'addr': u32
            142112..142114 'pc': ref<function, u32, read_write>
            142294..142301 'OP_LOAD': u32
            142324..142328 'addr': u32
            142331..142375 'resolv...s, 0u)': u32
            142343..142351 'shot_idx': u32
            142353..142358 'instr': Instruction
            142353..142363 'instr.src0': u32
            142365..142370 'flags': u32
            142372..142374 '0u': u32
            142396..142400 'addr': u32
            142396..142414 'addr >...MEMORY': bool
            142404..142414 'MAX_MEMORY': u32
            142437..142442 'shots': ref<storage, array<ShotData>, read_write>
            142437..142452 'shots[shot_idx]': ref<storage, ShotData, read_write>
            142437..142459 'shots[...interp': ref<storage, InterpreterState, read_write>
            142437..142469 'shots[...t_code': ref<storage, u32, read_write>
            142443..142451 'shot_idx': u32
            142472..142496 'ERR_ME...BOUNDS': u32
            142522..142529 'err_idx': u32
            142532..142561 '(shot_..._COUNT': u32
            142532..142565 '(shot_...NT - 1': u32
            142533..142541 'shot_idx': u32
            142533..142545 'shot_idx + 1': u32
            142544..142545 '1': integer
            142549..142561 'RESULT_COUNT': u32
            142564..142565 '1': integer
            142587..142661 'atomic...OUNDS)': __atomic_compare_exchange_result
            142613..142630 '&resul...r_idx]': ptr<storage, atomic<u32>, read_write>
            142614..142621 'results': ref<storage, array<atomic<u32>>, read_write>
            142614..142630 'result...r_idx]': ref<storage, atomic<u32>, read_write>
            142622..142629 'err_idx': u32
            142632..142634 '0u': u32
            142636..142660 'ERR_ME...BOUNDS': u32
            142683..142688 'shots': ref<storage, array<ShotData>, read_write>
            142683..142698 'shots[shot_idx]': ref<storage, ShotData, read_write>
            142683..142705 'shots[...interp': ref<storage, InterpreterState, read_write>
            142683..142712 'shots[...status': ref<storage, u32, read_write>
            142689..142697 'shot_idx': u32
            142715..142727 'STATUS_ERROR': u32
            142749..142794 'atomic...t, 1u)': u32
            142759..142789 '&diagn..._count': ptr<storage, atomic<u32>, read_write>
            142760..142771 'diagnostics': ref<storage, DiagnosticData, read_write>
            142760..142789 'diagno..._count': ref<storage, atomic<u32>, read_write>
            142791..142793 '1u': u32
            142816..142828 'should_break': ref<function, bool, read_write>
            142831..142835 'true': bool
            142902..142905 'val': [error]
            142908..142913 'shots': ref<storage, array<ShotData>, read_write>
            142908..142923 'shots[shot_idx]': ref<storage, ShotData, read_write>
            142908..142930 'shots[...interp': ref<storage, InterpreterState, read_write>
            142908..142937 'shots[...memory': ref<storage, [error], read_write>
            142908..142943 'shots[...[addr]': [error]
            142914..142922 'shot_idx': u32
            142938..142942 'addr': u32
            142961..142996 'write_..., val)': [error]
            142971..142979 'shot_idx': u32
            142981..142986 'instr': Instruction
            142981..142990 'instr.dst': u32
            142992..142995 'val': [error]
            143014..143016 'pc': ref<function, u32, read_write>
            143191..143199 'OP_STORE': u32
            143222..143225 'val': u32
            143228..143272 'resolv...s, 0u)': u32
            143240..143248 'shot_idx': u32
            143250..143255 'instr': Instruction
            143250..143260 'instr.src0': u32
            143262..143267 'flags': u32
            143269..143271 '0u': u32
            143294..143298 'addr': u32
            143301..143345 'resolv...s, 1u)': u32
            143313..143321 'shot_idx': u32
            143323..143328 'instr': Instruction
            143323..143333 'instr.src1': u32
            143335..143340 'flags': u32
            143342..143344 '1u': u32
            143366..143370 'addr': u32
            143366..143384 'addr >...MEMORY': bool
            143374..143384 'MAX_MEMORY': u32
            143407..143412 'shots': ref<storage, array<ShotData>, read_write>
            143407..143422 'shots[shot_idx]': ref<storage, ShotData, read_write>
            143407..143429 'shots[...interp': ref<storage, InterpreterState, read_write>
            143407..143439 'shots[...t_code': ref<storage, u32, read_write>
            143413..143421 'shot_idx': u32
            143442..143466 'ERR_ME...BOUNDS': u32
            143492..143499 'err_idx': u32
            143502..143531 '(shot_..._COUNT': u32
            143502..143535 '(shot_...NT - 1': u32
            143503..143511 'shot_idx': u32
            143503..143515 'shot_idx + 1': u32
            143514..143515 '1': integer
            143519..143531 'RESULT_COUNT': u32
            143534..143535 '1': integer
            143557..143631 'atomic...OUNDS)': __atomic_compare_exchange_result
            143583..143600 '&resul...r_idx]': ptr<storage, atomic<u32>, read_write>
            143584..143591 'results': ref<storage, array<atomic<u32>>, read_write>
            143584..143600 'result...r_idx]': ref<storage, atomic<u32>, read_write>
            143592..143599 'err_idx': u32
            143602..143604 '0u': u32
            143606..143630 'ERR_ME...BOUNDS': u32
            143653..143658 'shots': ref<storage, array<ShotData>, read_write>
            143653..143668 'shots[shot_idx]': ref<storage, ShotData, read_write>
            143653..143675 'shots[...interp': ref<storage, InterpreterState, read_write>
            143653..143682 'shots[...status': ref<storage, u32, read_write>
            143659..143667 'shot_idx': u32
            143685..143697 'STATUS_ERROR': u32
            143719..143764 'atomic...t, 1u)': u32
            143729..143759 '&diagn..._count': ptr<storage, atomic<u32>, read_write>
            143730..143741 'diagnostics': ref<storage, DiagnosticData, read_write>
            143730..143759 'diagno..._count': ref<storage, atomic<u32>, read_write>
            143761..143763 '1u': u32
            143786..143798 'should_break': ref<function, bool, read_write>
            143801..143805 'true': bool
            143868..143873 'shots': ref<storage, array<ShotData>, read_write>
            143868..143883 'shots[shot_idx]': ref<storage, ShotData, read_write>
            143868..143890 'shots[...interp': ref<storage, InterpreterState, read_write>
            143868..143897 'shots[...memory': ref<storage, [error], read_write>
            143868..143903 'shots[...[addr]': [error]
            143874..143882 'shot_idx': u32
            143898..143902 'addr': u32
            143906..143909 'val': u32
            143927..143929 'pc': ref<function, u32, read_write>
            144137..144143 'OP_GEP': u32
            144166..144170 'base': u32
            144173..144217 'resolv...s, 0u)': u32
            144185..144193 'shot_idx': u32
            144195..144200 'instr': Instruction
            144195..144205 'instr.src0': u32
            144207..144212 'flags': u32
            144214..144216 '0u': u32
            144239..144244 'index': u32
            144247..144291 'resolv...s, 1u)': u32
            144259..144267 'shot_idx': u32
            144269..144274 'instr': Instruction
            144269..144279 'instr.src1': u32
            144281..144286 'flags': u32
            144288..144290 '1u': u32
            144313..144322 'elem_size': u32
            144325..144369 'resolv...s, 3u)': u32
            144337..144345 'shot_idx': u32
            144347..144352 'instr': Instruction
            144347..144357 'instr.aux0': u32
            144359..144364 'flags': u32
            144366..144368 '3u': u32
            144391..144395 'addr': u32
            144398..144402 'base': u32
            144398..144422 'base +...m_size': u32
            144405..144410 'index': u32
            144405..144422 'index ...m_size': u32
            144413..144422 'elem_size': u32
            144440..144476 'write_... addr)': [error]
            144450..144458 'shot_idx': u32
            144460..144465 'instr': Instruction
            144460..144469 'instr.dst': u32
            144471..144475 'addr': u32
            144494..144496 'pc': ref<function, u32, read_write>
            144613..144618 'shots': ref<storage, array<ShotData>, read_write>
            144613..144628 'shots[shot_idx]': ref<storage, ShotData, read_write>
            144613..144635 'shots[...interp': ref<storage, InterpreterState, read_write>
            144613..144642 'shots[...status': ref<storage, u32, read_write>
            144619..144627 'shot_idx': u32
            144645..144657 'STATUS_ERROR': u32
            144675..144720 'atomic...t, 1u)': u32
            144685..144715 '&diagn..._count': ptr<storage, atomic<u32>, read_write>
            144686..144697 'diagnostics': ref<storage, DiagnosticData, read_write>
            144686..144715 'diagno..._count': ref<storage, atomic<u32>, read_write>
            144717..144719 '1u': u32
            144738..144750 'should_break': ref<function, bool, read_write>
            144753..144757 'true': bool
            144791..144796 'steps': ref<function, u32, read_write>
            144811..144823 'should_break': ref<function, bool, read_write>
            145052..145057 'shots': ref<storage, array<ShotData>, read_write>
            145052..145067 'shots[shot_idx]': ref<storage, ShotData, read_write>
            145052..145074 'shots[...interp': ref<storage, InterpreterState, read_write>
            145052..145077 'shots[...erp.pc': ref<storage, u32, read_write>
            145058..145066 'shot_idx': u32
            145080..145082 'pc': ref<function, u32, read_write>
            145088..145093 'shots': ref<storage, array<ShotData>, read_write>
            145088..145103 'shots[shot_idx]': ref<storage, ShotData, read_write>
            145088..145110 'shots[...interp': ref<storage, InterpreterState, read_write>
            145088..145127 'shots[...ock_id': ref<storage, u32, read_write>
            145094..145102 'shot_idx': u32
            145130..145138 'block_id': ref<function, u32, read_write>
            145144..145149 'shots': ref<storage, array<ShotData>, read_write>
            145144..145159 'shots[shot_idx]': ref<storage, ShotData, read_write>
            145144..145166 'shots[...interp': ref<storage, InterpreterState, read_write>
            145144..145184 'shots[...ock_id': ref<storage, u32, read_write>
            145150..145158 'shot_idx': u32
            145187..145197 'prev_block': ref<function, u32, read_write>
            111723..111764 'batch_...r.dst]': cannot index into type ref<storage, [error], read>
            112417..112459 'batch_....aux0]': cannot index into type ref<storage, [error], read>
            112567..112609 'batch_....aux1]': cannot index into type ref<storage, [error], read>
            113550..113598 'batch_...t + i]': cannot index into type ref<storage, [error], read>
            113878..113922 'batch_...block]': cannot index into type ref<storage, [error], read>
            115073..115115 'batch_...nc_id]': cannot index into type ref<storage, [error], read>
            116453..116502 'batch_...t + i]': cannot index into type ref<storage, [error], read>
            116579..116586 'arg_reg': expected u32 but got [error]
            116544..116558 'param_base + i': expected u32 but got [error]
            116741..116781 'batch_...ck_id]': cannot index into type ref<storage, [error], read>
            139328..139368 'batch_...t + i]': cannot index into type ref<storage, [error], read>
            139498..139511 'entry.val_reg': expected u32 but got [error]
            142908..142943 'shots[...[addr]': cannot index into type ref<storage, [error], read_write>
            142992..142995 'val': expected u32 but got [error]
            143868..143903 'shots[...[addr]': cannot index into type ref<storage, [error], read_write>
            143868..143903 'shots[...[addr]': cannot assign to non-reference `[error]`
            145649..145657 'shot_idx': u32
            145674..145678 'shot': ptr<storage, ShotData, read_write>
            145681..145697 '&shots...t_idx]': ptr<storage, ShotData, read_write>
            145682..145687 'shots': ref<storage, array<ShotData>, read_write>
            145682..145697 'shots[shot_idx]': ref<storage, ShotData, read_write>
            145688..145696 'shot_idx': u32
            145707..145712 'state': InterpreterState
            145715..145720 'shots': ref<storage, array<ShotData>, read_write>
            145715..145730 'shots[shot_idx]': ref<storage, ShotData, read_write>
            145715..145737 'shots[...interp': ref<storage, InterpreterState, read_write>
            145721..145729 'shot_idx': u32
            145747..145753 'status': u32
            145756..145761 'state': InterpreterState
            145756..145768 'state.status': u32
            145829..145835 'status': u32
            145829..145861 'status...ENDING': bool
            145839..145861 'STATUS...ENDING': u32
            145937..145941 'shot': ptr<storage, ShotData, read_write>
            145937..145949 'shot.op_type': ref<storage, u32, read_write>
            145952..145959 'OPID_ID': u32
            145969..145973 'shot': ptr<storage, ShotData, read_write>
            145969..145985 'shot.r...malize': ref<storage, f32, read_write>
            145988..145991 '1.0': float
            146001..146005 'shot': ptr<storage, ShotData, read_write>
            146001..146033 'shot.q...p_mask': ref<storage, u32, read_write>
            146036..146038 '0u': u32
            146119..146123 'shot': ptr<storage, ShotData, read_write>
            146119..146151 'shot.q...p_mask': ref<storage, u32, read_write>
            146119..146156 'shot.q...k != 0': bool
            146155..146156 '0': integer
            146167..146195 'update...t_idx)': [error]
            146186..146194 'shot_idx': u32
            146207..146233 'shot_i...t_idx)': [error]
            146224..146232 'shot_idx': u32
            146244..146250 'op_idx': u32
            146253..146258 'state': InterpreterState
            146253..146273 'state....op_idx': u32
            146283..146290 'op_type': u32
            146293..146298 'state': InterpreterState
            146293..146314 'state....p_type': u32
            146481..146488 'op_type': u32
            146481..146514 'op_typ...COMMIT': bool
            146492..146514 'PENDIN...COMMIT': u32
            146525..146559 'prep_l...p_idx)': [error]
            146542..146550 'shot_idx': u32
            146552..146558 'op_idx': u32
            146592..146594 'op': ptr<storage, Op, read>
            146597..146609 '&ops[op_idx]': ptr<storage, Op, read>
            146598..146601 'ops': ref<storage, array<Op>, read>
            146598..146609 'ops[op_idx]': ref<storage, Op, read>
            146602..146608 'op_idx': u32
            146817..146824 'op_type': u32
            146817..146830 'op_type == 0u': bool
            146817..146864 'op_typ..._NOISE': bool
            146828..146830 '0u': u32
            146834..146836 'op': ptr<storage, Op, read>
            146834..146839 'op.id': ref<storage, u32, read>
            146834..146864 'op.id ..._NOISE': bool
            146843..146864 'OPID_C..._NOISE': u32
            146879..146881 'pc': u32
            146884..146889 'state': InterpreterState
            146884..146892 'state.pc': u32
            146906..146917 'noise_instr': Instruction
            146920..146940 'fetch_... - 1u)': Instruction
            146932..146934 'pc': u32
            146932..146939 'pc - 1u': u32
            146937..146939 '1u': u32
            146954..146965 'qubit_count': u32
            146968..146979 'noise_instr': Instruction
            146968..146984 'noise_...r.aux1': u32
            146998..147008 'arg_offset': u32
            147011..147022 'noise_instr': Instruction
            147011..147027 'noise_...r.aux2': u32
            147037..147041 'shot': ptr<storage, ShotData, read_write>
            147037..147048 'shot.op_idx': ref<storage, u32, read_write>
            147051..147057 'op_idx': u32
            147067..147071 'shot': ptr<storage, ShotData, read_write>
            147067..147079 'shot.op_type': ref<storage, u32, read_write>
            147082..147084 'op': ptr<storage, Op, read>
            147082..147087 'op.id': ref<storage, u32, read>
            147097..147170 'prep_c...ffset)': [error]
            147128..147136 'shot_idx': u32
            147138..147144 'op_idx': u32
            147146..147157 'qubit_count': u32
            147159..147169 'arg_offset': u32
            147180..147185 'shots': ref<storage, array<ShotData>, read_write>
            147180..147195 'shots[shot_idx]': ref<storage, ShotData, read_write>
            147180..147202 'shots[...interp': ref<storage, InterpreterState, read_write>
            147180..147209 'shots[...status': ref<storage, u32, read_write>
            147186..147194 'shot_idx': u32
            147212..147226 'STATUS_RUNNING': u32
            147259..147261 'q1': u32
            147264..147284 'resolv...t_idx)': u32
            147275..147283 'shot_idx': u32
            147294..147296 'q2': u32
            147299..147319 'resolv...t_idx)': u32
            147310..147318 'shot_idx': u32
            147326..147330 'shot': ptr<storage, ShotData, read_write>
            147326..147338 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            147341..147343 'op': ptr<storage, Op, read>
            147341..147351 'op.unitary': ref<storage, array<vec2<f32>, 16>, read>
            147365..147372 'op_type': u32
            147388..147390 '0u': u32
            147667..147690 'is_rot...op.id)': bool
            147667..147720 'is_rot...t_idx)': bool
            147684..147686 'op': ptr<storage, Op, read>
            147684..147689 'op.id': ref<storage, u32, read>
            147694..147720 'is_dyn...t_idx)': bool
            147711..147719 'shot_idx': u32
            147742..147744 'op': ptr<storage, Op, read>
            147742..147747 'op.id': ref<storage, u32, read>
            147742..147758 'op.id ...PID_RX': bool
            147742..147778 'op.id ...PID_RY': bool
            147742..147798 'op.id ...PID_RZ': bool
            147751..147758 'OPID_RX': u32
            147762..147764 'op': ptr<storage, Op, read>
            147762..147767 'op.id': ref<storage, u32, read>
            147762..147778 'op.id ...PID_RY': bool
            147771..147778 'OPID_RY': u32
            147782..147784 'op': ptr<storage, Op, read>
            147782..147787 'op.id': ref<storage, u32, read>
            147782..147798 'op.id ...PID_RZ': bool
            147791..147798 'OPID_RZ': u32
            147825..147830 'angle': f32
            147833..147861 'resolv...t_idx)': f32
            147852..147860 'shot_idx': u32
            147887..147891 'half': f32
            147894..147899 'angle': f32
            147894..147905 'angle * 0.5': f32
            147902..147905 '0.5': float
            147931..147932 'c': f32
            147935..147944 'cos(half)': f32
            147939..147943 'half': f32
            147970..147971 's': f32
            147974..147983 'sin(half)': f32
            147978..147982 'half': f32
            148008..148010 'op': ptr<storage, Op, read>
            148008..148013 'op.id': ref<storage, u32, read>
            148008..148024 'op.id ...PID_RX': bool
            148017..148024 'OPID_RX': u32
            148135..148139 'shot': ptr<storage, ShotData, read_write>
            148135..148147 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            148135..148150 'shot.unitary[0]': ref<storage, vec2<f32>, read_write>
            148148..148149 '0': integer
            148153..148166 'vec2f(c, 0.0)': vec2<f32>
            148159..148160 'c': f32
            148162..148165 '0.0': float
            148192..148196 'shot': ptr<storage, ShotData, read_write>
            148192..148204 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            148192..148207 'shot.unitary[1]': ref<storage, vec2<f32>, read_write>
            148205..148206 '1': integer
            148210..148224 'vec2f(0.0, -s)': vec2<f32>
            148216..148219 '0.0': float
            148221..148223 '-s': f32
            148222..148223 's': f32
            148250..148254 'shot': ptr<storage, ShotData, read_write>
            148250..148262 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            148250..148265 'shot.unitary[4]': ref<storage, vec2<f32>, read_write>
            148263..148264 '4': integer
            148268..148282 'vec2f(0.0, -s)': vec2<f32>
            148274..148277 '0.0': float
            148279..148281 '-s': f32
            148280..148281 's': f32
            148308..148312 'shot': ptr<storage, ShotData, read_write>
            148308..148320 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            148308..148323 'shot.unitary[5]': ref<storage, vec2<f32>, read_write>
            148321..148322 '5': integer
            148326..148339 'vec2f(c, 0.0)': vec2<f32>
            148332..148333 'c': f32
            148335..148338 '0.0': float
            148491..148495 'shot': ptr<storage, ShotData, read_write>
            148491..148503 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            148491..148506 'shot.unitary[0]': ref<storage, vec2<f32>, read_write>
            148504..148505 '0': integer
            148509..148522 'vec2f(c, 0.0)': vec2<f32>
            148515..148516 'c': f32
            148518..148521 '0.0': float
            148548..148552 'shot': ptr<storage, ShotData, read_write>
            148548..148560 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            148548..148563 'shot.unitary[1]': ref<storage, vec2<f32>, read_write>
            148561..148562 '1': integer
            148566..148580 'vec2f(-s, 0.0)': vec2<f32>
            148572..148574 '-s': f32
            148573..148574 's': f32
            148576..148579 '0.0': float
            148606..148610 'shot': ptr<storage, ShotData, read_write>
            148606..148618 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            148606..148621 'shot.unitary[4]': ref<storage, vec2<f32>, read_write>
            148619..148620 '4': integer
            148624..148637 'vec2f(s, 0.0)': vec2<f32>
            148630..148631 's': f32
            148633..148636 '0.0': float
            148663..148667 'shot': ptr<storage, ShotData, read_write>
            148663..148675 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            148663..148678 'shot.unitary[5]': ref<storage, vec2<f32>, read_write>
            148676..148677 '5': integer
            148681..148694 'vec2f(c, 0.0)': vec2<f32>
            148687..148688 'c': f32
            148690..148693 '0.0': float
            148803..148807 'shot': ptr<storage, ShotData, read_write>
            148803..148815 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            148803..148818 'shot.unitary[0]': ref<storage, vec2<f32>, read_write>
            148816..148817 '0': integer
            148821..148836 'vec2f(1.0, 0.0)': vec2<f32>
            148827..148830 '1.0': float
            148832..148835 '0.0': float
            148862..148866 'shot': ptr<storage, ShotData, read_write>
            148862..148874 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            148862..148877 'shot.unitary[1]': ref<storage, vec2<f32>, read_write>
            148875..148876 '1': integer
            148880..148895 'vec2f(0.0, 0.0)': vec2<f32>
            148886..148889 '0.0': float
            148891..148894 '0.0': float
            148921..148925 'shot': ptr<storage, ShotData, read_write>
            148921..148933 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            148921..148936 'shot.unitary[4]': ref<storage, vec2<f32>, read_write>
            148934..148935 '4': integer
            148939..148954 'vec2f(0.0, 0.0)': vec2<f32>
            148945..148948 '0.0': float
            148950..148953 '0.0': float
            148980..148984 'shot': ptr<storage, ShotData, read_write>
            148980..148992 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            148980..148995 'shot.unitary[5]': ref<storage, vec2<f32>, read_write>
            148993..148994 '5': integer
            148998..149027 'vec2f(...ngle))': vec2<f32>
            149004..149014 'cos(angle)': f32
            149008..149013 'angle': f32
            149016..149026 'sin(angle)': f32
            149020..149025 'angle': f32
            149163..149168 'angle': f32
            149171..149199 'resolv...t_idx)': f32
            149190..149198 'shot_idx': u32
            149225..149229 'half': f32
            149232..149237 'angle': f32
            149232..149243 'angle * 0.5': f32
            149240..149243 '0.5': float
            149269..149270 'c': f32
            149273..149282 'cos(half)': f32
            149277..149281 'half': f32
            149308..149309 's': f32
            149312..149321 'sin(half)': f32
            149316..149320 'half': f32
            149346..149348 'op': ptr<storage, Op, read>
            149346..149351 'op.id': ref<storage, u32, read>
            149346..149363 'op.id ...ID_RXX': bool
            149355..149363 'OPID_RXX': u32
            149438..149442 'shot': ptr<storage, ShotData, read_write>
            149438..149450 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            149438..149453 'shot.unitary[0]': ref<storage, vec2<f32>, read_write>
            149451..149452 '0': integer
            149457..149470 'vec2f(c, 0.0)': vec2<f32>
            149463..149464 'c': f32
            149466..149469 '0.0': float
            149496..149500 'shot': ptr<storage, ShotData, read_write>
            149496..149508 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            149496..149511 'shot.unitary[3]': ref<storage, vec2<f32>, read_write>
            149509..149510 '3': integer
            149515..149529 'vec2f(0.0, -s)': vec2<f32>
            149521..149524 '0.0': float
            149526..149528 '-s': f32
            149527..149528 's': f32
            149555..149559 'shot': ptr<storage, ShotData, read_write>
            149555..149567 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            149555..149570 'shot.unitary[5]': ref<storage, vec2<f32>, read_write>
            149568..149569 '5': integer
            149574..149587 'vec2f(c, 0.0)': vec2<f32>
            149580..149581 'c': f32
            149583..149586 '0.0': float
            149613..149617 'shot': ptr<storage, ShotData, read_write>
            149613..149625 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            149613..149628 'shot.unitary[6]': ref<storage, vec2<f32>, read_write>
            149626..149627 '6': integer
            149632..149646 'vec2f(0.0, -s)': vec2<f32>
            149638..149641 '0.0': float
            149643..149645 '-s': f32
            149644..149645 's': f32
            149672..149676 'shot': ptr<storage, ShotData, read_write>
            149672..149684 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            149672..149687 'shot.unitary[9]': ref<storage, vec2<f32>, read_write>
            149685..149686 '9': integer
            149691..149705 'vec2f(0.0, -s)': vec2<f32>
            149697..149700 '0.0': float
            149702..149704 '-s': f32
            149703..149704 's': f32
            149731..149735 'shot': ptr<storage, ShotData, read_write>
            149731..149743 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            149731..149747 'shot.u...ry[10]': ref<storage, vec2<f32>, read_write>
            149744..149746 '10': integer
            149750..149763 'vec2f(c, 0.0)': vec2<f32>
            149756..149757 'c': f32
            149759..149762 '0.0': float
            149789..149793 'shot': ptr<storage, ShotData, read_write>
            149789..149801 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            149789..149805 'shot.u...ry[12]': ref<storage, vec2<f32>, read_write>
            149802..149804 '12': integer
            149808..149822 'vec2f(0.0, -s)': vec2<f32>
            149814..149817 '0.0': float
            149819..149821 '-s': f32
            149820..149821 's': f32
            149848..149852 'shot': ptr<storage, ShotData, read_write>
            149848..149860 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            149848..149864 'shot.u...ry[15]': ref<storage, vec2<f32>, read_write>
            149861..149863 '15': integer
            149867..149880 'vec2f(c, 0.0)': vec2<f32>
            149873..149874 'c': f32
            149876..149879 '0.0': float
            150004..150008 'shot': ptr<storage, ShotData, read_write>
            150004..150016 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            150004..150019 'shot.unitary[0]': ref<storage, vec2<f32>, read_write>
            150017..150018 '0': integer
            150023..150036 'vec2f(c, 0.0)': vec2<f32>
            150029..150030 'c': f32
            150032..150035 '0.0': float
            150062..150066 'shot': ptr<storage, ShotData, read_write>
            150062..150074 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            150062..150077 'shot.unitary[3]': ref<storage, vec2<f32>, read_write>
            150075..150076 '3': integer
            150081..150094 'vec2f(0.0, s)': vec2<f32>
            150087..150090 '0.0': float
            150092..150093 's': f32
            150120..150124 'shot': ptr<storage, ShotData, read_write>
            150120..150132 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            150120..150135 'shot.unitary[5]': ref<storage, vec2<f32>, read_write>
            150133..150134 '5': integer
            150139..150152 'vec2f(c, 0.0)': vec2<f32>
            150145..150146 'c': f32
            150148..150151 '0.0': float
            150178..150182 'shot': ptr<storage, ShotData, read_write>
            150178..150190 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            150178..150193 'shot.unitary[6]': ref<storage, vec2<f32>, read_write>
            150191..150192 '6': integer
            150197..150211 'vec2f(0.0, -s)': vec2<f32>
            150203..150206 '0.0': float
            150208..150210 '-s': f32
            150209..150210 's': f32
            150237..150241 'shot': ptr<storage, ShotData, read_write>
            150237..150249 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            150237..150252 'shot.unitary[9]': ref<storage, vec2<f32>, read_write>
            150250..150251 '9': integer
            150256..150270 'vec2f(0.0, -s)': vec2<f32>
            150262..150265 '0.0': float
            150267..150269 '-s': f32
            150268..150269 's': f32
            150296..150300 'shot': ptr<storage, ShotData, read_write>
            150296..150308 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            150296..150312 'shot.u...ry[10]': ref<storage, vec2<f32>, read_write>
            150309..150311 '10': integer
            150315..150328 'vec2f(c, 0.0)': vec2<f32>
            150321..150322 'c': f32
            150324..150327 '0.0': float
            150354..150358 'shot': ptr<storage, ShotData, read_write>
            150354..150366 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            150354..150370 'shot.u...ry[12]': ref<storage, vec2<f32>, read_write>
            150367..150369 '12': integer
            150373..150386 'vec2f(0.0, s)': vec2<f32>
            150379..150382 '0.0': float
            150384..150385 's': f32
            150412..150416 'shot': ptr<storage, ShotData, read_write>
            150412..150424 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            150412..150428 'shot.u...ry[15]': ref<storage, vec2<f32>, read_write>
            150425..150427 '15': integer
            150431..150444 'vec2f(c, 0.0)': vec2<f32>
            150437..150438 'c': f32
            150440..150443 '0.0': float
            150560..150564 'shot': ptr<storage, ShotData, read_write>
            150560..150572 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            150560..150575 'shot.unitary[0]': ref<storage, vec2<f32>, read_write>
            150573..150574 '0': integer
            150579..150594 'vec2f(1.0, 0.0)': vec2<f32>
            150585..150588 '1.0': float
            150590..150593 '0.0': float
            150620..150624 'shot': ptr<storage, ShotData, read_write>
            150620..150632 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            150620..150635 'shot.unitary[5]': ref<storage, vec2<f32>, read_write>
            150633..150634 '5': integer
            150639..150668 'vec2f(...ngle))': vec2<f32>
            150645..150655 'cos(angle)': f32
            150649..150654 'angle': f32
            150657..150667 'sin(angle)': f32
            150661..150666 'angle': f32
            150694..150698 'shot': ptr<storage, ShotData, read_write>
            150694..150706 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            150694..150710 'shot.u...ry[10]': ref<storage, vec2<f32>, read_write>
            150707..150709 '10': integer
            150713..150742 'vec2f(...ngle))': vec2<f32>
            150719..150729 'cos(angle)': f32
            150723..150728 'angle': f32
            150731..150741 'sin(angle)': f32
            150735..150740 'angle': f32
            150768..150772 'shot': ptr<storage, ShotData, read_write>
            150768..150780 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            150768..150784 'shot.u...ry[15]': ref<storage, vec2<f32>, read_write>
            150781..150783 '15': integer
            150787..150802 'vec2f(1.0, 0.0)': vec2<f32>
            150793..150796 '1.0': float
            150798..150801 '0.0': float
            150871..150875 'shot': ptr<storage, ShotData, read_write>
            150871..150882 'shot.op_idx': ref<storage, u32, read_write>
            150885..150891 'op_idx': u32
            150905..150909 'shot': ptr<storage, ShotData, read_write>
            150905..150917 'shot.op_type': ref<storage, u32, read_write>
            150920..150922 'op': ptr<storage, Op, read>
            150920..150925 'op.id': ref<storage, u32, read>
            151065..151081 'has_lo...perand': bool
            151084..151131 'gate_h...1, q2)': bool
            151106..151114 'shot_idx': u32
            151116..151122 'op_idx': u32
            151124..151126 'q1': u32
            151128..151130 'q2': u32
            151149..151165 'has_lo...perand': bool
            151185..151237 'handle...1, q2)': [error]
            151212..151220 'shot_idx': u32
            151222..151228 'op_idx': u32
            151230..151232 'q1': u32
            151234..151236 'q2': u32
            151337..151349 'pauli_op_idx': u32
            151352..151379 'get_pa...p_idx)': u32
            151372..151378 'op_idx': u32
            151484..151496 'pauli_op_idx': u32
            151484..151502 'pauli_... != 0u': bool
            151500..151502 '0u': u32
            151524..151527 'ops': ref<storage, array<Op>, read>
            151524..151541 'ops[pa...p_idx]': ref<storage, Op, read>
            151524..151544 'ops[pa...dx].id': ref<storage, u32, read>
            151524..151567 'ops[pa...ISE_1Q': bool
            151528..151540 'pauli_op_idx': u32
            151548..151567 'OPID_P...ISE_1Q': u32
            151743..151760 '!has_l...perand': bool
            151744..151760 'has_lo...perand': bool
            151788..151844 'apply_...x, q1)': [error]
            151809..151817 'shot_idx': u32
            151819..151825 'op_idx': u32
            151827..151839 'pauli_op_idx': u32
            151841..151843 'q1': u32
            151917..151933 'has_lo...perand': bool
            152118..152190 'apply_...1, q2)': [error]
            152151..152159 'shot_idx': u32
            152161..152167 'op_idx': u32
            152169..152181 'pauli_op_idx': u32
            152183..152185 'q1': u32
            152187..152189 'q2': u32
            152245..152305 'apply_...1, q2)': [error]
            152266..152274 'shot_idx': u32
            152276..152282 'op_idx': u32
            152284..152296 'pauli_op_idx': u32
            152298..152300 'q1': u32
            152302..152304 'q2': u32
            152363..152368 'shots': ref<storage, array<ShotData>, read_write>
            152363..152378 'shots[shot_idx]': ref<storage, ShotData, read_write>
            152363..152385 'shots[...interp': ref<storage, InterpreterState, read_write>
            152363..152392 'shots[...status': ref<storage, u32, read_write>
            152369..152377 'shot_idx': u32
            152395..152409 'STATUS_RUNNING': u32
            152630..152646 'has_lo...perand': bool
            152666..152671 'shots': ref<storage, array<ShotData>, read_write>
            152666..152681 'shots[shot_idx]': ref<storage, ShotData, read_write>
            152666..152688 'shots[...interp': ref<storage, InterpreterState, read_write>
            152666..152695 'shots[...status': ref<storage, u32, read_write>
            152672..152680 'shot_idx': u32
            152698..152712 'STATUS_RUNNING': u32
            152821..152863 'finali...1, q2)': [error]
            152838..152846 'shot_idx': u32
            152848..152854 'op_idx': u32
            152856..152858 'q1': u32
            152860..152862 'q2': u32
            152888..152890 '1u': u32
            153077..153089 'pauli_op_idx': u32
            153092..153119 'get_pa...p_idx)': u32
            153112..153118 'op_idx': u32
            153137..153149 'pauli_op_idx': u32
            153137..153155 'pauli_... != 0u': bool
            153153..153155 '0u': u32
            153433..153436 'ops': ref<storage, array<Op>, read>
            153433..153450 'ops[pa...p_idx]': ref<storage, Op, read>
            153433..153453 'ops[pa...dx].id': ref<storage, u32, read>
            153433..153476 'ops[pa...ISE_1Q': bool
            153437..153449 'pauli_op_idx': u32
            153457..153476 'OPID_P...ISE_1Q': u32
            153499..153555 'apply_...x, q1)': [error]
            153520..153528 'shot_idx': u32
            153530..153536 'op_idx': u32
            153538..153550 'pauli_op_idx': u32
            153552..153554 'q1': u32
            153602..153662 'apply_...1, q2)': [error]
            153623..153631 'shot_idx': u32
            153633..153639 'op_idx': u32
            153641..153653 'pauli_op_idx': u32
            153655..153657 'q1': u32
            153659..153661 'q2': u32
            153698..153703 'shots': ref<storage, array<ShotData>, read_write>
            153698..153713 'shots[shot_idx]': ref<storage, ShotData, read_write>
            153698..153720 'shots[...interp': ref<storage, InterpreterState, read_write>
            153698..153727 'shots[...status': ref<storage, u32, read_write>
            153704..153712 'shot_idx': u32
            153730..153744 'STATUS_RUNNING': u32
            153846..153852 'resets': bool
            153855..153857 'op': ptr<storage, Op, read>
            153855..153860 'op.id': ref<storage, u32, read>
            153855..153876 'op.id ...RESETZ': bool
            153864..153876 'OPID_MRESETZ': u32
            153890..153955 'prep_m...esets)': [error]
            153909..153917 'shot_idx': u32
            153919..153925 'op_idx': u32
            153927..153929 'q1': u32
            153931..153933 'q2': u32
            153935..153940 'false': bool
            153942..153946 'true': bool
            153948..153954 'resets': bool
            153980..153982 '2u': u32
            154006..154070 'prep_m... true)': [error]
            154025..154033 'shot_idx': u32
            154035..154041 'op_idx': u32
            154043..154045 'q1': u32
            154047..154049 'q2': u32
            154051..154056 'false': bool
            154058..154063 'false': bool
            154065..154069 'true': bool
            154112..154116 'shot': ptr<storage, ShotData, read_write>
            154112..154124 'shot.op_type': ref<storage, u32, read_write>
            154127..154134 'OPID_ID': u32
            154227..154232 'shots': ref<storage, array<ShotData>, read_write>
            154227..154242 'shots[shot_idx]': ref<storage, ShotData, read_write>
            154227..154249 'shots[...interp': ref<storage, InterpreterState, read_write>
            154227..154256 'shots[...status': ref<storage, u32, read_write>
            154233..154241 'shot_idx': u32
            154259..154273 'STATUS_RUNNING': u32
            154598..154606 'globalId': vec3<u32>
            154629..154640 'IS_ADAPTIVE': bool
            154652..154688 'prepar...lId.x)': [error]
            154677..154685 'globalId': vec3<u32>
            154677..154687 'globalId.x': u32
            154711..154743 'prepar...lId.x)': [error]
            154732..154740 'globalId': vec3<u32>
            154732..154742 'globalId.x': u32
            154845..154856 'workgroupId': vec3<u32>
            154910..154913 'tid': u32
            154930..154938 'shot_idx': i32
            154946..154964 'i32(wo...pId.x)': i32
            154946..154986 'i32(wo...R_SHOT': i32
            154950..154961 'workgroupId': vec3<u32>
            154950..154963 'workgroupId.x': u32
            154967..154986 'WORKGR...R_SHOT': i32
            154996..155000 'shot': ptr<storage, ShotData, read_write>
            155003..155019 '&shots...t_idx]': ptr<storage, ShotData, read_write>
            155004..155009 'shots': ref<storage, array<ShotData>, read_write>
            155004..155019 'shots[shot_idx]': ref<storage, ShotData, read_write>
            155010..155018 'shot_idx': i32
            155226..155238 'update_probs': bool
            155241..155245 'shot': ptr<storage, ShotData, read_write>
            155241..155253 'shot.op_type': ref<storage, u32, read_write>
            155241..155264 'shot.o...PID_ID': bool
            155241..155305 'shot.o..._NOISE': bool
            155241..155344 'shot.o...PID_RZ': bool
            155241..155371 'shot.o...PID_CZ': bool
            155241..155399 'shot.o...ID_RZZ': bool
            155257..155264 'OPID_ID': u32
            155268..155272 'shot': ptr<storage, ShotData, read_write>
            155268..155280 'shot.op_type': ref<storage, u32, read_write>
            155268..155305 'shot.o..._NOISE': bool
            155284..155305 'OPID_C..._NOISE': u32
            155321..155325 'shot': ptr<storage, ShotData, read_write>
            155321..155333 'shot.op_type': ref<storage, u32, read_write>
            155321..155344 'shot.o...PID_RZ': bool
            155337..155344 'OPID_RZ': u32
            155348..155352 'shot': ptr<storage, ShotData, read_write>
            155348..155360 'shot.op_type': ref<storage, u32, read_write>
            155348..155371 'shot.o...PID_CZ': bool
            155364..155371 'OPID_CZ': u32
            155375..155379 'shot': ptr<storage, ShotData, read_write>
            155375..155387 'shot.op_type': ref<storage, u32, read_write>
            155375..155399 'shot.o...ID_RZZ': bool
            155391..155399 'OPID_RZZ': u32
            155410..155414 'shot': ptr<storage, ShotData, read_write>
            155410..155422 'shot.op_type': ref<storage, u32, read_write>
            155410..155433 'shot.o...PID_ID': bool
            155426..155433 'OPID_ID': u32
            155519..155561 'apply_..., tid)': [error]
            155542..155553 'workgroupId': vec3<u32>
            155542..155555 'workgroupId.x': u32
            155557..155560 'tid': u32
            155723..155767 'apply_...p_idx)': [error]
            155735..155746 'workgroupId': vec3<u32>
            155735..155748 'workgroupId.x': u32
            155750..155753 'tid': u32
            155755..155759 'shot': ptr<storage, ShotData, read_write>
            155755..155766 'shot.op_idx': ref<storage, u32, read_write>
            155822..155824 'q1': ref<function, u32, read_write>
            155843..155854 'IS_ADAPTIVE': bool
            155870..155872 'q1': ref<function, u32, read_write>
            155875..155900 'resolv..._idx))': u32
            155886..155899 'u32(shot_idx)': u32
            155890..155898 'shot_idx': i32
            155931..155933 'q1': ref<function, u32, read_write>
            155936..155939 'ops': ref<storage, array<Op>, read>
            155936..155952 'ops[sh...p_idx]': ref<storage, Op, read>
            155936..155955 'ops[sh...dx].q1': ref<storage, u32, read>
            155940..155944 'shot': ptr<storage, ShotData, read_write>
            155940..155951 'shot.op_idx': ref<storage, u32, read_write>
            155975..156010 'apply_...d, q1)': [error]
            155987..155998 'workgroupId': vec3<u32>
            155987..156000 'workgroupId.x': u32
            156002..156005 'tid': u32
            156007..156009 'q1': ref<function, u32, read_write>
            156054..156056 'q1': ref<function, u32, read_write>
            156075..156077 'q2': ref<function, u32, read_write>
            156096..156107 'IS_ADAPTIVE': bool
            156123..156125 'q1': ref<function, u32, read_write>
            156128..156153 'resolv..._idx))': u32
            156139..156152 'u32(shot_idx)': u32
            156143..156151 'shot_idx': i32
            156167..156169 'q2': ref<function, u32, read_write>
            156172..156197 'resolv..._idx))': u32
            156183..156196 'u32(shot_idx)': u32
            156187..156195 'shot_idx': i32
            156228..156230 'q1': ref<function, u32, read_write>
            156233..156236 'ops': ref<storage, array<Op>, read>
            156233..156249 'ops[sh...p_idx]': ref<storage, Op, read>
            156233..156252 'ops[sh...dx].q1': ref<storage, u32, read>
            156237..156241 'shot': ptr<storage, ShotData, read_write>
            156237..156248 'shot.op_idx': ref<storage, u32, read_write>
            156266..156268 'q2': ref<function, u32, read_write>
            156271..156274 'ops': ref<storage, array<Op>, read>
            156271..156287 'ops[sh...p_idx]': ref<storage, Op, read>
            156271..156290 'ops[sh...dx].q2': ref<storage, u32, read>
            156275..156279 'shot': ptr<storage, ShotData, read_write>
            156275..156286 'shot.op_idx': ref<storage, u32, read_write>
            156310..156349 'apply_...1, q2)': [error]
            156322..156333 'workgroupId': vec3<u32>
            156322..156335 'workgroupId.x': u32
            156337..156340 'tid': u32
            156342..156344 'q1': ref<function, u32, read_write>
            156346..156348 'q2': ref<function, u32, read_write>
            156518..156536 'workgr...rier()': [error]
            156859..156862 'tid': u32
            156859..156867 'tid == 0': bool
            156859..156883 'tid ==..._probs': bool
            156866..156867 '0': integer
            156871..156883 'update_probs': bool
            156899..156922 'workgr...on_idx': i32
            156930..156985 'select...T > 1)': i32
            156937..156939 '-1': integer
            156938..156939 '1': integer
            156941..156959 'i32(wo...pId.x)': i32
            156945..156956 'workgroupId': vec3<u32>
            156945..156958 'workgroupId.x': u32
            156961..156980 'WORKGR...R_SHOT': i32
            156961..156984 'WORKGR...OT > 1': bool
            156983..156984 '1': integer
            157004..157005 'q': ref<function, u32, read_write>
            157013..157015 '0u': u32
            157017..157018 'q': ref<function, u32, read_write>
            157017..157037 'q < u3...COUNT)': bool
            157021..157037 'u32(QU...COUNT)': u32
            157025..157036 'QUBIT_COUNT': i32
            157039..157040 'q': ref<function, u32, read_write>
            157061..157113 '(shot.... != 0u': bool
            157062..157066 'shot': ptr<storage, ShotData, read_write>
            157062..157094 'shot.q...p_mask': ref<storage, u32, read_write>
            157062..157106 'shot.q... << q)': u32
            157098..157100 '1u': u32
            157098..157105 '1u << q': u32
            157104..157105 'q': ref<function, u32, read_write>
            157111..157113 '0u': u32
            157132..157195 'sum_th...n_idx)': [error]
            157158..157159 'q': ref<function, u32, read_write>
            157161..157169 'shot_idx': i32
            157171..157194 'workgr...on_idx': i32
        "#]],
    );
}

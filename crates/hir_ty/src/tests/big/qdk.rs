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
    // 1q matrix elements are stored in: 00, 01, 10, 11 (in other words, indices 0, 1, 4, and 5)
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
    /// The total probability of any noise (in other words, sum of all noise entries) in `Q1.63` format
    noise_probability_lo: u32,
    noise_probability_hi: u32,
    /// The start offset of this table's entries in the global `NoiseTableEntry` array
    start_offset: u32,
    /// The number of entries in this noise table
    entry_count: u32,
}

struct NoiseTableEntry {
    /// The correlated pauli string as bits (2 bits per qubit). If bit 0 is set, then it has bit-flip
    /// noise, and if bit 1 is set then it has phase-flip noise. For example, `110001 == "YIX"`.
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
// Use PCG hash function to generate a well-distributed hash from a simple integer input (for example, shot id)
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

    // Default to 1.0 renormalization (in other words, no renormalization needed). MResetZ or noise affecting the
    // overall probability distribution (for example, loss or amplitude damping) will update this if needed.
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
            // No result to store (for example, ResetZ). If the qubit is lost, it's already in the zero
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
                // APPLY_ANYWAY. Any other policy (for example, DEGRADE) is rejected by
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
// Used for conditions the host guarantees never occur.
// For example, a loss policy that is not valid for a given gate.
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
// uses real signs (in other words, `-i*Y`), matching `apply_2q_pauli_noise`; the resulting
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
    // Both lost (for example, PROPAGATE collapsed the survivor): nothing to apply.
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
        } else if (term == 3) { // Y (real-sign, in other words, `-i*Y`)
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

        // See if we can skip doing any work for this pair because the state vector entries to processes
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
// - This will only called if a result should be found. In other words:
//   - `count > 0`
//   - `rand < table[start + count - 1].probability`
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
// potentially divergent control flow paths (for example, after mid-circuit measurements).
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
//   aux0–3 : auxiliary fields whose meaning varies per opcode (for example, block
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
            // operator because WGSL i32 division truncates toward zero but
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
            // aux0 encodes the source bit width (for example, 1 for i1→i32).
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

            // FPEXT: Float widen (for example, f32→f64) — identity since GPU only uses f32.
            case OP_FPEXT {
                write_reg_f32(shot_idx, instr.dst, resolve_f32(shot_idx, instr.src0, flags, 0u));
                pc++;
            }

            // FPTRUNC: Float narrow (for example, f64→f32) — identity since GPU only uses f32.
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
            9863..9878 'MAX_QUBIT_COUNT': unexpected template argument, expected a `u32` or a `i32` greater than `0`, actual: [error]
            10006..10034 'MAX_WO...ITIONS': unexpected template argument, expected a `u32` or a `i32` greater than `0`, actual: [error]
            11207..11224 'INSTRU...S_SIZE': unexpected template argument, expected a `u32` or a `i32` greater than `0`, actual: [error]
            11299..11315 'BLOCK_...E_SIZE': unexpected template argument, expected a `u32` or a `i32` greater than `0`, actual: [error]
            11378..11397 'FUNCTI...E_SIZE': unexpected template argument, expected a `u32` or a `i32` greater than `0`, actual: [error]
            11512..11526 'PHI_TABLE_SIZE': unexpected template argument, expected a `u32` or a `i32` greater than `0`, actual: [error]
            11632..11649 'SWITCH...S_SIZE': unexpected template argument, expected a `u32` or a `i32` greater than `0`, actual: [error]
            11723..11737 'CALL_ARGS_SIZE': unexpected template argument, expected a `u32` or a `i32` greater than `0`, actual: [error]
            11832..11850 'CONSTA...A_SIZE': unexpected template argument, expected a `u32` or a `i32` greater than `0`, actual: [error]
            12850..12863 'MAX_REGISTERS': unexpected template argument, expected a `u32` or a `i32` greater than `0`, actual: [error]
            12948..12958 'MAX_MEMORY': unexpected template argument, expected a `u32` or a `i32` greater than `0`, actual: [error]
            14713..14728 'MAX_QUBIT_COUNT': unexpected template argument, expected a `u32` or a `i32` greater than `0`, actual: [error]
            17318..17335 'NOISE_..._COUNT': unexpected template argument, expected a `u32` or a `i32` greater than `0`, actual: [error]
            17391..17408 'NOISE_..._COUNT': unexpected template argument, expected a `u32` or a `i32` greater than `0`, actual: [error]
            17937..17952 'MAX_QUBIT_COUNT': unexpected template argument, expected a `u32` or a `i32` greater than `0`, actual: [error]
            17975..17990 'MAX_QUBIT_COUNT': unexpected template argument, expected a `u32` or a `i32` greater than `0`, actual: [error]
            18509..18530 'THREAD...KGROUP': unexpected template argument, expected a `u32` or a `i32` greater than `0`, actual: [error]
            18878..18897 'workgr...lation': ref<storage, WorkgroupCollationBuffer, read_write>
            19049..19054 'shots': ref<storage, array<ShotData>, read_write>
            19115..19118 'ops': ref<storage, array<Op>, read>
            19265..19276 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            19391..19398 'results': ref<storage, array<atomic<u32>>, read_write>
            19468..19479 'diagnostics': ref<storage, DiagnosticData, read_write>
            19533..19541 'uniforms': ref<uniform, Uniforms, read>
            19595..19605 'batch_data': ref<storage, BatchData, read>
            19634..19652 'qubitP...lities': ref<workgroup, [error], read_write>
            19687..19708 'THREAD...KGROUP': unexpected template argument, expected a `u32` or a `i32` greater than `0`, actual: [error]
            19895..19896 'a': vec2<f32>
            19926..19927 'a': vec2<f32>
            19926..19929 'a.x': f32
            19926..19935 'a.x * a.x': f32
            19926..19947 'a.x * ... * a.y': f32
            19932..19933 'a': vec2<f32>
            19932..19935 'a.x': f32
            19938..19939 'a': vec2<f32>
            19938..19941 'a.y': f32
            19938..19947 'a.y * a.y': f32
            19944..19945 'a': vec2<f32>
            19944..19947 'a.y': f32
            19990..19991 'a': vec2<f32>
            20000..20001 'b': vec2<f32>
            20032..20105 'vec2f(...     )': vec2<f32>
            20047..20048 'a': vec2<f32>
            20047..20050 'a.x': f32
            20047..20056 'a.x * b.x': f32
            20047..20068 'a.x * ... * b.y': f32
            20053..20054 'b': vec2<f32>
            20053..20056 'b.x': f32
            20059..20060 'a': vec2<f32>
            20059..20062 'a.y': f32
            20059..20068 'a.y * b.y': f32
            20065..20066 'b': vec2<f32>
            20065..20068 'b.y': f32
            20078..20079 'a': vec2<f32>
            20078..20081 'a.x': f32
            20078..20087 'a.x * b.y': f32
            20078..20099 'a.x * ... * b.x': f32
            20084..20085 'b': vec2<f32>
            20084..20087 'b.y': f32
            20090..20091 'a': vec2<f32>
            20090..20093 'a.y': f32
            20090..20099 'a.y * b.x': f32
            20096..20097 'b': vec2<f32>
            20096..20099 'b.x': f32
            20141..20142 'a': vec2<f32>
            20173..20190 'vec2f(... -a.y)': vec2<f32>
            20179..20183 '-a.x': f32
            20180..20181 'a': vec2<f32>
            20180..20183 'a.x': f32
            20185..20189 '-a.y': f32
            20186..20187 'a': vec2<f32>
            20186..20189 'a.y': f32
            20266..20267 'a': array<vec2<f32>, 4>
            20318..20426 'array<...a[3]))': array<vec2<f32>, 4>
            20343..20356 'cplxNeg(a[0])': vec2<f32>
            20351..20352 'a': array<vec2<f32>, 4>
            20351..20355 'a[0]': vec2<f32>
            20353..20354 '0': integer
            20366..20379 'cplxNeg(a[1])': vec2<f32>
            20374..20375 'a': array<vec2<f32>, 4>
            20374..20378 'a[1]': vec2<f32>
            20376..20377 '1': integer
            20389..20402 'cplxNeg(a[2])': vec2<f32>
            20397..20398 'a': array<vec2<f32>, 4>
            20397..20401 'a[2]': vec2<f32>
            20399..20400 '2': integer
            20412..20425 'cplxNeg(a[3])': vec2<f32>
            20420..20421 'a': array<vec2<f32>, 4>
            20420..20424 'a[3]': vec2<f32>
            20422..20423 '3': integer
            20517..20518 'a': array<vec2<f32>, 4>
            20537..20538 'b': array<vec2<f32>, 4>
            20576..20582 'result': ref<function, vec2<f32>, read_write>
            20592..20607 'vec2f(0.0, 0.0)': vec2<f32>
            20598..20601 '0.0': float
            20603..20606 '0.0': float
            20622..20623 'i': ref<function, u32, read_write>
            20631..20633 '0u': u32
            20635..20636 'i': ref<function, u32, read_write>
            20635..20641 'i < 4u': bool
            20639..20641 '4u': u32
            20643..20644 'i': ref<function, u32, read_write>
            20658..20664 'result': ref<function, vec2<f32>, read_write>
            20668..20687 'cplxMu... b[i])': vec2<f32>
            20676..20677 'a': array<vec2<f32>, 4>
            20676..20680 'a[i]': vec2<f32>
            20678..20679 'i': ref<function, u32, read_write>
            20682..20683 'b': array<vec2<f32>, 4>
            20682..20686 'b[i]': vec2<f32>
            20684..20685 'i': ref<function, u32, read_write>
            20706..20712 'result': ref<function, vec2<f32>, read_write>
            20729..20735 'op_idx': u32
            20742..20745 'row': u32
            20781..20783 'op': ptr<storage, Op, read>
            20786..20798 '&ops[op_idx]': ptr<storage, Op, read>
            20787..20790 'ops': ref<storage, array<Op>, read>
            20787..20798 'ops[op_idx]': ref<storage, Op, read>
            20791..20797 'op_idx': u32
            20811..20959 'array<... + 3])': array<vec2<f32>, 4>
            20836..20838 'op': ptr<storage, Op, read>
            20836..20846 'op.unitary': ref<storage, array<vec2<f32>, 16>, read>
            20836..20859 'op.uni...4 + 0]': ref<storage, vec2<f32>, read>
            20847..20850 'row': u32
            20847..20854 'row * 4': u32
            20847..20858 'row * 4 + 0': u32
            20853..20854 '4': integer
            20857..20858 '0': integer
            20869..20871 'op': ptr<storage, Op, read>
            20869..20879 'op.unitary': ref<storage, array<vec2<f32>, 16>, read>
            20869..20892 'op.uni...4 + 1]': ref<storage, vec2<f32>, read>
            20880..20883 'row': u32
            20880..20887 'row * 4': u32
            20880..20891 'row * 4 + 1': u32
            20886..20887 '4': integer
            20890..20891 '1': integer
            20902..20904 'op': ptr<storage, Op, read>
            20902..20912 'op.unitary': ref<storage, array<vec2<f32>, 16>, read>
            20902..20925 'op.uni...4 + 2]': ref<storage, vec2<f32>, read>
            20913..20916 'row': u32
            20913..20920 'row * 4': u32
            20913..20924 'row * 4 + 2': u32
            20919..20920 '4': integer
            20923..20924 '2': integer
            20935..20937 'op': ptr<storage, Op, read>
            20935..20945 'op.unitary': ref<storage, array<vec2<f32>, 16>, read>
            20935..20958 'op.uni...4 + 3]': ref<storage, vec2<f32>, read>
            20946..20949 'row': u32
            20946..20953 'row * 4': u32
            20946..20957 'row * 4 + 3': u32
            20952..20953 '4': integer
            20956..20957 '3': integer
            20981..20989 'shot_idx': i32
            20996..20999 'row': u32
            21035..21039 'shot': ptr<storage, ShotData, read_write>
            21042..21058 '&shots...t_idx]': ptr<storage, ShotData, read_write>
            21043..21048 'shots': ref<storage, array<ShotData>, read_write>
            21043..21058 'shots[shot_idx]': ref<storage, ShotData, read_write>
            21049..21057 'shot_idx': i32
            21071..21227 'array<... + 3])': array<vec2<f32>, 4>
            21096..21100 'shot': ptr<storage, ShotData, read_write>
            21096..21108 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            21096..21121 'shot.u...4 + 0]': ref<storage, vec2<f32>, read_write>
            21109..21112 'row': u32
            21109..21116 'row * 4': u32
            21109..21120 'row * 4 + 0': u32
            21115..21116 '4': integer
            21119..21120 '0': integer
            21131..21135 'shot': ptr<storage, ShotData, read_write>
            21131..21143 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            21131..21156 'shot.u...4 + 1]': ref<storage, vec2<f32>, read_write>
            21144..21147 'row': u32
            21144..21151 'row * 4': u32
            21144..21155 'row * 4 + 1': u32
            21150..21151 '4': integer
            21154..21155 '1': integer
            21166..21170 'shot': ptr<storage, ShotData, read_write>
            21166..21178 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            21166..21191 'shot.u...4 + 2]': ref<storage, vec2<f32>, read_write>
            21179..21182 'row': u32
            21179..21186 'row * 4': u32
            21179..21190 'row * 4 + 2': u32
            21185..21186 '4': integer
            21189..21190 '2': integer
            21201..21205 'shot': ptr<storage, ShotData, read_write>
            21201..21213 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            21201..21226 'shot.u...4 + 3]': ref<storage, vec2<f32>, read_write>
            21214..21217 'row': u32
            21214..21221 'row * 4': u32
            21214..21225 'row * 4 + 3': u32
            21220..21221 '4': integer
            21224..21225 '3': integer
            21249..21257 'shot_idx': u32
            21264..21267 'row': u32
            21274..21280 'newRow': array<vec2<f32>, 4>
            21309..21313 'shot': ptr<storage, ShotData, read_write>
            21316..21332 '&shots...t_idx]': ptr<storage, ShotData, read_write>
            21317..21322 'shots': ref<storage, array<ShotData>, read_write>
            21317..21332 'shots[shot_idx]': ref<storage, ShotData, read_write>
            21323..21331 'shot_idx': u32
            21338..21342 'shot': ptr<storage, ShotData, read_write>
            21338..21350 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            21338..21363 'shot.u...4 + 0]': ref<storage, vec2<f32>, read_write>
            21351..21354 'row': u32
            21351..21358 'row * 4': u32
            21351..21362 'row * 4 + 0': u32
            21357..21358 '4': integer
            21361..21362 '0': integer
            21366..21372 'newRow': array<vec2<f32>, 4>
            21366..21375 'newRow[0]': vec2<f32>
            21373..21374 '0': integer
            21381..21385 'shot': ptr<storage, ShotData, read_write>
            21381..21393 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            21381..21406 'shot.u...4 + 1]': ref<storage, vec2<f32>, read_write>
            21394..21397 'row': u32
            21394..21401 'row * 4': u32
            21394..21405 'row * 4 + 1': u32
            21400..21401 '4': integer
            21404..21405 '1': integer
            21409..21415 'newRow': array<vec2<f32>, 4>
            21409..21418 'newRow[1]': vec2<f32>
            21416..21417 '1': integer
            21424..21428 'shot': ptr<storage, ShotData, read_write>
            21424..21436 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            21424..21449 'shot.u...4 + 2]': ref<storage, vec2<f32>, read_write>
            21437..21440 'row': u32
            21437..21444 'row * 4': u32
            21437..21448 'row * 4 + 2': u32
            21443..21444 '4': integer
            21447..21448 '2': integer
            21452..21458 'newRow': array<vec2<f32>, 4>
            21452..21461 'newRow[2]': vec2<f32>
            21459..21460 '2': integer
            21467..21471 'shot': ptr<storage, ShotData, read_write>
            21467..21479 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            21467..21492 'shot.u...4 + 3]': ref<storage, vec2<f32>, read_write>
            21480..21483 'row': u32
            21480..21487 'row * 4': u32
            21480..21491 'row * 4 + 3': u32
            21486..21487 '4': integer
            21490..21491 '3': integer
            21495..21501 'newRow': array<vec2<f32>, 4>
            21495..21504 'newRow[3]': vec2<f32>
            21502..21503 '3': integer
            21763..21768 'input': u32
            21792..21797 'state': ref<function, u32, read_write>
            21800..21805 'input': u32
            21800..21818 'input ...96405u': u32
            21800..21832 'input ...36453u': u32
            21808..21818 '747796405u': u32
            21821..21832 '2891336453u': u32
            21842..21846 'word': ref<function, u32, read_write>
            21849..21904 '((stat...03737u': u32
            21850..21890 '(state... state': u32
            21851..21856 'state': ref<function, u32, read_write>
            21851..21881 'state ... + 4u)': u32
            21861..21880 '(state...) + 4u': u32
            21862..21867 'state': ref<function, u32, read_write>
            21862..21874 'state >> 28u': u32
            21871..21874 '28u': u32
            21878..21880 '4u': u32
            21885..21890 'state': ref<function, u32, read_write>
            21894..21904 '277803737u': u32
            21917..21937 '(word ...^ word': u32
            21918..21922 'word': ref<function, u32, read_write>
            21918..21929 'word >> 22u': u32
            21926..21929 '22u': u32
            21933..21937 'word': ref<function, u32, read_write>
            22019..22027 'shot_idx': u32
            22106..22115 'rng_state': ptr<storage, xorwow_state, read_write>
            22118..22144 '&shots..._state': ptr<storage, xorwow_state, read_write>
            22119..22124 'shots': ref<storage, array<ShotData>, read_write>
            22119..22134 'shots[shot_idx]': ref<storage, ShotData, read_write>
            22119..22144 'shots[..._state': ref<storage, xorwow_state, read_write>
            22125..22133 'shot_idx': u32
            22155..22156 't': u32
            22164..22173 'rng_state': ptr<storage, xorwow_state, read_write>
            22164..22175 'rng_state.x': ref<storage, array<u32, 5>, read_write>
            22164..22178 'rng_state.x[4]': ref<storage, u32, read_write>
            22176..22177 '4': integer
            22188..22189 's': u32
            22197..22206 'rng_state': ptr<storage, xorwow_state, read_write>
            22197..22208 'rng_state.x': ref<storage, array<u32, 5>, read_write>
            22197..22211 'rng_state.x[0]': ref<storage, u32, read_write>
            22209..22210 '0': integer
            22217..22226 'rng_state': ptr<storage, xorwow_state, read_write>
            22217..22228 'rng_state.x': ref<storage, array<u32, 5>, read_write>
            22217..22231 'rng_state.x[4]': ref<storage, u32, read_write>
            22229..22230 '4': integer
            22234..22243 'rng_state': ptr<storage, xorwow_state, read_write>
            22234..22245 'rng_state.x': ref<storage, array<u32, 5>, read_write>
            22234..22248 'rng_state.x[3]': ref<storage, u32, read_write>
            22246..22247 '3': integer
            22254..22263 'rng_state': ptr<storage, xorwow_state, read_write>
            22254..22265 'rng_state.x': ref<storage, array<u32, 5>, read_write>
            22254..22268 'rng_state.x[3]': ref<storage, u32, read_write>
            22266..22267 '3': integer
            22271..22280 'rng_state': ptr<storage, xorwow_state, read_write>
            22271..22282 'rng_state.x': ref<storage, array<u32, 5>, read_write>
            22271..22285 'rng_state.x[2]': ref<storage, u32, read_write>
            22283..22284 '2': integer
            22291..22300 'rng_state': ptr<storage, xorwow_state, read_write>
            22291..22302 'rng_state.x': ref<storage, array<u32, 5>, read_write>
            22291..22305 'rng_state.x[2]': ref<storage, u32, read_write>
            22303..22304 '2': integer
            22308..22317 'rng_state': ptr<storage, xorwow_state, read_write>
            22308..22319 'rng_state.x': ref<storage, array<u32, 5>, read_write>
            22308..22322 'rng_state.x[1]': ref<storage, u32, read_write>
            22320..22321 '1': integer
            22328..22337 'rng_state': ptr<storage, xorwow_state, read_write>
            22328..22339 'rng_state.x': ref<storage, array<u32, 5>, read_write>
            22328..22342 'rng_state.x[1]': ref<storage, u32, read_write>
            22340..22341 '1': integer
            22345..22346 's': u32
            22465..22467 't2': u32
            22470..22471 't': u32
            22470..22483 't ^ (t >> 2u)': u32
            22475..22476 't': u32
            22475..22482 't >> 2u': u32
            22480..22482 '2u': u32
            22493..22495 't3': u32
            22498..22500 't2': u32
            22498..22513 't2 ^ (t2 << 1u)': u32
            22504..22506 't2': u32
            22504..22512 't2 << 1u': u32
            22510..22512 '1u': u32
            22523..22525 't4': u32
            22528..22530 't3': u32
            22528..22534 't3 ^ s': u32
            22528..22546 't3 ^ s...<< 4u)': u32
            22533..22534 's': u32
            22538..22539 's': u32
            22538..22545 's << 4u': u32
            22543..22545 '4u': u32
            22552..22561 'rng_state': ptr<storage, xorwow_state, read_write>
            22552..22563 'rng_state.x': ref<storage, array<u32, 5>, read_write>
            22552..22566 'rng_state.x[0]': ref<storage, u32, read_write>
            22564..22565 '0': integer
            22569..22571 't4': u32
            22577..22586 'rng_state': ptr<storage, xorwow_state, read_write>
            22577..22594 'rng_st...ounter': ref<storage, u32, read_write>
            22597..22606 'rng_state': ptr<storage, xorwow_state, read_write>
            22597..22614 'rng_st...ounter': ref<storage, u32, read_write>
            22597..22624 'rng_st...62437u': u32
            22617..22624 '362437u': u32
            22637..22639 't4': u32
            22637..22659 't4 + r...ounter': u32
            22642..22651 'rng_state': ptr<storage, xorwow_state, read_write>
            22642..22659 'rng_st...ounter': ref<storage, u32, read_write>
            22681..22689 'shot_idx': u32
            22713..22721 'rand_u32': u32
            22729..22752 'next_r...t_idx)': u32
            22743..22751 'shot_idx': u32
            22934..22947 'rand_f32_bits': u32
            22950..22985 '(rand_...<< 23)': u32
            22951..22959 'rand_u32': u32
            22951..22970 'rand_u...7FFFFF': u32
            22962..22970 '0x7FFFFF': integer
            22975..22978 '127': integer
            22975..22984 '127 << 23': integer
            22982..22984 '23': integer
            23044..23045 'f': f32
            23053..23080 'bitcas..._bits)': f32
            23066..23079 'rand_f32_bits': u32
            23148..23149 'f': f32
            23148..23155 'f - 1.0': f32
            23152..23155 '1.0': float
            23238..23243 'op_id': u32
            23272..23277 'op_id': u32
            23272..23287 'op_id == OPID_S': bool
            23272..23309 'op_id ...D_SAdj': bool
            23272..23328 'op_id ...OPID_T': bool
            23272..23350 'op_id ...D_TAdj': bool
            23272..23370 'op_id ...PID_RZ': bool
            23281..23287 'OPID_S': u32
            23291..23296 'op_id': u32
            23291..23309 'op_id ...D_SAdj': bool
            23300..23309 'OPID_SAdj': u32
            23313..23318 'op_id': u32
            23313..23328 'op_id == OPID_T': bool
            23322..23328 'OPID_T': u32
            23332..23337 'op_id': u32
            23332..23350 'op_id ...D_TAdj': bool
            23341..23350 'OPID_TAdj': u32
            23354..23359 'op_id': u32
            23354..23370 'op_id ...PID_RZ': bool
            23363..23370 'OPID_RZ': u32
            23388..23393 'op_id': u32
            23422..23488 '(op_id...PID_MZ': bool
            23422..23513 '(op_id...RESETZ': bool
            23422..23544 '(op_id..._MAT1Q': bool
            23422..23574 '(op_id...UFF_1Q': bool
            23423..23428 'op_id': u32
            23423..23439 'op_id ...PID_ID': bool
            23423..23459 'op_id ...PID_RZ': bool
            23432..23439 'OPID_ID': u32
            23443..23448 'op_id': u32
            23443..23459 'op_id ...PID_RZ': bool
            23452..23459 'OPID_RZ': u32
            23472..23477 'op_id': u32
            23472..23488 'op_id ...PID_MZ': bool
            23481..23488 'OPID_MZ': u32
            23492..23497 'op_id': u32
            23492..23513 'op_id ...RESETZ': bool
            23501..23513 'OPID_MRESETZ': u32
            23525..23530 'op_id': u32
            23525..23544 'op_id ..._MAT1Q': bool
            23534..23544 'OPID_MAT1Q': u32
            23548..23553 'op_id': u32
            23548..23574 'op_id ...UFF_1Q': bool
            23557..23574 'OPID_S...UFF_1Q': u32
            23650..23658 'shot_idx': u32
            23675..23679 'shot': ptr<storage, ShotData, read_write>
            23682..23698 '&shots...t_idx]': ptr<storage, ShotData, read_write>
            23683..23688 'shots': ref<storage, array<ShotData>, read_write>
            23683..23698 'shots[shot_idx]': ref<storage, ShotData, read_write>
            23689..23697 'shot_idx': u32
            23928..23932 'shot': ptr<storage, ShotData, read_write>
            23928..23944 'shot.r...malize': ref<storage, f32, read_write>
            23947..23950 '1.0': float
            23956..23960 'shot': ptr<storage, ShotData, read_write>
            23956..23988 'shot.q...p_mask': ref<storage, u32, read_write>
            23991..23993 '0u': u32
            24080..24084 'shot': ptr<storage, ShotData, read_write>
            24080..24095 'shot.rand_pauli': ref<storage, f32, read_write>
            24098..24121 'next_r...t_idx)': f32
            24112..24120 'shot_idx': u32
            24127..24131 'shot': ptr<storage, ShotData, read_write>
            24127..24144 'shot.r...amping': ref<storage, f32, read_write>
            24147..24170 'next_r...t_idx)': f32
            24161..24169 'shot_idx': u32
            24176..24180 'shot': ptr<storage, ShotData, read_write>
            24176..24193 'shot.r...ephase': ref<storage, f32, read_write>
            24196..24219 'next_r...t_idx)': f32
            24210..24218 'shot_idx': u32
            24225..24229 'shot': ptr<storage, ShotData, read_write>
            24225..24242 'shot.r...easure': ref<storage, f32, read_write>
            24245..24268 'next_r...t_idx)': f32
            24259..24267 'shot_idx': u32
            24557..24580 'next_r...t_idx)': f32
            24571..24579 'shot_idx': u32
            24685..24693 'shot_idx': i32
            24710..24714 'shot': ptr<storage, ShotData, read_write>
            24717..24733 '&shots...t_idx]': ptr<storage, ShotData, read_write>
            24718..24723 'shots': ref<storage, array<ShotData>, read_write>
            24718..24733 'shots[shot_idx]': ref<storage, ShotData, read_write>
            24724..24732 'shot_idx': i32
            24831..24839 'rng_seed': u32
            24842..24850 'uniforms': ref<uniform, Uniforms, read>
            24842..24859 'unifor...g_seed': ref<uniform, u32, read>
            24869..24876 'shot_id': u32
            24879..24923 'u32(un...t_idx)': u32
            24883..24891 'uniforms': ref<uniform, Uniforms, read>
            24883..24911 'unifor...hot_id': ref<uniform, i32, read>
            24883..24922 'unifor...ot_idx': i32
            24914..24922 'shot_idx': i32
            25056..25061 '*shot': ref<storage, ShotData, read_write>
            25057..25061 'shot': ptr<storage, ShotData, read_write>
            25064..25074 'ShotData()': ShotData
            25102..25106 'shot': ptr<storage, ShotData, read_write>
            25102..25114 'shot.shot_id': ref<storage, u32, read_write>
            25117..25124 'shot_id': u32
            25184..25188 'shot': ptr<storage, ShotData, read_write>
            25184..25200 'shot.n...op_idx': ref<storage, u32, read_write>
            25203..25205 '0u': u32
            25212..25216 'shot': ptr<storage, ShotData, read_write>
            25212..25226 'shot.rng_state': ref<storage, xorwow_state, read_write>
            25212..25228 'shot.r...tate.x': ref<storage, array<u32, 5>, read_write>
            25212..25231 'shot.r...e.x[0]': ref<storage, u32, read_write>
            25229..25230 '0': integer
            25234..25242 'rng_seed': u32
            25234..25262 'rng_se...ot_id)': u32
            25245..25262 'hash_p...ot_id)': u32
            25254..25261 'shot_id': u32
            25268..25272 'shot': ptr<storage, ShotData, read_write>
            25268..25282 'shot.rng_state': ref<storage, xorwow_state, read_write>
            25268..25284 'shot.r...tate.x': ref<storage, array<u32, 5>, read_write>
            25268..25287 'shot.r...e.x[1]': ref<storage, u32, read_write>
            25285..25286 '1': integer
            25290..25298 'rng_seed': u32
            25290..25322 'rng_se...d + 1)': u32
            25301..25322 'hash_p...d + 1)': u32
            25310..25317 'shot_id': u32
            25310..25321 'shot_id + 1': u32
            25320..25321 '1': integer
            25328..25332 'shot': ptr<storage, ShotData, read_write>
            25328..25342 'shot.rng_state': ref<storage, xorwow_state, read_write>
            25328..25344 'shot.r...tate.x': ref<storage, array<u32, 5>, read_write>
            25328..25347 'shot.r...e.x[2]': ref<storage, u32, read_write>
            25345..25346 '2': integer
            25350..25358 'rng_seed': u32
            25350..25382 'rng_se...d + 2)': u32
            25361..25382 'hash_p...d + 2)': u32
            25370..25377 'shot_id': u32
            25370..25381 'shot_id + 2': u32
            25380..25381 '2': integer
            25388..25392 'shot': ptr<storage, ShotData, read_write>
            25388..25402 'shot.rng_state': ref<storage, xorwow_state, read_write>
            25388..25404 'shot.r...tate.x': ref<storage, array<u32, 5>, read_write>
            25388..25407 'shot.r...e.x[3]': ref<storage, u32, read_write>
            25405..25406 '3': integer
            25410..25418 'rng_seed': u32
            25410..25442 'rng_se...d + 3)': u32
            25421..25442 'hash_p...d + 3)': u32
            25430..25437 'shot_id': u32
            25430..25441 'shot_id + 3': u32
            25440..25441 '3': integer
            25448..25452 'shot': ptr<storage, ShotData, read_write>
            25448..25462 'shot.rng_state': ref<storage, xorwow_state, read_write>
            25448..25464 'shot.r...tate.x': ref<storage, array<u32, 5>, read_write>
            25448..25467 'shot.r...e.x[4]': ref<storage, u32, read_write>
            25465..25466 '4': integer
            25470..25478 'rng_seed': u32
            25470..25502 'rng_se...d + 4)': u32
            25481..25502 'hash_p...d + 4)': u32
            25490..25497 'shot_id': u32
            25490..25501 'shot_id + 4': u32
            25500..25501 '4': integer
            25509..25513 'shot': ptr<storage, ShotData, read_write>
            25509..25521 'shot.op_type': ref<storage, u32, read_write>
            25524..25525 '0': integer
            25531..25535 'shot': ptr<storage, ShotData, read_write>
            25531..25542 'shot.op_idx': ref<storage, u32, read_write>
            25545..25546 '0': integer
            25635..25639 'shot': ptr<storage, ShotData, read_write>
            25635..25648 'shot.duration': ref<storage, f32, read_write>
            25651..25654 '0.0': float
            25660..25664 'shot': ptr<storage, ShotData, read_write>
            25660..25676 'shot.r...malize': ref<storage, f32, read_write>
            25679..25682 '1.0': float
            25689..25693 'shot': ptr<storage, ShotData, read_write>
            25689..25709 'shot.q...0_mask': ref<storage, u32, read_write>
            25712..25741 '(1u <<...) - 1u': u32
            25713..25715 '1u': u32
            25713..25735 '1u << ...COUNT)': u32
            25719..25735 'u32(QU...COUNT)': u32
            25723..25734 'QUBIT_COUNT': i32
            25739..25741 '1u': u32
            25769..25773 'shot': ptr<storage, ShotData, read_write>
            25769..25789 'shot.q...1_mask': ref<storage, u32, read_write>
            25792..25794 '0u': u32
            25800..25804 'shot': ptr<storage, ShotData, read_write>
            25800..25832 'shot.q...p_mask': ref<storage, u32, read_write>
            25835..25836 '0': integer
            25842..25846 'shot': ptr<storage, ShotData, read_write>
            25842..25864 'shot.p...s_mask': ref<storage, u32, read_write>
            25867..25869 '0u': u32
            25939..25940 'i': ref<function, i32, read_write>
            25948..25949 '0': integer
            25951..25952 'i': ref<function, i32, read_write>
            25951..25966 'i < QUBIT_COUNT': bool
            25955..25966 'QUBIT_COUNT': i32
            25968..25969 'i': ref<function, i32, read_write>
            25983..25987 'shot': ptr<storage, ShotData, read_write>
            25983..25999 'shot.q..._state': ref<storage, [error], read_write>
            25983..26002 'shot.q...ate[i]': [error]
            25983..26019 'shot.q...bility': [error]
            26000..26001 'i': ref<function, i32, read_write>
            26022..26025 '1.0': float
            26035..26039 'shot': ptr<storage, ShotData, read_write>
            26035..26051 'shot.q..._state': ref<storage, [error], read_write>
            26035..26054 'shot.q...ate[i]': [error]
            26035..26070 'shot.q...bility': [error]
            26052..26053 'i': ref<function, i32, read_write>
            26073..26076 '0.0': float
            26086..26090 'shot': ptr<storage, ShotData, read_write>
            26086..26102 'shot.q..._state': ref<storage, [error], read_write>
            26086..26105 'shot.q...ate[i]': [error]
            26086..26110 'shot.q...].heat': [error]
            26103..26104 'i': ref<function, i32, read_write>
            26113..26116 '0.0': float
            26126..26130 'shot': ptr<storage, ShotData, read_write>
            26126..26142 'shot.q..._state': ref<storage, [error], read_write>
            26126..26145 'shot.q...ate[i]': [error]
            26126..26156 'shot.q..._since': [error]
            26143..26144 'i': ref<function, i32, read_write>
            26159..26162 '0.0': float
            26289..26297 'shot_idx': u32
            26314..26318 'shot': ptr<storage, ShotData, read_write>
            26321..26337 '&shots...t_idx]': ptr<storage, ShotData, read_write>
            26322..26327 'shots': ref<storage, array<ShotData>, read_write>
            26322..26337 'shots[shot_idx]': ref<storage, ShotData, read_write>
            26328..26336 'shot_idx': u32
            26714..26715 'q': ref<function, u32, read_write>
            26723..26725 '0u': u32
            26727..26728 'q': ref<function, u32, read_write>
            26727..26747 'q < u3...COUNT)': bool
            26731..26747 'u32(QU...COUNT)': u32
            26735..26746 'QUBIT_COUNT': i32
            26749..26750 'q': ref<function, u32, read_write>
            26768..26778 'qubit_mask': u32
            26786..26788 '1u': u32
            26786..26793 '1u << q': u32
            26792..26793 'q': ref<function, u32, read_write>
            26807..26860 '(shot.... != 0u': bool
            26808..26812 'shot': ptr<storage, ShotData, read_write>
            26808..26840 'shot.q...p_mask': ref<storage, u32, read_write>
            26808..26853 'shot.q...t_mask': u32
            26843..26853 'qubit_mask': u32
            26858..26860 '0u': u32
            27141..27151 'total_zero': ref<function, f32, read_write>
            27159..27162 '0.0': float
            27180..27189 'total_one': ref<function, f32, read_write>
            27197..27200 '0.0': float
            27219..27238 'WORKGR...R_SHOT': i32
            27219..27242 'WORKGR...OT > 1': bool
            27241..27242 '1': integer
            27344..27350 'offset': u32
            27353..27361 'shot_idx': u32
            27353..27388 'shot_i..._SHOT)': u32
            27364..27388 'u32(WO..._SHOT)': u32
            27368..27387 'WORKGR...R_SHOT': i32
            27415..27422 'wkg_idx': ref<function, u32, read_write>
            27430..27432 '0u': u32
            27434..27441 'wkg_idx': ref<function, u32, read_write>
            27434..27468 'wkg_id..._SHOT)': bool
            27444..27468 'u32(WO..._SHOT)': u32
            27448..27467 'WORKGR...R_SHOT': i32
            27470..27477 'wkg_idx': ref<function, u32, read_write>
            27507..27511 'sums': [error]
            27514..27533 'workgr...lation': ref<storage, WorkgroupCollationBuffer, read_write>
            27514..27538 'workgr...n.sums': ref<storage, [error], read_write>
            27514..27556 'workgr...ffset]': [error]
            27539..27546 'wkg_idx': ref<function, u32, read_write>
            27539..27555 'wkg_id...offset': u32
            27549..27555 'offset': u32
            27578..27588 'total_zero': ref<function, f32, read_write>
            27591..27601 'total_zero': ref<function, f32, read_write>
            27591..27620 'total_...s[q].x': [error]
            27604..27608 'sums': [error]
            27604..27615 'sums.qubits': [error]
            27604..27618 'sums.qubits[q]': [error]
            27604..27620 'sums.q...s[q].x': [error]
            27616..27617 'q': ref<function, u32, read_write>
            27642..27651 'total_one': ref<function, f32, read_write>
            27654..27663 'total_one': ref<function, f32, read_write>
            27654..27682 'total_...s[q].y': [error]
            27666..27670 'sums': [error]
            27666..27677 'sums.qubits': [error]
            27666..27680 'sums.qubits[q]': [error]
            27666..27682 'sums.q...s[q].y': [error]
            27678..27679 'q': ref<function, u32, read_write>
            27824..27834 'total_zero': ref<function, f32, read_write>
            27837..27841 'shot': ptr<storage, ShotData, read_write>
            27837..27853 'shot.q..._state': ref<storage, [error], read_write>
            27837..27856 'shot.q...ate[q]': [error]
            27837..27873 'shot.q...bility': [error]
            27854..27855 'q': ref<function, u32, read_write>
            27891..27900 'total_one': ref<function, f32, read_write>
            27903..27907 'shot': ptr<storage, ShotData, read_write>
            27903..27919 'shot.q..._state': ref<storage, [error], read_write>
            27903..27922 'shot.q...ate[q]': [error]
            27903..27938 'shot.q...bility': [error]
            27920..27921 'q': ref<function, u32, read_write>
            28183..28193 'total_zero': ref<function, f32, read_write>
            28183..28204 'total_...000001': bool
            28196..28204 '0.000001': float
            28208..28218 'total_zero': ref<function, f32, read_write>
            28221..28224 '0.0': float
            28244..28253 'total_one': ref<function, f32, read_write>
            28244..28264 'total_...000001': bool
            28256..28264 '0.000001': float
            28268..28277 'total_one': ref<function, f32, read_write>
            28280..28283 '0.0': float
            28303..28313 'total_zero': ref<function, f32, read_write>
            28303..28324 'total_...999999': bool
            28316..28324 '0.999999': float
            28328..28338 'total_zero': ref<function, f32, read_write>
            28341..28344 '1.0': float
            28364..28373 'total_one': ref<function, f32, read_write>
            28364..28384 'total_...999999': bool
            28376..28384 '0.999999': float
            28388..28397 'total_one': ref<function, f32, read_write>
            28400..28403 '1.0': float
            28420..28424 'shot': ptr<storage, ShotData, read_write>
            28420..28436 'shot.q..._state': ref<storage, [error], read_write>
            28420..28439 'shot.q...ate[q]': [error]
            28420..28456 'shot.q...bility': [error]
            28437..28438 'q': ref<function, u32, read_write>
            28459..28469 'total_zero': ref<function, f32, read_write>
            28483..28487 'shot': ptr<storage, ShotData, read_write>
            28483..28499 'shot.q..._state': ref<storage, [error], read_write>
            28483..28502 'shot.q...ate[q]': [error]
            28483..28518 'shot.q...bility': [error]
            28500..28501 'q': ref<function, u32, read_write>
            28521..28530 'total_one': ref<function, f32, read_write>
            28765..28781 'within...eshold': bool
            28784..28819 'abs(1...._one))': f32
            28784..28836 'abs(1....ESHOLD': bool
            28788..28791 '1.0': float
            28788..28818 '1.0 - ...l_one)': f32
            28795..28805 'total_zero': ref<function, f32, read_write>
            28795..28817 'total_...al_one': f32
            28808..28817 'total_one': ref<function, f32, read_write>
            28822..28836 'PROB_THRESHOLD': f32
            28853..28870 '!withi...eshold': bool
            28854..28870 'within...eshold': bool
            28964..28973 'old_value': __atomic_compare_exchange_result
            28976..29110 'atomic...PROBS)': __atomic_compare_exchange_result
            29023..29046 '&diagn...r_code': ptr<storage, atomic<u32>, read_write>
            29024..29035 'diagnostics': ref<storage, DiagnosticData, read_write>
            29024..29046 'diagno...r_code': ref<storage, atomic<u32>, read_write>
            29068..29070 '0u': u32
            29092..29109 'ERR_IN..._PROBS': u32
            29131..29140 'old_value': __atomic_compare_exchange_result
            29131..29150 'old_va...hanged': bool
            29242..29253 'diagnostics': ref<storage, DiagnosticData, read_write>
            29242..29260 'diagno...extra1': ref<storage, u32, read_write>
            29263..29264 'q': ref<function, u32, read_write>
            29286..29297 'diagnostics': ref<storage, DiagnosticData, read_write>
            29286..29304 'diagno...extra2': ref<storage, f32, read_write>
            29307..29317 'total_zero': ref<function, f32, read_write>
            29339..29350 'diagnostics': ref<storage, DiagnosticData, read_write>
            29339..29357 'diagno...extra3': ref<storage, f32, read_write>
            29360..29369 'total_one': ref<function, f32, read_write>
            29544..29555 'diagnostics': ref<storage, DiagnosticData, read_write>
            29544..29560 'diagno...s.shot': ref<storage, ShotData, read_write>
            29563..29568 '*shot': ref<storage, ShotData, read_write>
            29564..29568 'shot': ptr<storage, ShotData, read_write>
            29590..29601 'diagnostics': ref<storage, DiagnosticData, read_write>
            29590..29604 'diagnostics.op': ref<storage, Op, read_write>
            29607..29610 'ops': ref<storage, array<Op>, read>
            29607..29623 'ops[sh...p_idx]': ref<storage, Op, read>
            29611..29615 'shot': ptr<storage, ShotData, read_write>
            29611..29622 'shot.op_idx': ref<storage, u32, read_write>
            29764..29773 'err_index': u32
            29776..29805 '(shot_..._COUNT': u32
            29776..29809 '(shot_...NT - 1': u32
            29777..29785 'shot_idx': u32
            29777..29789 'shot_idx + 1': u32
            29788..29789 '1': integer
            29793..29805 'RESULT_COUNT': u32
            29808..29809 '1': integer
            29827..29957 'atomic...PROBS)': __atomic_compare_exchange_result
            29874..29893 '&resul...index]': ptr<storage, atomic<u32>, read_write>
            29875..29882 'results': ref<storage, array<atomic<u32>>, read_write>
            29875..29893 'result...index]': ref<storage, atomic<u32>, read_write>
            29883..29892 'err_index': u32
            29915..29917 '0u': u32
            29939..29956 'ERR_IN..._PROBS': u32
            30038..30042 'shot': ptr<storage, ShotData, read_write>
            30038..30058 'shot.q...0_mask': ref<storage, u32, read_write>
            30061..30206 'select...= 1.0)': u32
            30085..30089 'shot': ptr<storage, ShotData, read_write>
            30085..30105 'shot.q...0_mask': ref<storage, u32, read_write>
            30085..30119 'shot.q...t_mask': u32
            30108..30119 '~qubit_mask': u32
            30109..30119 'qubit_mask': u32
            30137..30141 'shot': ptr<storage, ShotData, read_write>
            30137..30157 'shot.q...0_mask': ref<storage, u32, read_write>
            30137..30170 'shot.q...t_mask': u32
            30160..30170 'qubit_mask': u32
            30188..30198 'total_zero': ref<function, f32, read_write>
            30188..30205 'total_...== 1.0': bool
            30202..30205 '1.0': float
            30220..30224 'shot': ptr<storage, ShotData, read_write>
            30220..30240 'shot.q...1_mask': ref<storage, u32, read_write>
            30243..30387 'select...= 1.0)': u32
            30267..30271 'shot': ptr<storage, ShotData, read_write>
            30267..30287 'shot.q...1_mask': ref<storage, u32, read_write>
            30267..30301 'shot.q...t_mask': u32
            30290..30301 '~qubit_mask': u32
            30291..30301 'qubit_mask': u32
            30319..30323 'shot': ptr<storage, ShotData, read_write>
            30319..30339 'shot.q...1_mask': ref<storage, u32, read_write>
            30319..30352 'shot.q...t_mask': u32
            30342..30352 'qubit_mask': u32
            30370..30379 'total_one': ref<function, f32, read_write>
            30370..30386 'total_...== 1.0': bool
            30383..30386 '1.0': float
            30542..30558 'stateV...rIndex': u32
            30565..30574 'amplitude': vec2<f32>
            30583..30586 'tid': u32
            30603..30607 'mask': ref<function, u32, read_write>
            30615..30617 '1u': u32
            30632..30633 'q': ref<function, u32, read_write>
            30641..30643 '0u': u32
            30645..30646 'q': ref<function, u32, read_write>
            30645..30665 'q < u3...COUNT)': bool
            30649..30665 'u32(QU...COUNT)': u32
            30653..30664 'QUBIT_COUNT': i32
            30667..30668 'q': ref<function, u32, read_write>
            30686..30692 'is_one': bool
            30701..30732 '(state... != 0u': bool
            30702..30718 'stateV...rIndex': u32
            30702..30725 'stateV...& mask': u32
            30721..30725 'mask': ref<function, u32, read_write>
            30730..30732 '0u': u32
            30746..30750 'prob': f32
            30758..30777 'cplxMa...itude)': f32
            30767..30776 'amplitude': vec2<f32>
            30791..30797 'is_one': bool
            30813..30831 'qubitP...lities': ref<workgroup, [error], read_write>
            30813..30836 'qubitP...s[tid]': [error]
            30813..30840 'qubitP...d].one': [error]
            30813..30843 'qubitP...one[q]': [error]
            30832..30835 'tid': u32
            30841..30842 'q': ref<function, u32, read_write>
            30847..30851 'prob': f32
            30882..30900 'qubitP...lities': ref<workgroup, [error], read_write>
            30882..30905 'qubitP...s[tid]': [error]
            30882..30910 'qubitP...].zero': [error]
            30882..30913 'qubitP...ero[q]': [error]
            30901..30904 'tid': u32
            30911..30912 'q': ref<function, u32, read_write>
            30917..30921 'prob': f32
            30941..30945 'mask': ref<function, u32, read_write>
            30948..30952 'mask': ref<function, u32, read_write>
            30948..30958 'mask << 1u': u32
            30956..30958 '1u': u32
            30998..30999 'q': u32
            31006..31014 'shot_idx': i32
            31021..31038 'wkg_co...on_idx': i32
            31055..31065 'total_zero': ref<function, f32, read_write>
            31073..31076 '0.0': float
            31086..31095 'total_one': ref<function, f32, read_write>
            31103..31106 '0.0': float
            31121..31122 'j': ref<function, i32, read_write>
            31125..31126 '0': integer
            31128..31129 'j': ref<function, i32, read_write>
            31128..31153 'j < TH...KGROUP': bool
            31132..31153 'THREAD...KGROUP': i32
            31155..31156 'j': ref<function, i32, read_write>
            31170..31180 'total_zero': ref<function, f32, read_write>
            31184..31202 'qubitP...lities': ref<workgroup, [error], read_write>
            31184..31205 'qubitP...ies[j]': [error]
            31184..31210 'qubitP...].zero': [error]
            31184..31213 'qubitP...ero[q]': [error]
            31203..31204 'j': ref<function, i32, read_write>
            31211..31212 'q': u32
            31223..31232 'total_one': ref<function, f32, read_write>
            31236..31254 'qubitP...lities': ref<workgroup, [error], read_write>
            31236..31257 'qubitP...ies[j]': [error]
            31236..31261 'qubitP...j].one': [error]
            31236..31264 'qubitP...one[q]': [error]
            31255..31256 'j': ref<function, i32, read_write>
            31262..31263 'q': u32
            31280..31297 'wkg_co...on_idx': i32
            31280..31302 'wkg_co...x >= 0': bool
            31301..31302 '0': integer
            31405..31424 'workgr...lation': ref<storage, WorkgroupCollationBuffer, read_write>
            31405..31429 'workgr...n.sums': ref<storage, [error], read_write>
            31405..31448 'workgr...n_idx]': [error]
            31405..31455 'workgr...qubits': [error]
            31405..31458 'workgr...its[q]': [error]
            31430..31447 'wkg_co...on_idx': i32
            31456..31457 'q': u32
            31461..31489 'vec2f(...l_one)': vec2<f32>
            31467..31477 'total_zero': ref<function, f32, read_write>
            31479..31488 'total_one': ref<function, f32, read_write>
            31593..31609 'within...eshold': bool
            31612..31647 'abs(1...._one))': f32
            31612..31664 'abs(1....ESHOLD': bool
            31616..31619 '1.0': float
            31616..31646 '1.0 - ...l_one)': f32
            31623..31633 'total_zero': ref<function, f32, read_write>
            31623..31645 'total_...al_one': f32
            31636..31645 'total_one': ref<function, f32, read_write>
            31650..31664 'PROB_THRESHOLD': f32
            31677..31694 '!withi...eshold': bool
            31678..31694 'within...eshold': bool
            31780..31789 'old_value': __atomic_compare_exchange_result
            31792..31921 'atomic...TOTAL)': __atomic_compare_exchange_result
            31835..31858 '&diagn...r_code': ptr<storage, atomic<u32>, read_write>
            31836..31847 'diagnostics': ref<storage, DiagnosticData, read_write>
            31836..31858 'diagno...r_code': ref<storage, atomic<u32>, read_write>
            31876..31878 '0u': u32
            31896..31920 'ERR_IN..._TOTAL': u32
            31938..31947 'old_value': __atomic_compare_exchange_result
            31938..31957 'old_va...hanged': bool
            32045..32049 'shot': ptr<storage, ShotData, read_write>
            32052..32068 '&shots...t_idx]': ptr<storage, ShotData, read_write>
            32053..32058 'shots': ref<storage, array<ShotData>, read_write>
            32053..32068 'shots[shot_idx]': ref<storage, ShotData, read_write>
            32059..32067 'shot_idx': i32
            32086..32097 'diagnostics': ref<storage, DiagnosticData, read_write>
            32086..32104 'diagno...extra1': ref<storage, u32, read_write>
            32107..32108 'q': u32
            32126..32137 'diagnostics': ref<storage, DiagnosticData, read_write>
            32126..32144 'diagno...extra2': ref<storage, f32, read_write>
            32147..32157 'total_zero': ref<function, f32, read_write>
            32175..32186 'diagnostics': ref<storage, DiagnosticData, read_write>
            32175..32193 'diagno...extra3': ref<storage, f32, read_write>
            32196..32205 'total_one': ref<function, f32, read_write>
            32366..32377 'diagnostics': ref<storage, DiagnosticData, read_write>
            32366..32382 'diagno...s.shot': ref<storage, ShotData, read_write>
            32385..32390 '*shot': ref<storage, ShotData, read_write>
            32386..32390 'shot': ptr<storage, ShotData, read_write>
            32408..32419 'diagnostics': ref<storage, DiagnosticData, read_write>
            32408..32422 'diagnostics.op': ref<storage, Op, read_write>
            32425..32428 'ops': ref<storage, array<Op>, read>
            32425..32441 'ops[sh...p_idx]': ref<storage, Op, read>
            32429..32433 'shot': ptr<storage, ShotData, read_write>
            32429..32440 'shot.op_idx': ref<storage, u32, read_write>
            32507..32516 'err_index': i32
            32519..32553 '(shot_...COUNT)': i32
            32519..32557 '(shot_...T) - 1': i32
            32520..32528 'shot_idx': i32
            32520..32532 'shot_idx + 1': i32
            32531..32532 '1': integer
            32536..32553 'i32(RE...COUNT)': i32
            32540..32552 'RESULT_COUNT': u32
            32556..32557 '1': integer
            32571..32708 'atomic...TOTAL)': __atomic_compare_exchange_result
            32618..32637 '&resul...index]': ptr<storage, atomic<u32>, read_write>
            32619..32626 'results': ref<storage, array<atomic<u32>>, read_write>
            32619..32637 'result...index]': ref<storage, atomic<u32>, read_write>
            32627..32636 'err_index': i32
            32659..32661 '0u': u32
            32683..32707 'ERR_IN..._TOTAL': u32
            32739..32744 'shots': ref<storage, array<ShotData>, read_write>
            32739..32754 'shots[shot_idx]': ref<storage, ShotData, read_write>
            32739..32766 'shots[..._state': ref<storage, [error], read_write>
            32739..32769 'shots[...ate[q]': [error]
            32739..32786 'shots[...bility': [error]
            32745..32753 'shot_idx': i32
            32767..32768 'q': u32
            32789..32799 'total_zero': ref<function, f32, read_write>
            32813..32818 'shots': ref<storage, array<ShotData>, read_write>
            32813..32828 'shots[shot_idx]': ref<storage, ShotData, read_write>
            32813..32840 'shots[..._state': ref<storage, [error], read_write>
            32813..32843 'shots[...ate[q]': [error]
            32813..32859 'shots[...bility': [error]
            32819..32827 'shot_idx': i32
            32841..32842 'q': u32
            32862..32871 'total_one': ref<function, f32, read_write>
            33352..33360 'shot_idx': u32
            33367..33372 'qubit': u32
            33379..33385 'result': u32
            33392..33406 'resets_to_zero': bool
            33424..33428 'shot': ptr<storage, ShotData, read_write>
            33431..33447 '&shots...t_idx]': ptr<storage, ShotData, read_write>
            33432..33437 'shots': ref<storage, array<ShotData>, read_write>
            33432..33447 'shots[shot_idx]': ref<storage, ShotData, read_write>
            33438..33446 'shot_idx': u32
            33617..33631 'resets_to_zero': bool
            33831..33835 'shot': ptr<storage, ShotData, read_write>
            33831..33843 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            33831..33846 'shot.unitary[0]': ref<storage, vec2<f32>, read_write>
            33844..33845 '0': integer
            33849..33903 'select...== 1u)': vec2<f32>
            33856..33871 'vec2f(1.0, 0.0)': vec2<f32>
            33862..33865 '1.0': float
            33867..33870 '0.0': float
            33873..33888 'vec2f(0.0, 0.0)': vec2<f32>
            33879..33882 '0.0': float
            33884..33887 '0.0': float
            33890..33896 'result': u32
            33890..33902 'result == 1u': bool
            33900..33902 '1u': u32
            33913..33917 'shot': ptr<storage, ShotData, read_write>
            33913..33925 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            33913..33928 'shot.unitary[1]': ref<storage, vec2<f32>, read_write>
            33926..33927 '1': integer
            33931..33985 'select...== 1u)': vec2<f32>
            33938..33953 'vec2f(0.0, 0.0)': vec2<f32>
            33944..33947 '0.0': float
            33949..33952 '0.0': float
            33955..33970 'vec2f(1.0, 0.0)': vec2<f32>
            33961..33964 '1.0': float
            33966..33969 '0.0': float
            33972..33978 'result': u32
            33972..33984 'result == 1u': bool
            33982..33984 '1u': u32
            33995..33999 'shot': ptr<storage, ShotData, read_write>
            33995..34007 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            33995..34010 'shot.unitary[4]': ref<storage, vec2<f32>, read_write>
            34008..34009 '4': integer
            34013..34020 'vec2f()': vec2<f32>
            34030..34034 'shot': ptr<storage, ShotData, read_write>
            34030..34042 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            34030..34045 'shot.unitary[5]': ref<storage, vec2<f32>, read_write>
            34043..34044 '5': integer
            34048..34055 'vec2f()': vec2<f32>
            34236..34240 'shot': ptr<storage, ShotData, read_write>
            34236..34248 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            34236..34251 'shot.unitary[0]': ref<storage, vec2<f32>, read_write>
            34249..34250 '0': integer
            34254..34308 'select...== 1u)': vec2<f32>
            34261..34276 'vec2f(1.0, 0.0)': vec2<f32>
            34267..34270 '1.0': float
            34272..34275 '0.0': float
            34278..34293 'vec2f(0.0, 0.0)': vec2<f32>
            34284..34287 '0.0': float
            34289..34292 '0.0': float
            34295..34301 'result': u32
            34295..34307 'result == 1u': bool
            34305..34307 '1u': u32
            34318..34322 'shot': ptr<storage, ShotData, read_write>
            34318..34330 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            34318..34333 'shot.unitary[1]': ref<storage, vec2<f32>, read_write>
            34331..34332 '1': integer
            34336..34343 'vec2f()': vec2<f32>
            34353..34357 'shot': ptr<storage, ShotData, read_write>
            34353..34365 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            34353..34368 'shot.unitary[4]': ref<storage, vec2<f32>, read_write>
            34366..34367 '4': integer
            34371..34378 'vec2f()': vec2<f32>
            34388..34392 'shot': ptr<storage, ShotData, read_write>
            34388..34400 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            34388..34403 'shot.unitary[5]': ref<storage, vec2<f32>, read_write>
            34401..34402 '5': integer
            34406..34460 'select...== 1u)': vec2<f32>
            34413..34428 'vec2f(0.0, 0.0)': vec2<f32>
            34419..34422 '0.0': float
            34424..34427 '0.0': float
            34430..34445 'vec2f(1.0, 0.0)': vec2<f32>
            34436..34439 '1.0': float
            34441..34444 '0.0': float
            34447..34453 'result': u32
            34447..34459 'result == 1u': bool
            34457..34459 '1u': u32
            34473..34477 'shot': ptr<storage, ShotData, read_write>
            34473..34489 'shot.r...malize': ref<storage, f32, read_write>
            34492..34644 'select...== 1u)': [error]
            34508..34511 '1.0': float
            34508..34560 '1.0 / ...ility)': [error]
            34514..34560 'sqrt(s...ility)': [error]
            34519..34523 'shot': ptr<storage, ShotData, read_write>
            34519..34535 'shot.q..._state': ref<storage, [error], read_write>
            34519..34542 'shot.q...qubit]': [error]
            34519..34559 'shot.q...bility': [error]
            34536..34541 'qubit': u32
            34570..34573 '1.0': float
            34570..34621 '1.0 / ...ility)': [error]
            34576..34621 'sqrt(s...ility)': [error]
            34581..34585 'shot': ptr<storage, ShotData, read_write>
            34581..34597 'shot.q..._state': ref<storage, [error], read_write>
            34581..34604 'shot.q...qubit]': [error]
            34581..34620 'shot.q...bility': [error]
            34598..34603 'qubit': u32
            34631..34637 'result': u32
            34631..34643 'result == 1u': bool
            34641..34643 '1u': u32
            34767..34771 'shot': ptr<storage, ShotData, read_write>
            34767..34787 'shot.q...1_mask': ref<storage, u32, read_write>
            34790..34794 'shot': ptr<storage, ShotData, read_write>
            34790..34810 'shot.q...1_mask': ref<storage, u32, read_write>
            34790..34827 'shot.q...qubit)': u32
            34813..34827 '~(1u << qubit)': u32
            34815..34817 '1u': u32
            34815..34826 '1u << qubit': u32
            34821..34826 'qubit': u32
            34833..34837 'shot': ptr<storage, ShotData, read_write>
            34833..34853 'shot.q...0_mask': ref<storage, u32, read_write>
            34856..34860 'shot': ptr<storage, ShotData, read_write>
            34856..34876 'shot.q...0_mask': ref<storage, u32, read_write>
            34856..34893 'shot.q...qubit)': u32
            34879..34893 '~(1u << qubit)': u32
            34881..34883 '1u': u32
            34881..34892 '1u << qubit': u32
            34887..34892 'qubit': u32
            35180..35184 'shot': ptr<storage, ShotData, read_write>
            35180..35212 'shot.q...p_mask': ref<storage, u32, read_write>
            35261..35406 '((1u <..._mask)': u32
            35262..35291 '(1u <<...) - 1u': u32
            35263..35265 '1u': u32
            35263..35285 '1u << ...COUNT)': u32
            35269..35285 'u32(QU...COUNT)': u32
            35273..35284 'QUBIT_COUNT': i32
            35289..35291 '1u': u32
            35360..35406 '~(shot..._mask)': u32
            35362..35366 'shot': ptr<storage, ShotData, read_write>
            35362..35382 'shot.q...0_mask': ref<storage, u32, read_write>
            35362..35405 'shot.q...1_mask': u32
            35385..35389 'shot': ptr<storage, ShotData, read_write>
            35385..35405 'shot.q...1_mask': ref<storage, u32, read_write>
            35659..35667 'shot_idx': u32
            35674..35680 'op_idx': u32
            35687..35692 'qubit': u32
            35699..35708 'result_id': u32
            35715..35722 'is_loss': bool
            35730..35743 'stores_result': bool
            35751..35765 'resets_to_zero': bool
            35783..35787 'shot': ptr<storage, ShotData, read_write>
            35790..35806 '&shots...t_idx]': ptr<storage, ShotData, read_write>
            35791..35796 'shots': ref<storage, array<ShotData>, read_write>
            35791..35806 'shots[shot_idx]': ref<storage, ShotData, read_write>
            35797..35805 'shot_idx': u32
            35897..35903 'result': [error]
            35906..35982 'select...ility)': [error]
            35913..35915 '1u': u32
            35917..35919 '0u': u32
            35921..35925 'shot': ptr<storage, ShotData, read_write>
            35921..35938 'shot.r...easure': ref<storage, f32, read_write>
            35921..35981 'shot.r...bility': [error]
            35941..35945 'shot': ptr<storage, ShotData, read_write>
            35941..35957 'shot.q..._state': ref<storage, [error], read_write>
            35941..35964 'shot.q...qubit]': [error]
            35941..35981 'shot.q...bility': [error]
            35958..35963 'qubit': u32
            36162..36170 '!is_loss': bool
            36163..36170 'is_loss': bool
            36184..36197 'stores_result': bool
            36418..36422 'shot': ptr<storage, ShotData, read_write>
            36418..36434 'shot.q..._state': ref<storage, [error], read_write>
            36418..36441 'shot.q...qubit]': [error]
            36418..36446 'shot.q...].heat': [error]
            36418..36454 'shot.q...= -1.0': [error]
            36435..36440 'qubit': u32
            36450..36454 '-1.0': float
            36451..36454 '1.0': float
            36473..36537 'atomic...], 2u)': [error]
            36485..36532 '&resul...lt_id]': ptr<storage, atomic<u32>, read_write>
            36486..36493 'results': ref<storage, array<atomic<u32>>, read_write>
            36486..36532 'result...lt_id]': ref<storage, atomic<u32>, read_write>
            36494..36531 '(shot_...ult_id': u32
            36495..36503 'shot_idx': u32
            36495..36518 'shot_i..._COUNT': u32
            36506..36518 'RESULT_COUNT': u32
            36522..36531 'result_id': u32
            36534..36536 '2u': u32
            36555..36559 'shot': ptr<storage, ShotData, read_write>
            36555..36567 'shot.op_type': ref<storage, u32, read_write>
            36570..36577 'OPID_ID': u32
            36595..36599 'shot': ptr<storage, ShotData, read_write>
            36595..36606 'shot.op_idx': ref<storage, u32, read_write>
            36609..36615 'op_idx': u32
            36720..36724 'shot': ptr<storage, ShotData, read_write>
            36720..36736 'shot.q..._state': ref<storage, [error], read_write>
            36720..36743 'shot.q...qubit]': [error]
            36720..36748 'shot.q...].heat': [error]
            36737..36742 'qubit': u32
            36751..36754 '0.0': float
            36817..36885 'atomic...esult)': [error]
            36829..36876 '&resul...lt_id]': ptr<storage, atomic<u32>, read_write>
            36830..36837 'results': ref<storage, array<atomic<u32>>, read_write>
            36830..36876 'result...lt_id]': ref<storage, atomic<u32>, read_write>
            36838..36875 '(shot_...ult_id': u32
            36839..36847 'shot_idx': u32
            36839..36862 'shot_i..._COUNT': u32
            36850..36862 'RESULT_COUNT': u32
            36866..36875 'result_id': u32
            36878..36884 'result': [error]
            37107..37111 'shot': ptr<storage, ShotData, read_write>
            37107..37123 'shot.q..._state': ref<storage, [error], read_write>
            37107..37130 'shot.q...qubit]': [error]
            37107..37135 'shot.q...].heat': [error]
            37107..37143 'shot.q...= -1.0': [error]
            37124..37129 'qubit': u32
            37139..37143 '-1.0': float
            37140..37143 '1.0': float
            37162..37166 'shot': ptr<storage, ShotData, read_write>
            37162..37174 'shot.op_type': ref<storage, u32, read_write>
            37177..37184 'OPID_ID': u32
            37202..37206 'shot': ptr<storage, ShotData, read_write>
            37202..37213 'shot.op_idx': ref<storage, u32, read_write>
            37216..37222 'op_idx': u32
            37293..37297 'shot': ptr<storage, ShotData, read_write>
            37293..37309 'shot.q..._state': ref<storage, [error], read_write>
            37293..37316 'shot.q...qubit]': [error]
            37293..37321 'shot.q...].heat': [error]
            37310..37315 'qubit': u32
            37324..37328 '-1.0': float
            37325..37328 '1.0': float
            37341..37411 'prep_m..._zero)': [error]
            37371..37379 'shot_idx': u32
            37381..37386 'qubit': u32
            37388..37394 'result': [error]
            37396..37410 'resets_to_zero': bool
            37418..37422 'shot': ptr<storage, ShotData, read_write>
            37418..37429 'shot.op_idx': ref<storage, u32, read_write>
            37432..37438 'op_idx': u32
            37597..37601 'shot': ptr<storage, ShotData, read_write>
            37597..37609 'shot.op_type': ref<storage, u32, read_write>
            37612..37624 'OPID_MRESETZ': u32
            38043..38051 'shot_idx': u32
            38058..38070 'target_is_q2': bool
            38104..38107 'm00': vec2<f32>
            38116..38119 'm01': vec2<f32>
            38128..38131 'm10': vec2<f32>
            38140..38143 'm11': vec2<f32>
            38162..38166 'shot': ptr<storage, ShotData, read_write>
            38169..38185 '&shots...t_idx]': ptr<storage, ShotData, read_write>
            38170..38175 'shots': ref<storage, array<ShotData>, read_write>
            38170..38185 'shots[shot_idx]': ref<storage, ShotData, read_write>
            38176..38184 'shot_idx': u32
            38233..38234 'i': ref<function, u32, read_write>
            38237..38239 '0u': u32
            38241..38242 'i': ref<function, u32, read_write>
            38241..38248 'i < 16u': bool
            38245..38248 '16u': u32
            38250..38251 'i': ref<function, u32, read_write>
            38265..38269 'shot': ptr<storage, ShotData, read_write>
            38265..38277 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            38265..38280 'shot.unitary[i]': ref<storage, vec2<f32>, read_write>
            38278..38279 'i': ref<function, u32, read_write>
            38283..38298 'vec2f(0.0, 0.0)': vec2<f32>
            38289..38292 '0.0': float
            38294..38297 '0.0': float
            38313..38325 'target_is_q2': bool
            38432..38436 'shot': ptr<storage, ShotData, read_write>
            38432..38444 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            38432..38447 'shot.unitary[0]': ref<storage, vec2<f32>, read_write>
            38445..38446 '0': integer
            38451..38454 'm00': vec2<f32>
            38456..38460 'shot': ptr<storage, ShotData, read_write>
            38456..38468 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            38456..38471 'shot.unitary[1]': ref<storage, vec2<f32>, read_write>
            38469..38470 '1': integer
            38475..38478 'm01': vec2<f32>
            38488..38492 'shot': ptr<storage, ShotData, read_write>
            38488..38500 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            38488..38503 'shot.unitary[4]': ref<storage, vec2<f32>, read_write>
            38501..38502 '4': integer
            38507..38510 'm10': vec2<f32>
            38512..38516 'shot': ptr<storage, ShotData, read_write>
            38512..38524 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            38512..38527 'shot.unitary[5]': ref<storage, vec2<f32>, read_write>
            38525..38526 '5': integer
            38531..38534 'm11': vec2<f32>
            38584..38588 'shot': ptr<storage, ShotData, read_write>
            38584..38596 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            38584..38600 'shot.u...ry[10]': ref<storage, vec2<f32>, read_write>
            38597..38599 '10': integer
            38603..38606 'm00': vec2<f32>
            38608..38612 'shot': ptr<storage, ShotData, read_write>
            38608..38620 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            38608..38624 'shot.u...ry[11]': ref<storage, vec2<f32>, read_write>
            38621..38623 '11': integer
            38627..38630 'm01': vec2<f32>
            38640..38644 'shot': ptr<storage, ShotData, read_write>
            38640..38652 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            38640..38656 'shot.u...ry[14]': ref<storage, vec2<f32>, read_write>
            38653..38655 '14': integer
            38659..38662 'm10': vec2<f32>
            38664..38668 'shot': ptr<storage, ShotData, read_write>
            38664..38676 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            38664..38680 'shot.u...ry[15]': ref<storage, vec2<f32>, read_write>
            38677..38679 '15': integer
            38683..38686 'm11': vec2<f32>
            38752..38756 'shot': ptr<storage, ShotData, read_write>
            38752..38764 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            38752..38767 'shot.unitary[0]': ref<storage, vec2<f32>, read_write>
            38765..38766 '0': integer
            38771..38774 'm00': vec2<f32>
            38776..38780 'shot': ptr<storage, ShotData, read_write>
            38776..38788 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            38776..38791 'shot.unitary[2]': ref<storage, vec2<f32>, read_write>
            38789..38790 '2': integer
            38795..38798 'm01': vec2<f32>
            38808..38812 'shot': ptr<storage, ShotData, read_write>
            38808..38820 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            38808..38823 'shot.unitary[8]': ref<storage, vec2<f32>, read_write>
            38821..38822 '8': integer
            38827..38830 'm10': vec2<f32>
            38832..38836 'shot': ptr<storage, ShotData, read_write>
            38832..38844 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            38832..38848 'shot.u...ry[10]': ref<storage, vec2<f32>, read_write>
            38845..38847 '10': integer
            38851..38854 'm11': vec2<f32>
            38864..38868 'shot': ptr<storage, ShotData, read_write>
            38864..38876 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            38864..38879 'shot.unitary[5]': ref<storage, vec2<f32>, read_write>
            38877..38878 '5': integer
            38883..38886 'm00': vec2<f32>
            38888..38892 'shot': ptr<storage, ShotData, read_write>
            38888..38900 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            38888..38903 'shot.unitary[7]': ref<storage, vec2<f32>, read_write>
            38901..38902 '7': integer
            38907..38910 'm01': vec2<f32>
            38920..38924 'shot': ptr<storage, ShotData, read_write>
            38920..38932 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            38920..38936 'shot.u...ry[13]': ref<storage, vec2<f32>, read_write>
            38933..38935 '13': integer
            38939..38942 'm10': vec2<f32>
            38944..38948 'shot': ptr<storage, ShotData, read_write>
            38944..38956 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            38944..38960 'shot.u...ry[15]': ref<storage, vec2<f32>, read_write>
            38957..38959 '15': integer
            38963..38966 'm11': vec2<f32>
            39285..39293 'shot_idx': u32
            39300..39303 'row': u32
            39320..39324 'shot': ptr<storage, ShotData, read_write>
            39327..39343 '&shots...t_idx]': ptr<storage, ShotData, read_write>
            39328..39333 'shots': ref<storage, array<ShotData>, read_write>
            39328..39343 'shots[shot_idx]': ref<storage, ShotData, read_write>
            39334..39342 'shot_idx': u32
            39358..39359 'c': ref<function, u32, read_write>
            39362..39364 '0u': u32
            39366..39367 'c': ref<function, u32, read_write>
            39366..39372 'c < 4u': bool
            39370..39372 '4u': u32
            39374..39375 'c': ref<function, u32, read_write>
            39393..39394 'e': vec2<f32>
            39397..39401 'shot': ptr<storage, ShotData, read_write>
            39397..39409 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            39397..39423 'shot.u...u + c]': ref<storage, vec2<f32>, read_write>
            39410..39413 'row': u32
            39410..39418 'row * 4u': u32
            39410..39422 'row * 4u + c': u32
            39416..39418 '4u': u32
            39421..39422 'c': ref<function, u32, read_write>
            39433..39437 'shot': ptr<storage, ShotData, read_write>
            39433..39445 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            39433..39459 'shot.u...u + c]': ref<storage, vec2<f32>, read_write>
            39446..39449 'row': u32
            39446..39454 'row * 4u': u32
            39446..39458 'row * 4u + c': u32
            39452..39454 '4u': u32
            39457..39458 'c': ref<function, u32, read_write>
            39462..39478 'vec2f(... -e.x)': vec2<f32>
            39468..39469 'e': vec2<f32>
            39468..39471 'e.y': f32
            39473..39477 '-e.x': f32
            39474..39475 'e': vec2<f32>
            39474..39477 'e.x': f32
            39594..39602 'shot_idx': u32
            39609..39615 'op_idx': u32
            39622..39624 'q1': u32
            39631..39633 'q2': u32
            39650..39654 'shot': ptr<storage, ShotData, read_write>
            39657..39673 '&shots...t_idx]': ptr<storage, ShotData, read_write>
            39658..39663 'shots': ref<storage, array<ShotData>, read_write>
            39658..39673 'shots[shot_idx]': ref<storage, ShotData, read_write>
            39664..39672 'shot_idx': u32
            39679..39683 'shot': ptr<storage, ShotData, read_write>
            39679..39690 'shot.op_idx': ref<storage, u32, read_write>
            39693..39699 'op_idx': u32
            39705..39709 'shot': ptr<storage, ShotData, read_write>
            39705..39717 'shot.op_type': ref<storage, u32, read_write>
            39720..39737 'OPID_S...UFF_2Q': u32
            39743..39747 'shot': ptr<storage, ShotData, read_write>
            39743..39775 'shot.q...p_mask': ref<storage, u32, read_write>
            39778..39801 '(1u <<...<< q2)': u32
            39779..39781 '1u': u32
            39779..39787 '1u << q1': u32
            39785..39787 'q1': u32
            39792..39794 '1u': u32
            39792..39800 '1u << q2': u32
            39798..39800 'q2': u32
            40003..40011 'shot_idx': u32
            40018..40024 'op_idx': u32
            40031..40033 'q1': u32
            40040..40042 'q2': u32
            40067..40071 'shot': ptr<storage, ShotData, read_write>
            40074..40090 '&shots...t_idx]': ptr<storage, ShotData, read_write>
            40075..40080 'shots': ref<storage, array<ShotData>, read_write>
            40075..40090 'shots[shot_idx]': ref<storage, ShotData, read_write>
            40081..40089 'shot_idx': u32
            40100..40102 'op': ptr<storage, Op, read>
            40105..40117 '&ops[op_idx]': ptr<storage, Op, read>
            40106..40109 'ops': ref<storage, array<Op>, read>
            40106..40117 'ops[op_idx]': ref<storage, Op, read>
            40110..40116 'op_idx': u32
            40127..40131 'shot': ptr<storage, ShotData, read_write>
            40127..40143 'shot.q..._state': ref<storage, [error], read_write>
            40127..40147 'shot.q...te[q1]': [error]
            40127..40152 'shot.q...].heat': [error]
            40127..40160 'shot.q...= -1.0': [error]
            40144..40146 'q1': u32
            40156..40160 '-1.0': float
            40157..40160 '1.0': float
            40179..40183 'true': bool
            40199..40204 'is_2q': bool
            40207..40223 '!is_1q...op.id)': bool
            40208..40223 'is_1q_op(op.id)': bool
            40217..40219 'op': ptr<storage, Op, read>
            40217..40222 'op.id': ref<storage, u32, read>
            40236..40241 'is_2q': bool
            40236..40280 'is_2q ... -1.0)': [error]
            40246..40250 'shot': ptr<storage, ShotData, read_write>
            40246..40262 'shot.q..._state': ref<storage, [error], read_write>
            40246..40266 'shot.q...te[q2]': [error]
            40246..40271 'shot.q...].heat': [error]
            40246..40279 'shot.q...= -1.0': [error]
            40263..40265 'q2': u32
            40275..40279 '-1.0': float
            40276..40279 '1.0': float
            40760..40768 'shot_idx': u32
            40775..40781 'op_idx': u32
            40788..40790 'q1': u32
            40797..40799 'q2': u32
            40806..40811 'qubit': u32
            40828..40832 'shot': ptr<storage, ShotData, read_write>
            40835..40851 '&shots...t_idx]': ptr<storage, ShotData, read_write>
            40836..40841 'shots': ref<storage, array<ShotData>, read_write>
            40836..40851 'shots[shot_idx]': ref<storage, ShotData, read_write>
            40842..40850 'shot_idx': u32
            40862..40868 'result': [error]
            40871..40947 'select...ility)': [error]
            40878..40880 '1u': u32
            40882..40884 '0u': u32
            40886..40890 'shot': ptr<storage, ShotData, read_write>
            40886..40903 'shot.r...easure': ref<storage, f32, read_write>
            40886..40946 'shot.r...bility': [error]
            40906..40910 'shot': ptr<storage, ShotData, read_write>
            40906..40922 'shot.q..._state': ref<storage, [error], read_write>
            40906..40929 'shot.q...qubit]': [error]
            40906..40946 'shot.q...bility': [error]
            40923..40928 'qubit': u32
            41103..41106 'm00': [error]
            41109..41163 'select...== 1u)': [error]
            41116..41131 'vec2f(1.0, 0.0)': vec2<f32>
            41122..41125 '1.0': float
            41127..41130 '0.0': float
            41133..41148 'vec2f(0.0, 0.0)': vec2<f32>
            41139..41142 '0.0': float
            41144..41147 '0.0': float
            41150..41156 'result': [error]
            41150..41162 'result == 1u': [error]
            41160..41162 '1u': u32
            41173..41176 'm01': [error]
            41179..41233 'select...== 1u)': [error]
            41186..41201 'vec2f(0.0, 0.0)': vec2<f32>
            41192..41195 '0.0': float
            41197..41200 '0.0': float
            41203..41218 'vec2f(1.0, 0.0)': vec2<f32>
            41209..41212 '1.0': float
            41214..41217 '0.0': float
            41220..41226 'result': [error]
            41220..41232 'result == 1u': [error]
            41230..41232 '1u': u32
            41243..41246 'm10': vec2<f32>
            41249..41264 'vec2f(0.0, 0.0)': vec2<f32>
            41255..41258 '0.0': float
            41260..41263 '0.0': float
            41274..41277 'm11': vec2<f32>
            41280..41295 'vec2f(0.0, 0.0)': vec2<f32>
            41286..41289 '0.0': float
            41291..41294 '0.0': float
            41306..41318 'target_is_q2': bool
            41322..41327 'qubit': u32
            41322..41333 'qubit == q2': bool
            41331..41333 'q2': u32
            41340..41406 'set_1q..., m11)': [error]
            41363..41371 'shot_idx': u32
            41373..41385 'target_is_q2': bool
            41387..41390 'm00': [error]
            41392..41395 'm01': [error]
            41397..41400 'm10': vec2<f32>
            41402..41405 'm11': vec2<f32>
            41468..41472 'shot': ptr<storage, ShotData, read_write>
            41468..41484 'shot.r...malize': ref<storage, f32, read_write>
            41487..41639 'select...== 1u)': [error]
            41503..41506 '1.0': float
            41503..41555 '1.0 / ...ility)': [error]
            41509..41555 'sqrt(s...ility)': [error]
            41514..41518 'shot': ptr<storage, ShotData, read_write>
            41514..41530 'shot.q..._state': ref<storage, [error], read_write>
            41514..41537 'shot.q...qubit]': [error]
            41514..41554 'shot.q...bility': [error]
            41531..41536 'qubit': u32
            41565..41568 '1.0': float
            41565..41616 '1.0 / ...ility)': [error]
            41571..41616 'sqrt(s...ility)': [error]
            41576..41580 'shot': ptr<storage, ShotData, read_write>
            41576..41592 'shot.q..._state': ref<storage, [error], read_write>
            41576..41599 'shot.q...qubit]': [error]
            41576..41615 'shot.q...bility': [error]
            41593..41598 'qubit': u32
            41626..41632 'result': [error]
            41626..41638 'result == 1u': [error]
            41636..41638 '1u': u32
            41753..41757 'shot': ptr<storage, ShotData, read_write>
            41753..41769 'shot.q..._state': ref<storage, [error], read_write>
            41753..41776 'shot.q...qubit]': [error]
            41753..41781 'shot.q...].heat': [error]
            41770..41775 'qubit': u32
            41784..41788 '-1.0': float
            41785..41788 '1.0': float
            41794..41798 'shot': ptr<storage, ShotData, read_write>
            41794..41814 'shot.q...0_mask': ref<storage, u32, read_write>
            41817..41821 'shot': ptr<storage, ShotData, read_write>
            41817..41837 'shot.q...0_mask': ref<storage, u32, read_write>
            41817..41854 'shot.q...qubit)': u32
            41840..41854 '~(1u << qubit)': u32
            41842..41844 '1u': u32
            41842..41853 '1u << qubit': u32
            41848..41853 'qubit': u32
            41860..41864 'shot': ptr<storage, ShotData, read_write>
            41860..41880 'shot.q...1_mask': ref<storage, u32, read_write>
            41883..41887 'shot': ptr<storage, ShotData, read_write>
            41883..41903 'shot.q...1_mask': ref<storage, u32, read_write>
            41883..41920 'shot.q...qubit)': u32
            41906..41920 '~(1u << qubit)': u32
            41908..41910 '1u': u32
            41908..41919 '1u << qubit': u32
            41914..41919 'qubit': u32
            41927..41974 'finish...1, q2)': [error]
            41949..41957 'shot_idx': u32
            41959..41965 'op_idx': u32
            41967..41969 'q1': u32
            41971..41973 'q2': u32
            42455..42463 'shot_idx': u32
            42470..42476 'op_idx': u32
            42483..42485 'q1': u32
            42492..42494 'q2': u32
            42511..42515 'shot': ptr<storage, ShotData, read_write>
            42518..42534 '&shots...t_idx]': ptr<storage, ShotData, read_write>
            42519..42524 'shots': ref<storage, array<ShotData>, read_write>
            42519..42534 'shots[shot_idx]': ref<storage, ShotData, read_write>
            42525..42533 'shot_idx': u32
            42544..42546 'op': ptr<storage, Op, read>
            42549..42561 '&ops[op_idx]': ptr<storage, Op, read>
            42550..42553 'ops': ref<storage, array<Op>, read>
            42550..42561 'ops[op_idx]': ref<storage, Op, read>
            42554..42560 'op_idx': u32
            42571..42576 'is_1q': bool
            42579..42594 'is_1q_op(op.id)': bool
            42588..42590 'op': ptr<storage, Op, read>
            42588..42593 'op.id': ref<storage, u32, read>
            42604..42609 'is_2q': bool
            42612..42618 '!is_1q': bool
            42613..42618 'is_1q': bool
            42628..42634 'policy': u32
            42637..42639 'op': ptr<storage, Op, read>
            42637..42646 'op.policy': ref<storage, u32, read>
            42774..42779 'is_1q': bool
            42791..42795 'shot': ptr<storage, ShotData, read_write>
            42791..42803 'shot.op_type': ref<storage, u32, read_write>
            42806..42813 'OPID_ID': u32
            42823..42827 'shot': ptr<storage, ShotData, read_write>
            42823..42834 'shot.op_idx': ref<storage, u32, read_write>
            42837..42843 'op_idx': u32
            42876..42883 'q1_lost': [error]
            42886..42890 'shot': ptr<storage, ShotData, read_write>
            42886..42902 'shot.q..._state': ref<storage, [error], read_write>
            42886..42906 'shot.q...te[q1]': [error]
            42886..42911 'shot.q...].heat': [error]
            42886..42919 'shot.q...= -1.0': [error]
            42903..42905 'q1': u32
            42915..42919 '-1.0': float
            42916..42919 '1.0': float
            42929..42936 'q2_lost': [error]
            42939..42944 'is_2q': bool
            42939..42983 'is_2q ... -1.0)': [error]
            42949..42953 'shot': ptr<storage, ShotData, read_write>
            42949..42965 'shot.q..._state': ref<storage, [error], read_write>
            42949..42969 'shot.q...te[q2]': [error]
            42949..42974 'shot.q...].heat': [error]
            42949..42982 'shot.q...= -1.0': [error]
            42966..42968 'q2': u32
            42978..42982 '-1.0': float
            42979..42982 '1.0': float
            42993..43005 'has_survivor': [error]
            43008..43013 'is_2q': bool
            43008..43038 'is_2q ..._lost)': [error]
            43017..43038 '!(q1_l..._lost)': [error]
            43019..43026 'q1_lost': [error]
            43019..43037 'q1_los...2_lost': [error]
            43030..43037 'q2_lost': [error]
            43122..43130 'survivor': [error]
            43133..43156 'select..._lost)': [error]
            43140..43142 'q1': u32
            43144..43146 'q2': u32
            43148..43155 'q1_lost': [error]
            43166..43180 'survivor_is_q2': [error]
            43183..43190 'q1_lost': [error]
            43423..43425 'op': ptr<storage, Op, read>
            43423..43428 'op.id': ref<storage, u32, read>
            43423..43441 'op.id ...D_SWAP': bool
            43432..43441 'OPID_SWAP': u32
            43460..43466 'policy': u32
            43486..43507 'LOSS_P...PAGATE': u32
            43526..43585 'propag...vivor)': [error]
            43550..43558 'shot_idx': u32
            43560..43566 'op_idx': u32
            43568..43570 'q1': u32
            43572..43574 'q2': u32
            43576..43584 'survivor': [error]
            43642..43671 'LOSS_P...DAGGER': u32
            44378..44386 'lost_row': [error]
            44389..44412 'select..._lost)': [error]
            44396..44398 '1u': u32
            44400..44402 '2u': u32
            44404..44411 'q1_lost': [error]
            44430..44481 'scale_...t_row)': [error]
            44462..44470 'shot_idx': u32
            44472..44480 'lost_row': [error]
            44499..44544 'scale_...x, 3u)': [error]
            44531..44539 'shot_idx': u32
            44541..44543 '3u': u32
            44646..44651 'heat1': [error]
            44654..44658 'shot': ptr<storage, ShotData, read_write>
            44654..44670 'shot.q..._state': ref<storage, [error], read_write>
            44654..44674 'shot.q...te[q1]': [error]
            44654..44679 'shot.q...].heat': [error]
            44671..44673 'q1': u32
            44697..44701 'shot': ptr<storage, ShotData, read_write>
            44697..44713 'shot.q..._state': ref<storage, [error], read_write>
            44697..44717 'shot.q...te[q1]': [error]
            44697..44722 'shot.q...].heat': [error]
            44714..44716 'q1': u32
            44725..44729 'shot': ptr<storage, ShotData, read_write>
            44725..44741 'shot.q..._state': ref<storage, [error], read_write>
            44725..44745 'shot.q...te[q2]': [error]
            44725..44750 'shot.q...].heat': [error]
            44742..44744 'q2': u32
            44768..44772 'shot': ptr<storage, ShotData, read_write>
            44768..44784 'shot.q..._state': ref<storage, [error], read_write>
            44768..44788 'shot.q...te[q2]': [error]
            44768..44793 'shot.q...].heat': [error]
            44785..44787 'q2': u32
            44796..44801 'heat1': [error]
            45082..45086 'shot': ptr<storage, ShotData, read_write>
            45082..45102 'shot.q...0_mask': ref<storage, u32, read_write>
            45105..45109 'shot': ptr<storage, ShotData, read_write>
            45105..45125 'shot.q...0_mask': ref<storage, u32, read_write>
            45105..45154 'shot.q...< q2))': u32
            45128..45154 '~((1u ...< q2))': u32
            45130..45153 '(1u <<...<< q2)': u32
            45131..45133 '1u': u32
            45131..45139 '1u << q1': u32
            45137..45139 'q1': u32
            45144..45146 '1u': u32
            45144..45152 '1u << q2': u32
            45150..45152 'q2': u32
            45172..45176 'shot': ptr<storage, ShotData, read_write>
            45172..45192 'shot.q...1_mask': ref<storage, u32, read_write>
            45195..45199 'shot': ptr<storage, ShotData, read_write>
            45195..45215 'shot.q...1_mask': ref<storage, u32, read_write>
            45195..45244 'shot.q...< q2))': u32
            45218..45244 '~((1u ...< q2))': u32
            45220..45243 '(1u <<...<< q2)': u32
            45221..45223 '1u': u32
            45221..45229 '1u << q1': u32
            45227..45229 'q1': u32
            45234..45236 '1u': u32
            45234..45242 '1u << q2': u32
            45240..45242 'q2': u32
            45331..45378 'finish...1, q2)': [error]
            45353..45361 'shot_idx': u32
            45363..45369 'op_idx': u32
            45371..45373 'q1': u32
            45375..45377 'q2': u32
            45435..45459 'LOSS_P...ANYWAY': u32
            45562..45567 'heat1': [error]
            45570..45574 'shot': ptr<storage, ShotData, read_write>
            45570..45586 'shot.q..._state': ref<storage, [error], read_write>
            45570..45590 'shot.q...te[q1]': [error]
            45570..45595 'shot.q...].heat': [error]
            45587..45589 'q1': u32
            45613..45617 'shot': ptr<storage, ShotData, read_write>
            45613..45629 'shot.q..._state': ref<storage, [error], read_write>
            45613..45633 'shot.q...te[q1]': [error]
            45613..45638 'shot.q...].heat': [error]
            45630..45632 'q1': u32
            45641..45645 'shot': ptr<storage, ShotData, read_write>
            45641..45657 'shot.q..._state': ref<storage, [error], read_write>
            45641..45661 'shot.q...te[q2]': [error]
            45641..45666 'shot.q...].heat': [error]
            45658..45660 'q2': u32
            45684..45688 'shot': ptr<storage, ShotData, read_write>
            45684..45700 'shot.q..._state': ref<storage, [error], read_write>
            45684..45704 'shot.q...te[q2]': [error]
            45684..45709 'shot.q...].heat': [error]
            45701..45703 'q2': u32
            45712..45717 'heat1': [error]
            45998..46002 'shot': ptr<storage, ShotData, read_write>
            45998..46018 'shot.q...0_mask': ref<storage, u32, read_write>
            46021..46025 'shot': ptr<storage, ShotData, read_write>
            46021..46041 'shot.q...0_mask': ref<storage, u32, read_write>
            46021..46070 'shot.q...< q2))': u32
            46044..46070 '~((1u ...< q2))': u32
            46046..46069 '(1u <<...<< q2)': u32
            46047..46049 '1u': u32
            46047..46055 '1u << q1': u32
            46053..46055 'q1': u32
            46060..46062 '1u': u32
            46060..46068 '1u << q2': u32
            46066..46068 'q2': u32
            46088..46092 'shot': ptr<storage, ShotData, read_write>
            46088..46108 'shot.q...1_mask': ref<storage, u32, read_write>
            46111..46115 'shot': ptr<storage, ShotData, read_write>
            46111..46131 'shot.q...1_mask': ref<storage, u32, read_write>
            46111..46160 'shot.q...< q2))': u32
            46134..46160 '~((1u ...< q2))': u32
            46136..46159 '(1u <<...<< q2)': u32
            46137..46139 '1u': u32
            46137..46145 '1u << q1': u32
            46143..46145 'q1': u32
            46150..46152 '1u': u32
            46150..46158 '1u << q2': u32
            46156..46158 'q2': u32
            46261..46308 'finish...1, q2)': [error]
            46283..46291 'shot_idx': u32
            46293..46299 'op_idx': u32
            46301..46303 'q1': u32
            46305..46307 'q2': u32
            46365..46381 'LOSS_P...Y_SKIP': u32
            46400..46404 'shot': ptr<storage, ShotData, read_write>
            46400..46412 'shot.op_type': ref<storage, u32, read_write>
            46415..46422 'OPID_ID': u32
            46440..46444 'shot': ptr<storage, ShotData, read_write>
            46440..46451 'shot.op_idx': ref<storage, u32, read_write>
            46454..46460 'op_idx': u32
            46767..46823 'report...OLICY)': [error]
            46785..46793 'shot_idx': u32
            46795..46822 'ERR_UN...POLICY': u32
            46841..46845 'shot': ptr<storage, ShotData, read_write>
            46841..46853 'shot.op_type': ref<storage, u32, read_write>
            46856..46863 'OPID_ID': u32
            46881..46885 'shot': ptr<storage, ShotData, read_write>
            46881..46892 'shot.op_idx': ref<storage, u32, read_write>
            46895..46901 'op_idx': u32
            47129..47135 'policy': u32
            47129..47163 'policy...ANYWAY': bool
            47139..47163 'LOSS_P...ANYWAY': u32
            47175..47231 'report...OLICY)': [error]
            47193..47201 'shot_idx': u32
            47203..47230 'ERR_UN...POLICY': u32
            47241..47245 'shot': ptr<storage, ShotData, read_write>
            47241..47253 'shot.op_type': ref<storage, u32, read_write>
            47256..47263 'OPID_ID': u32
            47273..47277 'shot': ptr<storage, ShotData, read_write>
            47273..47284 'shot.op_idx': ref<storage, u32, read_write>
            47287..47293 'op_idx': u32
            47326..47332 'policy': u32
            47326..47357 'policy...PAGATE': bool
            47326..47373 'policy...rvivor': [error]
            47336..47357 'LOSS_P...PAGATE': u32
            47361..47373 'has_survivor': [error]
            47385..47444 'propag...vivor)': [error]
            47409..47417 'shot_idx': u32
            47419..47425 'op_idx': u32
            47427..47429 'q1': u32
            47431..47433 'q2': u32
            47435..47443 'survivor': [error]
            47477..47483 'policy': u32
            47477..47516 'policy...DAGGER': bool
            47477..47532 'policy...rvivor': [error]
            47487..47516 'LOSS_P...DAGGER': u32
            47520..47532 'has_survivor': [error]
            47610..47751 'set_1q...-1.0))': [error]
            47633..47641 'shot_idx': u32
            47643..47657 'survivor_is_q2': [error]
            47671..47686 'vec2f(1.0, 0.0)': vec2<f32>
            47677..47680 '1.0': float
            47682..47685 '0.0': float
            47688..47703 'vec2f(0.0, 0.0)': vec2<f32>
            47694..47697 '0.0': float
            47699..47702 '0.0': float
            47717..47732 'vec2f(0.0, 0.0)': vec2<f32>
            47723..47726 '0.0': float
            47728..47731 '0.0': float
            47734..47750 'vec2f(... -1.0)': vec2<f32>
            47740..47743 '0.0': float
            47745..47749 '-1.0': float
            47746..47749 '1.0': float
            47761..47808 'finish...1, q2)': [error]
            47783..47791 'shot_idx': u32
            47793..47799 'op_idx': u32
            47801..47803 'q1': u32
            47805..47807 'q2': u32
            47986..47992 'policy': u32
            47986..48015 'policy...EGRADE': bool
            47986..48031 'policy...rvivor': [error]
            47996..48015 'LOSS_P...EGRADE': u32
            48019..48031 'has_survivor': [error]
            48264..48272 'cos_half': f32
            48275..48277 'op': ptr<storage, Op, read>
            48275..48285 'op.unitary': ref<storage, array<vec2<f32>, 16>, read>
            48275..48288 'op.unitary[0]': ref<storage, vec2<f32>, read>
            48275..48290 'op.unitary[0].x': ref<storage, f32, read>
            48286..48287 '0': integer
            48304..48306 'op': ptr<storage, Op, read>
            48304..48309 'op.id': ref<storage, u32, read>
            48304..48321 'op.id ...ID_RXX': bool
            48313..48321 'OPID_RXX': u32
            48410..48411 's': f32
            48414..48416 'op': ptr<storage, Op, read>
            48414..48424 'op.unitary': ref<storage, array<vec2<f32>, 16>, read>
            48414..48427 'op.unitary[3]': ref<storage, vec2<f32>, read>
            48414..48429 'op.unitary[3].y': ref<storage, f32, read>
            48414..48436 'op.uni...* -1.0': f32
            48425..48426 '3': integer
            48432..48436 '-1.0': float
            48433..48436 '1.0': float
            48482..48638 'set_1q... 0.0))': [error]
            48505..48513 'shot_idx': u32
            48515..48529 'survivor_is_q2': [error]
            48547..48567 'vec2f(..., 0.0)': vec2<f32>
            48553..48561 'cos_half': f32
            48563..48566 '0.0': float
            48569..48583 'vec2f(0.0, -s)': vec2<f32>
            48575..48578 '0.0': float
            48580..48582 '-s': f32
            48581..48582 's': f32
            48601..48615 'vec2f(0.0, -s)': vec2<f32>
            48607..48610 '0.0': float
            48612..48614 '-s': f32
            48613..48614 's': f32
            48617..48637 'vec2f(..., 0.0)': vec2<f32>
            48623..48631 'cos_half': f32
            48633..48636 '0.0': float
            48760..48761 's': f32
            48764..48766 'op': ptr<storage, Op, read>
            48764..48774 'op.unitary': ref<storage, array<vec2<f32>, 16>, read>
            48764..48777 'op.unitary[3]': ref<storage, vec2<f32>, read>
            48764..48779 'op.unitary[3].y': ref<storage, f32, read>
            48775..48776 '3': integer
            48832..48987 'set_1q... 0.0))': [error]
            48855..48863 'shot_idx': u32
            48865..48879 'survivor_is_q2': [error]
            48897..48917 'vec2f(..., 0.0)': vec2<f32>
            48903..48911 'cos_half': f32
            48913..48916 '0.0': float
            48919..48933 'vec2f(-s, 0.0)': vec2<f32>
            48925..48927 '-s': f32
            48926..48927 's': f32
            48929..48932 '0.0': float
            48951..48964 'vec2f(s, 0.0)': vec2<f32>
            48957..48958 's': f32
            48960..48963 '0.0': float
            48966..48986 'vec2f(..., 0.0)': vec2<f32>
            48972..48980 'cos_half': f32
            48982..48985 '0.0': float
            49169..49174 'phase': vec2<f32>
            49177..49179 'op': ptr<storage, Op, read>
            49177..49187 'op.unitary': ref<storage, array<vec2<f32>, 16>, read>
            49177..49190 'op.unitary[5]': ref<storage, vec2<f32>, read>
            49188..49189 '5': integer
            49204..49342 'set_1q...phase)': [error]
            49227..49235 'shot_idx': u32
            49237..49251 'survivor_is_q2': [error]
            49269..49284 'vec2f(1.0, 0.0)': vec2<f32>
            49275..49278 '1.0': float
            49280..49283 '0.0': float
            49286..49301 'vec2f(0.0, 0.0)': vec2<f32>
            49292..49295 '0.0': float
            49297..49300 '0.0': float
            49319..49334 'vec2f(0.0, 0.0)': vec2<f32>
            49325..49328 '0.0': float
            49330..49333 '0.0': float
            49336..49341 'phase': vec2<f32>
            49362..49409 'finish...1, q2)': [error]
            49384..49392 'shot_idx': u32
            49394..49400 'op_idx': u32
            49402..49404 'q1': u32
            49406..49408 'q2': u32
            49549..49553 'shot': ptr<storage, ShotData, read_write>
            49549..49561 'shot.op_type': ref<storage, u32, read_write>
            49564..49571 'OPID_ID': u32
            49577..49581 'shot': ptr<storage, ShotData, read_write>
            49577..49588 'shot.op_idx': ref<storage, u32, read_write>
            49591..49597 'op_idx': u32
            49948..49956 'shot_idx': u32
            49963..49967 'code': u32
            49980..50040 'atomic... code)': __atomic_compare_exchange_result
            50006..50029 '&diagn...r_code': ptr<storage, atomic<u32>, read_write>
            50007..50018 'diagnostics': ref<storage, DiagnosticData, read_write>
            50007..50029 'diagno...r_code': ref<storage, atomic<u32>, read_write>
            50031..50033 '0u': u32
            50035..50039 'code': u32
            50050..50059 'err_index': u32
            50062..50092 '(shot_..._COUNT': u32
            50062..50097 '(shot_...T - 1u': u32
            50063..50071 'shot_idx': u32
            50063..50076 'shot_idx + 1u': u32
            50074..50076 '1u': u32
            50080..50092 'RESULT_COUNT': u32
            50095..50097 '1u': u32
            50103..50159 'atomic... code)': __atomic_compare_exchange_result
            50129..50148 '&resul...index]': ptr<storage, atomic<u32>, read_write>
            50130..50137 'results': ref<storage, array<atomic<u32>>, read_write>
            50130..50148 'result...index]': ref<storage, atomic<u32>, read_write>
            50138..50147 'err_index': u32
            50150..50152 '0u': u32
            50154..50158 'code': u32
            50315..50321 'op_idx': u32
            50345..50362 'arrayL...(&ops)': u32
            50345..50377 'arrayL...x + 1)': bool
            50357..50361 '&ops': ptr<storage, array<Op>, read>
            50358..50361 'ops': ref<storage, array<Op>, read>
            50366..50372 'op_idx': u32
            50366..50376 'op_idx + 1': u32
            50375..50376 '1': integer
            50393..50395 'op': ptr<storage, Op, read>
            50398..50414 '&ops[o...x + 1]': ptr<storage, Op, read>
            50399..50402 'ops': ref<storage, array<Op>, read>
            50399..50414 'ops[op_idx + 1]': ref<storage, Op, read>
            50403..50409 'op_idx': u32
            50403..50413 'op_idx + 1': u32
            50412..50413 '1': integer
            50428..50430 'op': ptr<storage, Op, read>
            50428..50433 'op.id': ref<storage, u32, read>
            50428..50456 'op.id ...ISE_1Q': bool
            50428..50488 'op.id ...ISE_2Q': bool
            50437..50456 'OPID_P...ISE_1Q': u32
            50460..50462 'op': ptr<storage, Op, read>
            50460..50465 'op.id': ref<storage, u32, read>
            50460..50488 'op.id ...ISE_2Q': bool
            50469..50488 'OPID_P...ISE_2Q': u32
            50511..50517 'op_idx': u32
            50511..50522 'op_idx + 1u': u32
            50520..50522 '1u': u32
            50551..50553 '0u': u32
            50582..50590 'shot_idx': u32
            50597..50603 'op_idx': u32
            50610..50619 'noise_idx': u32
            50626..50628 'q1': u32
            50901..50905 'shot': ptr<storage, ShotData, read_write>
            50908..50924 '&shots...t_idx]': ptr<storage, ShotData, read_write>
            50909..50914 'shots': ref<storage, array<ShotData>, read_write>
            50909..50924 'shots[shot_idx]': ref<storage, ShotData, read_write>
            50915..50923 'shot_idx': u32
            50934..50936 'op': ptr<storage, Op, read>
            50939..50951 '&ops[op_idx]': ptr<storage, Op, read>
            50940..50943 'ops': ref<storage, array<Op>, read>
            50940..50951 'ops[op_idx]': ref<storage, Op, read>
            50944..50950 'op_idx': u32
            50961..50969 'noise_op': ptr<storage, Op, read>
            50972..50987 '&ops[noise_idx]': ptr<storage, Op, read>
            50973..50976 'ops': ref<storage, array<Op>, read>
            50973..50987 'ops[noise_idx]': ref<storage, Op, read>
            50977..50986 'noise_idx': u32
            51189..51192 'p_x': f32
            51195..51203 'noise_op': ptr<storage, Op, read>
            51195..51211 'noise_...nitary': ref<storage, array<vec2<f32>, 16>, read>
            51195..51214 'noise_...ary[0]': ref<storage, vec2<f32>, read>
            51195..51216 'noise_...y[0].y': ref<storage, f32, read>
            51212..51213 '0': integer
            51226..51229 'p_z': f32
            51232..51240 'noise_op': ptr<storage, Op, read>
            51232..51248 'noise_...nitary': ref<storage, array<vec2<f32>, 16>, read>
            51232..51251 'noise_...ary[1]': ref<storage, vec2<f32>, read>
            51232..51253 'noise_...y[1].x': ref<storage, f32, read>
            51249..51250 '1': integer
            51263..51266 'p_y': f32
            51269..51277 'noise_op': ptr<storage, Op, read>
            51269..51285 'noise_...nitary': ref<storage, array<vec2<f32>, 16>, read>
            51269..51288 'noise_...ary[1]': ref<storage, vec2<f32>, read>
            51269..51290 'noise_...y[1].y': ref<storage, f32, read>
            51286..51287 '1': integer
            51300..51306 'p_loss': f32
            51309..51317 'noise_op': ptr<storage, Op, read>
            51309..51325 'noise_...nitary': ref<storage, array<vec2<f32>, 16>, read>
            51309..51328 'noise_...ary[2]': ref<storage, vec2<f32>, read>
            51309..51330 'noise_...y[2].x': ref<storage, f32, read>
            51326..51327 '2': integer
            51337..51341 'shot': ptr<storage, ShotData, read_write>
            51337..51349 'shot.op_type': ref<storage, u32, read_write>
            51352..51369 'OPID_S...UFF_1Q': u32
            51429..51433 'rand': f32
            51436..51440 'shot': ptr<storage, ShotData, read_write>
            51436..51451 'shot.rand_pauli': ref<storage, f32, read_write>
            51461..51465 'rand': f32
            51461..51471 'rand < p_x': bool
            51468..51471 'p_x': f32
            51544..51548 'shot': ptr<storage, ShotData, read_write>
            51544..51556 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            51544..51559 'shot.unitary[0]': ref<storage, vec2<f32>, read_write>
            51557..51558 '0': integer
            51562..51564 'op': ptr<storage, Op, read>
            51562..51572 'op.unitary': ref<storage, array<vec2<f32>, 16>, read>
            51562..51575 'op.unitary[4]': ref<storage, vec2<f32>, read>
            51573..51574 '4': integer
            51585..51589 'shot': ptr<storage, ShotData, read_write>
            51585..51597 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            51585..51600 'shot.unitary[1]': ref<storage, vec2<f32>, read_write>
            51598..51599 '1': integer
            51603..51605 'op': ptr<storage, Op, read>
            51603..51613 'op.unitary': ref<storage, array<vec2<f32>, 16>, read>
            51603..51616 'op.unitary[5]': ref<storage, vec2<f32>, read>
            51614..51615 '5': integer
            51626..51630 'shot': ptr<storage, ShotData, read_write>
            51626..51638 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            51626..51641 'shot.unitary[4]': ref<storage, vec2<f32>, read_write>
            51639..51640 '4': integer
            51644..51646 'op': ptr<storage, Op, read>
            51644..51654 'op.unitary': ref<storage, array<vec2<f32>, 16>, read>
            51644..51657 'op.unitary[0]': ref<storage, vec2<f32>, read>
            51655..51656 '0': integer
            51667..51671 'shot': ptr<storage, ShotData, read_write>
            51667..51679 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            51667..51682 'shot.unitary[5]': ref<storage, vec2<f32>, read_write>
            51680..51681 '5': integer
            51685..51687 'op': ptr<storage, Op, read>
            51685..51695 'op.unitary': ref<storage, array<vec2<f32>, 16>, read>
            51685..51698 'op.unitary[1]': ref<storage, vec2<f32>, read>
            51696..51697 '1': integer
            51815..51819 'shot': ptr<storage, ShotData, read_write>
            51815..51827 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            51815..51830 'shot.unitary[0]': ref<storage, vec2<f32>, read_write>
            51828..51829 '0': integer
            51833..51855 'cplxNe...ry[4])': vec2<f32>
            51841..51843 'op': ptr<storage, Op, read>
            51841..51851 'op.unitary': ref<storage, array<vec2<f32>, 16>, read>
            51841..51854 'op.unitary[4]': ref<storage, vec2<f32>, read>
            51852..51853 '4': integer
            51865..51869 'shot': ptr<storage, ShotData, read_write>
            51865..51877 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            51865..51880 'shot.unitary[1]': ref<storage, vec2<f32>, read_write>
            51878..51879 '1': integer
            51883..51905 'cplxNe...ry[5])': vec2<f32>
            51891..51893 'op': ptr<storage, Op, read>
            51891..51901 'op.unitary': ref<storage, array<vec2<f32>, 16>, read>
            51891..51904 'op.unitary[5]': ref<storage, vec2<f32>, read>
            51902..51903 '5': integer
            51915..51919 'shot': ptr<storage, ShotData, read_write>
            51915..51927 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            51915..51930 'shot.unitary[4]': ref<storage, vec2<f32>, read_write>
            51928..51929 '4': integer
            51933..51935 'op': ptr<storage, Op, read>
            51933..51943 'op.unitary': ref<storage, array<vec2<f32>, 16>, read>
            51933..51946 'op.unitary[0]': ref<storage, vec2<f32>, read>
            51944..51945 '0': integer
            51956..51960 'shot': ptr<storage, ShotData, read_write>
            51956..51968 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            51956..51971 'shot.unitary[5]': ref<storage, vec2<f32>, read_write>
            51969..51970 '5': integer
            51974..51976 'op': ptr<storage, Op, read>
            51974..51984 'op.unitary': ref<storage, array<vec2<f32>, 16>, read>
            51974..51987 'op.unitary[1]': ref<storage, vec2<f32>, read>
            51985..51986 '1': integer
            52084..52088 'shot': ptr<storage, ShotData, read_write>
            52084..52096 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            52084..52099 'shot.unitary[0]': ref<storage, vec2<f32>, read_write>
            52097..52098 '0': integer
            52102..52104 'op': ptr<storage, Op, read>
            52102..52112 'op.unitary': ref<storage, array<vec2<f32>, 16>, read>
            52102..52115 'op.unitary[0]': ref<storage, vec2<f32>, read>
            52113..52114 '0': integer
            52125..52129 'shot': ptr<storage, ShotData, read_write>
            52125..52137 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            52125..52140 'shot.unitary[1]': ref<storage, vec2<f32>, read_write>
            52138..52139 '1': integer
            52143..52145 'op': ptr<storage, Op, read>
            52143..52153 'op.unitary': ref<storage, array<vec2<f32>, 16>, read>
            52143..52156 'op.unitary[1]': ref<storage, vec2<f32>, read>
            52154..52155 '1': integer
            52166..52170 'shot': ptr<storage, ShotData, read_write>
            52166..52178 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            52166..52181 'shot.unitary[4]': ref<storage, vec2<f32>, read_write>
            52179..52180 '4': integer
            52184..52206 'cplxNe...ry[4])': vec2<f32>
            52192..52194 'op': ptr<storage, Op, read>
            52192..52202 'op.unitary': ref<storage, array<vec2<f32>, 16>, read>
            52192..52205 'op.unitary[4]': ref<storage, vec2<f32>, read>
            52203..52204 '4': integer
            52216..52220 'shot': ptr<storage, ShotData, read_write>
            52216..52228 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            52216..52231 'shot.unitary[5]': ref<storage, vec2<f32>, read_write>
            52229..52230 '5': integer
            52234..52256 'cplxNe...ry[5])': vec2<f32>
            52242..52244 'op': ptr<storage, Op, read>
            52242..52252 'op.unitary': ref<storage, array<vec2<f32>, 16>, read>
            52242..52255 'op.unitary[5]': ref<storage, vec2<f32>, read>
            52253..52254 '5': integer
            52488..52492 'rand': f32
            52488..52521 'rand <..._loss)': bool
            52496..52499 'p_x': f32
            52496..52505 'p_x + p_z': f32
            52496..52511 'p_x + p_z + p_y': f32
            52496..52520 'p_x + ...p_loss': f32
            52502..52505 'p_z': f32
            52508..52511 'p_y': f32
            52514..52520 'p_loss': f32
            52537..52541 'shot': ptr<storage, ShotData, read_write>
            52537..52559 'shot.p...s_mask': ref<storage, u32, read_write>
            52564..52566 '1u': u32
            52564..52572 '1u << q1': u32
            52570..52572 'q1': u32
            52738..52740 'op': ptr<storage, Op, read>
            52738..52743 'op.id': ref<storage, u32, read>
            52738..52754 'op.id ...PID_ID': bool
            52738..52779 'op.id ...RESETZ': bool
            52738..52799 'op.id ...PID_MZ': bool
            52738..52823 'op.id ...RESETZ': bool
            52747..52754 'OPID_ID': u32
            52758..52760 'op': ptr<storage, Op, read>
            52758..52763 'op.id': ref<storage, u32, read>
            52758..52779 'op.id ...RESETZ': bool
            52767..52779 'OPID_MRESETZ': u32
            52783..52785 'op': ptr<storage, Op, read>
            52783..52788 'op.id': ref<storage, u32, read>
            52783..52799 'op.id ...PID_MZ': bool
            52792..52799 'OPID_MZ': u32
            52803..52805 'op': ptr<storage, Op, read>
            52803..52808 'op.id': ref<storage, u32, read>
            52803..52823 'op.id ...RESETZ': bool
            52812..52823 'OPID_RESETZ': u32
            52839..52843 'shot': ptr<storage, ShotData, read_write>
            52839..52851 'shot.op_type': ref<storage, u32, read_write>
            52854..52856 'op': ptr<storage, Op, read>
            52854..52859 'op.id': ref<storage, u32, read>
            52883..52906 'is_1q_...op.id)': bool
            52900..52902 'op': ptr<storage, Op, read>
            52900..52905 'op.id': ref<storage, u32, read>
            53000..53004 'shot': ptr<storage, ShotData, read_write>
            53000..53012 'shot.op_type': ref<storage, u32, read_write>
            53015..53022 'OPID_RZ': u32
            53045..53049 'shot': ptr<storage, ShotData, read_write>
            53045..53056 'shot.op_idx': ref<storage, u32, read_write>
            53059..53065 'op_idx': u32
            53075..53079 'shot': ptr<storage, ShotData, read_write>
            53075..53087 'shot.op_type': ref<storage, u32, read_write>
            53075..53098 'shot.o...PID_ID': bool
            53075..53125 'shot.o...PID_RZ': bool
            53091..53098 'OPID_ID': u32
            53102..53106 'shot': ptr<storage, ShotData, read_write>
            53102..53114 'shot.op_type': ref<storage, u32, read_write>
            53102..53125 'shot.o...PID_RZ': bool
            53118..53125 'OPID_RZ': u32
            53137..53141 'shot': ptr<storage, ShotData, read_write>
            53137..53169 'shot.q...p_mask': ref<storage, u32, read_write>
            53172..53174 '0u': u32
            53197..53201 'shot': ptr<storage, ShotData, read_write>
            53197..53229 'shot.q...p_mask': ref<storage, u32, read_write>
            53232..53234 '1u': u32
            53232..53240 '1u << q1': u32
            53238..53240 'q1': u32
            53276..53284 'shot_idx': u32
            53291..53297 'op_idx': u32
            53304..53313 'noise_idx': u32
            53320..53322 'q1': u32
            53329..53331 'q2': u32
            53348..53352 'shot': ptr<storage, ShotData, read_write>
            53355..53371 '&shots...t_idx]': ptr<storage, ShotData, read_write>
            53356..53361 'shots': ref<storage, array<ShotData>, read_write>
            53356..53371 'shots[shot_idx]': ref<storage, ShotData, read_write>
            53362..53370 'shot_idx': u32
            53381..53383 'op': ptr<storage, Op, read>
            53386..53398 '&ops[op_idx]': ptr<storage, Op, read>
            53387..53390 'ops': ref<storage, array<Op>, read>
            53387..53398 'ops[op_idx]': ref<storage, Op, read>
            53391..53397 'op_idx': u32
            53408..53416 'noise_op': ptr<storage, Op, read>
            53419..53434 '&ops[noise_idx]': ptr<storage, Op, read>
            53420..53423 'ops': ref<storage, array<Op>, read>
            53420..53434 'ops[noise_idx]': ref<storage, Op, read>
            53424..53433 'noise_idx': u32
            53741..53745 'rand': ref<function, f32, read_write>
            53748..53752 'shot': ptr<storage, ShotData, read_write>
            53748..53763 'shot.rand_pauli': ref<storage, f32, read_write>
            53773..53780 'q1_term': ref<function, i32, read_write>
            53783..53784 '0': integer
            53794..53801 'q2_term': ref<function, i32, read_write>
            53804..53805 '0': integer
            53901..53902 'a': ref<function, i32, read_write>
            53905..53906 '0': integer
            53908..53909 'a': ref<function, i32, read_write>
            53908..53913 'a < 5': bool
            53912..53913 '5': integer
            53915..53916 'a': ref<function, i32, read_write>
            53919..53920 'a': ref<function, i32, read_write>
            53919..53924 'a + 1': i32
            53923..53924 '1': integer
            53945..53946 'b': ref<function, i32, read_write>
            53949..53950 '0': integer
            53952..53953 'b': ref<function, i32, read_write>
            53952..53957 'b < 5': bool
            53956..53957 '5': integer
            53959..53960 'b': ref<function, i32, read_write>
            53963..53964 'b': ref<function, i32, read_write>
            53963..53968 'b + 1': i32
            53967..53968 '1': integer
            53988..53989 'k': i32
            53992..53993 'a': ref<function, i32, read_write>
            53992..53997 'a * 5': i32
            53992..54001 'a * 5 + b': i32
            53996..53997 '5': integer
            54000..54001 'b': ref<function, i32, read_write>
            54019..54020 'k': i32
            54019..54025 'k == 0': bool
            54024..54025 '0': integer
            54093..54097 'slot': vec2<f32>
            54100..54108 'noise_op': ptr<storage, Op, read>
            54100..54116 'noise_...nitary': ref<storage, array<vec2<f32>, 16>, read>
            54100..54123 'noise_...k / 2]': ref<storage, vec2<f32>, read>
            54117..54118 'k': i32
            54117..54122 'k / 2': i32
            54121..54122 '2': integer
            54141..54145 'p_ab': f32
            54148..54184 'select... == 1)': f32
            54155..54159 'slot': vec2<f32>
            54155..54161 'slot.x': f32
            54163..54167 'slot': vec2<f32>
            54163..54169 'slot.y': f32
            54171..54183 '(k & 1) == 1': bool
            54172..54173 'k': i32
            54172..54177 'k & 1': i32
            54176..54177 '1': integer
            54182..54183 '1': integer
            54202..54206 'rand': ref<function, f32, read_write>
            54202..54213 'rand < p_ab': bool
            54209..54213 'p_ab': f32
            54233..54240 'q1_term': ref<function, i32, read_write>
            54243..54244 'a': ref<function, i32, read_write>
            54262..54269 'q2_term': ref<function, i32, read_write>
            54272..54273 'b': ref<function, i32, read_write>
            54334..54335 'a': ref<function, i32, read_write>
            54338..54339 '5': integer
            54357..54358 'b': ref<function, i32, read_write>
            54361..54362 '5': integer
            54401..54405 'rand': ref<function, f32, read_write>
            54408..54412 'rand': ref<function, f32, read_write>
            54408..54419 'rand - p_ab': f32
            54415..54419 'p_ab': f32
            54603..54610 'q1_term': ref<function, i32, read_write>
            54603..54615 'q1_term == 4': bool
            54614..54615 '4': integer
            54619..54623 'shot': ptr<storage, ShotData, read_write>
            54619..54641 'shot.p...s_mask': ref<storage, u32, read_write>
            54646..54648 '1u': u32
            54646..54654 '1u << q1': u32
            54652..54654 'q1': u32
            54667..54674 'q2_term': ref<function, i32, read_write>
            54667..54679 'q2_term == 4': bool
            54678..54679 '4': integer
            54683..54687 'shot': ptr<storage, ShotData, read_write>
            54683..54705 'shot.p...s_mask': ref<storage, u32, read_write>
            54710..54712 '1u': u32
            54710..54718 '1u << q2': u32
            54716..54718 'q2': u32
            54894..54902 'q1_pauli': bool
            54905..54912 'q1_term': ref<function, i32, read_write>
            54905..54917 'q1_term >= 1': bool
            54905..54933 'q1_ter...m <= 3': bool
            54916..54917 '1': integer
            54921..54928 'q1_term': ref<function, i32, read_write>
            54921..54933 'q1_term <= 3': bool
            54932..54933 '3': integer
            54943..54951 'q2_pauli': bool
            54954..54961 'q2_term': ref<function, i32, read_write>
            54954..54966 'q2_term >= 1': bool
            54954..54982 'q2_ter...m <= 3': bool
            54965..54966 '1': integer
            54970..54977 'q2_term': ref<function, i32, read_write>
            54970..54982 'q2_term <= 3': bool
            54981..54982 '3': integer
            54993..55001 'q1_pauli': bool
            54993..55013 'q1_pau..._pauli': bool
            55005..55013 'q2_pauli': bool
            55076..55084 'op_row_0': ref<function, array<vec2<f32>, 4>, read_write>
            55087..55106 'getOpR...dx, 0)': array<vec2<f32>, 4>
            55096..55102 'op_idx': u32
            55104..55105 '0': integer
            55120..55128 'op_row_1': ref<function, array<vec2<f32>, 4>, read_write>
            55131..55150 'getOpR...dx, 1)': array<vec2<f32>, 4>
            55140..55146 'op_idx': u32
            55148..55149 '1': integer
            55164..55172 'op_row_2': ref<function, array<vec2<f32>, 4>, read_write>
            55175..55194 'getOpR...dx, 2)': array<vec2<f32>, 4>
            55184..55190 'op_idx': u32
            55192..55193 '2': integer
            55208..55216 'op_row_3': ref<function, array<vec2<f32>, 4>, read_write>
            55219..55238 'getOpR...dx, 3)': array<vec2<f32>, 4>
            55228..55234 'op_idx': u32
            55236..55237 '3': integer
            55716..55723 'q1_term': ref<function, i32, read_write>
            55716..55728 'q1_term == 1': bool
            55727..55728 '1': integer
            55787..55796 'old_row_0': array<vec2<f32>, 4>
            55799..55807 'op_row_0': ref<function, array<vec2<f32>, 4>, read_write>
            55825..55834 'old_row_1': array<vec2<f32>, 4>
            55837..55845 'op_row_1': ref<function, array<vec2<f32>, 4>, read_write>
            55859..55867 'op_row_0': ref<function, array<vec2<f32>, 4>, read_write>
            55870..55878 'op_row_2': ref<function, array<vec2<f32>, 4>, read_write>
            55892..55900 'op_row_1': ref<function, array<vec2<f32>, 4>, read_write>
            55903..55911 'op_row_3': ref<function, array<vec2<f32>, 4>, read_write>
            55925..55933 'op_row_2': ref<function, array<vec2<f32>, 4>, read_write>
            55936..55945 'old_row_0': array<vec2<f32>, 4>
            55959..55967 'op_row_3': ref<function, array<vec2<f32>, 4>, read_write>
            55970..55979 'old_row_1': array<vec2<f32>, 4>
            56071..56080 'old_row_0': array<vec2<f32>, 4>
            56083..56091 'op_row_0': ref<function, array<vec2<f32>, 4>, read_write>
            56109..56118 'old_row_1': array<vec2<f32>, 4>
            56121..56129 'op_row_1': ref<function, array<vec2<f32>, 4>, read_write>
            56143..56151 'op_row_0': ref<function, array<vec2<f32>, 4>, read_write>
            56154..56170 'rowNeg...row_2)': array<vec2<f32>, 4>
            56161..56169 'op_row_2': ref<function, array<vec2<f32>, 4>, read_write>
            56184..56192 'op_row_1': ref<function, array<vec2<f32>, 4>, read_write>
            56195..56211 'rowNeg...row_3)': array<vec2<f32>, 4>
            56202..56210 'op_row_3': ref<function, array<vec2<f32>, 4>, read_write>
            56225..56233 'op_row_2': ref<function, array<vec2<f32>, 4>, read_write>
            56236..56245 'old_row_0': array<vec2<f32>, 4>
            56259..56267 'op_row_3': ref<function, array<vec2<f32>, 4>, read_write>
            56270..56279 'old_row_1': array<vec2<f32>, 4>
            56363..56371 'op_row_2': ref<function, array<vec2<f32>, 4>, read_write>
            56374..56390 'rowNeg...row_2)': array<vec2<f32>, 4>
            56381..56389 'op_row_2': ref<function, array<vec2<f32>, 4>, read_write>
            56404..56412 'op_row_3': ref<function, array<vec2<f32>, 4>, read_write>
            56415..56431 'rowNeg...row_3)': array<vec2<f32>, 4>
            56422..56430 'op_row_3': ref<function, array<vec2<f32>, 4>, read_write>
            56502..56509 'q2_term': ref<function, i32, read_write>
            56502..56514 'q2_term == 1': bool
            56513..56514 '1': integer
            56573..56582 'old_row_0': array<vec2<f32>, 4>
            56585..56593 'op_row_0': ref<function, array<vec2<f32>, 4>, read_write>
            56611..56620 'old_row_2': array<vec2<f32>, 4>
            56623..56631 'op_row_2': ref<function, array<vec2<f32>, 4>, read_write>
            56645..56653 'op_row_0': ref<function, array<vec2<f32>, 4>, read_write>
            56656..56664 'op_row_1': ref<function, array<vec2<f32>, 4>, read_write>
            56678..56686 'op_row_2': ref<function, array<vec2<f32>, 4>, read_write>
            56689..56697 'op_row_3': ref<function, array<vec2<f32>, 4>, read_write>
            56711..56719 'op_row_1': ref<function, array<vec2<f32>, 4>, read_write>
            56722..56731 'old_row_0': array<vec2<f32>, 4>
            56745..56753 'op_row_3': ref<function, array<vec2<f32>, 4>, read_write>
            56756..56765 'old_row_2': array<vec2<f32>, 4>
            56857..56866 'old_row_0': array<vec2<f32>, 4>
            56869..56877 'op_row_0': ref<function, array<vec2<f32>, 4>, read_write>
            56895..56904 'old_row_2': array<vec2<f32>, 4>
            56907..56915 'op_row_2': ref<function, array<vec2<f32>, 4>, read_write>
            56929..56937 'op_row_0': ref<function, array<vec2<f32>, 4>, read_write>
            56940..56956 'rowNeg...row_1)': array<vec2<f32>, 4>
            56947..56955 'op_row_1': ref<function, array<vec2<f32>, 4>, read_write>
            56970..56978 'op_row_2': ref<function, array<vec2<f32>, 4>, read_write>
            56981..56997 'rowNeg...row_3)': array<vec2<f32>, 4>
            56988..56996 'op_row_3': ref<function, array<vec2<f32>, 4>, read_write>
            57011..57019 'op_row_1': ref<function, array<vec2<f32>, 4>, read_write>
            57022..57031 'old_row_0': array<vec2<f32>, 4>
            57045..57053 'op_row_3': ref<function, array<vec2<f32>, 4>, read_write>
            57056..57065 'old_row_2': array<vec2<f32>, 4>
            57149..57157 'op_row_1': ref<function, array<vec2<f32>, 4>, read_write>
            57160..57176 'rowNeg...row_1)': array<vec2<f32>, 4>
            57167..57175 'op_row_1': ref<function, array<vec2<f32>, 4>, read_write>
            57190..57198 'op_row_3': ref<function, array<vec2<f32>, 4>, read_write>
            57201..57217 'rowNeg...row_3)': array<vec2<f32>, 4>
            57208..57216 'op_row_3': ref<function, array<vec2<f32>, 4>, read_write>
            57295..57332 'setUni...row_0)': [error]
            57309..57317 'shot_idx': u32
            57319..57321 '0u': u32
            57323..57331 'op_row_0': ref<function, array<vec2<f32>, 4>, read_write>
            57342..57379 'setUni...row_1)': [error]
            57356..57364 'shot_idx': u32
            57366..57368 '1u': u32
            57370..57378 'op_row_1': ref<function, array<vec2<f32>, 4>, read_write>
            57389..57426 'setUni...row_2)': [error]
            57403..57411 'shot_idx': u32
            57413..57415 '2u': u32
            57417..57425 'op_row_2': ref<function, array<vec2<f32>, 4>, read_write>
            57436..57473 'setUni...row_3)': [error]
            57450..57458 'shot_idx': u32
            57460..57462 '3u': u32
            57464..57472 'op_row_3': ref<function, array<vec2<f32>, 4>, read_write>
            57483..57487 'shot': ptr<storage, ShotData, read_write>
            57483..57495 'shot.op_type': ref<storage, u32, read_write>
            57498..57515 'OPID_S...UFF_2Q': u32
            57673..57675 'op': ptr<storage, Op, read>
            57673..57678 'op.id': ref<storage, u32, read>
            57673..57689 'op.id ...PID_CX': bool
            57673..57709 'op.id ...PID_CY': bool
            57673..57729 'op.id ...PID_CZ': bool
            57673..57750 'op.id ...ID_RZZ': bool
            57682..57689 'OPID_CX': u32
            57693..57695 'op': ptr<storage, Op, read>
            57693..57698 'op.id': ref<storage, u32, read>
            57693..57709 'op.id ...PID_CY': bool
            57702..57709 'OPID_CY': u32
            57713..57715 'op': ptr<storage, Op, read>
            57713..57718 'op.id': ref<storage, u32, read>
            57713..57729 'op.id ...PID_CZ': bool
            57722..57729 'OPID_CZ': u32
            57733..57735 'op': ptr<storage, Op, read>
            57733..57738 'op.id': ref<storage, u32, read>
            57733..57750 'op.id ...ID_RZZ': bool
            57742..57750 'OPID_RZZ': u32
            57766..57770 'shot': ptr<storage, ShotData, read_write>
            57766..57778 'shot.op_type': ref<storage, u32, read_write>
            57781..57783 'op': ptr<storage, Op, read>
            57781..57786 'op.id': ref<storage, u32, read>
            57817..57821 'shot': ptr<storage, ShotData, read_write>
            57817..57829 'shot.op_type': ref<storage, u32, read_write>
            57832..57849 'OPID_S...UFF_2Q': u32
            57871..57875 'shot': ptr<storage, ShotData, read_write>
            57871..57882 'shot.op_idx': ref<storage, u32, read_write>
            57885..57891 'op_idx': u32
            57901..57905 'shot': ptr<storage, ShotData, read_write>
            57901..57913 'shot.op_type': ref<storage, u32, read_write>
            57901..57924 'shot.o...PID_CZ': bool
            57901..57952 'shot.o...ID_RZZ': bool
            57917..57924 'OPID_CZ': u32
            57928..57932 'shot': ptr<storage, ShotData, read_write>
            57928..57940 'shot.op_type': ref<storage, u32, read_write>
            57928..57952 'shot.o...ID_RZZ': bool
            57944..57952 'OPID_RZZ': u32
            57964..57968 'shot': ptr<storage, ShotData, read_write>
            57964..57996 'shot.q...p_mask': ref<storage, u32, read_write>
            57999..58001 '0u': u32
            58025..58029 'shot': ptr<storage, ShotData, read_write>
            58025..58057 'shot.q...p_mask': ref<storage, u32, read_write>
            58060..58084 '(1u <<...<< q2)': u32
            58061..58063 '1u': u32
            58061..58069 '1u << q1': u32
            58067..58069 'q1': u32
            58075..58077 '1u': u32
            58075..58083 '1u << q2': u32
            58081..58083 'q2': u32
            58595..58603 'shot_idx': u32
            58610..58622 'target_is_q2': bool
            58630..58634 'term': u32
            58651..58653 'si': i32
            58656..58669 'i32(shot_idx)': i32
            58660..58668 'shot_idx': u32
            58679..58684 'row_0': ref<function, array<vec2<f32>, 4>, read_write>
            58687..58708 'getUni...i, 0u)': array<vec2<f32>, 4>
            58701..58703 'si': i32
            58705..58707 '0u': u32
            58718..58723 'row_1': ref<function, array<vec2<f32>, 4>, read_write>
            58726..58747 'getUni...i, 1u)': array<vec2<f32>, 4>
            58740..58742 'si': i32
            58744..58746 '1u': u32
            58757..58762 'row_2': ref<function, array<vec2<f32>, 4>, read_write>
            58765..58786 'getUni...i, 2u)': array<vec2<f32>, 4>
            58779..58781 'si': i32
            58783..58785 '2u': u32
            58796..58801 'row_3': ref<function, array<vec2<f32>, 4>, read_write>
            58804..58825 'getUni...i, 3u)': array<vec2<f32>, 4>
            58818..58820 'si': i32
            58822..58824 '3u': u32
            58836..58849 '!target_is_q2': bool
            58837..58849 'target_is_q2': bool
            58923..58927 'term': u32
            58923..58933 'term == 1u': bool
            58931..58933 '1u': u32
            58969..58971 'o0': array<vec2<f32>, 4>
            58974..58979 'row_0': ref<function, array<vec2<f32>, 4>, read_write>
            58985..58987 'o1': array<vec2<f32>, 4>
            58990..58995 'row_1': ref<function, array<vec2<f32>, 4>, read_write>
            59009..59014 'row_0': ref<function, array<vec2<f32>, 4>, read_write>
            59017..59022 'row_2': ref<function, array<vec2<f32>, 4>, read_write>
            59024..59029 'row_1': ref<function, array<vec2<f32>, 4>, read_write>
            59032..59037 'row_3': ref<function, array<vec2<f32>, 4>, read_write>
            59051..59056 'row_2': ref<function, array<vec2<f32>, 4>, read_write>
            59059..59061 'o0': array<vec2<f32>, 4>
            59066..59071 'row_3': ref<function, array<vec2<f32>, 4>, read_write>
            59074..59076 'o1': array<vec2<f32>, 4>
            59136..59138 'o0': array<vec2<f32>, 4>
            59141..59146 'row_0': ref<function, array<vec2<f32>, 4>, read_write>
            59152..59154 'o1': array<vec2<f32>, 4>
            59157..59162 'row_1': ref<function, array<vec2<f32>, 4>, read_write>
            59176..59181 'row_0': ref<function, array<vec2<f32>, 4>, read_write>
            59184..59197 'rowNeg(row_2)': array<vec2<f32>, 4>
            59191..59196 'row_2': ref<function, array<vec2<f32>, 4>, read_write>
            59199..59204 'row_1': ref<function, array<vec2<f32>, 4>, read_write>
            59207..59220 'rowNeg(row_3)': array<vec2<f32>, 4>
            59214..59219 'row_3': ref<function, array<vec2<f32>, 4>, read_write>
            59234..59239 'row_2': ref<function, array<vec2<f32>, 4>, read_write>
            59242..59244 'o0': array<vec2<f32>, 4>
            59257..59262 'row_3': ref<function, array<vec2<f32>, 4>, read_write>
            59265..59267 'o1': array<vec2<f32>, 4>
            59323..59328 'row_2': ref<function, array<vec2<f32>, 4>, read_write>
            59331..59344 'rowNeg(row_2)': array<vec2<f32>, 4>
            59338..59343 'row_2': ref<function, array<vec2<f32>, 4>, read_write>
            59346..59351 'row_3': ref<function, array<vec2<f32>, 4>, read_write>
            59354..59367 'rowNeg(row_3)': array<vec2<f32>, 4>
            59361..59366 'row_3': ref<function, array<vec2<f32>, 4>, read_write>
            59461..59465 'term': u32
            59461..59471 'term == 1u': bool
            59469..59471 '1u': u32
            59507..59509 'o0': array<vec2<f32>, 4>
            59512..59517 'row_0': ref<function, array<vec2<f32>, 4>, read_write>
            59523..59525 'o2': array<vec2<f32>, 4>
            59528..59533 'row_2': ref<function, array<vec2<f32>, 4>, read_write>
            59547..59552 'row_0': ref<function, array<vec2<f32>, 4>, read_write>
            59555..59560 'row_1': ref<function, array<vec2<f32>, 4>, read_write>
            59562..59567 'row_2': ref<function, array<vec2<f32>, 4>, read_write>
            59570..59575 'row_3': ref<function, array<vec2<f32>, 4>, read_write>
            59589..59594 'row_1': ref<function, array<vec2<f32>, 4>, read_write>
            59597..59599 'o0': array<vec2<f32>, 4>
            59604..59609 'row_3': ref<function, array<vec2<f32>, 4>, read_write>
            59612..59614 'o2': array<vec2<f32>, 4>
            59674..59676 'o0': array<vec2<f32>, 4>
            59679..59684 'row_0': ref<function, array<vec2<f32>, 4>, read_write>
            59690..59692 'o2': array<vec2<f32>, 4>
            59695..59700 'row_2': ref<function, array<vec2<f32>, 4>, read_write>
            59714..59719 'row_0': ref<function, array<vec2<f32>, 4>, read_write>
            59722..59735 'rowNeg(row_1)': array<vec2<f32>, 4>
            59729..59734 'row_1': ref<function, array<vec2<f32>, 4>, read_write>
            59737..59742 'row_2': ref<function, array<vec2<f32>, 4>, read_write>
            59745..59758 'rowNeg(row_3)': array<vec2<f32>, 4>
            59752..59757 'row_3': ref<function, array<vec2<f32>, 4>, read_write>
            59772..59777 'row_1': ref<function, array<vec2<f32>, 4>, read_write>
            59780..59782 'o0': array<vec2<f32>, 4>
            59795..59800 'row_3': ref<function, array<vec2<f32>, 4>, read_write>
            59803..59805 'o2': array<vec2<f32>, 4>
            59861..59866 'row_1': ref<function, array<vec2<f32>, 4>, read_write>
            59869..59882 'rowNeg(row_1)': array<vec2<f32>, 4>
            59876..59881 'row_1': ref<function, array<vec2<f32>, 4>, read_write>
            59884..59889 'row_3': ref<function, array<vec2<f32>, 4>, read_write>
            59892..59905 'rowNeg(row_3)': array<vec2<f32>, 4>
            59899..59904 'row_3': ref<function, array<vec2<f32>, 4>, read_write>
            59928..59962 'setUni...row_0)': [error]
            59942..59950 'shot_idx': u32
            59952..59954 '0u': u32
            59956..59961 'row_0': ref<function, array<vec2<f32>, 4>, read_write>
            59968..60002 'setUni...row_1)': [error]
            59982..59990 'shot_idx': u32
            59992..59994 '1u': u32
            59996..60001 'row_1': ref<function, array<vec2<f32>, 4>, read_write>
            60008..60042 'setUni...row_2)': [error]
            60022..60030 'shot_idx': u32
            60032..60034 '2u': u32
            60036..60041 'row_2': ref<function, array<vec2<f32>, 4>, read_write>
            60048..60082 'setUni...row_3)': [error]
            60062..60070 'shot_idx': u32
            60072..60074 '3u': u32
            60076..60081 'row_3': ref<function, array<vec2<f32>, 4>, read_write>
            60729..60737 'shot_idx': u32
            60744..60750 'op_idx': u32
            60757..60766 'noise_idx': u32
            60773..60775 'q1': u32
            60782..60784 'q2': u32
            60801..60805 'shot': ptr<storage, ShotData, read_write>
            60808..60824 '&shots...t_idx]': ptr<storage, ShotData, read_write>
            60809..60814 'shots': ref<storage, array<ShotData>, read_write>
            60809..60824 'shots[shot_idx]': ref<storage, ShotData, read_write>
            60815..60823 'shot_idx': u32
            60834..60842 'noise_op': ptr<storage, Op, read>
            60845..60860 '&ops[noise_idx]': ptr<storage, Op, read>
            60846..60849 'ops': ref<storage, array<Op>, read>
            60846..60860 'ops[noise_idx]': ref<storage, Op, read>
            60850..60859 'noise_idx': u32
            60945..60953 'q1_alive': [error]
            60956..60960 'shot': ptr<storage, ShotData, read_write>
            60956..60972 'shot.q..._state': ref<storage, [error], read_write>
            60956..60976 'shot.q...te[q1]': [error]
            60956..60981 'shot.q...].heat': [error]
            60956..60989 'shot.q...= -1.0': [error]
            60973..60975 'q1': u32
            60985..60989 '-1.0': float
            60986..60989 '1.0': float
            60999..61007 'q2_alive': [error]
            61010..61014 'shot': ptr<storage, ShotData, read_write>
            61010..61026 'shot.q..._state': ref<storage, [error], read_write>
            61010..61030 'shot.q...te[q2]': [error]
            61010..61035 'shot.q...].heat': [error]
            61010..61043 'shot.q...= -1.0': [error]
            61027..61029 'q2': u32
            61039..61043 '-1.0': float
            61040..61043 '1.0': float
            61137..61146 '!q1_alive': [error]
            61137..61159 '!q1_al..._alive': [error]
            61138..61146 'q1_alive': [error]
            61150..61159 '!q2_alive': [error]
            61151..61159 'q2_alive': [error]
            61325..61329 'rand': ref<function, f32, read_write>
            61332..61336 'shot': ptr<storage, ShotData, read_write>
            61332..61347 'shot.rand_pauli': ref<storage, f32, read_write>
            61357..61364 'q1_term': ref<function, i32, read_write>
            61367..61368 '0': integer
            61378..61385 'q2_term': ref<function, i32, read_write>
            61388..61389 '0': integer
            61404..61405 'a': ref<function, i32, read_write>
            61408..61409 '0': integer
            61411..61412 'a': ref<function, i32, read_write>
            61411..61416 'a < 5': bool
            61415..61416 '5': integer
            61418..61419 'a': ref<function, i32, read_write>
            61422..61423 'a': ref<function, i32, read_write>
            61422..61427 'a + 1': i32
            61426..61427 '1': integer
            61448..61449 'b': ref<function, i32, read_write>
            61452..61453 '0': integer
            61455..61456 'b': ref<function, i32, read_write>
            61455..61460 'b < 5': bool
            61459..61460 '5': integer
            61462..61463 'b': ref<function, i32, read_write>
            61466..61467 'b': ref<function, i32, read_write>
            61466..61471 'b + 1': i32
            61470..61471 '1': integer
            61491..61492 'k': i32
            61495..61496 'a': ref<function, i32, read_write>
            61495..61500 'a * 5': i32
            61495..61504 'a * 5 + b': i32
            61499..61500 '5': integer
            61503..61504 'b': ref<function, i32, read_write>
            61522..61523 'k': i32
            61522..61528 'k == 0': bool
            61527..61528 '0': integer
            61560..61564 'slot': vec2<f32>
            61567..61575 'noise_op': ptr<storage, Op, read>
            61567..61583 'noise_...nitary': ref<storage, array<vec2<f32>, 16>, read>
            61567..61590 'noise_...k / 2]': ref<storage, vec2<f32>, read>
            61584..61585 'k': i32
            61584..61589 'k / 2': i32
            61588..61589 '2': integer
            61608..61612 'p_ab': f32
            61615..61651 'select... == 1)': f32
            61622..61626 'slot': vec2<f32>
            61622..61628 'slot.x': f32
            61630..61634 'slot': vec2<f32>
            61630..61636 'slot.y': f32
            61638..61650 '(k & 1) == 1': bool
            61639..61640 'k': i32
            61639..61644 'k & 1': i32
            61643..61644 '1': integer
            61649..61650 '1': integer
            61669..61673 'rand': ref<function, f32, read_write>
            61669..61680 'rand < p_ab': bool
            61676..61680 'p_ab': f32
            61700..61707 'q1_term': ref<function, i32, read_write>
            61710..61711 'a': ref<function, i32, read_write>
            61729..61736 'q2_term': ref<function, i32, read_write>
            61739..61740 'b': ref<function, i32, read_write>
            61758..61759 'a': ref<function, i32, read_write>
            61762..61763 '5': integer
            61781..61782 'b': ref<function, i32, read_write>
            61785..61786 '5': integer
            61825..61829 'rand': ref<function, f32, read_write>
            61832..61836 'rand': ref<function, f32, read_write>
            61832..61843 'rand - p_ab': f32
            61839..61843 'p_ab': f32
            61953..61967 'survivor_is_q2': [error]
            61970..61979 '!q1_alive': [error]
            61971..61979 'q1_alive': [error]
            61989..61997 'survivor': [error]
            62000..62030 'select...is_q2)': [error]
            62007..62009 'q1': u32
            62011..62013 'q2': u32
            62015..62029 'survivor_is_q2': [error]
            62040..62044 'term': [error]
            62047..62087 'select...is_q2)': [error]
            62054..62061 'q1_term': ref<function, i32, read_write>
            62063..62070 'q2_term': ref<function, i32, read_write>
            62072..62086 'survivor_is_q2': [error]
            62258..62262 'term': [error]
            62258..62267 'term == 4': [error]
            62266..62267 '4': integer
            62279..62283 'shot': ptr<storage, ShotData, read_write>
            62279..62301 'shot.p...s_mask': ref<storage, u32, read_write>
            62306..62308 '1u': u32
            62306..62320 '1u << survivor': [error]
            62312..62320 'survivor': [error]
            62428..62432 'term': [error]
            62428..62437 'term == 0': [error]
            62436..62437 '0': integer
            62526..62530 'shot': ptr<storage, ShotData, read_write>
            62526..62538 'shot.op_type': ref<storage, u32, read_write>
            62526..62559 'shot.o...UFF_2Q': bool
            62542..62559 'OPID_S...UFF_2Q': u32
            62681..62747 'fuse_1...term))': [error]
            62711..62719 'shot_idx': u32
            62721..62735 'survivor_is_q2': [error]
            62737..62746 'u32(term)': u32
            62741..62745 'term': [error]
            63008..63012 'term': [error]
            63008..63017 'term == 1': [error]
            63016..63017 '1': integer
            63045..63193 'set_1q... 0.0))': [error]
            63068..63076 'shot_idx': u32
            63078..63092 'survivor_is_q2': [error]
            63110..63125 'vec2f(0.0, 0.0)': vec2<f32>
            63116..63119 '0.0': float
            63121..63124 '0.0': float
            63127..63142 'vec2f(1.0, 0.0)': vec2<f32>
            63133..63136 '1.0': float
            63138..63141 '0.0': float
            63160..63175 'vec2f(1.0, 0.0)': vec2<f32>
            63166..63169 '1.0': float
            63171..63174 '0.0': float
            63177..63192 'vec2f(0.0, 0.0)': vec2<f32>
            63183..63186 '0.0': float
            63188..63191 '0.0': float
            63280..63429 'set_1q... 0.0))': [error]
            63303..63311 'shot_idx': u32
            63313..63327 'survivor_is_q2': [error]
            63345..63360 'vec2f(0.0, 0.0)': vec2<f32>
            63351..63354 '0.0': float
            63356..63359 '0.0': float
            63362..63378 'vec2f(..., 0.0)': vec2<f32>
            63368..63372 '-1.0': float
            63369..63372 '1.0': float
            63374..63377 '0.0': float
            63396..63411 'vec2f(1.0, 0.0)': vec2<f32>
            63402..63405 '1.0': float
            63407..63410 '0.0': float
            63413..63428 'vec2f(0.0, 0.0)': vec2<f32>
            63419..63422 '0.0': float
            63424..63427 '0.0': float
            63480..63629 'set_1q... 0.0))': [error]
            63503..63511 'shot_idx': u32
            63513..63527 'survivor_is_q2': [error]
            63545..63560 'vec2f(1.0, 0.0)': vec2<f32>
            63551..63554 '1.0': float
            63556..63559 '0.0': float
            63562..63577 'vec2f(0.0, 0.0)': vec2<f32>
            63568..63571 '0.0': float
            63573..63576 '0.0': float
            63595..63610 'vec2f(0.0, 0.0)': vec2<f32>
            63601..63604 '0.0': float
            63606..63609 '0.0': float
            63612..63628 'vec2f(..., 0.0)': vec2<f32>
            63618..63622 '-1.0': float
            63619..63622 '1.0': float
            63624..63627 '0.0': float
            63649..63696 'finish...1, q2)': [error]
            63671..63679 'shot_idx': u32
            63681..63687 'op_idx': u32
            63689..63691 'q1': u32
            63693..63695 'q2': u32
            63952..63956 'shot': ptr<storage, ShotData, read_write>
            63952..63972 'shot.q...0_mask': ref<storage, u32, read_write>
            63975..63979 'shot': ptr<storage, ShotData, read_write>
            63975..63995 'shot.q...0_mask': ref<storage, u32, read_write>
            63975..64015 'shot.q...vivor)': [error]
            63998..64015 '~(1u <...vivor)': [error]
            64000..64002 '1u': u32
            64000..64014 '1u << survivor': [error]
            64006..64014 'survivor': [error]
            64021..64025 'shot': ptr<storage, ShotData, read_write>
            64021..64041 'shot.q...1_mask': ref<storage, u32, read_write>
            64044..64048 'shot': ptr<storage, ShotData, read_write>
            64044..64064 'shot.q...1_mask': ref<storage, u32, read_write>
            64044..64084 'shot.q...vivor)': [error]
            64067..64084 '~(1u <...vivor)': [error]
            64069..64071 '1u': u32
            64069..64083 '1u << survivor': [error]
            64075..64083 'survivor': [error]
            64165..64176 'workgroupId': u32
            64191..64194 'tid': u32
            64209..64223 'op_qubit_count': i32
            64356..64364 'shot_idx': i32
            64372..64388 'i32(wo...oupId)': i32
            64372..64410 'i32(wo...R_SHOT': i32
            64376..64387 'workgroupId': u32
            64391..64410 'WORKGR...R_SHOT': i32
            64420..64443 'shot_s..._start': i32
            64451..64459 'shot_idx': i32
            64451..64486 'shot_i...OUNT))': i32
            64463..64465 '1i': i32
            64463..64485 '1i << ...COUNT)': i32
            64469..64485 'u32(QU...COUNT)': u32
            64473..64484 'QUBIT_COUNT': i32
            64496..64517 'workgr...n_shot': i32
            64525..64541 'i32(wo...oupId)': i32
            64525..64563 'i32(wo...R_SHOT': i32
            64529..64540 'workgroupId': u32
            64544..64563 'WORKGR...R_SHOT': i32
            64573..64591 'thread...n_shot': i32
            64599..64620 'workgr...n_shot': i32
            64599..64644 'workgr...KGROUP': i32
            64599..64655 'workgr...2(tid)': i32
            64623..64644 'THREAD...KGROUP': i32
            64647..64655 'i32(tid)': i32
            64651..64654 'tid': u32
            64665..64687 'total_...r_shot': i32
            64695..64714 'WORKGR...R_SHOT': i32
            64695..64738 'WORKGR...KGROUP': i32
            64717..64738 'THREAD...KGROUP': i32
            65078..65101 'workgr...on_idx': i32
            65109..65162 'select...T > 1)': i32
            65116..65118 '-1': integer
            65117..65118 '1': integer
            65120..65136 'i32(wo...oupId)': i32
            65124..65135 'workgroupId': u32
            65138..65157 'WORKGR...R_SHOT': i32
            65138..65161 'WORKGR...OT > 1': bool
            65160..65161 '1': integer
            65173..65189 'zero_e..._count': i32
            65197..65244 '(1i <<...count)': i32
            65198..65200 '1i': i32
            65198..65220 '1i << ...COUNT)': i32
            65204..65220 'u32(QU...COUNT)': u32
            65208..65219 'QUBIT_COUNT': i32
            65225..65244 'u32(op...count)': u32
            65229..65243 'op_qubit_count': i32
            65254..65267 'op_iterations': i32
            65275..65291 'zero_e..._count': i32
            65275..65316 'zero_e...r_shot': i32
            65294..65316 'total_...r_shot': i32
            65330..65570 'ShotPa...     )': ShotParams
            65350..65358 'shot_idx': i32
            65368..65391 'shot_s..._start': i32
            65401..65424 'workgr...on_idx': i32
            65434..65455 'workgr...n_shot': i32
            65465..65483 'thread...n_shot': i32
            65493..65515 'total_...r_shot': i32
            65525..65541 'zero_e..._count': i32
            65551..65564 'op_iterations': i32
            65648..65659 'workgroupId': u32
            65666..65669 'tid': u32
            65676..65678 'q1': u32
            65695..65701 'params': ShotParams
            65704..65760 'get_sh...op */)': ShotParams
            65720..65731 'workgroupId': u32
            65733..65736 'tid': u32
            65738..65739 '1': integer
            65770..65774 'shot': ptr<storage, ShotData, read_write>
            65777..65800 '&shots...t_idx]': ptr<storage, ShotData, read_write>
            65778..65783 'shots': ref<storage, array<ShotData>, read_write>
            65778..65800 'shots[...t_idx]': ref<storage, ShotData, read_write>
            65784..65790 'params': ShotParams
            65784..65799 'params.shot_idx': i32
            65810..65815 'scale': f32
            65818..65822 'shot': ptr<storage, ShotData, read_write>
            65818..65834 'shot.r...malize': ref<storage, f32, read_write>
            65844..65851 'lowMask': i32
            65854..65867 '(1 << q1) - 1': integer
            65855..65856 '1': integer
            65855..65862 '1 << q1': integer
            65860..65862 'q1': u32
            65866..65867 '1': integer
            65877..65885 'highMask': i32
            65888..65915 '(1 << ...)) - 1': integer
            65888..65925 '(1 << ...owMask': i32
            65889..65890 '1': integer
            65889..65910 '1 << u...COUNT)': integer
            65894..65910 'u32(QU...COUNT)': u32
            65898..65909 'QUBIT_COUNT': i32
            65914..65915 '1': integer
            65918..65925 'lowMask': i32
            65935..65950 'qubit_is_0_mask': i32
            65953..65996 'i32(sh..._mask)': i32
            65957..65962 'shots': ref<storage, array<ShotData>, read_write>
            65957..65979 'shots[...t_idx]': ref<storage, ShotData, read_write>
            65957..65995 'shots[...0_mask': ref<storage, u32, read_write>
            65963..65969 'params': ShotParams
            65963..65978 'params.shot_idx': i32
            66006..66021 'qubit_is_1_mask': i32
            66024..66067 'i32(sh..._mask)': i32
            66028..66033 'shots': ref<storage, array<ShotData>, read_write>
            66028..66050 'shots[...t_idx]': ref<storage, ShotData, read_write>
            66028..66066 'shots[...1_mask': ref<storage, u32, read_write>
            66034..66040 'params': ShotParams
            66034..66049 'params.shot_idx': i32
            66078..66090 'summed_probs': ref<function, vec4<f32>, read_write>
            66100..66107 'vec4f()': vec4<f32>
            66641..66652 'entry_index': ref<function, i32, read_write>
            66655..66661 'params': ShotParams
            66655..66680 'params...n_shot': i32
            66696..66697 'i': ref<function, i32, read_write>
            66700..66701 '0': integer
            66703..66704 'i': ref<function, i32, read_write>
            66703..66727 'i < pa...ations': bool
            66707..66713 'params': ShotParams
            66707..66727 'params...ations': i32
            66729..66730 'i': ref<function, i32, read_write>
            66748..66755 'offset0': i32
            66763..66820 '(entry... << 1)': i32
            66764..66775 'entry_index': ref<function, i32, read_write>
            66764..66785 'entry_...owMask': i32
            66778..66785 'lowMask': i32
            66790..66819 '(entry...) << 1': i32
            66791..66802 'entry_index': ref<function, i32, read_write>
            66791..66813 'entry_...ghMask': i32
            66805..66813 'highMask': i32
            66818..66819 '1': integer
            66834..66841 'offset1': i32
            66849..66856 'offset0': i32
            66849..66868 'offset...<< q1)': i32
            66860..66861 '1': integer
            66860..66867 '1 << q1': integer
            66865..66867 'q1': u32
            67106..67121 'skip_processing': bool
            67124..67197 '((offs... != 0)': bool
            67125..67157 '(offse...) != 0': bool
            67126..67133 'offset0': i32
            67126..67151 'offset...0_mask': i32
            67136..67151 'qubit_is_0_mask': i32
            67156..67157 '0': integer
            67163..67196 '(~offs...) != 0': bool
            67164..67172 '~offset1': i32
            67164..67190 '~offse...1_mask': i32
            67165..67172 'offset1': i32
            67175..67190 'qubit_is_1_mask': i32
            67195..67196 '0': integer
            67212..67228 '!skip_...essing': bool
            67213..67228 'skip_processing': bool
            67247..67251 'shot': ptr<storage, ShotData, read_write>
            67247..67259 'shot.op_type': ref<storage, u32, read_write>
            67247..67270 'shot.o...PID_RZ': bool
            67263..67270 'OPID_RZ': u32
            67482..67486 'amp1': vec2<f32>
            67496..67507 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            67496..67549 'stateV...fset1]': ref<storage, vec2<f32>, read_write>
            67508..67514 'params': ShotParams
            67508..67538 'params..._start': i32
            67508..67548 'params...ffset1': i32
            67541..67548 'offset1': i32
            67571..67575 'new1': vec2<f32>
            67578..67608 'cplxMu...ry[5])': vec2<f32>
            67586..67590 'amp1': vec2<f32>
            67592..67596 'shot': ptr<storage, ShotData, read_write>
            67592..67604 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            67592..67607 'shot.unitary[5]': ref<storage, vec2<f32>, read_write>
            67605..67606 '5': integer
            67626..67637 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            67626..67679 'stateV...fset1]': ref<storage, vec2<f32>, read_write>
            67638..67644 'params': ShotParams
            67638..67668 'params..._start': i32
            67638..67678 'params...ffset1': i32
            67671..67678 'offset1': i32
            67682..67686 'new1': vec2<f32>
            67729..67733 'amp0': vec2<f32>
            67743..67754 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            67743..67796 'stateV...fset0]': ref<storage, vec2<f32>, read_write>
            67755..67761 'params': ShotParams
            67755..67785 'params..._start': i32
            67755..67795 'params...ffset0': i32
            67788..67795 'offset0': i32
            67818..67822 'amp1': vec2<f32>
            67832..67843 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            67832..67885 'stateV...fset1]': ref<storage, vec2<f32>, read_write>
            67844..67850 'params': ShotParams
            67844..67874 'params..._start': i32
            67844..67884 'params...ffset1': i32
            67877..67884 'offset1': i32
            67908..67912 'new0': vec2<f32>
            67915..67920 'scale': f32
            67915..67988 'scale ...y[1]))': vec2<f32>
            67924..67954 'cplxMu...ry[0])': vec2<f32>
            67924..67987 'cplxMu...ry[1])': vec2<f32>
            67932..67936 'amp0': vec2<f32>
            67938..67942 'shot': ptr<storage, ShotData, read_write>
            67938..67950 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            67938..67953 'shot.unitary[0]': ref<storage, vec2<f32>, read_write>
            67951..67952 '0': integer
            67957..67987 'cplxMu...ry[1])': vec2<f32>
            67965..67969 'amp1': vec2<f32>
            67971..67975 'shot': ptr<storage, ShotData, read_write>
            67971..67983 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            67971..67986 'shot.unitary[1]': ref<storage, vec2<f32>, read_write>
            67984..67985 '1': integer
            68010..68014 'new1': vec2<f32>
            68017..68022 'scale': f32
            68017..68090 'scale ...y[5]))': vec2<f32>
            68026..68056 'cplxMu...ry[4])': vec2<f32>
            68026..68089 'cplxMu...ry[5])': vec2<f32>
            68034..68038 'amp0': vec2<f32>
            68040..68044 'shot': ptr<storage, ShotData, read_write>
            68040..68052 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            68040..68055 'shot.unitary[4]': ref<storage, vec2<f32>, read_write>
            68053..68054 '4': integer
            68059..68089 'cplxMu...ry[5])': vec2<f32>
            68067..68071 'amp1': vec2<f32>
            68073..68077 'shot': ptr<storage, ShotData, read_write>
            68073..68085 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            68073..68088 'shot.unitary[5]': ref<storage, vec2<f32>, read_write>
            68086..68087 '5': integer
            68109..68120 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            68109..68162 'stateV...fset0]': ref<storage, vec2<f32>, read_write>
            68121..68127 'params': ShotParams
            68121..68151 'params..._start': i32
            68121..68161 'params...ffset0': i32
            68154..68161 'offset0': i32
            68165..68169 'new0': vec2<f32>
            68187..68198 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            68187..68240 'stateV...fset1]': ref<storage, vec2<f32>, read_write>
            68199..68205 'params': ShotParams
            68199..68229 'params..._start': i32
            68199..68239 'params...ffset1': i32
            68232..68239 'offset1': i32
            68243..68247 'new1': vec2<f32>
            68269..68273 'shot': ptr<storage, ShotData, read_write>
            68269..68281 'shot.op_type': ref<storage, u32, read_write>
            68269..68297 'shot.o...RESETZ': bool
            68269..68332 'shot.o..._NOISE': bool
            68269..68348 'shot.o...!= 1.0': bool
            68285..68297 'OPID_MRESETZ': u32
            68301..68305 'shot': ptr<storage, ShotData, read_write>
            68301..68313 'shot.op_type': ref<storage, u32, read_write>
            68301..68332 'shot.o..._NOISE': bool
            68317..68332 'OPID_LOSS_NOISE': u32
            68336..68341 'scale': f32
            68336..68348 'scale != 1.0': bool
            68345..68348 '1.0': float
            68480..68527 'update..., tid)': [error]
            68503..68515 'u32(offset0)': u32
            68507..68514 'offset0': i32
            68517..68521 'new0': vec2<f32>
            68523..68526 'tid': u32
            68549..68596 'update..., tid)': [error]
            68572..68584 'u32(offset1)': u32
            68576..68583 'offset1': i32
            68586..68590 'new1': vec2<f32>
            68592..68595 'tid': u32
            68643..68655 'summed_probs': ref<function, vec4<f32>, read_write>
            68643..68658 'summed_probs[0]': ref<function, f32, read_write>
            68656..68657 '0': integer
            68662..68676 'cplxMag2(new0)': f32
            68671..68675 'new0': vec2<f32>
            68698..68710 'summed_probs': ref<function, vec4<f32>, read_write>
            68698..68713 'summed_probs[1]': ref<function, f32, read_write>
            68711..68712 '1': integer
            68717..68731 'cplxMag2(new1)': f32
            68726..68730 'new1': vec2<f32>
            68783..68794 'entry_index': ref<function, i32, read_write>
            68798..68804 'params': ShotParams
            68798..68827 'params...r_shot': i32
            68843..68848 'scale': f32
            68843..68855 'scale == 1.0': bool
            68843..68882 'scale ...PID_RZ': bool
            68843..68914 'scale ...RESETZ': bool
            68843..68949 'scale ..._NOISE': bool
            68852..68855 '1.0': float
            68859..68863 'shot': ptr<storage, ShotData, read_write>
            68859..68871 'shot.op_type': ref<storage, u32, read_write>
            68859..68882 'shot.o...PID_RZ': bool
            68875..68882 'OPID_RZ': u32
            68886..68890 'shot': ptr<storage, ShotData, read_write>
            68886..68898 'shot.op_type': ref<storage, u32, read_write>
            68886..68914 'shot.o...RESETZ': bool
            68902..68914 'OPID_MRESETZ': u32
            68918..68922 'shot': ptr<storage, ShotData, read_write>
            68918..68930 'shot.op_type': ref<storage, u32, read_write>
            68918..68949 'shot.o..._NOISE': bool
            68934..68949 'OPID_LOSS_NOISE': u32
            69043..69061 'qubitP...lities': ref<workgroup, [error], read_write>
            69043..69066 'qubitP...s[tid]': [error]
            69043..69071 'qubitP...].zero': [error]
            69043..69075 'qubitP...ro[q1]': [error]
            69062..69065 'tid': u32
            69072..69074 'q1': u32
            69078..69090 'summed_probs': ref<function, vec4<f32>, read_write>
            69078..69093 'summed_probs[0]': ref<function, f32, read_write>
            69091..69092 '0': integer
            69103..69121 'qubitP...lities': ref<workgroup, [error], read_write>
            69103..69126 'qubitP...s[tid]': [error]
            69103..69130 'qubitP...d].one': [error]
            69103..69134 'qubitP...ne[q1]': [error]
            69122..69125 'tid': u32
            69131..69133 'q1': u32
            69138..69150 'summed_probs': ref<function, vec4<f32>, read_write>
            69138..69153 'summed_probs[1]': ref<function, f32, read_write>
            69151..69152 '1': integer
            69179..69190 'workgroupId': u32
            69197..69200 'tid': u32
            69207..69209 'q1': u32
            69216..69218 'q2': u32
            69235..69241 'params': ShotParams
            69244..69300 'get_sh...op */)': ShotParams
            69260..69271 'workgroupId': u32
            69273..69276 'tid': u32
            69278..69279 '2': integer
            69310..69314 'shot': ptr<storage, ShotData, read_write>
            69317..69340 '&shots...t_idx]': ptr<storage, ShotData, read_write>
            69318..69323 'shots': ref<storage, array<ShotData>, read_write>
            69318..69340 'shots[...t_idx]': ref<storage, ShotData, read_write>
            69324..69330 'params': ShotParams
            69324..69339 'params.shot_idx': i32
            69350..69362 'update_probs': bool
            69365..69369 'shot': ptr<storage, ShotData, read_write>
            69365..69377 'shot.op_type': ref<storage, u32, read_write>
            69365..69388 'shot.o...PID_CZ': bool
            69365..69416 'shot.o...ID_RZZ': bool
            69381..69388 'OPID_CZ': u32
            69392..69396 'shot': ptr<storage, ShotData, read_write>
            69392..69404 'shot.op_type': ref<storage, u32, read_write>
            69392..69416 'shot.o...ID_RZZ': bool
            69408..69416 'OPID_RZZ': u32
            69639..69647 'lowQubit': u32
            69650..69673 'select... > q2)': u32
            69657..69659 'q1': u32
            69661..69663 'q2': u32
            69665..69667 'q1': u32
            69665..69672 'q1 > q2': bool
            69670..69672 'q2': u32
            69683..69690 'hiQubit': u32
            69693..69716 'select... < q2)': u32
            69700..69702 'q1': u32
            69704..69706 'q2': u32
            69708..69710 'q1': u32
            69708..69715 'q1 < q2': bool
            69713..69715 'q2': u32
            69765..69776 'lowBitCount': u32
            69779..69787 'lowQubit': u32
            69797..69808 'midBitCount': u32
            69811..69818 'hiQubit': u32
            69811..69829 'hiQubi...wQubit': u32
            69811..69833 'hiQubi...it - 1': u32
            69821..69829 'lowQubit': u32
            69832..69833 '1': integer
            69843..69853 'hiBitCount': u32
            69856..69872 'u32(QU...COUNT)': u32
            69856..69882 'u32(QU...iQubit': u32
            69856..69886 'u32(QU...it - 1': u32
            69860..69871 'QUBIT_COUNT': i32
            69875..69882 'hiQubit': u32
            69885..69886 '1': integer
            70017..70024 'lowMask': i32
            70027..70049 '(1 << ...t) - 1': integer
            70028..70029 '1': integer
            70028..70044 '1 << l...tCount': integer
            70033..70044 'lowBitCount': u32
            70048..70049 '1': integer
            70059..70066 'midMask': i32
            70069..70107 '(1 << ...)) - 1': integer
            70069..70117 '(1 << ...owMask': i32
            70070..70071 '1': integer
            70070..70102 '1 << (...Count)': integer
            70076..70087 'lowBitCount': u32
            70076..70101 'lowBit...tCount': u32
            70090..70101 'midBitCount': u32
            70106..70107 '1': integer
            70110..70117 'lowMask': i32
            70127..70133 'hiMask': i32
            70136..70163 '(1 << ...)) - 1': integer
            70136..70173 '(1 << ...idMask': i32
            70136..70183 '(1 << ...owMask': i32
            70137..70138 '1': integer
            70137..70158 '1 << u...COUNT)': integer
            70142..70158 'u32(QU...COUNT)': u32
            70146..70157 'QUBIT_COUNT': i32
            70162..70163 '1': integer
            70166..70173 'midMask': i32
            70176..70183 'lowMask': i32
            70324..70335 'entry_index': ref<function, i32, read_write>
            70338..70344 'params': ShotParams
            70338..70363 'params...n_shot': i32
            70373..70385 'summed_probs': ref<function, vec4<f32>, read_write>
            70395..70402 'vec4f()': vec4<f32>
            70418..70419 'i': ref<function, i32, read_write>
            70422..70423 '0': integer
            70425..70426 'i': ref<function, i32, read_write>
            70425..70449 'i < pa...ations': bool
            70429..70435 'params': ShotParams
            70429..70449 'params...ations': i32
            70451..70452 'i': ref<function, i32, read_write>
            70517..70525 'offset00': i32
            70533..70589 '(entry... << 1)': i32
            70533..70621 '(entry... << 2)': i32
            70534..70545 'entry_index': ref<function, i32, read_write>
            70534..70555 'entry_...owMask': i32
            70548..70555 'lowMask': i32
            70560..70588 '(entry...) << 1': i32
            70561..70572 'entry_index': ref<function, i32, read_write>
            70561..70582 'entry_...idMask': i32
            70575..70582 'midMask': i32
            70587..70588 '1': integer
            70593..70620 '(entry...) << 2': i32
            70594..70605 'entry_index': ref<function, i32, read_write>
            70594..70614 'entry_...hiMask': i32
            70608..70614 'hiMask': i32
            70619..70620 '2': integer
            70635..70643 'offset01': i32
            70651..70659 'offset00': i32
            70651..70671 'offset...<< q2)': i32
            70663..70664 '1': integer
            70663..70670 '1 << q2': integer
            70668..70670 'q2': u32
            70685..70693 'offset10': i32
            70701..70709 'offset00': i32
            70701..70721 'offset...<< q1)': i32
            70713..70714 '1': integer
            70713..70720 '1 << q1': integer
            70718..70720 'q1': u32
            70735..70743 'offset11': i32
            70751..70759 'offset10': i32
            70751..70771 'offset...<< q2)': i32
            70763..70764 '1': integer
            70763..70770 '1 << q2': integer
            70768..70770 'q2': u32
            70786..70805 'can_sk...essing': bool
            70821..70930 '((u32(... != 0)': bool
            70822..70865 '(u32(o...) != 0': bool
            70823..70836 'u32(offset00)': u32
            70823..70859 'u32(of...0_mask': u32
            70827..70835 'offset00': i32
            70839..70843 'shot': ptr<storage, ShotData, read_write>
            70839..70859 'shot.q...0_mask': ref<storage, u32, read_write>
            70864..70865 '0': integer
            70883..70929 '(~(u32...) != 0': bool
            70884..70900 '~(u32(...et11))': u32
            70884..70923 '~(u32(...1_mask': u32
            70886..70899 'u32(offset11)': u32
            70890..70898 'offset11': i32
            70903..70907 'shot': ptr<storage, ShotData, read_write>
            70903..70923 'shot.q...1_mask': ref<storage, u32, read_write>
            70928..70929 '0': integer
            70944..70964 '!can_s...essing': bool
            70945..70964 'can_sk...essing': bool
            70986..70990 'shot': ptr<storage, ShotData, read_write>
            70986..70998 'shot.op_type': ref<storage, u32, read_write>
            71018..71025 'OPID_CZ': u32
            71048..71053 'amp11': vec2<f32>
            71063..71074 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            71063..71117 'stateV...set11]': ref<storage, vec2<f32>, read_write>
            71075..71081 'params': ShotParams
            71075..71105 'params..._start': i32
            71075..71116 'params...fset11': i32
            71108..71116 'offset11': i32
            71135..71146 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            71135..71189 'stateV...set11]': ref<storage, vec2<f32>, read_write>
            71147..71153 'params': ShotParams
            71147..71177 'params..._start': i32
            71147..71188 'params...fset11': i32
            71180..71188 'offset11': i32
            71192..71206 'cplxNeg(amp11)': vec2<f32>
            71200..71205 'amp11': vec2<f32>
            71329..71337 'OPID_RZZ': u32
            71451..71456 'amp01': vec2<f32>
            71466..71477 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            71466..71520 'stateV...set01]': ref<storage, vec2<f32>, read_write>
            71478..71484 'params': ShotParams
            71478..71508 'params..._start': i32
            71478..71519 'params...fset01': i32
            71511..71519 'offset01': i32
            71542..71547 'amp10': vec2<f32>
            71557..71568 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            71557..71611 'stateV...set10]': ref<storage, vec2<f32>, read_write>
            71569..71575 'params': ShotParams
            71569..71599 'params..._start': i32
            71569..71610 'params...fset10': i32
            71602..71610 'offset10': i32
            71735..71746 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            71735..71789 'stateV...set01]': ref<storage, vec2<f32>, read_write>
            71747..71753 'params': ShotParams
            71747..71777 'params..._start': i32
            71747..71788 'params...fset01': i32
            71780..71788 'offset01': i32
            71792..71823 'cplxMu...ry[5])': vec2<f32>
            71800..71805 'amp01': vec2<f32>
            71807..71811 'shot': ptr<storage, ShotData, read_write>
            71807..71819 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            71807..71822 'shot.unitary[5]': ref<storage, vec2<f32>, read_write>
            71820..71821 '5': integer
            71841..71852 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            71841..71895 'stateV...set10]': ref<storage, vec2<f32>, read_write>
            71853..71859 'params': ShotParams
            71853..71883 'params..._start': i32
            71853..71894 'params...fset10': i32
            71886..71894 'offset10': i32
            71898..71930 'cplxMu...y[10])': vec2<f32>
            71906..71911 'amp10': vec2<f32>
            71913..71917 'shot': ptr<storage, ShotData, read_write>
            71913..71925 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            71913..71929 'shot.u...ry[10]': ref<storage, vec2<f32>, read_write>
            71926..71928 '10': integer
            71963..71970 'OPID_CX': u32
            72110..72115 'amp00': vec2<f32>
            72125..72136 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            72125..72179 'stateV...set00]': ref<storage, vec2<f32>, read_write>
            72137..72143 'params': ShotParams
            72137..72167 'params..._start': i32
            72137..72178 'params...fset00': i32
            72170..72178 'offset00': i32
            72201..72206 'amp01': vec2<f32>
            72216..72227 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            72216..72270 'stateV...set01]': ref<storage, vec2<f32>, read_write>
            72228..72234 'params': ShotParams
            72228..72258 'params..._start': i32
            72228..72269 'params...fset01': i32
            72261..72269 'offset01': i32
            72292..72297 'amp10': vec2<f32>
            72307..72318 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            72307..72361 'stateV...set10]': ref<storage, vec2<f32>, read_write>
            72319..72325 'params': ShotParams
            72319..72349 'params..._start': i32
            72319..72360 'params...fset10': i32
            72352..72360 'offset10': i32
            72383..72388 'amp11': vec2<f32>
            72398..72409 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            72398..72452 'stateV...set11]': ref<storage, vec2<f32>, read_write>
            72410..72416 'params': ShotParams
            72410..72440 'params..._start': i32
            72410..72451 'params...fset11': i32
            72443..72451 'offset11': i32
            72470..72481 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            72470..72524 'stateV...set10]': ref<storage, vec2<f32>, read_write>
            72482..72488 'params': ShotParams
            72482..72512 'params..._start': i32
            72482..72523 'params...fset10': i32
            72515..72523 'offset10': i32
            72527..72532 'amp11': vec2<f32>
            72550..72561 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            72550..72604 'stateV...set11]': ref<storage, vec2<f32>, read_write>
            72562..72568 'params': ShotParams
            72562..72592 'params..._start': i32
            72562..72603 'params...fset11': i32
            72595..72603 'offset11': i32
            72607..72612 'amp10': vec2<f32>
            72630..72642 'summed_probs': ref<function, vec4<f32>, read_write>
            72630..72645 'summed_probs[0]': ref<function, f32, read_write>
            72643..72644 '0': integer
            72650..72665 'cplxMag2(amp00)': f32
            72650..72683 'cplxMa...amp01)': f32
            72659..72664 'amp00': vec2<f32>
            72668..72683 'cplxMag2(amp01)': f32
            72677..72682 'amp01': vec2<f32>
            72702..72714 'summed_probs': ref<function, vec4<f32>, read_write>
            72702..72717 'summed_probs[1]': ref<function, f32, read_write>
            72715..72716 '1': integer
            72722..72737 'cplxMag2(amp11)': f32
            72722..72755 'cplxMa...amp10)': f32
            72731..72736 'amp11': vec2<f32>
            72740..72755 'cplxMag2(amp10)': f32
            72749..72754 'amp10': vec2<f32>
            72774..72786 'summed_probs': ref<function, vec4<f32>, read_write>
            72774..72789 'summed_probs[2]': ref<function, f32, read_write>
            72787..72788 '2': integer
            72794..72809 'cplxMag2(amp00)': f32
            72794..72827 'cplxMa...amp11)': f32
            72803..72808 'amp00': vec2<f32>
            72812..72827 'cplxMag2(amp11)': f32
            72821..72826 'amp11': vec2<f32>
            72846..72858 'summed_probs': ref<function, vec4<f32>, read_write>
            72846..72861 'summed_probs[3]': ref<function, f32, read_write>
            72859..72860 '3': integer
            72866..72881 'cplxMag2(amp01)': f32
            72866..72899 'cplxMa...amp10)': f32
            72875..72880 'amp01': vec2<f32>
            72884..72899 'cplxMag2(amp10)': f32
            72893..72898 'amp10': vec2<f32>
            72933..72940 'OPID_CY': u32
            73033..73038 'amp00': vec2<f32>
            73048..73059 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            73048..73102 'stateV...set00]': ref<storage, vec2<f32>, read_write>
            73060..73066 'params': ShotParams
            73060..73090 'params..._start': i32
            73060..73101 'params...fset00': i32
            73093..73101 'offset00': i32
            73124..73129 'amp01': vec2<f32>
            73139..73150 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            73139..73193 'stateV...set01]': ref<storage, vec2<f32>, read_write>
            73151..73157 'params': ShotParams
            73151..73181 'params..._start': i32
            73151..73192 'params...fset01': i32
            73184..73192 'offset01': i32
            73215..73220 'amp10': vec2<f32>
            73230..73241 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            73230..73284 'stateV...set10]': ref<storage, vec2<f32>, read_write>
            73242..73248 'params': ShotParams
            73242..73272 'params..._start': i32
            73242..73283 'params...fset10': i32
            73275..73283 'offset10': i32
            73306..73311 'amp11': vec2<f32>
            73321..73332 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            73321..73375 'stateV...set11]': ref<storage, vec2<f32>, read_write>
            73333..73339 'params': ShotParams
            73333..73363 'params..._start': i32
            73333..73374 'params...fset11': i32
            73366..73374 'offset11': i32
            73393..73404 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            73393..73447 'stateV...set10]': ref<storage, vec2<f32>, read_write>
            73405..73411 'params': ShotParams
            73405..73435 'params..._start': i32
            73405..73446 'params...fset10': i32
            73438..73446 'offset10': i32
            73450..73474 'vec2f(...p11.x)': vec2<f32>
            73456..73461 'amp11': vec2<f32>
            73456..73463 'amp11.y': f32
            73465..73473 '-amp11.x': f32
            73466..73471 'amp11': vec2<f32>
            73466..73473 'amp11.x': f32
            73505..73516 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            73505..73559 'stateV...set11]': ref<storage, vec2<f32>, read_write>
            73517..73523 'params': ShotParams
            73517..73547 'params..._start': i32
            73517..73558 'params...fset11': i32
            73550..73558 'offset11': i32
            73562..73586 'vec2f(...p10.x)': vec2<f32>
            73568..73576 '-amp10.y': f32
            73569..73574 'amp10': vec2<f32>
            73569..73576 'amp10.y': f32
            73578..73583 'amp10': vec2<f32>
            73578..73585 'amp10.x': f32
            73616..73628 'summed_probs': ref<function, vec4<f32>, read_write>
            73616..73631 'summed_probs[0]': ref<function, f32, read_write>
            73629..73630 '0': integer
            73636..73651 'cplxMag2(amp00)': f32
            73636..73669 'cplxMa...amp01)': f32
            73645..73650 'amp00': vec2<f32>
            73654..73669 'cplxMag2(amp01)': f32
            73663..73668 'amp01': vec2<f32>
            73688..73700 'summed_probs': ref<function, vec4<f32>, read_write>
            73688..73703 'summed_probs[1]': ref<function, f32, read_write>
            73701..73702 '1': integer
            73708..73723 'cplxMag2(amp11)': f32
            73708..73741 'cplxMa...amp10)': f32
            73717..73722 'amp11': vec2<f32>
            73726..73741 'cplxMag2(amp10)': f32
            73735..73740 'amp10': vec2<f32>
            73760..73772 'summed_probs': ref<function, vec4<f32>, read_write>
            73760..73775 'summed_probs[2]': ref<function, f32, read_write>
            73773..73774 '2': integer
            73780..73795 'cplxMag2(amp00)': f32
            73780..73813 'cplxMa...amp11)': f32
            73789..73794 'amp00': vec2<f32>
            73798..73813 'cplxMag2(amp11)': f32
            73807..73812 'amp11': vec2<f32>
            73832..73844 'summed_probs': ref<function, vec4<f32>, read_write>
            73832..73847 'summed_probs[3]': ref<function, f32, read_write>
            73845..73846 '3': integer
            73852..73867 'cplxMag2(amp01)': f32
            73852..73885 'cplxMa...amp10)': f32
            73861..73866 'amp01': vec2<f32>
            73870..73885 'cplxMag2(amp10)': f32
            73879..73884 'amp10': vec2<f32>
            74036..74042 'states': array<vec2<f32>, 4>
            74045..74381 'array<...     )': array<vec2<f32>, 4>
            74081..74092 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            74081..74135 'stateV...set00]': ref<storage, vec2<f32>, read_write>
            74093..74099 'params': ShotParams
            74093..74123 'params..._start': i32
            74093..74134 'params...fset00': i32
            74126..74134 'offset00': i32
            74157..74168 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            74157..74211 'stateV...set01]': ref<storage, vec2<f32>, read_write>
            74169..74175 'params': ShotParams
            74169..74199 'params..._start': i32
            74169..74210 'params...fset01': i32
            74202..74210 'offset01': i32
            74233..74244 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            74233..74287 'stateV...set10]': ref<storage, vec2<f32>, read_write>
            74245..74251 'params': ShotParams
            74245..74275 'params..._start': i32
            74245..74286 'params...fset10': i32
            74278..74286 'offset10': i32
            74309..74320 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            74309..74363 'stateV...set11]': ref<storage, vec2<f32>, read_write>
            74321..74327 'params': ShotParams
            74321..74351 'params..._start': i32
            74321..74362 'params...fset11': i32
            74354..74362 'offset11': i32
            74461..74469 'result00': vec2<f32>
            74472..74527 'innerP...tates)': vec2<f32>
            74485..74518 'getUni...dx, 0)': array<vec2<f32>, 4>
            74499..74505 'params': ShotParams
            74499..74514 'params.shot_idx': i32
            74516..74517 '0': integer
            74520..74526 'states': array<vec2<f32>, 4>
            74549..74557 'result01': vec2<f32>
            74560..74615 'innerP...tates)': vec2<f32>
            74573..74606 'getUni...dx, 1)': array<vec2<f32>, 4>
            74587..74593 'params': ShotParams
            74587..74602 'params.shot_idx': i32
            74604..74605 '1': integer
            74608..74614 'states': array<vec2<f32>, 4>
            74637..74645 'result10': vec2<f32>
            74648..74703 'innerP...tates)': vec2<f32>
            74661..74694 'getUni...dx, 2)': array<vec2<f32>, 4>
            74675..74681 'params': ShotParams
            74675..74690 'params.shot_idx': i32
            74692..74693 '2': integer
            74696..74702 'states': array<vec2<f32>, 4>
            74725..74733 'result11': vec2<f32>
            74736..74791 'innerP...tates)': vec2<f32>
            74749..74782 'getUni...dx, 3)': array<vec2<f32>, 4>
            74763..74769 'params': ShotParams
            74763..74778 'params.shot_idx': i32
            74780..74781 '3': integer
            74784..74790 'states': array<vec2<f32>, 4>
            74851..74862 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            74851..74905 'stateV...set00]': ref<storage, vec2<f32>, read_write>
            74863..74869 'params': ShotParams
            74863..74893 'params..._start': i32
            74863..74904 'params...fset00': i32
            74896..74904 'offset00': i32
            74908..74916 'result00': vec2<f32>
            74934..74945 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            74934..74988 'stateV...set01]': ref<storage, vec2<f32>, read_write>
            74946..74952 'params': ShotParams
            74946..74976 'params..._start': i32
            74946..74987 'params...fset01': i32
            74979..74987 'offset01': i32
            74991..74999 'result01': vec2<f32>
            75017..75028 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            75017..75071 'stateV...set10]': ref<storage, vec2<f32>, read_write>
            75029..75035 'params': ShotParams
            75029..75059 'params..._start': i32
            75029..75070 'params...fset10': i32
            75062..75070 'offset10': i32
            75074..75082 'result10': vec2<f32>
            75100..75111 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            75100..75154 'stateV...set11]': ref<storage, vec2<f32>, read_write>
            75112..75118 'params': ShotParams
            75112..75142 'params..._start': i32
            75112..75153 'params...fset11': i32
            75145..75153 'offset11': i32
            75157..75165 'result11': vec2<f32>
            75251..75263 'summed_probs': ref<function, vec4<f32>, read_write>
            75251..75266 'summed_probs[0]': ref<function, f32, read_write>
            75264..75265 '0': integer
            75271..75289 'cplxMa...ult00)': f32
            75271..75310 'cplxMa...ult01)': f32
            75280..75288 'result00': vec2<f32>
            75292..75310 'cplxMa...ult01)': f32
            75301..75309 'result01': vec2<f32>
            75329..75341 'summed_probs': ref<function, vec4<f32>, read_write>
            75329..75344 'summed_probs[1]': ref<function, f32, read_write>
            75342..75343 '1': integer
            75349..75367 'cplxMa...ult10)': f32
            75349..75388 'cplxMa...ult11)': f32
            75358..75366 'result10': vec2<f32>
            75370..75388 'cplxMa...ult11)': f32
            75379..75387 'result11': vec2<f32>
            75407..75419 'summed_probs': ref<function, vec4<f32>, read_write>
            75407..75422 'summed_probs[2]': ref<function, f32, read_write>
            75420..75421 '2': integer
            75427..75445 'cplxMa...ult00)': f32
            75427..75466 'cplxMa...ult10)': f32
            75436..75444 'result00': vec2<f32>
            75448..75466 'cplxMa...ult10)': f32
            75457..75465 'result10': vec2<f32>
            75485..75497 'summed_probs': ref<function, vec4<f32>, read_write>
            75485..75500 'summed_probs[3]': ref<function, f32, read_write>
            75498..75499 '3': integer
            75505..75523 'cplxMa...ult01)': f32
            75505..75544 'cplxMa...ult11)': f32
            75514..75522 'result01': vec2<f32>
            75526..75544 'cplxMa...ult11)': f32
            75535..75543 'result11': vec2<f32>
            75594..75605 'entry_index': ref<function, i32, read_write>
            75609..75615 'params': ShotParams
            75609..75638 'params...r_shot': i32
            75734..75746 'update_probs': bool
            75804..75822 'qubitP...lities': ref<workgroup, [error], read_write>
            75804..75827 'qubitP...s[tid]': [error]
            75804..75832 'qubitP...].zero': [error]
            75804..75836 'qubitP...ro[q1]': [error]
            75823..75826 'tid': u32
            75833..75835 'q1': u32
            75839..75851 'summed_probs': ref<function, vec4<f32>, read_write>
            75839..75854 'summed_probs[0]': ref<function, f32, read_write>
            75852..75853 '0': integer
            75864..75882 'qubitP...lities': ref<workgroup, [error], read_write>
            75864..75887 'qubitP...s[tid]': [error]
            75864..75891 'qubitP...d].one': [error]
            75864..75895 'qubitP...ne[q1]': [error]
            75883..75886 'tid': u32
            75892..75894 'q1': u32
            75899..75911 'summed_probs': ref<function, vec4<f32>, read_write>
            75899..75914 'summed_probs[1]': ref<function, f32, read_write>
            75912..75913 '1': integer
            75924..75942 'qubitP...lities': ref<workgroup, [error], read_write>
            75924..75947 'qubitP...s[tid]': [error]
            75924..75952 'qubitP...].zero': [error]
            75924..75956 'qubitP...ro[q2]': [error]
            75943..75946 'tid': u32
            75953..75955 'q2': u32
            75959..75971 'summed_probs': ref<function, vec4<f32>, read_write>
            75959..75974 'summed_probs[2]': ref<function, f32, read_write>
            75972..75973 '2': integer
            75984..76002 'qubitP...lities': ref<workgroup, [error], read_write>
            75984..76007 'qubitP...s[tid]': [error]
            75984..76011 'qubitP...d].one': [error]
            75984..76015 'qubitP...ne[q2]': [error]
            76003..76006 'tid': u32
            76012..76014 'q2': u32
            76019..76031 'summed_probs': ref<function, vec4<f32>, read_write>
            76019..76034 'summed_probs[3]': ref<function, f32, read_write>
            76032..76033 '3': integer
            76071..76082 'workgroupId': u32
            76089..76092 'tid': u32
            76109..76115 'params': ShotParams
            76118..76185 'get_sh...es */)': ShotParams
            76134..76145 'workgroupId': u32
            76147..76150 'tid': u32
            76152..76153 '0': integer
            76356..76360 'shot': ptr<storage, ShotData, read_write>
            76363..76386 '&shots...t_idx]': ptr<storage, ShotData, read_write>
            76364..76369 'shots': ref<storage, array<ShotData>, read_write>
            76364..76386 'shots[...t_idx]': ref<storage, ShotData, read_write>
            76370..76376 'params': ShotParams
            76370..76385 'params.shot_idx': i32
            76497..76510 'bit_flip_mask': u32
            76513..76544 'bitcas...[0].x)': u32
            76526..76530 'shot': ptr<storage, ShotData, read_write>
            76526..76538 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            76526..76541 'shot.unitary[0]': ref<storage, vec2<f32>, read_write>
            76526..76543 'shot.u...y[0].x': ref<storage, f32, read_write>
            76539..76540 '0': integer
            76554..76569 'phase_flip_mask': u32
            76572..76603 'bitcas...[0].y)': u32
            76585..76589 'shot': ptr<storage, ShotData, read_write>
            76585..76597 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            76585..76600 'shot.unitary[0]': ref<storage, vec2<f32>, read_write>
            76585..76602 'shot.u...y[0].y': ref<storage, f32, read_write>
            76598..76599 '0': integer
            76654..76667 'bit_flip_mask': u32
            76654..76673 'bit_fl... == 0u': bool
            76654..76698 'bit_fl... == 0u': bool
            76671..76673 '0u': u32
            76677..76692 'phase_flip_mask': u32
            76677..76698 'phase_... == 0u': bool
            76696..76698 '0u': u32
            76733..76744 'entry_index': ref<function, i32, read_write>
            76747..76753 'params': ShotParams
            76747..76772 'params...n_shot': i32
            76788..76789 'i': ref<function, i32, read_write>
            76792..76793 '0': integer
            76795..76796 'i': ref<function, i32, read_write>
            76795..76819 'i < pa...ations': bool
            76799..76805 'params': ShotParams
            76799..76819 'params...ations': i32
            76821..76822 'i': ref<function, i32, read_write>
            76950..76962 'target_index': i32
            76965..76976 'entry_index': ref<function, i32, read_write>
            76965..76997 'entry_..._mask)': i32
            76979..76997 'i32(bi..._mask)': i32
            76983..76996 'bit_flip_mask': u32
            77112..77124 'negate_index': f32
            77132..77210 'select... != 0)': float
            77139..77142 '1.0': float
            77144..77148 '-1.0': float
            77145..77148 '1.0': float
            77150..77209 '(count...) != 0': bool
            77151..77199 'countO...mask))': i32
            77151..77203 'countO...)) & 1': i32
            77164..77175 'entry_index': ref<function, i32, read_write>
            77164..77198 'entry_..._mask)': i32
            77178..77198 'i32(ph..._mask)': i32
            77182..77197 'phase_flip_mask': u32
            77202..77203 '1': integer
            77208..77209 '0': integer
            77225..77238 'bit_flip_mask': u32
            77225..77244 'bit_fl... == 0u': bool
            77225..77268 'bit_fl...= -1.0': bool
            77242..77244 '0u': u32
            77248..77260 'negate_index': f32
            77248..77268 'negate...= -1.0': bool
            77264..77268 '-1.0': float
            77265..77268 '1.0': float
            77372..77383 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            77372..77429 'stateV...index]': ref<storage, vec2<f32>, read_write>
            77384..77390 'params': ShotParams
            77384..77414 'params..._start': i32
            77384..77428 'params..._index': i32
            77417..77428 'entry_index': ref<function, i32, read_write>
            77432..77498 'cplxNe...ndex])': vec2<f32>
            77440..77451 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            77440..77497 'stateV...index]': ref<storage, vec2<f32>, read_write>
            77452..77458 'params': ShotParams
            77452..77482 'params..._start': i32
            77452..77496 'params..._index': i32
            77485..77496 'entry_index': ref<function, i32, read_write>
            77821..77830 'amp_entry': vec2<f32>
            77840..77851 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            77840..77897 'stateV...index]': ref<storage, vec2<f32>, read_write>
            77852..77858 'params': ShotParams
            77852..77882 'params..._start': i32
            77852..77896 'params..._index': i32
            77885..77896 'entry_index': ref<function, i32, read_write>
            77915..77925 'amp_target': vec2<f32>
            77935..77946 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            77935..77993 'stateV...index]': ref<storage, vec2<f32>, read_write>
            77947..77953 'params': ShotParams
            77947..77977 'params..._start': i32
            77947..77992 'params..._index': i32
            77980..77992 'target_index': i32
            78122..78135 'negate_target': f32
            78143..78222 'select... != 0)': float
            78150..78153 '1.0': float
            78155..78159 '-1.0': float
            78156..78159 '1.0': float
            78161..78221 '(count...) != 0': bool
            78162..78211 'countO...mask))': i32
            78162..78215 'countO...)) & 1': i32
            78175..78187 'target_index': i32
            78175..78210 'target..._mask)': i32
            78190..78210 'i32(ph..._mask)': i32
            78194..78209 'phase_flip_mask': u32
            78214..78215 '1': integer
            78220..78221 '0': integer
            78503..78514 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            78503..78560 'stateV...index]': ref<storage, vec2<f32>, read_write>
            78515..78521 'params': ShotParams
            78515..78545 'params..._start': i32
            78515..78559 'params..._index': i32
            78548..78559 'entry_index': ref<function, i32, read_write>
            78563..78608 'cplxMu... 0.0))': vec2<f32>
            78571..78581 'amp_target': vec2<f32>
            78583..78607 'vec2f(..., 0.0)': vec2<f32>
            78589..78601 'negate_index': f32
            78603..78606 '0.0': float
            78622..78633 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            78622..78680 'stateV...index]': ref<storage, vec2<f32>, read_write>
            78634..78640 'params': ShotParams
            78634..78664 'params..._start': i32
            78634..78679 'params..._index': i32
            78667..78679 'target_index': i32
            78683..78728 'cplxMu... 0.0))': vec2<f32>
            78691..78700 'amp_entry': vec2<f32>
            78702..78727 'vec2f(..., 0.0)': vec2<f32>
            78708..78721 'negate_target': f32
            78723..78726 '0.0': float
            78800..78811 'entry_index': ref<function, i32, read_write>
            78815..78821 'params': ShotParams
            78815..78844 'params...r_shot': i32
            79137..79145 'shot_idx': u32
            79152..79158 'op_idx': u32
            79165..79180 'noise_table_idx': u32
            79222..79226 'shot': ptr<storage, ShotData, read_write>
            79229..79245 '&shots...t_idx]': ptr<storage, ShotData, read_write>
            79230..79235 'shots': ref<storage, array<ShotData>, read_write>
            79230..79245 'shots[shot_idx]': ref<storage, ShotData, read_write>
            79236..79244 'shot_idx': u32
            79255..79260 'table': [error]
            79263..79315 '&batch...e_idx]': [error]
            79264..79274 'batch_data': ref<storage, BatchData, read>
            79264..79298 'batch_...tables': ref<storage, [error], read>
            79264..79315 'batch_...e_idx]': [error]
            79299..79314 'noise_table_idx': u32
            79482..79489 'rand_lo': u32
            79492..79515 'next_r...t_idx)': u32
            79506..79514 'shot_idx': u32
            79525..79532 'rand_hi': u32
            79535..79558 'next_r...t_idx)': u32
            79535..79572 'next_r...FFFFFu': u32
            79549..79557 'shot_idx': u32
            79561..79572 '0x7FFFFFFFu': u32
            79646..79659 'noise_prob_lo': [error]
            79662..79667 'table': [error]
            79662..79688 'table....ity_lo': [error]
            79698..79711 'noise_prob_hi': [error]
            79714..79719 'table': [error]
            79714..79740 'table....ity_hi': [error]
            79927..79934 'rand_hi': u32
            79927..79950 'rand_h...rob_hi': [error]
            79927..80008 'rand_h...ob_lo)': [error]
            79937..79950 'noise_prob_hi': [error]
            79955..79962 'rand_hi': u32
            79955..79979 'rand_h...rob_hi': [error]
            79955..80007 'rand_h...rob_lo': [error]
            79966..79979 'noise_prob_hi': [error]
            79983..79990 'rand_lo': u32
            79983..80007 'rand_l...rob_lo': [error]
            79994..80007 'noise_prob_lo': [error]
            80068..80072 'shot': ptr<storage, ShotData, read_write>
            80068..80080 'shot.op_type': ref<storage, u32, read_write>
            80083..80090 'OPID_ID': u32
            80100..80104 'shot': ptr<storage, ShotData, read_write>
            80100..80111 'shot.op_idx': ref<storage, u32, read_write>
            80114..80120 'op_idx': u32
            80130..80134 'shot': ptr<storage, ShotData, read_write>
            80130..80162 'shot.q...p_mask': ref<storage, u32, read_write>
            80165..80167 '0u': u32
            80184..80217 'Correl...u, 0u)': CorrelatedNoiseSample
            80206..80208 '0u': u32
            80210..80212 '0u': u32
            80214..80216 '0u': u32
            80317..80322 'start': i32
            80325..80348 'i32(ta...ffset)': i32
            80329..80334 'table': [error]
            80329..80347 'table....offset': [error]
            80358..80363 'count': i32
            80366..80388 'i32(ta...count)': i32
            80370..80375 'table': [error]
            80370..80387 'table...._count': [error]
            80398..80407 'entry_idx': i32
            80410..80467 'binary...count)': i32
            80436..80443 'rand_lo': u32
            80445..80452 'rand_hi': u32
            80454..80459 'start': i32
            80461..80466 'count': i32
            80477..80482 'entry': [error]
            80485..80540 '&batch...y_idx]': [error]
            80486..80496 'batch_data': ref<storage, BatchData, read>
            80486..80521 'batch_...ntries': ref<storage, [error], read>
            80486..80540 'batch_...y_idx]': [error]
            80522..80527 'start': i32
            80522..80539 'start ...ry_idx': i32
            80530..80539 'entry_idx': i32
            80554..80613 'Correl...is_hi)': CorrelatedNoiseSample
            80576..80578 '1u': u32
            80580..80585 'entry': [error]
            80580..80595 'entry.paulis_lo': [error]
            80597..80602 'entry': [error]
            80597..80612 'entry.paulis_hi': [error]
            81032..81041 'paulis_lo': u32
            81048..81057 'paulis_hi': u32
            81064..81075 'qubit_count': u32
            81082..81083 'i': u32
            81107..81119 'bit_position': u32
            81122..81149 '(qubit...) * 3u': u32
            81123..81134 'qubit_count': u32
            81123..81139 'qubit_...t - 1u': u32
            81123..81143 'qubit_...1u - i': u32
            81137..81139 '1u': u32
            81142..81143 'i': u32
            81147..81149 '3u': u32
            81159..81171 'bit_position': u32
            81159..81176 'bit_po...n + 3u': u32
            81159..81183 'bit_po...<= 32u': bool
            81174..81176 '3u': u32
            81180..81183 '32u': u32
            81202..81236 '(pauli...& 0x7u': u32
            81203..81212 'paulis_lo': u32
            81203..81228 'paulis...sition': u32
            81216..81228 'bit_position': u32
            81232..81236 '0x7u': u32
            81291..81333 '(pauli...& 0x7u': u32
            81292..81301 'paulis_hi': u32
            81292..81325 'paulis...- 32u)': u32
            81306..81318 'bit_position': u32
            81306..81324 'bit_po... - 32u': u32
            81321..81324 '32u': u32
            81329..81333 '0x7u': u32
            81438..81446 'low_part': u32
            81449..81458 'paulis_lo': u32
            81449..81474 'paulis...sition': u32
            81462..81474 'bit_position': u32
            81488..81497 'high_part': u32
            81500..81509 'paulis_hi': u32
            81500..81533 'paulis...ition)': u32
            81514..81517 '32u': u32
            81514..81532 '32u - ...sition': u32
            81520..81532 'bit_position': u32
            81550..81579 '(low_p...& 0x7u': u32
            81551..81559 'low_part': u32
            81551..81571 'low_pa...h_part': u32
            81562..81571 'high_part': u32
            81575..81579 '0x7u': u32
            81941..81949 'shot_idx': u32
            81956..81962 'op_idx': u32
            81969..81982 'bit_flip_mask': u32
            81989..82004 'phase_flip_mask': u32
            82011..82020 'loss_mask': u32
            82037..82041 'shot': ptr<storage, ShotData, read_write>
            82044..82060 '&shots...t_idx]': ptr<storage, ShotData, read_write>
            82045..82050 'shots': ref<storage, array<ShotData>, read_write>
            82045..82060 'shots[shot_idx]': ref<storage, ShotData, read_write>
            82051..82059 'shot_idx': u32
            82247..82251 'shot': ptr<storage, ShotData, read_write>
            82247..82269 'shot.p...s_mask': ref<storage, u32, read_write>
            82273..82282 'loss_mask': u32
            82434..82438 'shot': ptr<storage, ShotData, read_write>
            82434..82446 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            82434..82449 'shot.unitary[0]': ref<storage, vec2<f32>, read_write>
            82447..82448 '0': integer
            82452..82517 'vec2f(...mask))': vec2<f32>
            82458..82485 'bitcas..._mask)': f32
            82471..82484 'bit_flip_mask': u32
            82487..82516 'bitcas..._mask)': f32
            82500..82515 'phase_flip_mask': u32
            82692..82693 'q': ref<function, u32, read_write>
            82701..82703 '0u': u32
            82705..82706 'q': ref<function, u32, read_write>
            82705..82725 'q < u3...COUNT)': bool
            82709..82725 'u32(QU...COUNT)': u32
            82713..82724 'QUBIT_COUNT': i32
            82727..82728 'q': ref<function, u32, read_write>
            82746..82756 'qubit_mask': u32
            82759..82761 '1u': u32
            82759..82766 '1u << q': u32
            82765..82766 'q': ref<function, u32, read_write>
            82780..82814 '(bit_f... != 0u': bool
            82781..82794 'bit_flip_mask': u32
            82781..82807 'bit_fl...t_mask': u32
            82797..82807 'qubit_mask': u32
            82812..82814 '0u': u32
            82872..82876 'temp': [error]
            82879..82883 'shot': ptr<storage, ShotData, read_write>
            82879..82895 'shot.q..._state': ref<storage, [error], read_write>
            82879..82898 'shot.q...ate[q]': [error]
            82879..82915 'shot.q...bility': [error]
            82896..82897 'q': ref<function, u32, read_write>
            82929..82933 'shot': ptr<storage, ShotData, read_write>
            82929..82945 'shot.q..._state': ref<storage, [error], read_write>
            82929..82948 'shot.q...ate[q]': [error]
            82929..82965 'shot.q...bility': [error]
            82946..82947 'q': ref<function, u32, read_write>
            82968..82972 'shot': ptr<storage, ShotData, read_write>
            82968..82984 'shot.q..._state': ref<storage, [error], read_write>
            82968..82987 'shot.q...ate[q]': [error]
            82968..83003 'shot.q...bility': [error]
            82985..82986 'q': ref<function, u32, read_write>
            83017..83021 'shot': ptr<storage, ShotData, read_write>
            83017..83033 'shot.q..._state': ref<storage, [error], read_write>
            83017..83036 'shot.q...ate[q]': [error]
            83017..83052 'shot.q...bility': [error]
            83034..83035 'q': ref<function, u32, read_write>
            83055..83059 'temp': [error]
            83146..83151 'was_0': bool
            83154..83195 '(shot.... != 0u': bool
            83155..83159 'shot': ptr<storage, ShotData, read_write>
            83155..83175 'shot.q...0_mask': ref<storage, u32, read_write>
            83155..83188 'shot.q...t_mask': u32
            83178..83188 'qubit_mask': u32
            83193..83195 '0u': u32
            83213..83218 'was_1': bool
            83221..83262 '(shot.... != 0u': bool
            83222..83226 'shot': ptr<storage, ShotData, read_write>
            83222..83242 'shot.q...1_mask': ref<storage, u32, read_write>
            83222..83255 'shot.q...t_mask': u32
            83245..83255 'qubit_mask': u32
            83260..83262 '0u': u32
            83280..83285 'was_0': bool
            83305..83309 'shot': ptr<storage, ShotData, read_write>
            83305..83325 'shot.q...0_mask': ref<storage, u32, read_write>
            83329..83340 '~qubit_mask': u32
            83330..83340 'qubit_mask': u32
            83358..83362 'shot': ptr<storage, ShotData, read_write>
            83358..83378 'shot.q...1_mask': ref<storage, u32, read_write>
            83382..83392 'qubit_mask': u32
            83442..83446 'shot': ptr<storage, ShotData, read_write>
            83442..83462 'shot.q...1_mask': ref<storage, u32, read_write>
            83466..83477 '~qubit_mask': u32
            83467..83477 'qubit_mask': u32
            83495..83499 'shot': ptr<storage, ShotData, read_write>
            83495..83515 'shot.q...0_mask': ref<storage, u32, read_write>
            83519..83529 'qubit_mask': u32
            83630..83634 'shot': ptr<storage, ShotData, read_write>
            83630..83642 'shot.op_type': ref<storage, u32, read_write>
            83645..83666 'OPID_C..._NOISE': u32
            83672..83676 'shot': ptr<storage, ShotData, read_write>
            83672..83683 'shot.op_idx': ref<storage, u32, read_write>
            83686..83692 'op_idx': u32
            83796..83800 'shot': ptr<storage, ShotData, read_write>
            83796..83828 'shot.q...p_mask': ref<storage, u32, read_write>
            83831..83833 '0u': u32
            84609..84616 'rand_lo': u32
            84623..84630 'rand_hi': u32
            84637..84642 'start': i32
            84649..84654 'count': i32
            84678..84681 'low': ref<function, i32, read_write>
            84689..84690 '0': integer
            84700..84704 'high': ref<function, i32, read_write>
            84712..84717 'count': i32
            84731..84734 'low': ref<function, i32, read_write>
            84731..84741 'low < high': bool
            84737..84741 'high': ref<function, i32, read_write>
            84757..84760 'mid': i32
            84768..84771 'low': ref<function, i32, read_write>
            84768..84790 'low + ...w) / 2': i32
            84774..84790 '(high ...w) / 2': i32
            84775..84779 'high': ref<function, i32, read_write>
            84775..84785 'high - low': i32
            84782..84785 'low': ref<function, i32, read_write>
            84789..84790 '2': integer
            84804..84808 'p_lo': [error]
            84811..84821 'batch_data': ref<storage, BatchData, read>
            84811..84846 'batch_...ntries': ref<storage, [error], read>
            84811..84859 'batch_...+ mid]': [error]
            84811..84874 'batch_...ity_lo': [error]
            84847..84852 'start': i32
            84847..84858 'start + mid': i32
            84855..84858 'mid': i32
            84888..84892 'p_hi': [error]
            84895..84905 'batch_data': ref<storage, BatchData, read>
            84895..84930 'batch_...ntries': ref<storage, [error], read>
            84895..84943 'batch_...+ mid]': [error]
            84895..84958 'batch_...ity_hi': [error]
            84931..84936 'start': i32
            84931..84942 'start + mid': i32
            84939..84942 'mid': i32
            84973..84980 'rand_hi': u32
            84973..84987 'rand_hi < p_hi': [error]
            84973..85026 'rand_h... p_lo)': [error]
            84983..84987 'p_hi': [error]
            84992..84999 'rand_hi': u32
            84992..85007 'rand_hi == p_hi': [error]
            84992..85025 'rand_h...< p_lo': [error]
            85003..85007 'p_hi': [error]
            85011..85018 'rand_lo': u32
            85011..85025 'rand_lo < p_lo': [error]
            85021..85025 'p_lo': [error]
            85042..85046 'high': ref<function, i32, read_write>
            85049..85052 'mid': i32
            85083..85086 'low': ref<function, i32, read_write>
            85089..85092 'mid': i32
            85089..85096 'mid + 1': i32
            85095..85096 '1': integer
            85125..85128 'low': ref<function, i32, read_write>
            85314..85320 'op_idx': u32
            85327..85332 'index': u32
            85459..85466 'vec_idx': u32
            85469..85474 'index': u32
            85469..85479 'index / 2u': u32
            85477..85479 '2u': u32
            85489..85498 'component': u32
            85501..85506 'index': u32
            85501..85511 'index % 2u': u32
            85509..85511 '2u': u32
            85521..85530 'component': u32
            85521..85536 'component == 0u': bool
            85534..85536 '0u': u32
            85555..85590 'u32(op...dx].x)': u32
            85559..85562 'ops': ref<storage, array<Op>, read>
            85559..85570 'ops[op_idx]': ref<storage, Op, read>
            85559..85578 'ops[op...nitary': ref<storage, array<vec2<f32>, 16>, read>
            85559..85587 'ops[op...c_idx]': ref<storage, vec2<f32>, read>
            85559..85589 'ops[op...idx].x': ref<storage, f32, read>
            85563..85569 'op_idx': u32
            85579..85586 'vec_idx': u32
            85620..85655 'u32(op...dx].y)': u32
            85624..85627 'ops': ref<storage, array<Op>, read>
            85624..85635 'ops[op_idx]': ref<storage, Op, read>
            85624..85643 'ops[op...nitary': ref<storage, array<vec2<f32>, 16>, read>
            85624..85652 'ops[op...c_idx]': ref<storage, vec2<f32>, read>
            85624..85654 'ops[op...idx].y': ref<storage, f32, read>
            85628..85634 'op_idx': u32
            85644..85651 'vec_idx': u32
            85865..85873 'shot_idx': u32
            85880..85886 'op_idx': u32
            85903..85905 'op': ptr<storage, Op, read>
            85908..85920 '&ops[op_idx]': ptr<storage, Op, read>
            85909..85912 'ops': ref<storage, array<Op>, read>
            85909..85920 'ops[op_idx]': ref<storage, Op, read>
            85913..85919 'op_idx': u32
            85930..85945 'noise_table_idx': u32
            85948..85950 'op': ptr<storage, Op, read>
            85948..85953 'op.q1': ref<storage, u32, read>
            85963..85974 'qubit_count': u32
            85977..85979 'op': ptr<storage, Op, read>
            85977..85982 'op.q2': ref<storage, u32, read>
            85993..85999 'sample': CorrelatedNoiseSample
            86002..86060 'sample...e_idx)': CorrelatedNoiseSample
            86026..86034 'shot_idx': u32
            86036..86042 'op_idx': u32
            86044..86059 'noise_table_idx': u32
            86070..86076 'sample': CorrelatedNoiseSample
            86070..86089 'sample..._apply': u32
            86070..86095 'sample... == 0u': bool
            86093..86095 '0u': u32
            86213..86226 'bit_flip_mask': ref<function, u32, read_write>
            86234..86236 '0u': u32
            86246..86261 'phase_flip_mask': ref<function, u32, read_write>
            86269..86271 '0u': u32
            86281..86290 'loss_mask': ref<function, u32, read_write>
            86298..86300 '0u': u32
            86315..86316 'i': ref<function, u32, read_write>
            86324..86326 '0u': u32
            86328..86329 'i': ref<function, u32, read_write>
            86328..86343 'i < qubit_count': bool
            86332..86343 'qubit_count': u32
            86345..86346 'i': ref<function, u32, read_write>
            86364..86374 'pauli_bits': u32
            86377..86443 'get_pa...nt, i)': u32
            86392..86398 'sample': CorrelatedNoiseSample
            86392..86408 'sample...lis_lo': u32
            86410..86416 'sample': CorrelatedNoiseSample
            86410..86426 'sample...lis_hi': u32
            86428..86439 'qubit_count': u32
            86441..86442 'i': ref<function, u32, read_write>
            86457..86467 'qubit_mask': u32
            86470..86472 '1u': u32
            86470..86513 '1u << ...dx, i)': u32
            86476..86513 'get_co...dx, i)': u32
            86503..86509 'op_idx': u32
            86511..86512 'i': ref<function, u32, read_write>
            86527..86552 '(pauli... != 0u': bool
            86528..86538 'pauli_bits': u32
            86528..86545 'pauli_...& 0x4u': u32
            86541..86545 '0x4u': u32
            86550..86552 '0u': u32
            86648..86657 'loss_mask': ref<function, u32, read_write>
            86661..86671 'qubit_mask': u32
            86706..86731 '(pauli... != 0u': bool
            86707..86717 'pauli_bits': u32
            86707..86724 'pauli_...& 0x1u': u32
            86720..86724 '0x1u': u32
            86729..86731 '0u': u32
            86735..86748 'bit_flip_mask': ref<function, u32, read_write>
            86752..86762 'qubit_mask': u32
            86782..86807 '(pauli... != 0u': bool
            86783..86793 'pauli_bits': u32
            86783..86800 'pauli_...& 0x2u': u32
            86796..86800 '0x2u': u32
            86805..86807 '0u': u32
            86811..86826 'phase_flip_mask': ref<function, u32, read_write>
            86830..86840 'qubit_mask': u32
            86865..86949 'commit..._mask)': [error]
            86889..86897 'shot_idx': u32
            86899..86905 'op_idx': u32
            86907..86920 'bit_flip_mask': ref<function, u32, read_write>
            86922..86937 'phase_flip_mask': ref<function, u32, read_write>
            86939..86948 'loss_mask': ref<function, u32, read_write>
            87234..87242 'shot_idx': u32
            87249..87252 'reg': u32
            87279..87284 'shots': ref<storage, array<ShotData>, read_write>
            87279..87294 'shots[shot_idx]': ref<storage, ShotData, read_write>
            87279..87301 'shots[...interp': ref<storage, InterpreterState, read_write>
            87279..87311 'shots[...isters': ref<storage, [error], read_write>
            87279..87316 'shots[...s[reg]': [error]
            87285..87293 'shot_idx': u32
            87312..87315 'reg': u32
            87334..87342 'shot_idx': u32
            87349..87352 'reg': u32
            87359..87362 'val': u32
            87375..87380 'shots': ref<storage, array<ShotData>, read_write>
            87375..87390 'shots[shot_idx]': ref<storage, ShotData, read_write>
            87375..87397 'shots[...interp': ref<storage, InterpreterState, read_write>
            87375..87407 'shots[...isters': ref<storage, [error], read_write>
            87375..87412 'shots[...s[reg]': [error]
            87381..87389 'shot_idx': u32
            87408..87411 'reg': u32
            87415..87418 'val': u32
            87439..87447 'shot_idx': u32
            87454..87457 'reg': u32
            87484..87521 'bitcas... reg))': i32
            87497..87520 'read_r..., reg)': u32
            87506..87514 'shot_idx': u32
            87516..87519 'reg': u32
            87543..87551 'shot_idx': u32
            87558..87561 'reg': u32
            87568..87571 'val': i32
            87584..87627 'write_...(val))': [error]
            87594..87602 'shot_idx': u32
            87604..87607 'reg': u32
            87609..87626 'bitcas...>(val)': u32
            87622..87625 'val': i32
            87648..87656 'shot_idx': u32
            87663..87666 'reg': u32
            87693..87730 'bitcas... reg))': f32
            87706..87729 'read_r..., reg)': u32
            87715..87723 'shot_idx': u32
            87725..87728 'reg': u32
            87752..87760 'shot_idx': u32
            87767..87770 'reg': u32
            87777..87780 'val': f32
            87793..87836 'write_...(val))': [error]
            87803..87811 'shot_idx': u32
            87813..87816 'reg': u32
            87818..87835 'bitcas...>(val)': u32
            87831..87834 'val': f32
            88087..88089 'pc': u32
            88124..88134 'batch_data': ref<storage, BatchData, read>
            88124..88142 'batch_...rogram': ref<storage, Program, read>
            88124..88155 'batch_...ctions': ref<storage, [error], read>
            88124..88159 'batch_...ns[pc]': [error]
            88156..88158 'pc': u32
            88178..88184 'packed': u32
            88209..88215 'packed': u32
            88209..88223 'packed & 0xFFu': u32
            88218..88223 '0xFFu': u32
            88242..88248 'packed': u32
            88272..88294 '(packe... 0xFFu': u32
            88273..88279 'packed': u32
            88273..88285 'packed >> 8u': u32
            88283..88285 '8u': u32
            88289..88294 '0xFFu': u32
            88311..88317 'packed': u32
            88343..88366 '(packe... 0xFFu': u32
            88344..88350 'packed': u32
            88344..88357 'packed >> 16u': u32
            88354..88357 '16u': u32
            88361..88366 '0xFFu': u32
            88386..88394 'shot_idx': u32
            88401..88408 'operand': u32
            88415..88420 'flags': u32
            88427..88438 'operand_idx': u32
            88461..88496 '(flags... != 0u': bool
            88462..88467 'flags': u32
            88462..88489 'flags ...d_idx)': u32
            88471..88473 '1u': u32
            88471..88488 '1u << ...nd_idx': u32
            88477..88488 'operand_idx': u32
            88494..88496 '0u': u32
            88514..88535 'bitcas...erand)': i32
            88527..88534 'operand': u32
            88568..88599 'read_r...erand)': i32
            88581..88589 'shot_idx': u32
            88591..88598 'operand': u32
            88632..88640 'shot_idx': u32
            88647..88654 'operand': u32
            88661..88666 'flags': u32
            88673..88684 'operand_idx': u32
            88707..88742 '(flags... != 0u': bool
            88708..88713 'flags': u32
            88708..88735 'flags ...d_idx)': u32
            88717..88719 '1u': u32
            88717..88734 '1u << ...nd_idx': u32
            88723..88734 'operand_idx': u32
            88740..88742 '0u': u32
            88760..88767 'operand': u32
            88786..88813 'read_r...erand)': u32
            88795..88803 'shot_idx': u32
            88805..88812 'operand': u32
            88833..88841 'shot_idx': u32
            88848..88855 'operand': u32
            88862..88867 'flags': u32
            88874..88885 'operand_idx': u32
            88908..88943 '(flags... != 0u': bool
            88909..88914 'flags': u32
            88909..88936 'flags ...d_idx)': u32
            88918..88920 '1u': u32
            88918..88935 '1u << ...nd_idx': u32
            88924..88935 'operand_idx': u32
            88941..88943 '0u': u32
            88961..88982 'bitcas...erand)': f32
            88974..88981 'operand': u32
            89038..89069 'read_r...erand)': f32
            89051..89059 'shot_idx': u32
            89061..89068 'operand': u32
            89140..89148 'shot_idx': u32
            89172..89177 'state': InterpreterState
            89180..89185 'shots': ref<storage, array<ShotData>, read_write>
            89180..89195 'shots[shot_idx]': ref<storage, ShotData, read_write>
            89180..89202 'shots[...interp': ref<storage, InterpreterState, read_write>
            89186..89194 'shot_idx': u32
            89212..89217 'instr': Instruction
            89220..89245 'fetch_...c - 1)': Instruction
            89232..89237 'state': InterpreterState
            89232..89240 'state.pc': u32
            89232..89244 'state.pc - 1': u32
            89243..89244 '1': integer
            89254..89289 '(instr...) != 0': bool
            89255..89260 'instr': Instruction
            89255..89267 'instr.opcode': u32
            89255..89283 'instr....X1_IMM': u32
            89270..89283 'FLAG_AUX1_IMM': u32
            89288..89289 '0': integer
            89307..89312 'instr': Instruction
            89307..89317 'instr.aux1': u32
            89336..89366 'read_r....aux1)': u32
            89345..89353 'shot_idx': u32
            89355..89360 'instr': Instruction
            89355..89365 'instr.aux1': u32
            89437..89445 'shot_idx': u32
            89469..89474 'state': InterpreterState
            89477..89482 'shots': ref<storage, array<ShotData>, read_write>
            89477..89492 'shots[shot_idx]': ref<storage, ShotData, read_write>
            89477..89499 'shots[...interp': ref<storage, InterpreterState, read_write>
            89483..89491 'shot_idx': u32
            89509..89514 'instr': Instruction
            89517..89542 'fetch_...c - 1)': Instruction
            89529..89534 'state': InterpreterState
            89529..89537 'state.pc': u32
            89529..89541 'state.pc - 1': u32
            89540..89541 '1': integer
            89551..89586 '(instr...) != 0': bool
            89552..89557 'instr': Instruction
            89552..89564 'instr.opcode': u32
            89552..89580 'instr....X2_IMM': u32
            89567..89580 'FLAG_AUX2_IMM': u32
            89585..89586 '0': integer
            89604..89609 'instr': Instruction
            89604..89614 'instr.aux2': u32
            89633..89663 'read_r....aux2)': u32
            89642..89650 'shot_idx': u32
            89652..89657 'instr': Instruction
            89652..89662 'instr.aux2': u32
            89838..89846 'shot_idx': u32
            89870..89875 'state': InterpreterState
            89878..89883 'shots': ref<storage, array<ShotData>, read_write>
            89878..89893 'shots[shot_idx]': ref<storage, ShotData, read_write>
            89878..89900 'shots[...interp': ref<storage, InterpreterState, read_write>
            89884..89892 'shot_idx': u32
            89910..89915 'instr': Instruction
            89918..89943 'fetch_...c - 1)': Instruction
            89930..89935 'state': InterpreterState
            89930..89938 'state.pc': u32
            89930..89942 'state.pc - 1': u32
            89941..89942 '1': integer
            89953..89958 'flags': u32
            89961..89984 'get_fl...pcode)': u32
            89971..89976 'instr': Instruction
            89971..89983 'instr.opcode': u32
            89997..90041 'resolv...s, 0u)': f32
            90009..90017 'shot_idx': u32
            90019..90024 'instr': Instruction
            90019..90029 'instr.src0': u32
            90031..90036 'flags': u32
            90038..90040 '0u': u32
            90213..90221 'shot_idx': u32
            90228..90237 'result_id': u32
            90265..90322 'atomic...t_id])': u32
            90265..90328 'atomic... == 1u': bool
            90276..90321 '&resul...lt_id]': ptr<storage, atomic<u32>, read_write>
            90277..90284 'results': ref<storage, array<atomic<u32>>, read_write>
            90277..90321 'result...lt_id]': ref<storage, atomic<u32>, read_write>
            90285..90293 'shot_idx': u32
            90285..90308 'shot_i..._COUNT': u32
            90285..90320 'shot_i...ult_id': u32
            90296..90308 'RESULT_COUNT': u32
            90311..90320 'result_id': u32
            90326..90328 '1u': u32
            90410..90412 'id': u32
            90440..90488 '(12 <=...<= 19)': bool
            90441..90443 '12': integer
            90441..90449 '12 <= id': bool
            90441..90461 '12 <= ... <= 14': bool
            90447..90449 'id': u32
            90453..90455 'id': u32
            90453..90461 'id <= 14': bool
            90459..90461 '14': integer
            90467..90469 '17': integer
            90467..90475 '17 <= id': bool
            90467..90487 '17 <= ... <= 19': bool
            90473..90475 'id': u32
            90479..90481 'id': u32
            90479..90487 'id <= 19': bool
            90485..90487 '19': integer
            90583..90591 'shot_idx': u32
            90616..90621 'state': InterpreterState
            90624..90629 'shots': ref<storage, array<ShotData>, read_write>
            90624..90639 'shots[shot_idx]': ref<storage, ShotData, read_write>
            90624..90646 'shots[...interp': ref<storage, InterpreterState, read_write>
            90630..90638 'shot_idx': u32
            90656..90661 'instr': Instruction
            90664..90689 'fetch_...c - 1)': Instruction
            90676..90681 'state': InterpreterState
            90676..90684 'state.pc': u32
            90676..90688 'state.pc - 1': u32
            90687..90688 '1': integer
            90702..90737 '(instr...) != 0': bool
            90703..90708 'instr': Instruction
            90703..90715 'instr.opcode': u32
            90703..90731 'instr....C0_IMM': u32
            90718..90731 'FLAG_SRC0_IMM': u32
            90736..90737 '0': integer
            91041..91049 'shot_idx': u32
            91056..91061 'qubit': u32
            91078..91082 'shot': ptr<storage, ShotData, read_write>
            91085..91101 '&shots...t_idx]': ptr<storage, ShotData, read_write>
            91086..91091 'shots': ref<storage, array<ShotData>, read_write>
            91086..91101 'shots[shot_idx]': ref<storage, ShotData, read_write>
            91092..91100 'shot_idx': u32
            91111..91117 'result': [error]
            91120..91196 'select...ility)': [error]
            91127..91129 '1u': u32
            91131..91133 '0u': u32
            91135..91139 'shot': ptr<storage, ShotData, read_write>
            91135..91152 'shot.r...easure': ref<storage, f32, read_write>
            91135..91195 'shot.r...bility': [error]
            91155..91159 'shot': ptr<storage, ShotData, read_write>
            91155..91171 'shot.q..._state': ref<storage, [error], read_write>
            91155..91178 'shot.q...qubit]': [error]
            91155..91195 'shot.q...bility': [error]
            91172..91177 'qubit': u32
            91202..91206 'shot': ptr<storage, ShotData, read_write>
            91202..91218 'shot.q..._state': ref<storage, [error], read_write>
            91202..91225 'shot.q...qubit]': [error]
            91202..91230 'shot.q...].heat': [error]
            91219..91224 'qubit': u32
            91233..91237 '-1.0': float
            91234..91237 '1.0': float
            91243..91324 'prep_m...ro */)': [error]
            91273..91281 'shot_idx': u32
            91283..91288 'qubit': u32
            91290..91296 'result': [error]
            91298..91302 'true': bool
            91330..91334 'shot': ptr<storage, ShotData, read_write>
            91330..91341 'shot.op_idx': ref<storage, u32, read_write>
            91344..91349 'qubit': u32
            91399..91403 'shot': ptr<storage, ShotData, read_write>
            91399..91411 'shot.op_type': ref<storage, u32, read_write>
            91414..91429 'OPID_LOSS_NOISE': u32
            91640..91648 'shot_idx': u32
            91655..91661 'op_idx': u32
            91668..91679 'qubit_count': u32
            91686..91696 'arg_offset': u32
            91713..91728 'noise_table_idx': u32
            91731..91734 'ops': ref<storage, array<Op>, read>
            91731..91742 'ops[op_idx]': ref<storage, Op, read>
            91731..91745 'ops[op_idx].q1': ref<storage, u32, read>
            91735..91741 'op_idx': u32
            91756..91762 'sample': CorrelatedNoiseSample
            91765..91823 'sample...e_idx)': CorrelatedNoiseSample
            91789..91797 'shot_idx': u32
            91799..91805 'op_idx': u32
            91807..91822 'noise_table_idx': u32
            91833..91839 'sample': CorrelatedNoiseSample
            91833..91852 'sample..._apply': u32
            91833..91858 'sample... == 0u': bool
            91856..91858 '0u': u32
            91981..91994 'bit_flip_mask': ref<function, u32, read_write>
            92002..92004 '0u': u32
            92014..92029 'phase_flip_mask': ref<function, u32, read_write>
            92037..92039 '0u': u32
            92049..92058 'loss_mask': ref<function, u32, read_write>
            92066..92068 '0u': u32
            92083..92084 'i': ref<function, u32, read_write>
            92092..92094 '0u': u32
            92096..92097 'i': ref<function, u32, read_write>
            92096..92111 'i < qubit_count': bool
            92100..92111 'qubit_count': u32
            92113..92114 'i': ref<function, u32, read_write>
            92132..92142 'pauli_bits': u32
            92145..92211 'get_pa...nt, i)': u32
            92160..92166 'sample': CorrelatedNoiseSample
            92160..92176 'sample...lis_lo': u32
            92178..92184 'sample': CorrelatedNoiseSample
            92178..92194 'sample...lis_hi': u32
            92196..92207 'qubit_count': u32
            92209..92210 'i': ref<function, u32, read_write>
            92225..92232 'arg_reg': [error]
            92235..92245 'batch_data': ref<storage, BatchData, read>
            92235..92253 'batch_...rogram': ref<storage, Program, read>
            92235..92268 'batch_..._table': ref<storage, [error], read>
            92235..92284 'batch_...t + i]': [error]
            92269..92279 'arg_offset': u32
            92269..92283 'arg_offset + i': u32
            92282..92283 'i': ref<function, u32, read_write>
            92298..92308 'qubit_mask': u32
            92311..92313 '1u': u32
            92311..92344 '1u << ...g_reg)': u32
            92317..92344 'read_r...g_reg)': u32
            92326..92334 'shot_idx': u32
            92336..92343 'arg_reg': [error]
            92358..92383 '(pauli... != 0u': bool
            92359..92369 'pauli_bits': u32
            92359..92376 'pauli_...& 0x4u': u32
            92372..92376 '0x4u': u32
            92381..92383 '0u': u32
            92479..92488 'loss_mask': ref<function, u32, read_write>
            92492..92502 'qubit_mask': u32
            92537..92562 '(pauli... != 0u': bool
            92538..92548 'pauli_bits': u32
            92538..92555 'pauli_...& 0x1u': u32
            92551..92555 '0x1u': u32
            92560..92562 '0u': u32
            92566..92579 'bit_flip_mask': ref<function, u32, read_write>
            92583..92593 'qubit_mask': u32
            92613..92638 '(pauli... != 0u': bool
            92614..92624 'pauli_bits': u32
            92614..92631 'pauli_...& 0x2u': u32
            92627..92631 '0x2u': u32
            92636..92638 '0u': u32
            92642..92657 'phase_flip_mask': ref<function, u32, read_write>
            92661..92671 'qubit_mask': u32
            92696..92780 'commit..._mask)': [error]
            92720..92728 'shot_idx': u32
            92730..92736 'op_idx': u32
            92738..92751 'bit_flip_mask': ref<function, u32, read_write>
            92753..92768 'phase_flip_mask': ref<function, u32, read_write>
            92770..92779 'loss_mask': ref<function, u32, read_write>
            93101..93107 'params': ShotParams
            93332..93333 'i': ref<function, i32, read_write>
            93336..93337 '0': integer
            93339..93340 'i': ref<function, i32, read_write>
            93339..93363 'i < pa...ations': bool
            93343..93349 'params': ShotParams
            93343..93363 'params...ations': i32
            93365..93366 'i': ref<function, i32, read_write>
            93384..93395 'entry_index': i32
            93403..93409 'params': ShotParams
            93403..93428 'params...n_shot': i32
            93403..93464 'params...r_shot': i32
            93431..93432 'i': ref<function, i32, read_write>
            93431..93464 'i * pa...r_shot': i32
            93435..93441 'params': ShotParams
            93435..93464 'params...r_shot': i32
            93474..93485 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            93474..93531 'stateV...index]': ref<storage, vec2<f32>, read_write>
            93486..93492 'params': ShotParams
            93486..93516 'params..._start': i32
            93486..93530 'params..._index': i32
            93519..93530 'entry_index': i32
            93534..93549 'vec2f(0.0, 0.0)': vec2<f32>
            93540..93543 '0.0': float
            93545..93548 '0.0': float
            93654..93660 'params': ShotParams
            93654..93679 'params...n_shot': i32
            93654..93684 'params...t == 0': bool
            93683..93684 '0': integer
            93787..93798 'stateVector': ref<storage, array<vec2<f32>>, read_write>
            93787..93830 'stateV...start]': ref<storage, vec2<f32>, read_write>
            93799..93805 'params': ShotParams
            93799..93829 'params..._start': i32
            93833..93848 'vec2f(1.0, 0.0)': vec2<f32>
            93839..93842 '1.0': float
            93844..93847 '0.0': float
            93858..93884 'reset_...t_idx)': [error]
            93868..93874 'params': ShotParams
            93868..93883 'params.shot_idx': i32
            94298..94306 'shot_idx': u32
            94313..94319 'op_idx': u32
            94326..94328 'q1': u32
            94335..94337 'q2': u32
            94354..94358 'shot': ptr<storage, ShotData, read_write>
            94361..94377 '&shots...t_idx]': ptr<storage, ShotData, read_write>
            94362..94367 'shots': ref<storage, array<ShotData>, read_write>
            94362..94377 'shots[shot_idx]': ref<storage, ShotData, read_write>
            94368..94376 'shot_idx': u32
            94387..94389 'op': ptr<storage, Op, read>
            94392..94404 '&ops[op_idx]': ptr<storage, Op, read>
            94393..94396 'ops': ref<storage, array<Op>, read>
            94393..94404 'ops[op_idx]': ref<storage, Op, read>
            94397..94403 'op_idx': u32
            94411..94415 'shot': ptr<storage, ShotData, read_write>
            94411..94422 'shot.op_idx': ref<storage, u32, read_write>
            94425..94431 'op_idx': u32
            94437..94441 'shot': ptr<storage, ShotData, read_write>
            94437..94449 'shot.op_type': ref<storage, u32, read_write>
            94452..94454 'op': ptr<storage, Op, read>
            94452..94457 'op.id': ref<storage, u32, read>
            94596..94598 'op': ptr<storage, Op, read>
            94596..94601 'op.id': ref<storage, u32, read>
            94596..94613 'op.id ...ID_RXX': bool
            94596..94634 'op.id ...ID_RYY': bool
            94596..94657 'op.id ..._MAT2Q': bool
            94596..94679 'op.id ...D_SWAP': bool
            94605..94613 'OPID_RXX': u32
            94617..94619 'op': ptr<storage, Op, read>
            94617..94622 'op.id': ref<storage, u32, read>
            94617..94634 'op.id ...ID_RYY': bool
            94626..94634 'OPID_RYY': u32
            94638..94640 'op': ptr<storage, Op, read>
            94638..94643 'op.id': ref<storage, u32, read>
            94638..94657 'op.id ..._MAT2Q': bool
            94647..94657 'OPID_MAT2Q': u32
            94661..94663 'op': ptr<storage, Op, read>
            94661..94666 'op.id': ref<storage, u32, read>
            94661..94679 'op.id ...D_SWAP': bool
            94670..94679 'OPID_SWAP': u32
            94691..94695 'shot': ptr<storage, ShotData, read_write>
            94691..94703 'shot.op_type': ref<storage, u32, read_write>
            94706..94723 'OPID_S...UFF_2Q': u32
            94789..94791 'op': ptr<storage, Op, read>
            94789..94794 'op.id': ref<storage, u32, read>
            94789..94804 'op.id >= OPID_X': bool
            94789..94823 'op.id ...PID_CX': bool
            94798..94804 'OPID_X': u32
            94808..94810 'op': ptr<storage, Op, read>
            94808..94813 'op.id': ref<storage, u32, read>
            94808..94823 'op.id < OPID_CX': bool
            94816..94823 'OPID_CX': u32
            94835..94839 'shot': ptr<storage, ShotData, read_write>
            94835..94847 'shot.op_type': ref<storage, u32, read_write>
            94850..94867 'OPID_S...UFF_1Q': u32
            94933..94956 'is_1q_...op.id)': bool
            94950..94952 'op': ptr<storage, Op, read>
            94950..94955 'op.id': ref<storage, u32, read>
            95042..95046 'shot': ptr<storage, ShotData, read_write>
            95042..95054 'shot.op_type': ref<storage, u32, read_write>
            95057..95064 'OPID_RZ': u32
            95176..95180 'shot': ptr<storage, ShotData, read_write>
            95176..95188 'shot.op_type': ref<storage, u32, read_write>
            95202..95209 'OPID_ID': u32
            95211..95218 'OPID_CZ': u32
            95220..95227 'OPID_RZ': u32
            95229..95237 'OPID_RZZ': u32
            95248..95252 'shot': ptr<storage, ShotData, read_write>
            95248..95280 'shot.q...p_mask': ref<storage, u32, read_write>
            95283..95285 '0u': u32
            95306..95323 'OPID_S...UFF_1Q': u32
            95334..95338 'shot': ptr<storage, ShotData, read_write>
            95334..95366 'shot.q...p_mask': ref<storage, u32, read_write>
            95369..95371 '1u': u32
            95369..95377 '1u << q1': u32
            95375..95377 'q1': u32
            95398..95405 'OPID_CX': u32
            95407..95414 'OPID_CY': u32
            95416..95433 'OPID_S...UFF_2Q': u32
            95444..95448 'shot': ptr<storage, ShotData, read_write>
            95444..95476 'shot.q...p_mask': ref<storage, u32, read_write>
            95479..95502 '(1u <<...<< q2)': u32
            95480..95482 '1u': u32
            95480..95488 '1u << q1': u32
            95486..95488 'q1': u32
            95493..95495 '1u': u32
            95493..95501 '1u << q2': u32
            95499..95501 'q2': u32
            96824..96832 'shot_idx': u32
            96961..96965 'shot': ptr<storage, ShotData, read_write>
            96968..96984 '&shots...t_idx]': ptr<storage, ShotData, read_write>
            96969..96974 'shots': ref<storage, array<ShotData>, read_write>
            96969..96984 'shots[shot_idx]': ref<storage, ShotData, read_write>
            96975..96983 'shot_idx': u32
            97112..97118 'op_idx': u32
            97121..97125 'shot': ptr<storage, ShotData, read_write>
            97121..97137 'shot.n...op_idx': ref<storage, u32, read_write>
            97247..97253 'op_idx': u32
            97247..97279 'op_idx...&ops))': bool
            97257..97279 'u32(ar...&ops))': u32
            97261..97278 'arrayL...(&ops)': u32
            97273..97277 '&ops': ptr<storage, array<Op>, read>
            97274..97277 'ops': ref<storage, array<Op>, read>
            97339..97343 'shot': ptr<storage, ShotData, read_write>
            97339..97351 'shot.op_type': ref<storage, u32, read_write>
            97354..97361 'OPID_ID': u32
            97371..97375 'shot': ptr<storage, ShotData, read_write>
            97371..97387 'shot.r...malize': ref<storage, f32, read_write>
            97390..97393 '1.0': float
            97403..97407 'shot': ptr<storage, ShotData, read_write>
            97403..97435 'shot.q...p_mask': ref<storage, u32, read_write>
            97438..97440 '0u': u32
            97473..97475 'op': ptr<storage, Op, read>
            97478..97490 '&ops[op_idx]': ptr<storage, Op, read>
            97479..97482 'ops': ref<storage, array<Op>, read>
            97479..97490 'ops[op_idx]': ref<storage, Op, read>
            97483..97489 'op_idx': u32
            97587..97591 'shot': ptr<storage, ShotData, read_write>
            97587..97619 'shot.q...p_mask': ref<storage, u32, read_write>
            97587..97624 'shot.q...k != 0': bool
            97623..97624 '0': integer
            97636..97664 'update...t_idx)': [error]
            97655..97663 'shot_idx': u32
            97677..97703 'shot_i...t_idx)': [error]
            97694..97702 'shot_idx': u32
            97709..97713 'shot': ptr<storage, ShotData, read_write>
            97709..97721 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            97724..97726 'op': ptr<storage, Op, read>
            97724..97734 'op.unitary': ref<storage, array<vec2<f32>, 16>, read>
            97868..97870 'op': ptr<storage, Op, read>
            97868..97873 'op.id': ref<storage, u32, read>
            97868..97889 'op.id ...RESETZ': bool
            97877..97889 'OPID_MRESETZ': u32
            97901..98025 'prep_m...ro */)': [error]
            97920..97928 'shot_idx': u32
            97930..97936 'op_idx': u32
            97938..97940 'op': ptr<storage, Op, read>
            97938..97943 'op.q1': ref<storage, u32, read>
            97945..97947 'op': ptr<storage, Op, read>
            97945..97950 'op.q2': ref<storage, u32, read>
            97952..97957 'false': bool
            97973..97977 'true': bool
            97999..98003 'true': bool
            98035..98039 'shot': ptr<storage, ShotData, read_write>
            98035..98051 'shot.n...op_idx': ref<storage, u32, read_write>
            98054..98060 'op_idx': u32
            98054..98065 'op_idx + 1u': u32
            98063..98065 '1u': u32
            98146..98148 'op': ptr<storage, Op, read>
            98146..98151 'op.id': ref<storage, u32, read>
            98146..98162 'op.id ...PID_MZ': bool
            98155..98162 'OPID_MZ': u32
            98174..98299 'prep_m...ro */)': [error]
            98193..98201 'shot_idx': u32
            98203..98209 'op_idx': u32
            98211..98213 'op': ptr<storage, Op, read>
            98211..98216 'op.q1': ref<storage, u32, read>
            98218..98220 'op': ptr<storage, Op, read>
            98218..98223 'op.q2': ref<storage, u32, read>
            98225..98230 'false': bool
            98246..98250 'true': bool
            98272..98277 'false': bool
            98309..98313 'shot': ptr<storage, ShotData, read_write>
            98309..98325 'shot.n...op_idx': ref<storage, u32, read_write>
            98328..98334 'op_idx': u32
            98328..98339 'op_idx + 1u': u32
            98337..98339 '1u': u32
            98371..98373 'op': ptr<storage, Op, read>
            98371..98376 'op.id': ref<storage, u32, read>
            98371..98391 'op.id ...RESETZ': bool
            98380..98391 'OPID_RESETZ': u32
            98403..98528 'prep_m...ro */)': [error]
            98422..98430 'shot_idx': u32
            98432..98438 'op_idx': u32
            98440..98442 'op': ptr<storage, Op, read>
            98440..98445 'op.q1': ref<storage, u32, read>
            98447..98449 'op': ptr<storage, Op, read>
            98447..98452 'op.q2': ref<storage, u32, read>
            98454..98459 'false': bool
            98475..98480 'false': bool
            98502..98506 'true': bool
            98538..98542 'shot': ptr<storage, ShotData, read_write>
            98538..98554 'shot.n...op_idx': ref<storage, u32, read_write>
            98557..98563 'op_idx': u32
            98557..98568 'op_idx + 1u': u32
            98566..98568 '1u': u32
            98751..98753 'op': ptr<storage, Op, read>
            98751..98756 'op.id': ref<storage, u32, read>
            98751..98775 'op.id ..._NOISE': bool
            98760..98775 'OPID_LOSS_NOISE': u32
            98787..98791 'shot': ptr<storage, ShotData, read_write>
            98787..98803 'shot.n...op_idx': ref<storage, u32, read_write>
            98806..98812 'op_idx': u32
            98806..98817 'op_idx + 1u': u32
            98815..98817 '1u': u32
            98831..98839 'loss_bit': u32
            98842..98844 '1u': u32
            98842..98853 '1u << op.q1': u32
            98848..98850 'op': ptr<storage, Op, read>
            98848..98853 'op.q1': ref<storage, u32, read>
            98867..98908 '(shot.... != 0u': bool
            98868..98872 'shot': ptr<storage, ShotData, read_write>
            98868..98890 'shot.p...s_mask': ref<storage, u32, read_write>
            98868..98901 'shot.p...ss_bit': u32
            98893..98901 'loss_bit': u32
            98906..98908 '0u': u32
            98924..98928 'shot': ptr<storage, ShotData, read_write>
            98924..98946 'shot.p...s_mask': ref<storage, u32, read_write>
            98950..98959 '~loss_bit': u32
            98951..98959 'loss_bit': u32
            98973..99097 'prep_m...ro */)': [error]
            98992..99000 'shot_idx': u32
            99002..99008 'op_idx': u32
            99010..99012 'op': ptr<storage, Op, read>
            99010..99015 'op.q1': ref<storage, u32, read>
            99017..99019 'op': ptr<storage, Op, read>
            99017..99022 'op.q2': ref<storage, u32, read>
            99024..99028 'true': bool
            99044..99049 'false': bool
            99071..99075 'true': bool
            99128..99132 'shot': ptr<storage, ShotData, read_write>
            99128..99140 'shot.op_type': ref<storage, u32, read_write>
            99143..99150 'OPID_ID': u32
            99164..99168 'shot': ptr<storage, ShotData, read_write>
            99164..99175 'shot.op_idx': ref<storage, u32, read_write>
            99178..99184 'op_idx': u32
            99198..99202 'shot': ptr<storage, ShotData, read_write>
            99198..99230 'shot.q...p_mask': ref<storage, u32, read_write>
            99233..99235 '0u': u32
            99736..99748 'pauli_op_idx': u32
            99751..99778 'get_pa...p_idx)': u32
            99771..99777 'op_idx': u32
            99946..99950 'shot': ptr<storage, ShotData, read_write>
            99946..99962 'shot.n...op_idx': ref<storage, u32, read_write>
            99965..99990 'max(op...p_idx)': u32
            99965..99995 'max(op...) + 1u': u32
            99969..99975 'op_idx': u32
            99977..99989 'pauli_op_idx': u32
            99993..99995 '1u': u32
            100048..100050 'op': ptr<storage, Op, read>
            100048..100053 'op.id': ref<storage, u32, read>
            100048..100078 'op.id ..._NOISE': bool
            100057..100078 'OPID_C..._NOISE': u32
            100090..100129 'prep_c...p_idx)': [error]
            100112..100120 'shot_idx': u32
            100122..100128 'op_idx': u32
            100305..100321 'has_lo...perand': bool
            100324..100377 'gate_h...op.q2)': bool
            100346..100354 'shot_idx': u32
            100356..100362 'op_idx': u32
            100364..100366 'op': ptr<storage, Op, read>
            100364..100369 'op.q1': ref<storage, u32, read>
            100371..100373 'op': ptr<storage, Op, read>
            100371..100376 'op.q2': ref<storage, u32, read>
            100387..100403 'has_lo...perand': bool
            100415..100473 'handle...op.q2)': [error]
            100442..100450 'shot_idx': u32
            100452..100458 'op_idx': u32
            100460..100462 'op': ptr<storage, Op, read>
            100460..100465 'op.q1': ref<storage, u32, read>
            100467..100469 'op': ptr<storage, Op, read>
            100467..100472 'op.q2': ref<storage, u32, read>
            100489..100501 'pauli_op_idx': u32
            100489..100506 'pauli_...x != 0': bool
            100505..100506 '0': integer
            100520..100523 'ops': ref<storage, array<Op>, read>
            100520..100537 'ops[pa...p_idx]': ref<storage, Op, read>
            100520..100540 'ops[pa...dx].id': ref<storage, u32, read>
            100520..100563 'ops[pa...ISE_1Q': bool
            100524..100536 'pauli_op_idx': u32
            100544..100563 'OPID_P...ISE_1Q': u32
            100734..100751 '!has_l...perand': bool
            100735..100751 'has_lo...perand': bool
            100771..100830 'apply_...op.q1)': [error]
            100792..100800 'shot_idx': u32
            100802..100808 'op_idx': u32
            100810..100822 'pauli_op_idx': u32
            100824..100826 'op': ptr<storage, Op, read>
            100824..100829 'op.q1': ref<storage, u32, read>
            100899..100915 'has_lo...perand': bool
            101097..101175 'apply_...op.q2)': [error]
            101130..101138 'shot_idx': u32
            101140..101146 'op_idx': u32
            101148..101160 'pauli_op_idx': u32
            101162..101164 'op': ptr<storage, Op, read>
            101162..101167 'op.q1': ref<storage, u32, read>
            101169..101171 'op': ptr<storage, Op, read>
            101169..101174 'op.q2': ref<storage, u32, read>
            101214..101280 'apply_...op.q2)': [error]
            101235..101243 'shot_idx': u32
            101245..101251 'op_idx': u32
            101253..101265 'pauli_op_idx': u32
            101267..101269 'op': ptr<storage, Op, read>
            101267..101272 'op.q1': ref<storage, u32, read>
            101274..101276 'op': ptr<storage, Op, read>
            101274..101279 'op.q2': ref<storage, u32, read>
            101489..101505 'has_lo...perand': bool
            101607..101655 'finali...op.q2)': [error]
            101624..101632 'shot_idx': u32
            101634..101640 'op_idx': u32
            101642..101644 'op': ptr<storage, Op, read>
            101642..101647 'op.q1': ref<storage, u32, read>
            101649..101651 'op': ptr<storage, Op, read>
            101649..101654 'op.q2': ref<storage, u32, read>
            101797..101808 'workgroupId': vec3<u32>
            101862..101865 'tid': u32
            101904..101910 'params': ShotParams
            101913..101971 'get_sh...op */)': ShotParams
            101929..101940 'workgroupId': vec3<u32>
            101929..101942 'workgroupId.x': u32
            101944..101947 'tid': u32
            101949..101950 '0': integer
            102059..102084 'init_s...arams)': [error]
            102077..102083 'params': ShotParams
            102172..102183 'IS_ADAPTIVE': bool
            102172..102217 'IS_ADA...t == 0': bool
            102187..102193 'params': ShotParams
            102187..102212 'params...n_shot': i32
            102187..102217 'params...t == 0': bool
            102216..102217 '0': integer
            102382..102394 'results_base': u32
            102397..102417 'u32(pa...t_idx)': u32
            102397..102432 'u32(pa..._COUNT': u32
            102401..102407 'params': ShotParams
            102401..102416 'params.shot_idx': i32
            102420..102432 'RESULT_COUNT': u32
            102451..102452 'r': ref<function, u32, read_write>
            102455..102457 '0u': u32
            102459..102460 'r': ref<function, u32, read_write>
            102459..102475 'r < RE..._COUNT': bool
            102463..102475 'RESULT_COUNT': u32
            102477..102478 'r': ref<function, u32, read_write>
            102496..102539 'atomic...], 0u)': [error]
            102508..102534 '&resul...e + r]': ptr<storage, atomic<u32>, read_write>
            102509..102516 'results': ref<storage, array<atomic<u32>>, read_write>
            102509..102534 'result...e + r]': ref<storage, atomic<u32>, read_write>
            102517..102529 'results_base': u32
            102517..102533 'result...se + r': u32
            102532..102533 'r': ref<function, u32, read_write>
            102536..102538 '0u': u32
            102617..102618 'm': ref<function, u32, read_write>
            102621..102623 '0u': u32
            102625..102626 'm': ref<function, u32, read_write>
            102625..102647 'm < CO...A_SIZE': bool
            102629..102647 'CONSTA...A_SIZE': u32
            102649..102650 'm': ref<function, u32, read_write>
            102668..102673 'shots': ref<storage, array<ShotData>, read_write>
            102668..102690 'shots[...t_idx]': ref<storage, ShotData, read_write>
            102668..102697 'shots[...interp': ref<storage, InterpreterState, read_write>
            102668..102704 'shots[...memory': ref<storage, [error], read_write>
            102668..102707 'shots[...ory[m]': [error]
            102674..102680 'params': ShotParams
            102674..102689 'params.shot_idx': i32
            102705..102706 'm': ref<function, u32, read_write>
            102710..102720 'batch_data': ref<storage, BatchData, read>
            102710..102728 'batch_...rogram': ref<storage, Program, read>
            102710..102742 'batch_...t_data': ref<storage, [error], read>
            102710..102745 'batch_...ata[m]': [error]
            102743..102744 'm': ref<function, u32, read_write>
            102827..102828 'm': ref<function, u32, read_write>
            102831..102849 'CONSTA...A_SIZE': u32
            102851..102852 'm': ref<function, u32, read_write>
            102851..102865 'm < MAX_MEMORY': bool
            102855..102865 'MAX_MEMORY': u32
            102867..102868 'm': ref<function, u32, read_write>
            102886..102891 'shots': ref<storage, array<ShotData>, read_write>
            102886..102908 'shots[...t_idx]': ref<storage, ShotData, read_write>
            102886..102915 'shots[...interp': ref<storage, InterpreterState, read_write>
            102886..102922 'shots[...memory': ref<storage, [error], read_write>
            102886..102925 'shots[...ory[m]': [error]
            102892..102898 'params': ShotParams
            102892..102907 'params.shot_idx': i32
            102923..102924 'm': ref<function, u32, read_write>
            102928..102930 '0u': u32
            105562..105565 'gid': vec3<u32>
            105702..105710 'shot_idx': u32
            105713..105716 'gid': vec3<u32>
            105713..105718 'gid.x': u32
            105728..105733 'state': InterpreterState
            105736..105741 'shots': ref<storage, array<ShotData>, read_write>
            105736..105751 'shots[shot_idx]': ref<storage, ShotData, read_write>
            105736..105758 'shots[...interp': ref<storage, InterpreterState, read_write>
            105742..105750 'shot_idx': u32
            105836..105842 'status': u32
            105845..105850 'state': InterpreterState
            105845..105857 'state.status': u32
            105866..105872 'status': u32
            105866..105893 'status...INATED': bool
            105866..105919 'status..._ERROR': bool
            105876..105893 'STATUS...INATED': u32
            105897..105903 'status': u32
            105897..105919 'status..._ERROR': bool
            105907..105919 'STATUS_ERROR': u32
            106340..106345 'shots': ref<storage, array<ShotData>, read_write>
            106340..106355 'shots[shot_idx]': ref<storage, ShotData, read_write>
            106340..106373 'shots[...s_mask': ref<storage, u32, read_write>
            106340..106379 'shots[... != 0u': bool
            106346..106354 'shot_idx': u32
            106377..106379 '0u': u32
            106394..106395 'q': u32
            106398..106449 'firstT..._mask)': u32
            106415..106420 'shots': ref<storage, array<ShotData>, read_write>
            106415..106430 'shots[shot_idx]': ref<storage, ShotData, read_write>
            106415..106448 'shots[...s_mask': ref<storage, u32, read_write>
            106421..106429 'shot_idx': u32
            106459..106464 'shots': ref<storage, array<ShotData>, read_write>
            106459..106474 'shots[shot_idx]': ref<storage, ShotData, read_write>
            106459..106492 'shots[...s_mask': ref<storage, u32, read_write>
            106465..106473 'shot_idx': u32
            106496..106506 '~(1u << q)': u32
            106498..106500 '1u': u32
            106498..106505 '1u << q': u32
            106504..106505 'q': u32
            106516..106521 'shots': ref<storage, array<ShotData>, read_write>
            106516..106531 'shots[shot_idx]': ref<storage, ShotData, read_write>
            106516..106538 'shots[...interp': ref<storage, InterpreterState, read_write>
            106516..106553 'shots[...op_idx': ref<storage, u32, read_write>
            106522..106530 'shot_idx': u32
            106556..106557 'q': u32
            106567..106572 'shots': ref<storage, array<ShotData>, read_write>
            106567..106582 'shots[shot_idx]': ref<storage, ShotData, read_write>
            106567..106589 'shots[...interp': ref<storage, InterpreterState, read_write>
            106567..106605 'shots[...p_type': ref<storage, u32, read_write>
            106573..106581 'shot_idx': u32
            106608..106630 'PENDIN...COMMIT': u32
            106640..106645 'shots': ref<storage, array<ShotData>, read_write>
            106640..106655 'shots[shot_idx]': ref<storage, ShotData, read_write>
            106640..106662 'shots[...interp': ref<storage, InterpreterState, read_write>
            106640..106669 'shots[...status': ref<storage, u32, read_write>
            106646..106654 'shot_idx': u32
            106672..106694 'STATUS...ENDING': u32
            106941..106947 'status': u32
            106941..106965 'status...UNNING': bool
            106951..106965 'STATUS_RUNNING': u32
            106976..106981 'shots': ref<storage, array<ShotData>, read_write>
            106976..106991 'shots[shot_idx]': ref<storage, ShotData, read_write>
            106976..106998 'shots[...interp': ref<storage, InterpreterState, read_write>
            106976..107005 'shots[...status': ref<storage, u32, read_write>
            106982..106990 'shot_idx': u32
            107008..107022 'STATUS_RUNNING': u32
            107269..107271 'pc': ref<function, u32, read_write>
            107279..107284 'state': InterpreterState
            107279..107287 'state.pc': u32
            107317..107325 'block_id': ref<function, u32, read_write>
            107333..107338 'state': InterpreterState
            107333..107355 'state....ock_id': u32
            107365..107375 'prev_block': ref<function, u32, read_write>
            107383..107388 'state': InterpreterState
            107383..107406 'state....ock_id': u32
            107427..107432 'steps': ref<function, u32, read_write>
            107440..107442 '0u': u32
            107510..107522 'should_break': ref<function, bool, read_write>
            107531..107536 'false': bool
            108011..108016 'steps': ref<function, u32, read_write>
            108011..108039 'steps ..._STEPS': bool
            108020..108039 'MAX_CL..._STEPS': u32
            108187..108192 'state': InterpreterState
            108187..108199 'state.status': u32
            108187..108215 'state...._ERROR': bool
            108203..108215 'STATUS_ERROR': u32
            108234..108239 'shots': ref<storage, array<ShotData>, read_write>
            108234..108249 'shots[shot_idx]': ref<storage, ShotData, read_write>
            108234..108256 'shots[...interp': ref<storage, InterpreterState, read_write>
            108234..108263 'shots[...status': ref<storage, u32, read_write>
            108240..108248 'shot_idx': u32
            108266..108278 'STATUS_YIELD': u32
            108468..108473 'instr': Instruction
            108476..108491 'fetch_instr(pc)': Instruction
            108488..108490 'pc': ref<function, u32, read_write>
            108978..108980 'op': u32
            108983..109007 'get_op...pcode)': u32
            108994..108999 'instr': Instruction
            108994..109006 'instr.opcode': u32
            109021..109028 'subcond': u32
            109031..109056 'get_su...pcode)': u32
            109043..109048 'instr': Instruction
            109043..109055 'instr.opcode': u32
            109070..109075 'flags': u32
            109078..109101 'get_fl...pcode)': u32
            109088..109093 'instr': Instruction
            109088..109100 'instr.opcode': u32
            109902..109904 'op': u32
            110179..110185 'OP_NOP': u32
            110204..110206 'pc': ref<function, u32, read_write>
            110784..110790 'OP_RET': u32
            110813..110822 'exit_code': u32
            110825..110868 'resolv...s, 2u)': u32
            110837..110845 'shot_idx': u32
            110847..110852 'instr': Instruction
            110847..110856 'instr.dst': u32
            110858..110863 'flags': u32
            110865..110867 '2u': u32
            110886..110891 'shots': ref<storage, array<ShotData>, read_write>
            110886..110901 'shots[shot_idx]': ref<storage, ShotData, read_write>
            110886..110908 'shots[...interp': ref<storage, InterpreterState, read_write>
            110886..110918 'shots[...t_code': ref<storage, u32, read_write>
            110892..110900 'shot_idx': u32
            110921..110930 'exit_code': u32
            111107..111116 'err_index': u32
            111119..111148 '(shot_..._COUNT': u32
            111119..111152 '(shot_...NT - 1': u32
            111120..111128 'shot_idx': u32
            111120..111132 'shot_idx + 1': u32
            111131..111132 '1': integer
            111136..111148 'RESULT_COUNT': u32
            111151..111152 '1': integer
            111170..111231 'atomic..._code)': __atomic_compare_exchange_result
            111196..111215 '&resul...index]': ptr<storage, atomic<u32>, read_write>
            111197..111204 'results': ref<storage, array<atomic<u32>>, read_write>
            111197..111215 'result...index]': ref<storage, atomic<u32>, read_write>
            111205..111214 'err_index': u32
            111217..111219 '0u': u32
            111221..111230 'exit_code': u32
            111249..111254 'shots': ref<storage, array<ShotData>, read_write>
            111249..111264 'shots[shot_idx]': ref<storage, ShotData, read_write>
            111249..111271 'shots[...interp': ref<storage, InterpreterState, read_write>
            111249..111278 'shots[...status': ref<storage, u32, read_write>
            111255..111263 'shot_idx': u32
            111281..111298 'STATUS...INATED': u32
            111316..111361 'atomic...t, 1u)': u32
            111326..111356 '&diagn..._count': ptr<storage, atomic<u32>, read_write>
            111327..111338 'diagnostics': ref<storage, DiagnosticData, read_write>
            111327..111356 'diagno..._count': ref<storage, atomic<u32>, read_write>
            111358..111360 '1u': u32
            111379..111391 'should_break': ref<function, bool, read_write>
            111394..111398 'true': bool
            111750..111757 'OP_JUMP': u32
            111776..111786 'prev_block': ref<function, u32, read_write>
            111789..111797 'block_id': ref<function, u32, read_write>
            111815..111823 'block_id': ref<function, u32, read_write>
            111826..111831 'instr': Instruction
            111826..111835 'instr.dst': u32
            111853..111855 'pc': ref<function, u32, read_write>
            111858..111868 'batch_data': ref<storage, BatchData, read>
            111858..111876 'batch_...rogram': ref<storage, Program, read>
            111858..111888 'batch_..._table': ref<storage, [error], read>
            111858..111899 'batch_...r.dst]': [error]
            111858..111912 'batch_...offset': [error]
            111889..111894 'instr': Instruction
            111889..111898 'instr.dst': u32
            112328..112337 'OP_BRANCH': u32
            112360..112364 'cond': bool
            112367..112411 'resolv...s, 0u)': u32
            112367..112417 'resolv... != 0u': bool
            112379..112387 'shot_idx': u32
            112389..112394 'instr': Instruction
            112389..112399 'instr.src0': u32
            112401..112406 'flags': u32
            112408..112410 '0u': u32
            112415..112417 '0u': u32
            112435..112445 'prev_block': ref<function, u32, read_write>
            112448..112456 'block_id': ref<function, u32, read_write>
            112477..112481 'cond': bool
            112504..112512 'block_id': ref<function, u32, read_write>
            112515..112520 'instr': Instruction
            112515..112525 'instr.aux0': u32
            112547..112549 'pc': ref<function, u32, read_write>
            112552..112562 'batch_data': ref<storage, BatchData, read>
            112552..112570 'batch_...rogram': ref<storage, Program, read>
            112552..112582 'batch_..._table': ref<storage, [error], read>
            112552..112594 'batch_....aux0]': [error]
            112552..112607 'batch_...offset': [error]
            112583..112588 'instr': Instruction
            112583..112593 'instr.aux0': u32
            112654..112662 'block_id': ref<function, u32, read_write>
            112665..112670 'instr': Instruction
            112665..112675 'instr.aux1': u32
            112697..112699 'pc': ref<function, u32, read_write>
            112702..112712 'batch_data': ref<storage, BatchData, read>
            112702..112720 'batch_...rogram': ref<storage, Program, read>
            112702..112732 'batch_..._table': ref<storage, [error], read>
            112702..112744 'batch_....aux1]': [error]
            112702..112757 'batch_...offset': [error]
            112733..112738 'instr': Instruction
            112733..112743 'instr.aux1': u32
            113324..113333 'OP_SWITCH': u32
            113356..113359 'val': u32
            113362..113406 'resolv...s, 0u)': u32
            113374..113382 'shot_idx': u32
            113384..113389 'instr': Instruction
            113384..113394 'instr.src0': u32
            113396..113401 'flags': u32
            113403..113405 '0u': u32
            113428..113441 'default_block': u32
            113444..113449 'instr': Instruction
            113444..113454 'instr.aux0': u32
            113476..113487 'case_offset': u32
            113490..113495 'instr': Instruction
            113490..113500 'instr.aux1': u32
            113522..113532 'case_count': u32
            113535..113540 'instr': Instruction
            113535..113545 'instr.aux2': u32
            113567..113579 'target_block': ref<function, u32, read_write>
            113582..113595 'default_block': u32
            113622..113623 'i': ref<function, u32, read_write>
            113626..113628 '0u': u32
            113630..113631 'i': ref<function, u32, read_write>
            113630..113644 'i < case_count': bool
            113634..113644 'case_count': u32
            113646..113647 'i': ref<function, u32, read_write>
            113677..113682 'entry': [error]
            113685..113695 'batch_data': ref<storage, BatchData, read>
            113685..113703 'batch_...rogram': ref<storage, Program, read>
            113685..113716 'batch_..._table': ref<storage, [error], read>
            113685..113733 'batch_...t + i]': [error]
            113717..113728 'case_offset': u32
            113717..113732 'case_offset + i': u32
            113731..113732 'i': ref<function, u32, read_write>
            113758..113763 'entry': [error]
            113758..113772 'entry.case_val': [error]
            113758..113779 'entry....== val': [error]
            113776..113779 'val': u32
            113806..113818 'target_block': ref<function, u32, read_write>
            113821..113826 'entry': [error]
            113821..113839 'entry...._block': [error]
            113928..113938 'prev_block': ref<function, u32, read_write>
            113941..113949 'block_id': ref<function, u32, read_write>
            113967..113975 'block_id': ref<function, u32, read_write>
            113978..113990 'target_block': ref<function, u32, read_write>
            114008..114010 'pc': ref<function, u32, read_write>
            114013..114023 'batch_data': ref<storage, BatchData, read>
            114013..114031 'batch_...rogram': ref<storage, Program, read>
            114013..114043 'batch_..._table': ref<storage, [error], read>
            114013..114057 'batch_...block]': [error]
            114013..114070 'batch_...offset': [error]
            114044..114056 'target_block': ref<function, u32, read_write>
            115040..115047 'OP_CALL': u32
            115070..115077 'func_id': u32
            115080..115085 'instr': Instruction
            115080..115090 'instr.aux0': u32
            115112..115121 'arg_count': u32
            115124..115129 'instr': Instruction
            115124..115134 'instr.aux1': u32
            115156..115166 'arg_offset': u32
            115169..115174 'instr': Instruction
            115169..115179 'instr.aux2': u32
            115201..115205 'func': [error]
            115208..115218 'batch_data': ref<storage, BatchData, read>
            115208..115226 'batch_...rogram': ref<storage, Program, read>
            115208..115241 'batch_..._table': ref<storage, [error], read>
            115208..115250 'batch_...nc_id]': [error]
            115242..115249 'func_id': u32
            115328..115330 'sp': u32
            115333..115338 'shots': ref<storage, array<ShotData>, read_write>
            115333..115348 'shots[shot_idx]': ref<storage, ShotData, read_write>
            115333..115355 'shots[...interp': ref<storage, InterpreterState, read_write>
            115333..115363 'shots[...all_sp': ref<storage, u32, read_write>
            115339..115347 'shot_idx': u32
            115453..115455 'sp': u32
            115453..115461 'sp >= 8u': bool
            115459..115461 '8u': u32
            115484..115489 'shots': ref<storage, array<ShotData>, read_write>
            115484..115499 'shots[shot_idx]': ref<storage, ShotData, read_write>
            115484..115506 'shots[...interp': ref<storage, InterpreterState, read_write>
            115484..115516 'shots[...t_code': ref<storage, u32, read_write>
            115490..115498 'shot_idx': u32
            115519..115542 'ERR_CA...ERFLOW': u32
            115568..115575 'err_idx': u32
            115578..115607 '(shot_..._COUNT': u32
            115578..115611 '(shot_...NT - 1': u32
            115579..115587 'shot_idx': u32
            115579..115591 'shot_idx + 1': u32
            115590..115591 '1': integer
            115595..115607 'RESULT_COUNT': u32
            115610..115611 '1': integer
            115633..115706 'atomic...RFLOW)': __atomic_compare_exchange_result
            115659..115676 '&resul...r_idx]': ptr<storage, atomic<u32>, read_write>
            115660..115667 'results': ref<storage, array<atomic<u32>>, read_write>
            115660..115676 'result...r_idx]': ref<storage, atomic<u32>, read_write>
            115668..115675 'err_idx': u32
            115678..115680 '0u': u32
            115682..115705 'ERR_CA...ERFLOW': u32
            115728..115733 'shots': ref<storage, array<ShotData>, read_write>
            115728..115743 'shots[shot_idx]': ref<storage, ShotData, read_write>
            115728..115750 'shots[...interp': ref<storage, InterpreterState, read_write>
            115728..115757 'shots[...status': ref<storage, u32, read_write>
            115734..115742 'shot_idx': u32
            115760..115772 'STATUS_ERROR': u32
            115794..115839 'atomic...t, 1u)': u32
            115804..115834 '&diagn..._count': ptr<storage, atomic<u32>, read_write>
            115805..115816 'diagnostics': ref<storage, DiagnosticData, read_write>
            115805..115834 'diagno..._count': ref<storage, atomic<u32>, read_write>
            115836..115838 '1u': u32
            115861..115873 'should_break': ref<function, bool, read_write>
            115876..115880 'true': bool
            115943..115948 'shots': ref<storage, array<ShotData>, read_write>
            115943..115958 'shots[shot_idx]': ref<storage, ShotData, read_write>
            115943..115965 'shots[...interp': ref<storage, InterpreterState, read_write>
            115943..115983 'shots[...frames': ref<storage, array<CallStackFrame, 14>, read_write>
            115943..115987 'shots[...es[sp]': ref<storage, CallStackFrame, read_write>
            115943..115996 'shots[...ock_id': ref<storage, u32, read_write>
            115949..115957 'shot_idx': u32
            115984..115986 'sp': u32
            115999..116007 'block_id': ref<function, u32, read_write>
            116070..116075 'shots': ref<storage, array<ShotData>, read_write>
            116070..116085 'shots[shot_idx]': ref<storage, ShotData, read_write>
            116070..116092 'shots[...interp': ref<storage, InterpreterState, read_write>
            116070..116110 'shots[...frames': ref<storage, array<CallStackFrame, 14>, read_write>
            116070..116114 'shots[...es[sp]': ref<storage, CallStackFrame, read_write>
            116070..116124 'shots[...urn_pc': ref<storage, u32, read_write>
            116076..116084 'shot_idx': u32
            116111..116113 'sp': u32
            116127..116129 'pc': ref<function, u32, read_write>
            116127..116134 'pc + 1u': u32
            116132..116134 '1u': u32
            116199..116204 'shots': ref<storage, array<ShotData>, read_write>
            116199..116214 'shots[shot_idx]': ref<storage, ShotData, read_write>
            116199..116221 'shots[...interp': ref<storage, InterpreterState, read_write>
            116199..116239 'shots[...frames': ref<storage, array<CallStackFrame, 14>, read_write>
            116199..116243 'shots[...es[sp]': ref<storage, CallStackFrame, read_write>
            116199..116254 'shots[...rn_reg': ref<storage, u32, read_write>
            116205..116213 'shot_idx': u32
            116240..116242 'sp': u32
            116257..116262 'instr': Instruction
            116257..116266 'instr.dst': u32
            116324..116329 'shots': ref<storage, array<ShotData>, read_write>
            116324..116339 'shots[shot_idx]': ref<storage, ShotData, read_write>
            116324..116346 'shots[...interp': ref<storage, InterpreterState, read_write>
            116324..116354 'shots[...all_sp': ref<storage, u32, read_write>
            116330..116338 'shot_idx': u32
            116357..116359 'sp': u32
            116357..116364 'sp + 1u': u32
            116362..116364 '1u': u32
            116465..116475 'param_base': [error]
            116478..116482 'func': [error]
            116478..116497 'func.p...se_reg': [error]
            116524..116525 'i': ref<function, u32, read_write>
            116528..116530 '0u': u32
            116532..116533 'i': ref<function, u32, read_write>
            116532..116545 'i < arg_count': bool
            116536..116545 'arg_count': u32
            116547..116548 'i': ref<function, u32, read_write>
            116578..116585 'arg_reg': [error]
            116588..116598 'batch_data': ref<storage, BatchData, read>
            116588..116606 'batch_...rogram': ref<storage, Program, read>
            116588..116621 'batch_..._table': ref<storage, [error], read>
            116588..116637 'batch_...t + i]': [error]
            116622..116632 'arg_offset': u32
            116622..116636 'arg_offset + i': u32
            116635..116636 'i': ref<function, u32, read_write>
            116659..116723 'write_..._reg))': [error]
            116669..116677 'shot_idx': u32
            116679..116689 'param_base': [error]
            116679..116693 'param_base + i': [error]
            116692..116693 'i': ref<function, u32, read_write>
            116695..116722 'read_r...g_reg)': u32
            116704..116712 'shot_idx': u32
            116714..116721 'arg_reg': [error]
            116823..116831 'block_id': ref<function, u32, read_write>
            116834..116838 'func': [error]
            116834..116853 'func.e...ock_id': [error]
            116871..116873 'pc': ref<function, u32, read_write>
            116876..116886 'batch_data': ref<storage, BatchData, read>
            116876..116894 'batch_...rogram': ref<storage, Program, read>
            116876..116906 'batch_..._table': ref<storage, [error], read>
            116876..116916 'batch_...ck_id]': [error]
            116876..116929 'batch_...offset': [error]
            116907..116915 'block_id': ref<function, u32, read_write>
            117368..117382 'OP_CALL_RETURN': u32
            117404..117409 'shots': ref<storage, array<ShotData>, read_write>
            117404..117419 'shots[shot_idx]': ref<storage, ShotData, read_write>
            117404..117426 'shots[...interp': ref<storage, InterpreterState, read_write>
            117404..117434 'shots[...all_sp': ref<storage, u32, read_write>
            117404..117440 'shots[... == 0u': bool
            117410..117418 'shot_idx': u32
            117438..117440 '0u': u32
            117463..117468 'shots': ref<storage, array<ShotData>, read_write>
            117463..117478 'shots[shot_idx]': ref<storage, ShotData, read_write>
            117463..117485 'shots[...interp': ref<storage, InterpreterState, read_write>
            117463..117495 'shots[...t_code': ref<storage, u32, read_write>
            117469..117477 'shot_idx': u32
            117498..117522 'ERR_CA...ERFLOW': u32
            117548..117555 'err_idx': u32
            117558..117587 '(shot_..._COUNT': u32
            117558..117591 '(shot_...NT - 1': u32
            117559..117567 'shot_idx': u32
            117559..117571 'shot_idx + 1': u32
            117570..117571 '1': integer
            117575..117587 'RESULT_COUNT': u32
            117590..117591 '1': integer
            117613..117687 'atomic...RFLOW)': __atomic_compare_exchange_result
            117639..117656 '&resul...r_idx]': ptr<storage, atomic<u32>, read_write>
            117640..117647 'results': ref<storage, array<atomic<u32>>, read_write>
            117640..117656 'result...r_idx]': ref<storage, atomic<u32>, read_write>
            117648..117655 'err_idx': u32
            117658..117660 '0u': u32
            117662..117686 'ERR_CA...ERFLOW': u32
            117709..117714 'shots': ref<storage, array<ShotData>, read_write>
            117709..117724 'shots[shot_idx]': ref<storage, ShotData, read_write>
            117709..117731 'shots[...interp': ref<storage, InterpreterState, read_write>
            117709..117738 'shots[...status': ref<storage, u32, read_write>
            117715..117723 'shot_idx': u32
            117741..117753 'STATUS_ERROR': u32
            117775..117820 'atomic...t, 1u)': u32
            117785..117815 '&diagn..._count': ptr<storage, atomic<u32>, read_write>
            117786..117797 'diagnostics': ref<storage, DiagnosticData, read_write>
            117786..117815 'diagno..._count': ref<storage, atomic<u32>, read_write>
            117817..117819 '1u': u32
            117842..117854 'should_break': ref<function, bool, read_write>
            117857..117861 'true': bool
            117929..117931 'sp': u32
            117934..117939 'shots': ref<storage, array<ShotData>, read_write>
            117934..117949 'shots[shot_idx]': ref<storage, ShotData, read_write>
            117934..117956 'shots[...interp': ref<storage, InterpreterState, read_write>
            117934..117964 'shots[...all_sp': ref<storage, u32, read_write>
            117934..117968 'shots[...sp - 1': u32
            117940..117948 'shot_idx': u32
            117967..117968 '1': integer
            117986..117991 'shots': ref<storage, array<ShotData>, read_write>
            117986..118001 'shots[shot_idx]': ref<storage, ShotData, read_write>
            117986..118008 'shots[...interp': ref<storage, InterpreterState, read_write>
            117986..118016 'shots[...all_sp': ref<storage, u32, read_write>
            117992..118000 'shot_idx': u32
            118019..118021 'sp': u32
            118039..118047 'block_id': ref<function, u32, read_write>
            118050..118055 'shots': ref<storage, array<ShotData>, read_write>
            118050..118065 'shots[shot_idx]': ref<storage, ShotData, read_write>
            118050..118072 'shots[...interp': ref<storage, InterpreterState, read_write>
            118050..118090 'shots[...frames': ref<storage, array<CallStackFrame, 14>, read_write>
            118050..118094 'shots[...es[sp]': ref<storage, CallStackFrame, read_write>
            118050..118103 'shots[...ock_id': ref<storage, u32, read_write>
            118056..118064 'shot_idx': u32
            118091..118093 'sp': u32
            118121..118123 'pc': ref<function, u32, read_write>
            118126..118131 'shots': ref<storage, array<ShotData>, read_write>
            118126..118141 'shots[shot_idx]': ref<storage, ShotData, read_write>
            118126..118148 'shots[...interp': ref<storage, InterpreterState, read_write>
            118126..118166 'shots[...frames': ref<storage, array<CallStackFrame, 14>, read_write>
            118126..118170 'shots[...es[sp]': ref<storage, CallStackFrame, read_write>
            118126..118180 'shots[...urn_pc': ref<storage, u32, read_write>
            118132..118140 'shot_idx': u32
            118167..118169 'sp': u32
            118202..118212 'return_reg': u32
            118215..118220 'shots': ref<storage, array<ShotData>, read_write>
            118215..118230 'shots[shot_idx]': ref<storage, ShotData, read_write>
            118215..118237 'shots[...interp': ref<storage, InterpreterState, read_write>
            118215..118255 'shots[...frames': ref<storage, array<CallStackFrame, 14>, read_write>
            118215..118259 'shots[...es[sp]': ref<storage, CallStackFrame, read_write>
            118215..118270 'shots[...rn_reg': ref<storage, u32, read_write>
            118221..118229 'shot_idx': u32
            118256..118258 'sp': u32
            118291..118301 'return_reg': u32
            118291..118316 'return...RETURN': bool
            118305..118316 'VOID_RETURN': u32
            118339..118402 'write_...src0))': [error]
            118349..118357 'shot_idx': u32
            118359..118369 'return_reg': u32
            118371..118401 'read_r....src0)': u32
            118380..118388 'shot_idx': u32
            118390..118395 'instr': Instruction
            118390..118400 'instr.src0': u32
            119950..119965 'OP_QUANTUM_GATE': u32
            119984..119989 'shots': ref<storage, array<ShotData>, read_write>
            119984..119999 'shots[shot_idx]': ref<storage, ShotData, read_write>
            119984..120006 'shots[...interp': ref<storage, InterpreterState, read_write>
            119984..120021 'shots[...op_idx': ref<storage, u32, read_write>
            119990..119998 'shot_idx': u32
            120024..120029 'instr': Instruction
            120024..120034 'instr.aux0': u32
            120052..120057 'shots': ref<storage, array<ShotData>, read_write>
            120052..120067 'shots[shot_idx]': ref<storage, ShotData, read_write>
            120052..120074 'shots[...interp': ref<storage, InterpreterState, read_write>
            120052..120090 'shots[...p_type': ref<storage, u32, read_write>
            120058..120066 'shot_idx': u32
            120093..120095 '0u': u32
            120357..120362 'shots': ref<storage, array<ShotData>, read_write>
            120357..120372 'shots[shot_idx]': ref<storage, ShotData, read_write>
            120357..120379 'shots[...interp': ref<storage, InterpreterState, read_write>
            120357..120386 'shots[...status': ref<storage, u32, read_write>
            120363..120371 'shot_idx': u32
            120389..120411 'STATUS...ENDING': u32
            120429..120431 'pc': ref<function, u32, read_write>
            120451..120463 'should_break': ref<function, bool, read_write>
            120466..120470 'true': bool
            120741..120751 'OP_MEASURE': u32
            120770..120775 'shots': ref<storage, array<ShotData>, read_write>
            120770..120785 'shots[shot_idx]': ref<storage, ShotData, read_write>
            120770..120792 'shots[...interp': ref<storage, InterpreterState, read_write>
            120770..120807 'shots[...op_idx': ref<storage, u32, read_write>
            120776..120784 'shot_idx': u32
            120810..120815 'instr': Instruction
            120810..120820 'instr.aux0': u32
            120838..120843 'shots': ref<storage, array<ShotData>, read_write>
            120838..120853 'shots[shot_idx]': ref<storage, ShotData, read_write>
            120838..120860 'shots[...interp': ref<storage, InterpreterState, read_write>
            120838..120876 'shots[...p_type': ref<storage, u32, read_write>
            120844..120852 'shot_idx': u32
            120879..120881 '1u': u32
            121047..121052 'shots': ref<storage, array<ShotData>, read_write>
            121047..121062 'shots[shot_idx]': ref<storage, ShotData, read_write>
            121047..121069 'shots[...interp': ref<storage, InterpreterState, read_write>
            121047..121076 'shots[...status': ref<storage, u32, read_write>
            121053..121061 'shot_idx': u32
            121079..121101 'STATUS...ENDING': u32
            121119..121121 'pc': ref<function, u32, read_write>
            121141..121153 'should_break': ref<function, bool, read_write>
            121156..121160 'true': bool
            121383..121391 'OP_RESET': u32
            121410..121415 'shots': ref<storage, array<ShotData>, read_write>
            121410..121425 'shots[shot_idx]': ref<storage, ShotData, read_write>
            121410..121432 'shots[...interp': ref<storage, InterpreterState, read_write>
            121410..121447 'shots[...op_idx': ref<storage, u32, read_write>
            121416..121424 'shot_idx': u32
            121450..121455 'instr': Instruction
            121450..121460 'instr.aux0': u32
            121478..121483 'shots': ref<storage, array<ShotData>, read_write>
            121478..121493 'shots[shot_idx]': ref<storage, ShotData, read_write>
            121478..121500 'shots[...interp': ref<storage, InterpreterState, read_write>
            121478..121516 'shots[...p_type': ref<storage, u32, read_write>
            121484..121492 'shot_idx': u32
            121519..121521 '2u': u32
            121634..121639 'shots': ref<storage, array<ShotData>, read_write>
            121634..121649 'shots[shot_idx]': ref<storage, ShotData, read_write>
            121634..121656 'shots[...interp': ref<storage, InterpreterState, read_write>
            121634..121663 'shots[...status': ref<storage, u32, read_write>
            121640..121648 'shot_idx': u32
            121666..121688 'STATUS...ENDING': u32
            121706..121708 'pc': ref<function, u32, read_write>
            121728..121740 'should_break': ref<function, bool, read_write>
            121743..121747 'true': bool
            122499..122513 'OP_READ_RESULT': u32
            122536..122545 'result_id': u32
            122548..122553 'instr': Instruction
            122548..122558 'instr.src0': u32
            122580..122590 'result_val': bool
            122593..122637 'read_m...lt_id)': bool
            122617..122625 'shot_idx': u32
            122627..122636 'result_id': u32
            122655..122713 'write_..._val))': [error]
            122665..122673 'shot_idx': u32
            122675..122680 'instr': Instruction
            122675..122684 'instr.dst': u32
            122686..122712 'select...t_val)': u32
            122693..122695 '0u': u32
            122697..122699 '1u': u32
            122701..122711 'result_val': bool
            122731..122733 'pc': ref<function, u32, read_write>
            123063..123079 'OP_REC...OUTPUT': u32
            123098..123100 'pc': ref<function, u32, read_write>
            123470..123482 'OP_READ_LOSS': u32
            123505..123514 'result_id': u32
            123517..123522 'instr': Instruction
            123517..123527 'instr.src0': u32
            123549..123552 'val': u32
            123555..123612 'atomic...t_id])': u32
            123566..123611 '&resul...lt_id]': ptr<storage, atomic<u32>, read_write>
            123567..123574 'results': ref<storage, array<atomic<u32>>, read_write>
            123567..123611 'result...lt_id]': ref<storage, atomic<u32>, read_write>
            123575..123583 'shot_idx': u32
            123575..123598 'shot_i..._COUNT': u32
            123575..123610 'shot_i...ult_id': u32
            123586..123598 'RESULT_COUNT': u32
            123601..123610 'result_id': u32
            123630..123687 'write_...= 2u))': [error]
            123640..123648 'shot_idx': u32
            123650..123655 'instr': Instruction
            123650..123659 'instr.dst': u32
            123661..123686 'select...== 2u)': u32
            123668..123670 '0u': u32
            123672..123674 '1u': u32
            123676..123679 'val': u32
            123676..123685 'val == 2u': bool
            123683..123685 '2u': u32
            123705..123707 'pc': ref<function, u32, read_write>
            124315..124321 'OP_ADD': u32
            124344..124345 'a': i32
            124348..124392 'resolv...s, 0u)': i32
            124360..124368 'shot_idx': u32
            124370..124375 'instr': Instruction
            124370..124380 'instr.src0': u32
            124382..124387 'flags': u32
            124389..124391 '0u': u32
            124414..124415 'b': i32
            124418..124462 'resolv...s, 1u)': i32
            124430..124438 'shot_idx': u32
            124440..124445 'instr': Instruction
            124440..124450 'instr.src1': u32
            124452..124457 'flags': u32
            124459..124461 '1u': u32
            124480..124521 'write_...a + b)': [error]
            124494..124502 'shot_idx': u32
            124504..124509 'instr': Instruction
            124504..124513 'instr.dst': u32
            124515..124516 'a': i32
            124515..124520 'a + b': i32
            124519..124520 'b': i32
            124539..124541 'pc': ref<function, u32, read_write>
            124644..124650 'OP_SUB': u32
            124673..124674 'a': i32
            124677..124721 'resolv...s, 0u)': i32
            124689..124697 'shot_idx': u32
            124699..124704 'instr': Instruction
            124699..124709 'instr.src0': u32
            124711..124716 'flags': u32
            124718..124720 '0u': u32
            124743..124744 'b': i32
            124747..124791 'resolv...s, 1u)': i32
            124759..124767 'shot_idx': u32
            124769..124774 'instr': Instruction
            124769..124779 'instr.src1': u32
            124781..124786 'flags': u32
            124788..124790 '1u': u32
            124809..124850 'write_...a - b)': [error]
            124823..124831 'shot_idx': u32
            124833..124838 'instr': Instruction
            124833..124842 'instr.dst': u32
            124844..124845 'a': i32
            124844..124849 'a - b': i32
            124848..124849 'b': i32
            124868..124870 'pc': ref<function, u32, read_write>
            124976..124982 'OP_MUL': u32
            125005..125006 'a': i32
            125009..125053 'resolv...s, 0u)': i32
            125021..125029 'shot_idx': u32
            125031..125036 'instr': Instruction
            125031..125041 'instr.src0': u32
            125043..125048 'flags': u32
            125050..125052 '0u': u32
            125075..125076 'b': i32
            125079..125123 'resolv...s, 1u)': i32
            125091..125099 'shot_idx': u32
            125101..125106 'instr': Instruction
            125101..125111 'instr.src1': u32
            125113..125118 'flags': u32
            125120..125122 '1u': u32
            125141..125182 'write_...a * b)': [error]
            125155..125163 'shot_idx': u32
            125165..125170 'instr': Instruction
            125165..125174 'instr.dst': u32
            125176..125177 'a': i32
            125176..125181 'a * b': i32
            125180..125181 'b': i32
            125200..125202 'pc': ref<function, u32, read_write>
            125305..125312 'OP_UDIV': u32
            125335..125336 'a': u32
            125339..125383 'resolv...s, 0u)': u32
            125351..125359 'shot_idx': u32
            125361..125366 'instr': Instruction
            125361..125371 'instr.src0': u32
            125373..125378 'flags': u32
            125380..125382 '0u': u32
            125405..125406 'b': u32
            125409..125453 'resolv...s, 1u)': u32
            125421..125429 'shot_idx': u32
            125431..125436 'instr': Instruction
            125431..125441 'instr.src1': u32
            125443..125448 'flags': u32
            125450..125452 '1u': u32
            125471..125508 'write_...a / b)': [error]
            125481..125489 'shot_idx': u32
            125491..125496 'instr': Instruction
            125491..125500 'instr.dst': u32
            125502..125503 'a': u32
            125502..125507 'a / b': u32
            125506..125507 'b': u32
            125526..125528 'pc': ref<function, u32, read_write>
            125653..125660 'OP_SDIV': u32
            125683..125684 'a': i32
            125687..125731 'resolv...s, 0u)': i32
            125699..125707 'shot_idx': u32
            125709..125714 'instr': Instruction
            125709..125719 'instr.src0': u32
            125721..125726 'flags': u32
            125728..125730 '0u': u32
            125753..125754 'b': i32
            125757..125801 'resolv...s, 1u)': i32
            125769..125777 'shot_idx': u32
            125779..125784 'instr': Instruction
            125779..125789 'instr.src1': u32
            125791..125796 'flags': u32
            125798..125800 '1u': u32
            125819..125860 'write_...a / b)': [error]
            125833..125841 'shot_idx': u32
            125843..125848 'instr': Instruction
            125843..125852 'instr.dst': u32
            125854..125855 'a': i32
            125854..125859 'a / b': i32
            125858..125859 'b': i32
            125878..125880 'pc': ref<function, u32, read_write>
            125984..125991 'OP_UREM': u32
            126014..126015 'a': u32
            126018..126062 'resolv...s, 0u)': u32
            126030..126038 'shot_idx': u32
            126040..126045 'instr': Instruction
            126040..126050 'instr.src0': u32
            126052..126057 'flags': u32
            126059..126061 '0u': u32
            126084..126085 'b': u32
            126088..126132 'resolv...s, 1u)': u32
            126100..126108 'shot_idx': u32
            126110..126115 'instr': Instruction
            126110..126120 'instr.src1': u32
            126122..126127 'flags': u32
            126129..126131 '1u': u32
            126150..126187 'write_...a % b)': [error]
            126160..126168 'shot_idx': u32
            126170..126175 'instr': Instruction
            126170..126179 'instr.dst': u32
            126181..126182 'a': u32
            126181..126186 'a % b': u32
            126185..126186 'b': u32
            126205..126207 'pc': ref<function, u32, read_write>
            126586..126593 'OP_SREM': u32
            126616..126617 'a': i32
            126620..126664 'resolv...s, 0u)': i32
            126632..126640 'shot_idx': u32
            126642..126647 'instr': Instruction
            126642..126652 'instr.src0': u32
            126654..126659 'flags': u32
            126661..126663 '0u': u32
            126686..126687 'b': i32
            126690..126734 'resolv...s, 1u)': i32
            126702..126710 'shot_idx': u32
            126712..126717 'instr': Instruction
            126712..126722 'instr.src1': u32
            126724..126729 'flags': u32
            126731..126733 '1u': u32
            126752..126803 'write_... / b))': [error]
            126766..126774 'shot_idx': u32
            126776..126781 'instr': Instruction
            126776..126785 'instr.dst': u32
            126787..126788 'a': i32
            126787..126802 'a - b * (a / b)': i32
            126791..126792 'b': i32
            126791..126802 'b * (a / b)': i32
            126796..126797 'a': i32
            126796..126801 'a / b': i32
            126800..126801 'b': i32
            126821..126823 'pc': ref<function, u32, read_write>
            127182..127188 'OP_AND': u32
            127207..127350 'write_..., 1u))': [error]
            127217..127225 'shot_idx': u32
            127227..127232 'instr': Instruction
            127227..127236 'instr.dst': u32
            127258..127302 'resolv...s, 0u)': u32
            127258..127349 'resolv...s, 1u)': u32
            127270..127278 'shot_idx': u32
            127280..127285 'instr': Instruction
            127280..127290 'instr.src0': u32
            127292..127297 'flags': u32
            127299..127301 '0u': u32
            127305..127349 'resolv...s, 1u)': u32
            127317..127325 'shot_idx': u32
            127327..127332 'instr': Instruction
            127327..127337 'instr.src1': u32
            127339..127344 'flags': u32
            127346..127348 '1u': u32
            127368..127370 'pc': ref<function, u32, read_write>
            127456..127461 'OP_OR': u32
            127480..127623 'write_..., 1u))': [error]
            127490..127498 'shot_idx': u32
            127500..127505 'instr': Instruction
            127500..127509 'instr.dst': u32
            127531..127575 'resolv...s, 0u)': u32
            127531..127622 'resolv...s, 1u)': u32
            127543..127551 'shot_idx': u32
            127553..127558 'instr': Instruction
            127553..127563 'instr.src0': u32
            127565..127570 'flags': u32
            127572..127574 '0u': u32
            127578..127622 'resolv...s, 1u)': u32
            127590..127598 'shot_idx': u32
            127600..127605 'instr': Instruction
            127600..127610 'instr.src1': u32
            127612..127617 'flags': u32
            127619..127621 '1u': u32
            127641..127643 'pc': ref<function, u32, read_write>
            127740..127746 'OP_XOR': u32
            127765..127908 'write_..., 1u))': [error]
            127775..127783 'shot_idx': u32
            127785..127790 'instr': Instruction
            127785..127794 'instr.dst': u32
            127816..127860 'resolv...s, 0u)': u32
            127816..127907 'resolv...s, 1u)': u32
            127828..127836 'shot_idx': u32
            127838..127843 'instr': Instruction
            127838..127848 'instr.src0': u32
            127850..127855 'flags': u32
            127857..127859 '0u': u32
            127863..127907 'resolv...s, 1u)': u32
            127875..127883 'shot_idx': u32
            127885..127890 'instr': Instruction
            127885..127895 'instr.src1': u32
            127897..127902 'flags': u32
            127904..127906 '1u': u32
            127926..127928 'pc': ref<function, u32, read_write>
            128024..128030 'OP_SHL': u32
            128049..128193 'write_..., 1u))': [error]
            128059..128067 'shot_idx': u32
            128069..128074 'instr': Instruction
            128069..128078 'instr.dst': u32
            128100..128144 'resolv...s, 0u)': u32
            128100..128192 'resolv...s, 1u)': u32
            128112..128120 'shot_idx': u32
            128122..128127 'instr': Instruction
            128122..128132 'instr.src0': u32
            128134..128139 'flags': u32
            128141..128143 '0u': u32
            128148..128192 'resolv...s, 1u)': u32
            128160..128168 'shot_idx': u32
            128170..128175 'instr': Instruction
            128170..128180 'instr.src1': u32
            128182..128187 'flags': u32
            128189..128191 '1u': u32
            128211..128213 'pc': ref<function, u32, read_write>
            128323..128330 'OP_LSHR': u32
            128349..128493 'write_..., 1u))': [error]
            128359..128367 'shot_idx': u32
            128369..128374 'instr': Instruction
            128369..128378 'instr.dst': u32
            128400..128444 'resolv...s, 0u)': u32
            128400..128492 'resolv...s, 1u)': u32
            128412..128420 'shot_idx': u32
            128422..128427 'instr': Instruction
            128422..128432 'instr.src0': u32
            128434..128439 'flags': u32
            128441..128443 '0u': u32
            128448..128492 'resolv...s, 1u)': u32
            128460..128468 'shot_idx': u32
            128470..128475 'instr': Instruction
            128470..128480 'instr.src1': u32
            128482..128487 'flags': u32
            128489..128491 '1u': u32
            128511..128513 'pc': ref<function, u32, read_write>
            128698..128705 'OP_ASHR': u32
            128728..128729 'a': i32
            128732..128776 'resolv...s, 0u)': i32
            128744..128752 'shot_idx': u32
            128754..128759 'instr': Instruction
            128754..128764 'instr.src0': u32
            128766..128771 'flags': u32
            128773..128775 '0u': u32
            128798..128799 'b': u32
            128802..128846 'resolv...s, 1u)': u32
            128814..128822 'shot_idx': u32
            128824..128829 'instr': Instruction
            128824..128834 'instr.src1': u32
            128836..128841 'flags': u32
            128843..128845 '1u': u32
            128864..128906 'write_... >> b)': [error]
            128878..128886 'shot_idx': u32
            128888..128893 'instr': Instruction
            128888..128897 'instr.dst': u32
            128899..128900 'a': i32
            128899..128905 'a >> b': i32
            128904..128905 'b': u32
            128924..128926 'pc': ref<function, u32, read_write>
            129578..129585 'OP_ICMP': u32
            129608..129609 'a': i32
            129612..129656 'resolv...s, 0u)': i32
            129624..129632 'shot_idx': u32
            129634..129639 'instr': Instruction
            129634..129644 'instr.src0': u32
            129646..129651 'flags': u32
            129653..129655 '0u': u32
            129678..129679 'b': i32
            129682..129726 'resolv...s, 1u)': i32
            129694..129702 'shot_idx': u32
            129704..129709 'instr': Instruction
            129704..129714 'instr.src1': u32
            129716..129721 'flags': u32
            129723..129725 '1u': u32
            129748..129754 'result': ref<function, bool, read_write>
            129763..129768 'false': bool
            129793..129800 'subcond': u32
            129828..129835 'ICMP_EQ': u32
            129839..129845 'result': ref<function, bool, read_write>
            129849..129850 'a': i32
            129849..129855 'a == b': bool
            129854..129855 'b': i32
            129885..129892 'ICMP_NE': u32
            129896..129902 'result': ref<function, bool, read_write>
            129906..129907 'a': i32
            129906..129912 'a != b': bool
            129911..129912 'b': i32
            129942..129950 'ICMP_SLT': u32
            129953..129959 'result': ref<function, bool, read_write>
            129963..129964 'a': i32
            129963..129968 'a < b': bool
            129967..129968 'b': i32
            129998..130006 'ICMP_SLE': u32
            130009..130015 'result': ref<function, bool, read_write>
            130019..130020 'a': i32
            130019..130025 'a <= b': bool
            130024..130025 'b': i32
            130055..130063 'ICMP_SGT': u32
            130066..130072 'result': ref<function, bool, read_write>
            130076..130077 'a': i32
            130076..130081 'a > b': bool
            130080..130081 'b': i32
            130111..130119 'ICMP_SGE': u32
            130122..130128 'result': ref<function, bool, read_write>
            130132..130133 'a': i32
            130132..130138 'a >= b': bool
            130137..130138 'b': i32
            130168..130176 'ICMP_ULT': u32
            130179..130185 'result': ref<function, bool, read_write>
            130189..130204 'bitcast<u32>(a)': u32
            130189..130222 'bitcas...32>(b)': bool
            130202..130203 'a': i32
            130207..130222 'bitcast<u32>(b)': u32
            130220..130221 'b': i32
            130252..130260 'ICMP_ULE': u32
            130263..130269 'result': ref<function, bool, read_write>
            130273..130288 'bitcast<u32>(a)': u32
            130273..130307 'bitcas...32>(b)': bool
            130286..130287 'a': i32
            130292..130307 'bitcast<u32>(b)': u32
            130305..130306 'b': i32
            130337..130345 'ICMP_UGT': u32
            130348..130354 'result': ref<function, bool, read_write>
            130358..130373 'bitcast<u32>(a)': u32
            130358..130391 'bitcas...32>(b)': bool
            130371..130372 'a': i32
            130376..130391 'bitcast<u32>(b)': u32
            130389..130390 'b': i32
            130421..130429 'ICMP_UGE': u32
            130432..130438 'result': ref<function, bool, read_write>
            130442..130457 'bitcast<u32>(a)': u32
            130442..130476 'bitcas...32>(b)': bool
            130455..130456 'a': i32
            130461..130476 'bitcast<u32>(b)': u32
            130474..130475 'b': i32
            130535..130540 'shots': ref<storage, array<ShotData>, read_write>
            130535..130550 'shots[shot_idx]': ref<storage, ShotData, read_write>
            130535..130557 'shots[...interp': ref<storage, InterpreterState, read_write>
            130535..130564 'shots[...status': ref<storage, u32, read_write>
            130541..130549 'shot_idx': u32
            130567..130590 'ERR_IN...UCTION': u32
            130616..130621 'shots': ref<storage, array<ShotData>, read_write>
            130616..130631 'shots[shot_idx]': ref<storage, ShotData, read_write>
            130616..130638 'shots[...interp': ref<storage, InterpreterState, read_write>
            130616..130648 'shots[...t_code': ref<storage, u32, read_write>
            130622..130630 'shot_idx': u32
            130651..130674 'ERR_IN...UCTION': u32
            130704..130711 'err_idx': u32
            130714..130743 '(shot_..._COUNT': u32
            130714..130747 '(shot_...NT - 1': u32
            130715..130723 'shot_idx': u32
            130715..130727 'shot_idx + 1': u32
            130726..130727 '1': integer
            130731..130743 'RESULT_COUNT': u32
            130746..130747 '1': integer
            130773..130846 'atomic...CTION)': __atomic_compare_exchange_result
            130799..130816 '&resul...r_idx]': ptr<storage, atomic<u32>, read_write>
            130800..130807 'results': ref<storage, array<atomic<u32>>, read_write>
            130800..130816 'result...r_idx]': ref<storage, atomic<u32>, read_write>
            130808..130815 'err_idx': u32
            130818..130820 '0u': u32
            130822..130845 'ERR_IN...UCTION': u32
            130872..130877 'shots': ref<storage, array<ShotData>, read_write>
            130872..130887 'shots[shot_idx]': ref<storage, ShotData, read_write>
            130872..130894 'shots[...interp': ref<storage, InterpreterState, read_write>
            130872..130901 'shots[...status': ref<storage, u32, read_write>
            130878..130886 'shot_idx': u32
            130904..130916 'STATUS_ERROR': u32
            130942..130987 'atomic...t, 1u)': u32
            130952..130982 '&diagn..._count': ptr<storage, atomic<u32>, read_write>
            130953..130964 'diagnostics': ref<storage, DiagnosticData, read_write>
            130953..130982 'diagno..._count': ref<storage, atomic<u32>, read_write>
            130984..130986 '1u': u32
            131013..131025 'should_break': ref<function, bool, read_write>
            131028..131032 'true': bool
            131090..131144 'write_...sult))': [error]
            131100..131108 'shot_idx': u32
            131110..131115 'instr': Instruction
            131110..131119 'instr.dst': u32
            131121..131143 'select...esult)': u32
            131128..131130 '0u': u32
            131132..131134 '1u': u32
            131136..131142 'result': ref<function, bool, read_write>
            131162..131164 'pc': ref<function, u32, read_write>
            131614..131621 'OP_FCMP': u32
            131644..131645 'a': f32
            131648..131692 'resolv...s, 0u)': f32
            131660..131668 'shot_idx': u32
            131670..131675 'instr': Instruction
            131670..131680 'instr.src0': u32
            131682..131687 'flags': u32
            131689..131691 '0u': u32
            131714..131715 'b': f32
            131718..131762 'resolv...s, 1u)': f32
            131730..131738 'shot_idx': u32
            131740..131745 'instr': Instruction
            131740..131750 'instr.src1': u32
            131752..131757 'flags': u32
            131759..131761 '1u': u32
            131784..131790 'result': ref<function, bool, read_write>
            131799..131804 'false': bool
            131829..131836 'subcond': u32
            131864..131872 'FCMP_OEQ': u32
            131875..131881 'result': ref<function, bool, read_write>
            131885..131886 'a': f32
            131885..131891 'a == b': bool
            131890..131891 'b': f32
            131921..131929 'FCMP_ONE': u32
            131932..131938 'result': ref<function, bool, read_write>
            131942..131943 'a': f32
            131942..131948 'a != b': bool
            131947..131948 'b': f32
            131978..131986 'FCMP_OLT': u32
            131989..131995 'result': ref<function, bool, read_write>
            131999..132000 'a': f32
            131999..132004 'a < b': bool
            132003..132004 'b': f32
            132034..132042 'FCMP_OLE': u32
            132045..132051 'result': ref<function, bool, read_write>
            132055..132056 'a': f32
            132055..132061 'a <= b': bool
            132060..132061 'b': f32
            132091..132099 'FCMP_OGT': u32
            132102..132108 'result': ref<function, bool, read_write>
            132112..132113 'a': f32
            132112..132117 'a > b': bool
            132116..132117 'b': f32
            132147..132155 'FCMP_OGE': u32
            132158..132164 'result': ref<function, bool, read_write>
            132168..132169 'a': f32
            132168..132174 'a >= b': bool
            132173..132174 'b': f32
            132233..132238 'shots': ref<storage, array<ShotData>, read_write>
            132233..132248 'shots[shot_idx]': ref<storage, ShotData, read_write>
            132233..132255 'shots[...interp': ref<storage, InterpreterState, read_write>
            132233..132265 'shots[...t_code': ref<storage, u32, read_write>
            132239..132247 'shot_idx': u32
            132268..132291 'ERR_IN...UCTION': u32
            132321..132328 'err_idx': u32
            132331..132360 '(shot_..._COUNT': u32
            132331..132364 '(shot_...NT - 1': u32
            132332..132340 'shot_idx': u32
            132332..132344 'shot_idx + 1': u32
            132343..132344 '1': integer
            132348..132360 'RESULT_COUNT': u32
            132363..132364 '1': integer
            132390..132463 'atomic...CTION)': __atomic_compare_exchange_result
            132416..132433 '&resul...r_idx]': ptr<storage, atomic<u32>, read_write>
            132417..132424 'results': ref<storage, array<atomic<u32>>, read_write>
            132417..132433 'result...r_idx]': ref<storage, atomic<u32>, read_write>
            132425..132432 'err_idx': u32
            132435..132437 '0u': u32
            132439..132462 'ERR_IN...UCTION': u32
            132489..132494 'shots': ref<storage, array<ShotData>, read_write>
            132489..132504 'shots[shot_idx]': ref<storage, ShotData, read_write>
            132489..132511 'shots[...interp': ref<storage, InterpreterState, read_write>
            132489..132518 'shots[...status': ref<storage, u32, read_write>
            132495..132503 'shot_idx': u32
            132521..132533 'STATUS_ERROR': u32
            132559..132604 'atomic...t, 1u)': u32
            132569..132599 '&diagn..._count': ptr<storage, atomic<u32>, read_write>
            132570..132581 'diagnostics': ref<storage, DiagnosticData, read_write>
            132570..132599 'diagno..._count': ref<storage, atomic<u32>, read_write>
            132601..132603 '1u': u32
            132630..132642 'should_break': ref<function, bool, read_write>
            132645..132649 'true': bool
            132707..132761 'write_...sult))': [error]
            132717..132725 'shot_idx': u32
            132727..132732 'instr': Instruction
            132727..132736 'instr.dst': u32
            132738..132760 'select...esult)': u32
            132745..132747 '0u': u32
            132749..132751 '1u': u32
            132753..132759 'result': ref<function, bool, read_write>
            132779..132781 'pc': ref<function, u32, read_write>
            133237..133244 'OP_FADD': u32
            133263..133410 'write_..., 1u))': [error]
            133277..133285 'shot_idx': u32
            133287..133292 'instr': Instruction
            133287..133296 'instr.dst': u32
            133318..133362 'resolv...s, 0u)': f32
            133318..133409 'resolv...s, 1u)': f32
            133330..133338 'shot_idx': u32
            133340..133345 'instr': Instruction
            133340..133350 'instr.src0': u32
            133352..133357 'flags': u32
            133359..133361 '0u': u32
            133365..133409 'resolv...s, 1u)': f32
            133377..133385 'shot_idx': u32
            133387..133392 'instr': Instruction
            133387..133397 'instr.src1': u32
            133399..133404 'flags': u32
            133406..133408 '1u': u32
            133428..133430 'pc': ref<function, u32, read_write>
            133525..133532 'OP_FSUB': u32
            133551..133698 'write_..., 1u))': [error]
            133565..133573 'shot_idx': u32
            133575..133580 'instr': Instruction
            133575..133584 'instr.dst': u32
            133606..133650 'resolv...s, 0u)': f32
            133606..133697 'resolv...s, 1u)': f32
            133618..133626 'shot_idx': u32
            133628..133633 'instr': Instruction
            133628..133638 'instr.src0': u32
            133640..133645 'flags': u32
            133647..133649 '0u': u32
            133653..133697 'resolv...s, 1u)': f32
            133665..133673 'shot_idx': u32
            133675..133680 'instr': Instruction
            133675..133685 'instr.src1': u32
            133687..133692 'flags': u32
            133694..133696 '1u': u32
            133716..133718 'pc': ref<function, u32, read_write>
            133816..133823 'OP_FMUL': u32
            133842..133989 'write_..., 1u))': [error]
            133856..133864 'shot_idx': u32
            133866..133871 'instr': Instruction
            133866..133875 'instr.dst': u32
            133897..133941 'resolv...s, 0u)': f32
            133897..133988 'resolv...s, 1u)': f32
            133909..133917 'shot_idx': u32
            133919..133924 'instr': Instruction
            133919..133929 'instr.src0': u32
            133931..133936 'flags': u32
            133938..133940 '0u': u32
            133944..133988 'resolv...s, 1u)': f32
            133956..133964 'shot_idx': u32
            133966..133971 'instr': Instruction
            133966..133976 'instr.src1': u32
            133978..133983 'flags': u32
            133985..133987 '1u': u32
            134007..134009 'pc': ref<function, u32, read_write>
            134101..134108 'OP_FDIV': u32
            134127..134274 'write_..., 1u))': [error]
            134141..134149 'shot_idx': u32
            134151..134156 'instr': Instruction
            134151..134160 'instr.dst': u32
            134182..134226 'resolv...s, 0u)': f32
            134182..134273 'resolv...s, 1u)': f32
            134194..134202 'shot_idx': u32
            134204..134209 'instr': Instruction
            134204..134214 'instr.src0': u32
            134216..134221 'flags': u32
            134223..134225 '0u': u32
            134229..134273 'resolv...s, 1u)': f32
            134241..134249 'shot_idx': u32
            134251..134256 'instr': Instruction
            134251..134261 'instr.src1': u32
            134263..134268 'flags': u32
            134270..134272 '1u': u32
            134292..134294 'pc': ref<function, u32, read_write>
            134527..134534 'OP_FREM': u32
            134557..134558 'a': f32
            134561..134605 'resolv...s, 0u)': f32
            134573..134581 'shot_idx': u32
            134583..134588 'instr': Instruction
            134583..134593 'instr.src0': u32
            134595..134600 'flags': u32
            134602..134604 '0u': u32
            134627..134628 'b': f32
            134631..134675 'resolv...s, 1u)': f32
            134643..134651 'shot_idx': u32
            134653..134658 'instr': Instruction
            134653..134663 'instr.src1': u32
            134665..134670 'flags': u32
            134672..134674 '1u': u32
            134693..134749 'write_...) * b)': [error]
            134707..134715 'shot_idx': u32
            134717..134722 'instr': Instruction
            134717..134726 'instr.dst': u32
            134728..134729 'a': f32
            134728..134748 'a - tr...b) * b': f32
            134732..134744 'trunc(a / b)': f32
            134732..134748 'trunc(...b) * b': f32
            134738..134739 'a': f32
            134738..134743 'a / b': f32
            134742..134743 'b': f32
            134747..134748 'b': f32
            134767..134769 'pc': ref<function, u32, read_write>
            135366..135373 'OP_ZEXT': u32
            135392..135468 'write_..., 0u))': [error]
            135402..135410 'shot_idx': u32
            135412..135417 'instr': Instruction
            135412..135421 'instr.dst': u32
            135423..135467 'resolv...s, 0u)': u32
            135435..135443 'shot_idx': u32
            135445..135450 'instr': Instruction
            135445..135455 'instr.src0': u32
            135457..135462 'flags': u32
            135464..135466 '0u': u32
            135486..135488 'pc': ref<function, u32, read_write>
            135823..135830 'OP_SEXT': u32
            135853..135856 'val': i32
            135859..135903 'resolv...s, 0u)': i32
            135871..135879 'shot_idx': u32
            135881..135886 'instr': Instruction
            135881..135891 'instr.src0': u32
            135893..135898 'flags': u32
            135900..135902 '0u': u32
            135925..135933 'src_bits': u32
            135936..135941 'instr': Instruction
            135936..135946 'instr.aux0': u32
            135993..136001 'src_bits': u32
            135993..136006 'src_bits > 0u': bool
            135993..136024 'src_bi... < 32u': bool
            136004..136006 '0u': u32
            136010..136018 'src_bits': u32
            136010..136024 'src_bits < 32u': bool
            136021..136024 '32u': u32
            136051..136056 'shift': u32
            136059..136062 '32u': u32
            136059..136073 '32u - src_bits': u32
            136065..136073 'src_bits': u32
            136095..136154 'write_...shift)': [error]
            136109..136117 'shot_idx': u32
            136119..136124 'instr': Instruction
            136119..136128 'instr.dst': u32
            136130..136153 '(val <... shift': i32
            136131..136134 'val': i32
            136131..136143 'val << shift': i32
            136138..136143 'shift': u32
            136148..136153 'shift': u32
            136201..136240 'write_..., val)': [error]
            136215..136223 'shot_idx': u32
            136225..136230 'instr': Instruction
            136225..136234 'instr.dst': u32
            136236..136239 'val': i32
            136276..136278 'pc': ref<function, u32, read_write>
            136400..136408 'OP_TRUNC': u32
            136427..136503 'write_..., 0u))': [error]
            136437..136445 'shot_idx': u32
            136447..136452 'instr': Instruction
            136447..136456 'instr.dst': u32
            136458..136502 'resolv...s, 0u)': u32
            136470..136478 'shot_idx': u32
            136480..136485 'instr': Instruction
            136480..136490 'instr.src0': u32
            136492..136497 'flags': u32
            136499..136501 '0u': u32
            136521..136523 'pc': ref<function, u32, read_write>
            136656..136664 'OP_FPEXT': u32
            136683..136763 'write_..., 0u))': [error]
            136697..136705 'shot_idx': u32
            136707..136712 'instr': Instruction
            136707..136716 'instr.dst': u32
            136718..136762 'resolv...s, 0u)': f32
            136730..136738 'shot_idx': u32
            136740..136745 'instr': Instruction
            136740..136750 'instr.src0': u32
            136752..136757 'flags': u32
            136759..136761 '0u': u32
            136781..136783 'pc': ref<function, u32, read_write>
            136919..136929 'OP_FPTRUNC': u32
            136948..137028 'write_..., 0u))': [error]
            136962..136970 'shot_idx': u32
            136972..136977 'instr': Instruction
            136972..136981 'instr.dst': u32
            136983..137027 'resolv...s, 0u)': f32
            136995..137003 'shot_idx': u32
            137005..137010 'instr': Instruction
            137005..137015 'instr.src0': u32
            137017..137022 'flags': u32
            137024..137026 '0u': u32
            137046..137048 'pc': ref<function, u32, read_write>
            137172..137183 'OP_INTTOPTR': u32
            137202..137278 'write_..., 0u))': [error]
            137212..137220 'shot_idx': u32
            137222..137227 'instr': Instruction
            137222..137231 'instr.dst': u32
            137233..137277 'resolv...s, 0u)': u32
            137245..137253 'shot_idx': u32
            137255..137260 'instr': Instruction
            137255..137265 'instr.src0': u32
            137267..137272 'flags': u32
            137274..137276 '0u': u32
            137296..137298 'pc': ref<function, u32, read_write>
            137410..137419 'OP_FPTOSI': u32
            137438..137523 'write_... 0u)))': [error]
            137452..137460 'shot_idx': u32
            137462..137467 'instr': Instruction
            137462..137471 'instr.dst': u32
            137473..137522 'i32(re..., 0u))': i32
            137477..137521 'resolv...s, 0u)': f32
            137489..137497 'shot_idx': u32
            137499..137504 'instr': Instruction
            137499..137509 'instr.src0': u32
            137511..137516 'flags': u32
            137518..137520 '0u': u32
            137541..137543 'pc': ref<function, u32, read_write>
            137655..137664 'OP_SITOFP': u32
            137683..137768 'write_... 0u)))': [error]
            137697..137705 'shot_idx': u32
            137707..137712 'instr': Instruction
            137707..137716 'instr.dst': u32
            137718..137767 'f32(re..., 0u))': f32
            137722..137766 'resolv...s, 0u)': i32
            137734..137742 'shot_idx': u32
            137744..137749 'instr': Instruction
            137744..137754 'instr.src0': u32
            137756..137761 'flags': u32
            137763..137765 '0u': u32
            137786..137788 'pc': ref<function, u32, read_write>
            137902..137911 'OP_FPTOUI': u32
            137930..138011 'write_... 0u)))': [error]
            137940..137948 'shot_idx': u32
            137950..137955 'instr': Instruction
            137950..137959 'instr.dst': u32
            137961..138010 'u32(re..., 0u))': u32
            137965..138009 'resolv...s, 0u)': f32
            137977..137985 'shot_idx': u32
            137987..137992 'instr': Instruction
            137987..137997 'instr.src0': u32
            137999..138004 'flags': u32
            138006..138008 '0u': u32
            138029..138031 'pc': ref<function, u32, read_write>
            138145..138154 'OP_UITOFP': u32
            138173..138258 'write_... 0u)))': [error]
            138187..138195 'shot_idx': u32
            138197..138202 'instr': Instruction
            138197..138206 'instr.dst': u32
            138208..138257 'f32(re..., 0u))': f32
            138212..138256 'resolv...s, 0u)': u32
            138224..138232 'shot_idx': u32
            138234..138239 'instr': Instruction
            138234..138244 'instr.src0': u32
            138246..138251 'flags': u32
            138253..138255 '0u': u32
            138276..138278 'pc': ref<function, u32, read_write>
            139310..139316 'OP_PHI': u32
            139339..139345 'offset': u32
            139348..139353 'instr': Instruction
            139348..139358 'instr.aux0': u32
            139380..139385 'count': u32
            139388..139393 'instr': Instruction
            139388..139398 'instr.aux1': u32
            139425..139426 'i': ref<function, u32, read_write>
            139429..139431 '0u': u32
            139433..139434 'i': ref<function, u32, read_write>
            139433..139442 'i < count': bool
            139437..139442 'count': u32
            139444..139445 'i': ref<function, u32, read_write>
            139475..139480 'entry': [error]
            139483..139493 'batch_data': ref<storage, BatchData, read>
            139483..139501 'batch_...rogram': ref<storage, Program, read>
            139483..139511 'batch_..._table': ref<storage, [error], read>
            139483..139523 'batch_...t + i]': [error]
            139512..139518 'offset': u32
            139512..139522 'offset + i': u32
            139521..139522 'i': ref<function, u32, read_write>
            139548..139553 'entry': [error]
            139548..139562 'entry.block_id': [error]
            139548..139576 'entry...._block': [error]
            139566..139576 'prev_block': ref<function, u32, read_write>
            139603..139668 'write_..._reg))': [error]
            139613..139621 'shot_idx': u32
            139623..139628 'instr': Instruction
            139623..139632 'instr.dst': u32
            139634..139667 'read_r...l_reg)': u32
            139643..139651 'shot_idx': u32
            139653..139658 'entry': [error]
            139653..139666 'entry.val_reg': [error]
            139757..139759 'pc': ref<function, u32, read_write>
            140205..140214 'OP_SELECT': u32
            140237..140241 'cond': bool
            140244..140288 'resolv...s, 0u)': u32
            140244..140294 'resolv... != 0u': bool
            140256..140264 'shot_idx': u32
            140266..140271 'instr': Instruction
            140266..140276 'instr.src0': u32
            140278..140283 'flags': u32
            140285..140287 '0u': u32
            140292..140294 '0u': u32
            140316..140324 'true_val': u32
            140327..140371 'resolv...s, 3u)': u32
            140339..140347 'shot_idx': u32
            140349..140354 'instr': Instruction
            140349..140359 'instr.aux0': u32
            140361..140366 'flags': u32
            140368..140370 '3u': u32
            140393..140402 'false_val': u32
            140405..140449 'resolv...s, 4u)': u32
            140417..140425 'shot_idx': u32
            140427..140432 'instr': Instruction
            140427..140437 'instr.aux1': u32
            140439..140444 'flags': u32
            140446..140448 '4u': u32
            140467..140532 'write_...cond))': [error]
            140477..140485 'shot_idx': u32
            140487..140492 'instr': Instruction
            140487..140496 'instr.dst': u32
            140498..140531 'select... cond)': u32
            140505..140514 'false_val': u32
            140516..140524 'true_val': u32
            140526..140530 'cond': bool
            140550..140552 'pc': ref<function, u32, read_write>
            140748..140754 'OP_MOV': u32
            140773..140849 'write_..., 0u))': [error]
            140783..140791 'shot_idx': u32
            140793..140798 'instr': Instruction
            140793..140802 'instr.dst': u32
            140804..140848 'resolv...s, 0u)': u32
            140816..140824 'shot_idx': u32
            140826..140831 'instr': Instruction
            140826..140836 'instr.src0': u32
            140838..140843 'flags': u32
            140845..140847 '0u': u32
            140867..140869 'pc': ref<function, u32, read_write>
            141050..141058 'OP_CONST': u32
            141077..141119 'write_....src0)': [error]
            141087..141095 'shot_idx': u32
            141097..141102 'instr': Instruction
            141097..141106 'instr.dst': u32
            141108..141113 'instr': Instruction
            141108..141118 'instr.src0': u32
            141137..141139 'pc': ref<function, u32, read_write>
            141518..141527 'OP_ALLOCA': u32
            141550..141559 'num_words': u32
            141562..141606 'resolv...s, 0u)': u32
            141574..141582 'shot_idx': u32
            141584..141589 'instr': Instruction
            141584..141594 'instr.src0': u32
            141596..141601 'flags': u32
            141603..141605 '0u': u32
            141628..141632 'addr': u32
            141635..141679 'resolv...s, 1u)': u32
            141647..141655 'shot_idx': u32
            141657..141662 'instr': Instruction
            141657..141667 'instr.src1': u32
            141669..141674 'flags': u32
            141676..141678 '1u': u32
            141700..141704 'addr': u32
            141700..141716 'addr +..._words': u32
            141700..141729 'addr +...MEMORY': bool
            141707..141716 'num_words': u32
            141719..141729 'MAX_MEMORY': u32
            141752..141757 'shots': ref<storage, array<ShotData>, read_write>
            141752..141767 'shots[shot_idx]': ref<storage, ShotData, read_write>
            141752..141774 'shots[...interp': ref<storage, InterpreterState, read_write>
            141752..141784 'shots[...t_code': ref<storage, u32, read_write>
            141758..141766 'shot_idx': u32
            141787..141811 'ERR_AL...BOUNDS': u32
            141837..141844 'err_idx': u32
            141847..141876 '(shot_..._COUNT': u32
            141847..141880 '(shot_...NT - 1': u32
            141848..141856 'shot_idx': u32
            141848..141860 'shot_idx + 1': u32
            141859..141860 '1': integer
            141864..141876 'RESULT_COUNT': u32
            141879..141880 '1': integer
            141902..141976 'atomic...OUNDS)': __atomic_compare_exchange_result
            141928..141945 '&resul...r_idx]': ptr<storage, atomic<u32>, read_write>
            141929..141936 'results': ref<storage, array<atomic<u32>>, read_write>
            141929..141945 'result...r_idx]': ref<storage, atomic<u32>, read_write>
            141937..141944 'err_idx': u32
            141947..141949 '0u': u32
            141951..141975 'ERR_AL...BOUNDS': u32
            141998..142003 'shots': ref<storage, array<ShotData>, read_write>
            141998..142013 'shots[shot_idx]': ref<storage, ShotData, read_write>
            141998..142020 'shots[...interp': ref<storage, InterpreterState, read_write>
            141998..142027 'shots[...status': ref<storage, u32, read_write>
            142004..142012 'shot_idx': u32
            142030..142042 'STATUS_ERROR': u32
            142064..142109 'atomic...t, 1u)': u32
            142074..142104 '&diagn..._count': ptr<storage, atomic<u32>, read_write>
            142075..142086 'diagnostics': ref<storage, DiagnosticData, read_write>
            142075..142104 'diagno..._count': ref<storage, atomic<u32>, read_write>
            142106..142108 '1u': u32
            142131..142143 'should_break': ref<function, bool, read_write>
            142146..142150 'true': bool
            142213..142249 'write_... addr)': [error]
            142223..142231 'shot_idx': u32
            142233..142238 'instr': Instruction
            142233..142242 'instr.dst': u32
            142244..142248 'addr': u32
            142267..142269 'pc': ref<function, u32, read_write>
            142449..142456 'OP_LOAD': u32
            142479..142483 'addr': u32
            142486..142530 'resolv...s, 0u)': u32
            142498..142506 'shot_idx': u32
            142508..142513 'instr': Instruction
            142508..142518 'instr.src0': u32
            142520..142525 'flags': u32
            142527..142529 '0u': u32
            142551..142555 'addr': u32
            142551..142569 'addr >...MEMORY': bool
            142559..142569 'MAX_MEMORY': u32
            142592..142597 'shots': ref<storage, array<ShotData>, read_write>
            142592..142607 'shots[shot_idx]': ref<storage, ShotData, read_write>
            142592..142614 'shots[...interp': ref<storage, InterpreterState, read_write>
            142592..142624 'shots[...t_code': ref<storage, u32, read_write>
            142598..142606 'shot_idx': u32
            142627..142651 'ERR_ME...BOUNDS': u32
            142677..142684 'err_idx': u32
            142687..142716 '(shot_..._COUNT': u32
            142687..142720 '(shot_...NT - 1': u32
            142688..142696 'shot_idx': u32
            142688..142700 'shot_idx + 1': u32
            142699..142700 '1': integer
            142704..142716 'RESULT_COUNT': u32
            142719..142720 '1': integer
            142742..142816 'atomic...OUNDS)': __atomic_compare_exchange_result
            142768..142785 '&resul...r_idx]': ptr<storage, atomic<u32>, read_write>
            142769..142776 'results': ref<storage, array<atomic<u32>>, read_write>
            142769..142785 'result...r_idx]': ref<storage, atomic<u32>, read_write>
            142777..142784 'err_idx': u32
            142787..142789 '0u': u32
            142791..142815 'ERR_ME...BOUNDS': u32
            142838..142843 'shots': ref<storage, array<ShotData>, read_write>
            142838..142853 'shots[shot_idx]': ref<storage, ShotData, read_write>
            142838..142860 'shots[...interp': ref<storage, InterpreterState, read_write>
            142838..142867 'shots[...status': ref<storage, u32, read_write>
            142844..142852 'shot_idx': u32
            142870..142882 'STATUS_ERROR': u32
            142904..142949 'atomic...t, 1u)': u32
            142914..142944 '&diagn..._count': ptr<storage, atomic<u32>, read_write>
            142915..142926 'diagnostics': ref<storage, DiagnosticData, read_write>
            142915..142944 'diagno..._count': ref<storage, atomic<u32>, read_write>
            142946..142948 '1u': u32
            142971..142983 'should_break': ref<function, bool, read_write>
            142986..142990 'true': bool
            143057..143060 'val': [error]
            143063..143068 'shots': ref<storage, array<ShotData>, read_write>
            143063..143078 'shots[shot_idx]': ref<storage, ShotData, read_write>
            143063..143085 'shots[...interp': ref<storage, InterpreterState, read_write>
            143063..143092 'shots[...memory': ref<storage, [error], read_write>
            143063..143098 'shots[...[addr]': [error]
            143069..143077 'shot_idx': u32
            143093..143097 'addr': u32
            143116..143151 'write_..., val)': [error]
            143126..143134 'shot_idx': u32
            143136..143141 'instr': Instruction
            143136..143145 'instr.dst': u32
            143147..143150 'val': [error]
            143169..143171 'pc': ref<function, u32, read_write>
            143346..143354 'OP_STORE': u32
            143377..143380 'val': u32
            143383..143427 'resolv...s, 0u)': u32
            143395..143403 'shot_idx': u32
            143405..143410 'instr': Instruction
            143405..143415 'instr.src0': u32
            143417..143422 'flags': u32
            143424..143426 '0u': u32
            143449..143453 'addr': u32
            143456..143500 'resolv...s, 1u)': u32
            143468..143476 'shot_idx': u32
            143478..143483 'instr': Instruction
            143478..143488 'instr.src1': u32
            143490..143495 'flags': u32
            143497..143499 '1u': u32
            143521..143525 'addr': u32
            143521..143539 'addr >...MEMORY': bool
            143529..143539 'MAX_MEMORY': u32
            143562..143567 'shots': ref<storage, array<ShotData>, read_write>
            143562..143577 'shots[shot_idx]': ref<storage, ShotData, read_write>
            143562..143584 'shots[...interp': ref<storage, InterpreterState, read_write>
            143562..143594 'shots[...t_code': ref<storage, u32, read_write>
            143568..143576 'shot_idx': u32
            143597..143621 'ERR_ME...BOUNDS': u32
            143647..143654 'err_idx': u32
            143657..143686 '(shot_..._COUNT': u32
            143657..143690 '(shot_...NT - 1': u32
            143658..143666 'shot_idx': u32
            143658..143670 'shot_idx + 1': u32
            143669..143670 '1': integer
            143674..143686 'RESULT_COUNT': u32
            143689..143690 '1': integer
            143712..143786 'atomic...OUNDS)': __atomic_compare_exchange_result
            143738..143755 '&resul...r_idx]': ptr<storage, atomic<u32>, read_write>
            143739..143746 'results': ref<storage, array<atomic<u32>>, read_write>
            143739..143755 'result...r_idx]': ref<storage, atomic<u32>, read_write>
            143747..143754 'err_idx': u32
            143757..143759 '0u': u32
            143761..143785 'ERR_ME...BOUNDS': u32
            143808..143813 'shots': ref<storage, array<ShotData>, read_write>
            143808..143823 'shots[shot_idx]': ref<storage, ShotData, read_write>
            143808..143830 'shots[...interp': ref<storage, InterpreterState, read_write>
            143808..143837 'shots[...status': ref<storage, u32, read_write>
            143814..143822 'shot_idx': u32
            143840..143852 'STATUS_ERROR': u32
            143874..143919 'atomic...t, 1u)': u32
            143884..143914 '&diagn..._count': ptr<storage, atomic<u32>, read_write>
            143885..143896 'diagnostics': ref<storage, DiagnosticData, read_write>
            143885..143914 'diagno..._count': ref<storage, atomic<u32>, read_write>
            143916..143918 '1u': u32
            143941..143953 'should_break': ref<function, bool, read_write>
            143956..143960 'true': bool
            144023..144028 'shots': ref<storage, array<ShotData>, read_write>
            144023..144038 'shots[shot_idx]': ref<storage, ShotData, read_write>
            144023..144045 'shots[...interp': ref<storage, InterpreterState, read_write>
            144023..144052 'shots[...memory': ref<storage, [error], read_write>
            144023..144058 'shots[...[addr]': [error]
            144029..144037 'shot_idx': u32
            144053..144057 'addr': u32
            144061..144064 'val': u32
            144082..144084 'pc': ref<function, u32, read_write>
            144292..144298 'OP_GEP': u32
            144321..144325 'base': u32
            144328..144372 'resolv...s, 0u)': u32
            144340..144348 'shot_idx': u32
            144350..144355 'instr': Instruction
            144350..144360 'instr.src0': u32
            144362..144367 'flags': u32
            144369..144371 '0u': u32
            144394..144399 'index': u32
            144402..144446 'resolv...s, 1u)': u32
            144414..144422 'shot_idx': u32
            144424..144429 'instr': Instruction
            144424..144434 'instr.src1': u32
            144436..144441 'flags': u32
            144443..144445 '1u': u32
            144468..144477 'elem_size': u32
            144480..144524 'resolv...s, 3u)': u32
            144492..144500 'shot_idx': u32
            144502..144507 'instr': Instruction
            144502..144512 'instr.aux0': u32
            144514..144519 'flags': u32
            144521..144523 '3u': u32
            144546..144550 'addr': u32
            144553..144557 'base': u32
            144553..144577 'base +...m_size': u32
            144560..144565 'index': u32
            144560..144577 'index ...m_size': u32
            144568..144577 'elem_size': u32
            144595..144631 'write_... addr)': [error]
            144605..144613 'shot_idx': u32
            144615..144620 'instr': Instruction
            144615..144624 'instr.dst': u32
            144626..144630 'addr': u32
            144649..144651 'pc': ref<function, u32, read_write>
            144768..144773 'shots': ref<storage, array<ShotData>, read_write>
            144768..144783 'shots[shot_idx]': ref<storage, ShotData, read_write>
            144768..144790 'shots[...interp': ref<storage, InterpreterState, read_write>
            144768..144797 'shots[...status': ref<storage, u32, read_write>
            144774..144782 'shot_idx': u32
            144800..144812 'STATUS_ERROR': u32
            144830..144875 'atomic...t, 1u)': u32
            144840..144870 '&diagn..._count': ptr<storage, atomic<u32>, read_write>
            144841..144852 'diagnostics': ref<storage, DiagnosticData, read_write>
            144841..144870 'diagno..._count': ref<storage, atomic<u32>, read_write>
            144872..144874 '1u': u32
            144893..144905 'should_break': ref<function, bool, read_write>
            144908..144912 'true': bool
            144946..144951 'steps': ref<function, u32, read_write>
            144966..144978 'should_break': ref<function, bool, read_write>
            145207..145212 'shots': ref<storage, array<ShotData>, read_write>
            145207..145222 'shots[shot_idx]': ref<storage, ShotData, read_write>
            145207..145229 'shots[...interp': ref<storage, InterpreterState, read_write>
            145207..145232 'shots[...erp.pc': ref<storage, u32, read_write>
            145213..145221 'shot_idx': u32
            145235..145237 'pc': ref<function, u32, read_write>
            145243..145248 'shots': ref<storage, array<ShotData>, read_write>
            145243..145258 'shots[shot_idx]': ref<storage, ShotData, read_write>
            145243..145265 'shots[...interp': ref<storage, InterpreterState, read_write>
            145243..145282 'shots[...ock_id': ref<storage, u32, read_write>
            145249..145257 'shot_idx': u32
            145285..145293 'block_id': ref<function, u32, read_write>
            145299..145304 'shots': ref<storage, array<ShotData>, read_write>
            145299..145314 'shots[shot_idx]': ref<storage, ShotData, read_write>
            145299..145321 'shots[...interp': ref<storage, InterpreterState, read_write>
            145299..145339 'shots[...ock_id': ref<storage, u32, read_write>
            145305..145313 'shot_idx': u32
            145342..145352 'prev_block': ref<function, u32, read_write>
            145804..145812 'shot_idx': u32
            145829..145833 'shot': ptr<storage, ShotData, read_write>
            145836..145852 '&shots...t_idx]': ptr<storage, ShotData, read_write>
            145837..145842 'shots': ref<storage, array<ShotData>, read_write>
            145837..145852 'shots[shot_idx]': ref<storage, ShotData, read_write>
            145843..145851 'shot_idx': u32
            145862..145867 'state': InterpreterState
            145870..145875 'shots': ref<storage, array<ShotData>, read_write>
            145870..145885 'shots[shot_idx]': ref<storage, ShotData, read_write>
            145870..145892 'shots[...interp': ref<storage, InterpreterState, read_write>
            145876..145884 'shot_idx': u32
            145902..145908 'status': u32
            145911..145916 'state': InterpreterState
            145911..145923 'state.status': u32
            145984..145990 'status': u32
            145984..146016 'status...ENDING': bool
            145994..146016 'STATUS...ENDING': u32
            146092..146096 'shot': ptr<storage, ShotData, read_write>
            146092..146104 'shot.op_type': ref<storage, u32, read_write>
            146107..146114 'OPID_ID': u32
            146124..146128 'shot': ptr<storage, ShotData, read_write>
            146124..146140 'shot.r...malize': ref<storage, f32, read_write>
            146143..146146 '1.0': float
            146156..146160 'shot': ptr<storage, ShotData, read_write>
            146156..146188 'shot.q...p_mask': ref<storage, u32, read_write>
            146191..146193 '0u': u32
            146274..146278 'shot': ptr<storage, ShotData, read_write>
            146274..146306 'shot.q...p_mask': ref<storage, u32, read_write>
            146274..146311 'shot.q...k != 0': bool
            146310..146311 '0': integer
            146322..146350 'update...t_idx)': [error]
            146341..146349 'shot_idx': u32
            146362..146388 'shot_i...t_idx)': [error]
            146379..146387 'shot_idx': u32
            146399..146405 'op_idx': u32
            146408..146413 'state': InterpreterState
            146408..146428 'state....op_idx': u32
            146438..146445 'op_type': u32
            146448..146453 'state': InterpreterState
            146448..146469 'state....p_type': u32
            146636..146643 'op_type': u32
            146636..146669 'op_typ...COMMIT': bool
            146647..146669 'PENDIN...COMMIT': u32
            146680..146714 'prep_l...p_idx)': [error]
            146697..146705 'shot_idx': u32
            146707..146713 'op_idx': u32
            146747..146749 'op': ptr<storage, Op, read>
            146752..146764 '&ops[op_idx]': ptr<storage, Op, read>
            146753..146756 'ops': ref<storage, array<Op>, read>
            146753..146764 'ops[op_idx]': ref<storage, Op, read>
            146757..146763 'op_idx': u32
            146972..146979 'op_type': u32
            146972..146985 'op_type == 0u': bool
            146972..147019 'op_typ..._NOISE': bool
            146983..146985 '0u': u32
            146989..146991 'op': ptr<storage, Op, read>
            146989..146994 'op.id': ref<storage, u32, read>
            146989..147019 'op.id ..._NOISE': bool
            146998..147019 'OPID_C..._NOISE': u32
            147034..147036 'pc': u32
            147039..147044 'state': InterpreterState
            147039..147047 'state.pc': u32
            147061..147072 'noise_instr': Instruction
            147075..147095 'fetch_... - 1u)': Instruction
            147087..147089 'pc': u32
            147087..147094 'pc - 1u': u32
            147092..147094 '1u': u32
            147109..147120 'qubit_count': u32
            147123..147134 'noise_instr': Instruction
            147123..147139 'noise_...r.aux1': u32
            147153..147163 'arg_offset': u32
            147166..147177 'noise_instr': Instruction
            147166..147182 'noise_...r.aux2': u32
            147192..147196 'shot': ptr<storage, ShotData, read_write>
            147192..147203 'shot.op_idx': ref<storage, u32, read_write>
            147206..147212 'op_idx': u32
            147222..147226 'shot': ptr<storage, ShotData, read_write>
            147222..147234 'shot.op_type': ref<storage, u32, read_write>
            147237..147239 'op': ptr<storage, Op, read>
            147237..147242 'op.id': ref<storage, u32, read>
            147252..147325 'prep_c...ffset)': [error]
            147283..147291 'shot_idx': u32
            147293..147299 'op_idx': u32
            147301..147312 'qubit_count': u32
            147314..147324 'arg_offset': u32
            147335..147340 'shots': ref<storage, array<ShotData>, read_write>
            147335..147350 'shots[shot_idx]': ref<storage, ShotData, read_write>
            147335..147357 'shots[...interp': ref<storage, InterpreterState, read_write>
            147335..147364 'shots[...status': ref<storage, u32, read_write>
            147341..147349 'shot_idx': u32
            147367..147381 'STATUS_RUNNING': u32
            147414..147416 'q1': u32
            147419..147439 'resolv...t_idx)': u32
            147430..147438 'shot_idx': u32
            147449..147451 'q2': u32
            147454..147474 'resolv...t_idx)': u32
            147465..147473 'shot_idx': u32
            147481..147485 'shot': ptr<storage, ShotData, read_write>
            147481..147493 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            147496..147498 'op': ptr<storage, Op, read>
            147496..147506 'op.unitary': ref<storage, array<vec2<f32>, 16>, read>
            147520..147527 'op_type': u32
            147543..147545 '0u': u32
            147822..147845 'is_rot...op.id)': bool
            147822..147875 'is_rot...t_idx)': bool
            147839..147841 'op': ptr<storage, Op, read>
            147839..147844 'op.id': ref<storage, u32, read>
            147849..147875 'is_dyn...t_idx)': bool
            147866..147874 'shot_idx': u32
            147897..147899 'op': ptr<storage, Op, read>
            147897..147902 'op.id': ref<storage, u32, read>
            147897..147913 'op.id ...PID_RX': bool
            147897..147933 'op.id ...PID_RY': bool
            147897..147953 'op.id ...PID_RZ': bool
            147906..147913 'OPID_RX': u32
            147917..147919 'op': ptr<storage, Op, read>
            147917..147922 'op.id': ref<storage, u32, read>
            147917..147933 'op.id ...PID_RY': bool
            147926..147933 'OPID_RY': u32
            147937..147939 'op': ptr<storage, Op, read>
            147937..147942 'op.id': ref<storage, u32, read>
            147937..147953 'op.id ...PID_RZ': bool
            147946..147953 'OPID_RZ': u32
            147980..147985 'angle': f32
            147988..148016 'resolv...t_idx)': f32
            148007..148015 'shot_idx': u32
            148042..148046 'half': f32
            148049..148054 'angle': f32
            148049..148060 'angle * 0.5': f32
            148057..148060 '0.5': float
            148086..148087 'c': f32
            148090..148099 'cos(half)': f32
            148094..148098 'half': f32
            148125..148126 's': f32
            148129..148138 'sin(half)': f32
            148133..148137 'half': f32
            148163..148165 'op': ptr<storage, Op, read>
            148163..148168 'op.id': ref<storage, u32, read>
            148163..148179 'op.id ...PID_RX': bool
            148172..148179 'OPID_RX': u32
            148290..148294 'shot': ptr<storage, ShotData, read_write>
            148290..148302 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            148290..148305 'shot.unitary[0]': ref<storage, vec2<f32>, read_write>
            148303..148304 '0': integer
            148308..148321 'vec2f(c, 0.0)': vec2<f32>
            148314..148315 'c': f32
            148317..148320 '0.0': float
            148347..148351 'shot': ptr<storage, ShotData, read_write>
            148347..148359 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            148347..148362 'shot.unitary[1]': ref<storage, vec2<f32>, read_write>
            148360..148361 '1': integer
            148365..148379 'vec2f(0.0, -s)': vec2<f32>
            148371..148374 '0.0': float
            148376..148378 '-s': f32
            148377..148378 's': f32
            148405..148409 'shot': ptr<storage, ShotData, read_write>
            148405..148417 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            148405..148420 'shot.unitary[4]': ref<storage, vec2<f32>, read_write>
            148418..148419 '4': integer
            148423..148437 'vec2f(0.0, -s)': vec2<f32>
            148429..148432 '0.0': float
            148434..148436 '-s': f32
            148435..148436 's': f32
            148463..148467 'shot': ptr<storage, ShotData, read_write>
            148463..148475 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            148463..148478 'shot.unitary[5]': ref<storage, vec2<f32>, read_write>
            148476..148477 '5': integer
            148481..148494 'vec2f(c, 0.0)': vec2<f32>
            148487..148488 'c': f32
            148490..148493 '0.0': float
            148646..148650 'shot': ptr<storage, ShotData, read_write>
            148646..148658 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            148646..148661 'shot.unitary[0]': ref<storage, vec2<f32>, read_write>
            148659..148660 '0': integer
            148664..148677 'vec2f(c, 0.0)': vec2<f32>
            148670..148671 'c': f32
            148673..148676 '0.0': float
            148703..148707 'shot': ptr<storage, ShotData, read_write>
            148703..148715 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            148703..148718 'shot.unitary[1]': ref<storage, vec2<f32>, read_write>
            148716..148717 '1': integer
            148721..148735 'vec2f(-s, 0.0)': vec2<f32>
            148727..148729 '-s': f32
            148728..148729 's': f32
            148731..148734 '0.0': float
            148761..148765 'shot': ptr<storage, ShotData, read_write>
            148761..148773 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            148761..148776 'shot.unitary[4]': ref<storage, vec2<f32>, read_write>
            148774..148775 '4': integer
            148779..148792 'vec2f(s, 0.0)': vec2<f32>
            148785..148786 's': f32
            148788..148791 '0.0': float
            148818..148822 'shot': ptr<storage, ShotData, read_write>
            148818..148830 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            148818..148833 'shot.unitary[5]': ref<storage, vec2<f32>, read_write>
            148831..148832 '5': integer
            148836..148849 'vec2f(c, 0.0)': vec2<f32>
            148842..148843 'c': f32
            148845..148848 '0.0': float
            148958..148962 'shot': ptr<storage, ShotData, read_write>
            148958..148970 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            148958..148973 'shot.unitary[0]': ref<storage, vec2<f32>, read_write>
            148971..148972 '0': integer
            148976..148991 'vec2f(1.0, 0.0)': vec2<f32>
            148982..148985 '1.0': float
            148987..148990 '0.0': float
            149017..149021 'shot': ptr<storage, ShotData, read_write>
            149017..149029 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            149017..149032 'shot.unitary[1]': ref<storage, vec2<f32>, read_write>
            149030..149031 '1': integer
            149035..149050 'vec2f(0.0, 0.0)': vec2<f32>
            149041..149044 '0.0': float
            149046..149049 '0.0': float
            149076..149080 'shot': ptr<storage, ShotData, read_write>
            149076..149088 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            149076..149091 'shot.unitary[4]': ref<storage, vec2<f32>, read_write>
            149089..149090 '4': integer
            149094..149109 'vec2f(0.0, 0.0)': vec2<f32>
            149100..149103 '0.0': float
            149105..149108 '0.0': float
            149135..149139 'shot': ptr<storage, ShotData, read_write>
            149135..149147 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            149135..149150 'shot.unitary[5]': ref<storage, vec2<f32>, read_write>
            149148..149149 '5': integer
            149153..149182 'vec2f(...ngle))': vec2<f32>
            149159..149169 'cos(angle)': f32
            149163..149168 'angle': f32
            149171..149181 'sin(angle)': f32
            149175..149180 'angle': f32
            149318..149323 'angle': f32
            149326..149354 'resolv...t_idx)': f32
            149345..149353 'shot_idx': u32
            149380..149384 'half': f32
            149387..149392 'angle': f32
            149387..149398 'angle * 0.5': f32
            149395..149398 '0.5': float
            149424..149425 'c': f32
            149428..149437 'cos(half)': f32
            149432..149436 'half': f32
            149463..149464 's': f32
            149467..149476 'sin(half)': f32
            149471..149475 'half': f32
            149501..149503 'op': ptr<storage, Op, read>
            149501..149506 'op.id': ref<storage, u32, read>
            149501..149518 'op.id ...ID_RXX': bool
            149510..149518 'OPID_RXX': u32
            149593..149597 'shot': ptr<storage, ShotData, read_write>
            149593..149605 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            149593..149608 'shot.unitary[0]': ref<storage, vec2<f32>, read_write>
            149606..149607 '0': integer
            149612..149625 'vec2f(c, 0.0)': vec2<f32>
            149618..149619 'c': f32
            149621..149624 '0.0': float
            149651..149655 'shot': ptr<storage, ShotData, read_write>
            149651..149663 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            149651..149666 'shot.unitary[3]': ref<storage, vec2<f32>, read_write>
            149664..149665 '3': integer
            149670..149684 'vec2f(0.0, -s)': vec2<f32>
            149676..149679 '0.0': float
            149681..149683 '-s': f32
            149682..149683 's': f32
            149710..149714 'shot': ptr<storage, ShotData, read_write>
            149710..149722 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            149710..149725 'shot.unitary[5]': ref<storage, vec2<f32>, read_write>
            149723..149724 '5': integer
            149729..149742 'vec2f(c, 0.0)': vec2<f32>
            149735..149736 'c': f32
            149738..149741 '0.0': float
            149768..149772 'shot': ptr<storage, ShotData, read_write>
            149768..149780 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            149768..149783 'shot.unitary[6]': ref<storage, vec2<f32>, read_write>
            149781..149782 '6': integer
            149787..149801 'vec2f(0.0, -s)': vec2<f32>
            149793..149796 '0.0': float
            149798..149800 '-s': f32
            149799..149800 's': f32
            149827..149831 'shot': ptr<storage, ShotData, read_write>
            149827..149839 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            149827..149842 'shot.unitary[9]': ref<storage, vec2<f32>, read_write>
            149840..149841 '9': integer
            149846..149860 'vec2f(0.0, -s)': vec2<f32>
            149852..149855 '0.0': float
            149857..149859 '-s': f32
            149858..149859 's': f32
            149886..149890 'shot': ptr<storage, ShotData, read_write>
            149886..149898 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            149886..149902 'shot.u...ry[10]': ref<storage, vec2<f32>, read_write>
            149899..149901 '10': integer
            149905..149918 'vec2f(c, 0.0)': vec2<f32>
            149911..149912 'c': f32
            149914..149917 '0.0': float
            149944..149948 'shot': ptr<storage, ShotData, read_write>
            149944..149956 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            149944..149960 'shot.u...ry[12]': ref<storage, vec2<f32>, read_write>
            149957..149959 '12': integer
            149963..149977 'vec2f(0.0, -s)': vec2<f32>
            149969..149972 '0.0': float
            149974..149976 '-s': f32
            149975..149976 's': f32
            150003..150007 'shot': ptr<storage, ShotData, read_write>
            150003..150015 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            150003..150019 'shot.u...ry[15]': ref<storage, vec2<f32>, read_write>
            150016..150018 '15': integer
            150022..150035 'vec2f(c, 0.0)': vec2<f32>
            150028..150029 'c': f32
            150031..150034 '0.0': float
            150159..150163 'shot': ptr<storage, ShotData, read_write>
            150159..150171 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            150159..150174 'shot.unitary[0]': ref<storage, vec2<f32>, read_write>
            150172..150173 '0': integer
            150178..150191 'vec2f(c, 0.0)': vec2<f32>
            150184..150185 'c': f32
            150187..150190 '0.0': float
            150217..150221 'shot': ptr<storage, ShotData, read_write>
            150217..150229 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            150217..150232 'shot.unitary[3]': ref<storage, vec2<f32>, read_write>
            150230..150231 '3': integer
            150236..150249 'vec2f(0.0, s)': vec2<f32>
            150242..150245 '0.0': float
            150247..150248 's': f32
            150275..150279 'shot': ptr<storage, ShotData, read_write>
            150275..150287 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            150275..150290 'shot.unitary[5]': ref<storage, vec2<f32>, read_write>
            150288..150289 '5': integer
            150294..150307 'vec2f(c, 0.0)': vec2<f32>
            150300..150301 'c': f32
            150303..150306 '0.0': float
            150333..150337 'shot': ptr<storage, ShotData, read_write>
            150333..150345 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            150333..150348 'shot.unitary[6]': ref<storage, vec2<f32>, read_write>
            150346..150347 '6': integer
            150352..150366 'vec2f(0.0, -s)': vec2<f32>
            150358..150361 '0.0': float
            150363..150365 '-s': f32
            150364..150365 's': f32
            150392..150396 'shot': ptr<storage, ShotData, read_write>
            150392..150404 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            150392..150407 'shot.unitary[9]': ref<storage, vec2<f32>, read_write>
            150405..150406 '9': integer
            150411..150425 'vec2f(0.0, -s)': vec2<f32>
            150417..150420 '0.0': float
            150422..150424 '-s': f32
            150423..150424 's': f32
            150451..150455 'shot': ptr<storage, ShotData, read_write>
            150451..150463 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            150451..150467 'shot.u...ry[10]': ref<storage, vec2<f32>, read_write>
            150464..150466 '10': integer
            150470..150483 'vec2f(c, 0.0)': vec2<f32>
            150476..150477 'c': f32
            150479..150482 '0.0': float
            150509..150513 'shot': ptr<storage, ShotData, read_write>
            150509..150521 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            150509..150525 'shot.u...ry[12]': ref<storage, vec2<f32>, read_write>
            150522..150524 '12': integer
            150528..150541 'vec2f(0.0, s)': vec2<f32>
            150534..150537 '0.0': float
            150539..150540 's': f32
            150567..150571 'shot': ptr<storage, ShotData, read_write>
            150567..150579 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            150567..150583 'shot.u...ry[15]': ref<storage, vec2<f32>, read_write>
            150580..150582 '15': integer
            150586..150599 'vec2f(c, 0.0)': vec2<f32>
            150592..150593 'c': f32
            150595..150598 '0.0': float
            150715..150719 'shot': ptr<storage, ShotData, read_write>
            150715..150727 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            150715..150730 'shot.unitary[0]': ref<storage, vec2<f32>, read_write>
            150728..150729 '0': integer
            150734..150749 'vec2f(1.0, 0.0)': vec2<f32>
            150740..150743 '1.0': float
            150745..150748 '0.0': float
            150775..150779 'shot': ptr<storage, ShotData, read_write>
            150775..150787 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            150775..150790 'shot.unitary[5]': ref<storage, vec2<f32>, read_write>
            150788..150789 '5': integer
            150794..150823 'vec2f(...ngle))': vec2<f32>
            150800..150810 'cos(angle)': f32
            150804..150809 'angle': f32
            150812..150822 'sin(angle)': f32
            150816..150821 'angle': f32
            150849..150853 'shot': ptr<storage, ShotData, read_write>
            150849..150861 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            150849..150865 'shot.u...ry[10]': ref<storage, vec2<f32>, read_write>
            150862..150864 '10': integer
            150868..150897 'vec2f(...ngle))': vec2<f32>
            150874..150884 'cos(angle)': f32
            150878..150883 'angle': f32
            150886..150896 'sin(angle)': f32
            150890..150895 'angle': f32
            150923..150927 'shot': ptr<storage, ShotData, read_write>
            150923..150935 'shot.unitary': ref<storage, array<vec2<f32>, 16>, read_write>
            150923..150939 'shot.u...ry[15]': ref<storage, vec2<f32>, read_write>
            150936..150938 '15': integer
            150942..150957 'vec2f(1.0, 0.0)': vec2<f32>
            150948..150951 '1.0': float
            150953..150956 '0.0': float
            151026..151030 'shot': ptr<storage, ShotData, read_write>
            151026..151037 'shot.op_idx': ref<storage, u32, read_write>
            151040..151046 'op_idx': u32
            151060..151064 'shot': ptr<storage, ShotData, read_write>
            151060..151072 'shot.op_type': ref<storage, u32, read_write>
            151075..151077 'op': ptr<storage, Op, read>
            151075..151080 'op.id': ref<storage, u32, read>
            151220..151236 'has_lo...perand': bool
            151239..151286 'gate_h...1, q2)': bool
            151261..151269 'shot_idx': u32
            151271..151277 'op_idx': u32
            151279..151281 'q1': u32
            151283..151285 'q2': u32
            151304..151320 'has_lo...perand': bool
            151340..151392 'handle...1, q2)': [error]
            151367..151375 'shot_idx': u32
            151377..151383 'op_idx': u32
            151385..151387 'q1': u32
            151389..151391 'q2': u32
            151492..151504 'pauli_op_idx': u32
            151507..151534 'get_pa...p_idx)': u32
            151527..151533 'op_idx': u32
            151639..151651 'pauli_op_idx': u32
            151639..151657 'pauli_... != 0u': bool
            151655..151657 '0u': u32
            151679..151682 'ops': ref<storage, array<Op>, read>
            151679..151696 'ops[pa...p_idx]': ref<storage, Op, read>
            151679..151699 'ops[pa...dx].id': ref<storage, u32, read>
            151679..151722 'ops[pa...ISE_1Q': bool
            151683..151695 'pauli_op_idx': u32
            151703..151722 'OPID_P...ISE_1Q': u32
            151898..151915 '!has_l...perand': bool
            151899..151915 'has_lo...perand': bool
            151943..151999 'apply_...x, q1)': [error]
            151964..151972 'shot_idx': u32
            151974..151980 'op_idx': u32
            151982..151994 'pauli_op_idx': u32
            151996..151998 'q1': u32
            152072..152088 'has_lo...perand': bool
            152273..152345 'apply_...1, q2)': [error]
            152306..152314 'shot_idx': u32
            152316..152322 'op_idx': u32
            152324..152336 'pauli_op_idx': u32
            152338..152340 'q1': u32
            152342..152344 'q2': u32
            152400..152460 'apply_...1, q2)': [error]
            152421..152429 'shot_idx': u32
            152431..152437 'op_idx': u32
            152439..152451 'pauli_op_idx': u32
            152453..152455 'q1': u32
            152457..152459 'q2': u32
            152518..152523 'shots': ref<storage, array<ShotData>, read_write>
            152518..152533 'shots[shot_idx]': ref<storage, ShotData, read_write>
            152518..152540 'shots[...interp': ref<storage, InterpreterState, read_write>
            152518..152547 'shots[...status': ref<storage, u32, read_write>
            152524..152532 'shot_idx': u32
            152550..152564 'STATUS_RUNNING': u32
            152785..152801 'has_lo...perand': bool
            152821..152826 'shots': ref<storage, array<ShotData>, read_write>
            152821..152836 'shots[shot_idx]': ref<storage, ShotData, read_write>
            152821..152843 'shots[...interp': ref<storage, InterpreterState, read_write>
            152821..152850 'shots[...status': ref<storage, u32, read_write>
            152827..152835 'shot_idx': u32
            152853..152867 'STATUS_RUNNING': u32
            152976..153018 'finali...1, q2)': [error]
            152993..153001 'shot_idx': u32
            153003..153009 'op_idx': u32
            153011..153013 'q1': u32
            153015..153017 'q2': u32
            153043..153045 '1u': u32
            153232..153244 'pauli_op_idx': u32
            153247..153274 'get_pa...p_idx)': u32
            153267..153273 'op_idx': u32
            153292..153304 'pauli_op_idx': u32
            153292..153310 'pauli_... != 0u': bool
            153308..153310 '0u': u32
            153588..153591 'ops': ref<storage, array<Op>, read>
            153588..153605 'ops[pa...p_idx]': ref<storage, Op, read>
            153588..153608 'ops[pa...dx].id': ref<storage, u32, read>
            153588..153631 'ops[pa...ISE_1Q': bool
            153592..153604 'pauli_op_idx': u32
            153612..153631 'OPID_P...ISE_1Q': u32
            153654..153710 'apply_...x, q1)': [error]
            153675..153683 'shot_idx': u32
            153685..153691 'op_idx': u32
            153693..153705 'pauli_op_idx': u32
            153707..153709 'q1': u32
            153757..153817 'apply_...1, q2)': [error]
            153778..153786 'shot_idx': u32
            153788..153794 'op_idx': u32
            153796..153808 'pauli_op_idx': u32
            153810..153812 'q1': u32
            153814..153816 'q2': u32
            153853..153858 'shots': ref<storage, array<ShotData>, read_write>
            153853..153868 'shots[shot_idx]': ref<storage, ShotData, read_write>
            153853..153875 'shots[...interp': ref<storage, InterpreterState, read_write>
            153853..153882 'shots[...status': ref<storage, u32, read_write>
            153859..153867 'shot_idx': u32
            153885..153899 'STATUS_RUNNING': u32
            154001..154007 'resets': bool
            154010..154012 'op': ptr<storage, Op, read>
            154010..154015 'op.id': ref<storage, u32, read>
            154010..154031 'op.id ...RESETZ': bool
            154019..154031 'OPID_MRESETZ': u32
            154045..154110 'prep_m...esets)': [error]
            154064..154072 'shot_idx': u32
            154074..154080 'op_idx': u32
            154082..154084 'q1': u32
            154086..154088 'q2': u32
            154090..154095 'false': bool
            154097..154101 'true': bool
            154103..154109 'resets': bool
            154135..154137 '2u': u32
            154161..154225 'prep_m... true)': [error]
            154180..154188 'shot_idx': u32
            154190..154196 'op_idx': u32
            154198..154200 'q1': u32
            154202..154204 'q2': u32
            154206..154211 'false': bool
            154213..154218 'false': bool
            154220..154224 'true': bool
            154267..154271 'shot': ptr<storage, ShotData, read_write>
            154267..154279 'shot.op_type': ref<storage, u32, read_write>
            154282..154289 'OPID_ID': u32
            154382..154387 'shots': ref<storage, array<ShotData>, read_write>
            154382..154397 'shots[shot_idx]': ref<storage, ShotData, read_write>
            154382..154404 'shots[...interp': ref<storage, InterpreterState, read_write>
            154382..154411 'shots[...status': ref<storage, u32, read_write>
            154388..154396 'shot_idx': u32
            154414..154428 'STATUS_RUNNING': u32
            154753..154761 'globalId': vec3<u32>
            154784..154795 'IS_ADAPTIVE': bool
            154807..154843 'prepar...lId.x)': [error]
            154832..154840 'globalId': vec3<u32>
            154832..154842 'globalId.x': u32
            154866..154898 'prepar...lId.x)': [error]
            154887..154895 'globalId': vec3<u32>
            154887..154897 'globalId.x': u32
            155000..155011 'workgroupId': vec3<u32>
            155065..155068 'tid': u32
            155085..155093 'shot_idx': i32
            155101..155119 'i32(wo...pId.x)': i32
            155101..155141 'i32(wo...R_SHOT': i32
            155105..155116 'workgroupId': vec3<u32>
            155105..155118 'workgroupId.x': u32
            155122..155141 'WORKGR...R_SHOT': i32
            155151..155155 'shot': ptr<storage, ShotData, read_write>
            155158..155174 '&shots...t_idx]': ptr<storage, ShotData, read_write>
            155159..155164 'shots': ref<storage, array<ShotData>, read_write>
            155159..155174 'shots[shot_idx]': ref<storage, ShotData, read_write>
            155165..155173 'shot_idx': i32
            155381..155393 'update_probs': bool
            155396..155400 'shot': ptr<storage, ShotData, read_write>
            155396..155408 'shot.op_type': ref<storage, u32, read_write>
            155396..155419 'shot.o...PID_ID': bool
            155396..155460 'shot.o..._NOISE': bool
            155396..155499 'shot.o...PID_RZ': bool
            155396..155526 'shot.o...PID_CZ': bool
            155396..155554 'shot.o...ID_RZZ': bool
            155412..155419 'OPID_ID': u32
            155423..155427 'shot': ptr<storage, ShotData, read_write>
            155423..155435 'shot.op_type': ref<storage, u32, read_write>
            155423..155460 'shot.o..._NOISE': bool
            155439..155460 'OPID_C..._NOISE': u32
            155476..155480 'shot': ptr<storage, ShotData, read_write>
            155476..155488 'shot.op_type': ref<storage, u32, read_write>
            155476..155499 'shot.o...PID_RZ': bool
            155492..155499 'OPID_RZ': u32
            155503..155507 'shot': ptr<storage, ShotData, read_write>
            155503..155515 'shot.op_type': ref<storage, u32, read_write>
            155503..155526 'shot.o...PID_CZ': bool
            155519..155526 'OPID_CZ': u32
            155530..155534 'shot': ptr<storage, ShotData, read_write>
            155530..155542 'shot.op_type': ref<storage, u32, read_write>
            155530..155554 'shot.o...ID_RZZ': bool
            155546..155554 'OPID_RZZ': u32
            155565..155569 'shot': ptr<storage, ShotData, read_write>
            155565..155577 'shot.op_type': ref<storage, u32, read_write>
            155565..155588 'shot.o...PID_ID': bool
            155581..155588 'OPID_ID': u32
            155674..155716 'apply_..., tid)': [error]
            155697..155708 'workgroupId': vec3<u32>
            155697..155710 'workgroupId.x': u32
            155712..155715 'tid': u32
            155878..155922 'apply_...p_idx)': [error]
            155890..155901 'workgroupId': vec3<u32>
            155890..155903 'workgroupId.x': u32
            155905..155908 'tid': u32
            155910..155914 'shot': ptr<storage, ShotData, read_write>
            155910..155921 'shot.op_idx': ref<storage, u32, read_write>
            155977..155979 'q1': ref<function, u32, read_write>
            155998..156009 'IS_ADAPTIVE': bool
            156025..156027 'q1': ref<function, u32, read_write>
            156030..156055 'resolv..._idx))': u32
            156041..156054 'u32(shot_idx)': u32
            156045..156053 'shot_idx': i32
            156086..156088 'q1': ref<function, u32, read_write>
            156091..156094 'ops': ref<storage, array<Op>, read>
            156091..156107 'ops[sh...p_idx]': ref<storage, Op, read>
            156091..156110 'ops[sh...dx].q1': ref<storage, u32, read>
            156095..156099 'shot': ptr<storage, ShotData, read_write>
            156095..156106 'shot.op_idx': ref<storage, u32, read_write>
            156130..156165 'apply_...d, q1)': [error]
            156142..156153 'workgroupId': vec3<u32>
            156142..156155 'workgroupId.x': u32
            156157..156160 'tid': u32
            156162..156164 'q1': ref<function, u32, read_write>
            156209..156211 'q1': ref<function, u32, read_write>
            156230..156232 'q2': ref<function, u32, read_write>
            156251..156262 'IS_ADAPTIVE': bool
            156278..156280 'q1': ref<function, u32, read_write>
            156283..156308 'resolv..._idx))': u32
            156294..156307 'u32(shot_idx)': u32
            156298..156306 'shot_idx': i32
            156322..156324 'q2': ref<function, u32, read_write>
            156327..156352 'resolv..._idx))': u32
            156338..156351 'u32(shot_idx)': u32
            156342..156350 'shot_idx': i32
            156383..156385 'q1': ref<function, u32, read_write>
            156388..156391 'ops': ref<storage, array<Op>, read>
            156388..156404 'ops[sh...p_idx]': ref<storage, Op, read>
            156388..156407 'ops[sh...dx].q1': ref<storage, u32, read>
            156392..156396 'shot': ptr<storage, ShotData, read_write>
            156392..156403 'shot.op_idx': ref<storage, u32, read_write>
            156421..156423 'q2': ref<function, u32, read_write>
            156426..156429 'ops': ref<storage, array<Op>, read>
            156426..156442 'ops[sh...p_idx]': ref<storage, Op, read>
            156426..156445 'ops[sh...dx].q2': ref<storage, u32, read>
            156430..156434 'shot': ptr<storage, ShotData, read_write>
            156430..156441 'shot.op_idx': ref<storage, u32, read_write>
            156465..156504 'apply_...1, q2)': [error]
            156477..156488 'workgroupId': vec3<u32>
            156477..156490 'workgroupId.x': u32
            156492..156495 'tid': u32
            156497..156499 'q1': ref<function, u32, read_write>
            156501..156503 'q2': ref<function, u32, read_write>
            156673..156691 'workgr...rier()': [error]
            157014..157017 'tid': u32
            157014..157022 'tid == 0': bool
            157014..157038 'tid ==..._probs': bool
            157021..157022 '0': integer
            157026..157038 'update_probs': bool
            157054..157077 'workgr...on_idx': i32
            157085..157140 'select...T > 1)': i32
            157092..157094 '-1': integer
            157093..157094 '1': integer
            157096..157114 'i32(wo...pId.x)': i32
            157100..157111 'workgroupId': vec3<u32>
            157100..157113 'workgroupId.x': u32
            157116..157135 'WORKGR...R_SHOT': i32
            157116..157139 'WORKGR...OT > 1': bool
            157138..157139 '1': integer
            157159..157160 'q': ref<function, u32, read_write>
            157168..157170 '0u': u32
            157172..157173 'q': ref<function, u32, read_write>
            157172..157192 'q < u3...COUNT)': bool
            157176..157192 'u32(QU...COUNT)': u32
            157180..157191 'QUBIT_COUNT': i32
            157194..157195 'q': ref<function, u32, read_write>
            157216..157268 '(shot.... != 0u': bool
            157217..157221 'shot': ptr<storage, ShotData, read_write>
            157217..157249 'shot.q...p_mask': ref<storage, u32, read_write>
            157217..157261 'shot.q... << q)': u32
            157253..157255 '1u': u32
            157253..157260 '1u << q': u32
            157259..157260 'q': ref<function, u32, read_write>
            157266..157268 '0u': u32
            157287..157350 'sum_th...n_idx)': [error]
            157313..157314 'q': ref<function, u32, read_write>
            157316..157324 'shot_idx': i32
            157326..157349 'workgr...on_idx': i32
        "#]],
    );
}

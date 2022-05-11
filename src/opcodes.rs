use crate::types::{
    RefBytes, RefField, RefFloat, RefFun, RefGlobal, RefInt, RefString, RefType, Reg, ValBool,
};

/*
static OPCODE_ARGS: &[i8; 99] = &[
    2, 2, 2, 2, 2, 2, 1, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 2, 2, 1, 1, 2, 3, 4, 5, 6, -1, -1,
    -1, -1, 2, 3, 3, 2, 2, 3, 3, 2, 2, 3, 3, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 1, 2, 2, 2,
    2, 2, 2, 2, 0, 1, 1, 1, -1, 1, 2, 1, 3, 3, 3, 3, 3, 3, 3, 3, 1, 2, 2, 2, 2, 2, 2, 2, -1, 2, 2,
    4, 3, 0, 2, 3, 0,
];*/

pub type JumpOffset = i32;

#[hlbc_macros::gen_decode]
#[derive(Debug, Clone, strum_macros::IntoStaticStr)]
pub enum Opcode {
    Mov {
        dst: Reg,
        src: Reg,
    },
    Int {
        dst: Reg,
        ptr: RefInt,
    },
    Float {
        dst: Reg,
        ptr: RefFloat,
    },
    Bool {
        dst: Reg,
        value: ValBool,
    },
    Bytes {
        dst: Reg,
        ptr: RefBytes,
    },
    String {
        dst: Reg,
        ptr: RefString,
    },
    Null {
        dst: Reg,
    },
    Add {
        dst: Reg,
        a: Reg,
        b: Reg,
    },
    Sub {
        dst: Reg,
        a: Reg,
        b: Reg,
    },
    Mul {
        dst: Reg,
        a: Reg,
        b: Reg,
    },
    SDiv {
        dst: Reg,
        a: Reg,
        b: Reg,
    },
    UDiv {
        dst: Reg,
        a: Reg,
        b: Reg,
    },
    SMod {
        dst: Reg,
        a: Reg,
        b: Reg,
    },
    UMod {
        dst: Reg,
        a: Reg,
        b: Reg,
    },
    Shl {
        dst: Reg,
        a: Reg,
        b: Reg,
    },
    SShr {
        dst: Reg,
        a: Reg,
        b: Reg,
    },
    UShr {
        dst: Reg,
        a: Reg,
        b: Reg,
    },
    And {
        dst: Reg,
        a: Reg,
        b: Reg,
    },
    Or {
        dst: Reg,
        a: Reg,
        b: Reg,
    },
    Xor {
        dst: Reg,
        a: Reg,
        b: Reg,
    },
    Neg {
        dst: Reg,
        src: Reg,
    },
    Not {
        dst: Reg,
        src: Reg,
    },
    Incr {
        dst: Reg,
    },
    Decr {
        dst: Reg,
    },
    Call0 {
        dst: Reg,
        fun: RefFun,
    },
    Call1 {
        dst: Reg,
        fun: RefFun,
        arg0: Reg,
    },
    Call2 {
        dst: Reg,
        fun: RefFun,
        arg0: Reg,
        arg1: Reg,
    },
    Call3 {
        dst: Reg,
        fun: RefFun,
        arg0: Reg,
        arg1: Reg,
        arg2: Reg,
    },
    Call4 {
        dst: Reg,
        fun: RefFun,
        arg0: Reg,
        arg1: Reg,
        arg2: Reg,
        arg3: Reg,
    },
    CallN {
        dst: Reg,
        fun: RefFun,
        args: Vec<Reg>,
    },
    CallMethod {
        dst: Reg,
        obj: Reg,
        field: Reg,
        args: Vec<Reg>,
    },
    // Equivalent to CallMethod with obj = reg0
    CallThis {
        dst: Reg,
        field: Reg,
        args: Vec<Reg>,
    },
    CallClosure {
        dst: Reg,
        fun: Reg,
        args: Vec<Reg>,
    },
    StaticClosure {
        dst: Reg,
        fun: RefFun,
    },
    InstanceClosure {
        dst: Reg,
        fun: RefFun,
        obj: Reg,
    },
    VirtualClosure {
        dst: Reg,
        obj: Reg,
        field: Reg,
    },
    GetGlobal {
        dst: Reg,
        global: RefGlobal,
    },
    SetGlobal {
        global: RefGlobal,
        src: Reg,
    },
    Field {
        dst: Reg,
        obj: Reg,
        field: RefField,
    },
    SetField {
        obj: Reg,
        field: RefField,
        src: Reg,
    },
    // Equivalent to RefField with obj = reg0
    GetThis {
        dst: Reg,
        field: RefField,
    },
    SetThis {
        field: RefField,
        src: Reg,
    },
    DynGet {
        dst: Reg,
        obj: Reg,
        field: Reg,
    },
    DynSet {
        obj: Reg,
        field: Reg,
        src: Reg,
    },
    JTrue {
        cond: Reg,
        offset: JumpOffset,
    },
    JFalse {
        cond: Reg,
        offset: JumpOffset,
    },
    JNull {
        reg: Reg,
        offset: JumpOffset,
    },
    JNotNull {
        reg: Reg,
        offset: JumpOffset,
    },
    JSLt {
        a: Reg,
        b: Reg,
        offset: JumpOffset,
    },
    JSGte {
        a: Reg,
        b: Reg,
        offset: JumpOffset,
    },
    JSGt {
        a: Reg,
        b: Reg,
        offset: JumpOffset,
    },
    JSLte {
        a: Reg,
        b: Reg,
        offset: JumpOffset,
    },
    JULt {
        a: Reg,
        b: Reg,
        offset: JumpOffset,
    },
    JUGte {
        a: Reg,
        b: Reg,
        offset: JumpOffset,
    },
    JNotLt {
        a: Reg,
        b: Reg,
        offset: JumpOffset,
    },
    JNotGte {
        a: Reg,
        b: Reg,
        offset: JumpOffset,
    },
    JEq {
        a: Reg,
        b: Reg,
        offset: JumpOffset,
    },
    JNotEq {
        a: Reg,
        b: Reg,
        offset: JumpOffset,
    },
    JAlways {
        offset: JumpOffset,
    },
    ToDyn {
        dst: Reg,
        src: Reg,
    },
    ToSFloat {
        dst: Reg,
        src: Reg,
    },
    ToUFloat {
        dst: Reg,
        src: Reg,
    },
    ToInt {
        dst: Reg,
        src: Reg,
    },
    SafeCast {
        dst: Reg,
        src: Reg,
    },
    UnsafeCast {
        dst: Reg,
        src: Reg,
    },
    ToVirtual {
        dst: Reg,
        src: Reg,
    },
    // Negative jump offsets must target a label
    Label,
    Ret {
        ret: Reg,
    },
    Throw {
        exc: Reg,
    },
    Rethrow {
        exc: Reg,
    },
    Switch {
        reg: Reg,
        offsets: Vec<JumpOffset>,
        end: JumpOffset,
    },
    NullCheck {
        reg: Reg,
    },
    Trap {
        exc: Reg,
        offset: JumpOffset,
    },
    EndTrap {
        exc: Reg,
    },
    GetI8 {
        dst: Reg,
        bytes: Reg,
        index: Reg,
    },
    GetI16 {
        dst: Reg,
        bytes: Reg,
        index: Reg,
    },
    GetMem {
        dst: Reg,
        bytes: Reg,
        index: Reg,
    },
    GetArray {
        dst: Reg,
        array: Reg,
        index: Reg,
    },
    SetI8 {
        bytes: Reg,
        index: Reg,
        src: Reg,
    },
    SetI16 {
        bytes: Reg,
        index: Reg,
        src: Reg,
    },
    SetMem {
        bytes: Reg,
        index: Reg,
        src: Reg,
    },
    SetArray {
        array: Reg,
        index: Reg,
        src: Reg,
    },
    New {
        dst: Reg,
    },
    ArraySize {
        dst: Reg,
        array: Reg,
    },
    Type {
        dst: Reg,
        ty: RefType,
    },
    GetType {
        dst: Reg,
        src: Reg,
    },
    GetTID {
        dst: Reg,
        src: Reg,
    },
    Ref {
        dst: Reg,
        src: Reg,
    },
    Unref {
        dst: Reg,
        src: Reg,
    },
    Setref {
        dst: Reg,
        value: Reg,
    },
    MakeEnum {
        dst: Reg,
        construct: usize,
        args: Vec<Reg>,
    },
    EnumAlloc {
        dst: Reg,
        construct: usize,
    },
    EnumIndex {
        dst: Reg,
        construct: Reg,
    },
    EnumField {
        dst: Reg,
        enum_: Reg,
        construct: usize,
        field: RefField,
    },
    SetEnumField {
        enum_: Reg,
        field: RefField,
        src: Reg,
    },
    Assert,
    RefData {
        dst: Reg,
        src: Reg,
    },
    RefOffset {
        dst: Reg,
        reg: Reg,
        offset: usize,
    },
    Nop,
}
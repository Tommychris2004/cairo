use super::ap_tracking::ApTrackingLibfunc;
use super::array::{ArrayLibfunc, ArrayType};
use super::bitwise::{BitwiseLibfunc, BitwiseType};
use super::boolean::BoolLibfunc;
use super::branch_align::BranchAlignLibfunc;
use super::builtin_cost::{BuiltinCostLibfunc, BuiltinCostsType};
use super::casts::CastLibfunc;
use super::debug::DebugLibfunc;
use super::dict_felt252_to::{DictFelt252ToLibfunc, DictFelt252ToType};
use super::drop::DropLibfunc;
use super::duplicate::DupLibfunc;
use super::ec::{EcLibfunc, EcOpType, EcPointType, EcStateType};
use super::enm::{EnumLibfunc, EnumType};
use super::modules::boxing::{BoxLibfunc, BoxType};
use super::modules::felt252::{Felt252Libfunc, Felt252Type};
use super::modules::function_call::FunctionCallLibfunc;
use super::modules::gas::{GasBuiltinType, GasLibfunc};
use super::modules::mem::MemLibfunc;
use super::modules::non_zero::{NonZeroType, UnwrapNonZeroLibfunc};
use super::modules::uint128::{Uint128Libfunc, Uint128Type};
use super::modules::unconditional_jump::UnconditionalJumpLibfunc;
use super::nullable::{NullableLibfunc, NullableType};
use super::pedersen::{PedersenLibfunc, PedersenType};
use super::range_check::RangeCheckType;
use super::segment_arena::SegmentArenaType;
use super::snapshot::{SnapshotTakeLibfunc, SnapshotType};
use super::squashed_dict_felt252_to::SquashedDictFelt252ToType;
use super::starknet::{StarkNetLibfunc, StarkNetType};
use super::structure::{StructLibfunc, StructType};
use super::uint::{
    Uint16Libfunc, Uint16Type, Uint32Libfunc, Uint32Type, Uint64Libfunc, Uint64Type, Uint8Libfunc,
    Uint8Type,
};
use super::uninitialized::UninitializedType;
use crate::{define_libfunc_hierarchy, define_type_hierarchy};

define_type_hierarchy! {
    pub enum CoreType {
        Array(ArrayType),
        Bitwise(BitwiseType),
        Box(BoxType),
        EcOp(EcOpType),
        EcPoint(EcPointType),
        EcState(EcStateType),
        Felt252(Felt252Type),
        GasBuiltin(GasBuiltinType),
        BuiltinCosts(BuiltinCostsType),
        Uint8(Uint8Type),
        Uint16(Uint16Type),
        Uint32(Uint32Type),
        Uint64(Uint64Type),
        Uint128(Uint128Type),
        NonZero(NonZeroType),
        Nullable(NullableType),
        RangeCheck(RangeCheckType),
        Uninitialized(UninitializedType),
        Enum(EnumType),
        Struct(StructType),
        DictFelt252To(DictFelt252ToType),
        SquashedDictFelt252To(SquashedDictFelt252ToType),
        Pedersen(PedersenType),
        StarkNet(StarkNetType),
        SegmentArena(SegmentArenaType),
        Snapshot(SnapshotType),
    }, CoreTypeConcrete
}

define_libfunc_hierarchy! {
    pub enum CoreLibfunc {
        ApTracking(ApTrackingLibfunc),
        Array(ArrayLibfunc),
        Bitwise(BitwiseLibfunc),
        BranchAlign(BranchAlignLibfunc),
        Bool(BoolLibfunc),
        Box(BoxLibfunc),
        BuiltinCost(BuiltinCostLibfunc),
        Cast(CastLibfunc),
        Drop(DropLibfunc),
        Dup(DupLibfunc),
        Ec(EcLibfunc),
        Felt252(Felt252Libfunc),
        FunctionCall(FunctionCallLibfunc),
        Gas(GasLibfunc),
        Uint8(Uint8Libfunc),
        Uint16(Uint16Libfunc),
        Uint32(Uint32Libfunc),
        Uint64(Uint64Libfunc),
        Uint128(Uint128Libfunc),
        Mem(MemLibfunc),
        Nullable(NullableLibfunc),
        UnwrapNonZero(UnwrapNonZeroLibfunc),
        UnconditionalJump(UnconditionalJumpLibfunc),
        Enum(EnumLibfunc),
        Struct(StructLibfunc),
        DictFelt252To(DictFelt252ToLibfunc),
        Pedersen(PedersenLibfunc),
        StarkNet(StarkNetLibfunc),
        Debug(DebugLibfunc),
        SnapshotTake(SnapshotTakeLibfunc),
    }, CoreConcreteLibfunc
}

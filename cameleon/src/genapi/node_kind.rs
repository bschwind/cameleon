//! This module contains types which implement either one of `IInterface` defined in `GenICam
//! Starndard`.

use cameleon_genapi::{
    elem_type::{DisplayNotation, FloatRepresentation, IntegerRepresentation},
    interface::IncrementMode,
    prelude::*,
    Device, EnumEntryNode, GenApiResult, NodeId, ValueCtxt,
};

use super::{GenApiCtxt, ParamsCtxt};

/// A node that has `IInteger` interface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IntegerNode(NodeId);

/// A node that has `IFloat` interface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FloatNode(NodeId);

/// A node that has `IString` interface.
pub struct StringNode(NodeId);

/// A node that has `IEnumeration` interface.
pub struct EnumerationNode(NodeId);

/// A node that has `ICommand` interface.
pub struct CommandNode(NodeId);

macro_rules! delegate {
    (
        $expect_kind:ident,
        $(
            $(#[$meta:meta])*
            $vis:vis fn $method:ident<$Ctrl:ident, $Ctxt:ident>($self:ident, ctxt: &mut ParamsCtxt<Ctrl, Ctxt> $(,$arg:ident: $arg_ty:ty)*) -> $ret_ty:ty,)*) => {
        $(
            $(#[$meta])*
            $vis fn $method<$Ctrl, $Ctxt>($self $(,$arg: $arg_ty)*, ctxt: &mut ParamsCtxt<$Ctrl, $Ctxt>) -> $ret_ty
            where $Ctrl: Device,
                  $Ctxt: GenApiCtxt
            {
                with_ctxt(ctxt, |ctrl, ns, vc| {
                    $self.0
                        .$expect_kind(ns)
                        .unwrap()
                        .$method($($arg,)* ctrl, ns, vc)
                })
            }
        )*
    };

    (
        no_vc,
        $expect_kind:ident,
        $(
            $(#[$meta:meta])*
            $vis:vis fn $method:ident<$Ctrl:ident, $Ctxt:ident>($self:ident, ctxt: &mut ParamsCtxt<Ctrl, Ctxt> $(,$arg:ident: $arg_ty:ty)*) -> $ret_ty:ty,)*) => {
        $(
            $(#[$meta])*
            $vis fn $method<$Ctrl, $Ctxt>($self $(,$arg: $arg_ty)*, ctxt: &mut ParamsCtxt<$Ctrl, $Ctxt>) -> $ret_ty
            where $Ctrl: Device,
                  $Ctxt: GenApiCtxt
            {
                with_ctxt(ctxt, |_, ns, _| $self.0.$expect_kind(ns).unwrap().$method($($arg,)* ns))
            }
        )*
    };
}

fn with_ctxt<Ctrl, Ctxt, F, R>(ctxt: &mut ParamsCtxt<Ctrl, Ctxt>, mut f: F) -> R
where
    Ctrl: Device,
    Ctxt: GenApiCtxt,
    F: FnMut(&mut Ctrl, &Ctxt::NS, &mut ValueCtxt<Ctxt::VS, Ctxt::CS>) -> R,
{
    ctxt.enter(|ctrl, ctxt| ctxt.enter(|node_store, value_ctxt| f(ctrl, node_store, value_ctxt)))
}

impl IntegerNode {
    delegate! {
        expect_iinteger_kind,
        /// Returns the value of the node.
        pub fn value<Ctrl, Ctxt>(self, ctxt: &mut ParamsCtxt<Ctrl, Ctxt>) -> GenApiResult<i64>,
        /// Sets the value of the node.
        pub fn set_value<Ctrl, Ctxt>(self, ctxt: &mut ParamsCtxt<Ctrl, Ctxt>, value: i64) -> GenApiResult<()>,
        /// Returns the minimum value which the node can take.
        pub fn min<Ctrl, Ctxt>(self, ctxt: &mut ParamsCtxt<Ctrl, Ctxt>) -> GenApiResult<i64>,
        /// Restricts minimum value of the node.
        pub fn set_min<Ctrl, Ctxt>(self, ctxt: &mut ParamsCtxt<Ctrl, Ctxt>, value: i64) -> GenApiResult<()>,
        /// Returns the maximum value which the node can take.
        pub fn max<Ctrl, Ctxt>(self, ctxt: &mut ParamsCtxt<Ctrl, Ctxt>) -> GenApiResult<i64>,
        /// Restricts maximum value of the node.
        pub fn set_max<Ctrl, Ctxt>(self, ctxt: &mut ParamsCtxt<Ctrl, Ctxt>, value: i64) -> GenApiResult<()>,
        /// Returns the increment value if `inc_mode` returns IncrementMode::FixedIncrement. The value
        /// to set must be `min + i * Increment`.
        ///
        /// NOTE: Some nodes like `MaskedIntReg` doesn't have this element, though `IInteger`
        /// defines getter of the value.
        pub fn inc<Ctrl, Ctxt>(self, ctxt: &mut ParamsCtxt<Ctrl, Ctxt>) -> GenApiResult<Option<i64>>,
        /// Returns `true` if the node is readable.
        pub fn is_readable<Ctrl, Ctxt>(self, ctxt: &mut ParamsCtxt<Ctrl, Ctxt>) -> GenApiResult<bool>,
        /// Returns `true` if the node is writable.
        pub fn is_writable<Ctrl, Ctxt>(self, ctxt: &mut ParamsCtxt<Ctrl, Ctxt>) -> GenApiResult<bool>,
    }
    delegate! {
       no_vc,
       expect_iinteger_kind,
       /// Returns [`IncrementMode`] of the node.
       pub fn inc_mode<Ctrl, Ctxt>(self, ctxt: &mut ParamsCtxt<Ctrl, Ctxt>) -> Option<IncrementMode>,
       /// Returns [`IntegerRepresentation`] of the node. This feature is mainly for GUI.
       pub fn representation<Ctrl, Ctxt>(self, ctxt: &mut ParamsCtxt<Ctrl, Ctxt>) -> IntegerRepresentation,
    }
}

impl FloatNode {
    delegate! {
        expect_ifloat_kind,
        /// Returns the value of the node.
        pub fn value<Ctrl, Ctxt>(self, ctxt: &mut ParamsCtxt<Ctrl, Ctxt>) -> GenApiResult<f64>,
        /// Sets the value of the node.
        pub fn set_value<Ctrl, Ctxt>(self, ctxt: &mut ParamsCtxt<Ctrl, Ctxt>, value: f64) -> GenApiResult<()>,
        /// Returns minimum value which the node can take.
        pub fn min<Ctrl, Ctxt>(self, ctxt: &mut ParamsCtxt<Ctrl, Ctxt>) -> GenApiResult<f64>,
        /// Returns maximum value which the node can take.
        pub fn max<Ctrl, Ctxt>(self, ctxt: &mut ParamsCtxt<Ctrl, Ctxt>) -> GenApiResult<f64>,
        /// Returns the increment value if `inc_mode` returns IncrementMode::FixedIncrement. The value
        /// to set must be `min + i * Increment`.
        ///
        /// NOTE: Some nodes like `MaskedIntReg` doesn't have this element, though `IFloat`
        /// defines getter of the value.
        pub fn inc<Ctrl, Ctxt>(self, ctxt: &mut ParamsCtxt<Ctrl, Ctxt>) -> GenApiResult<Option<f64>>,
        /// Returns `true` if the node is readable.
        pub fn is_readable<Ctrl, Ctxt>(self, ctxt: &mut ParamsCtxt<Ctrl, Ctxt>) -> GenApiResult<bool>,
        /// Returns `true` if the node is writable.
        pub fn is_writable<Ctrl, Ctxt>(self, ctxt: &mut ParamsCtxt<Ctrl, Ctxt>) -> GenApiResult<bool>,
    }

    delegate! {
       no_vc,
       expect_ifloat_kind,
       /// Returns [`IncrementMode`] of the node.
       pub fn inc_mode<Ctrl, Ctxt>(self, ctxt: &mut ParamsCtxt<Ctrl, Ctxt>) -> Option<IncrementMode>,
       /// Returns [`FloatRepresentation`] of the node. This feature is mainly for GUI.
       pub fn representation<Ctrl, Ctxt>(self, ctxt: &mut ParamsCtxt<Ctrl, Ctxt>) ->FloatRepresentation,
       /// Returns [`DisplayNotation`]. This featres is mainly for GUI.
       pub fn display_notation<Ctrl, Ctxt>(self, ctxt: &mut ParamsCtxt<Ctrl, Ctxt>) -> DisplayNotation,
    }

    /// Returns unit that describes phisical meaning of the value. e.g. "Hz" or "ms". This
    /// feature is mainly for GUI.
    pub fn unit<Ctrl, Ctxt>(self, ctxt: &mut ParamsCtxt<Ctrl, Ctxt>) -> Option<String>
    where
        Ctrl: Device,
        Ctxt: GenApiCtxt,
    {
        with_ctxt(ctxt, |_, ns, _| {
            self.0
                .expect_ifloat_kind(ns)
                .unwrap()
                .unit(ns)
                .map(String::from)
        })
    }
}

impl StringNode {
    delegate! {
        expect_istring_kind,
        /// Returns the value of the node.
        pub fn value<Ctrl, Ctxt>(self, ctxt: &mut ParamsCtxt<Ctrl, Ctxt>) -> GenApiResult<String>,
        /// Sets the value of the node.
        pub fn set_value<Ctrl, Ctxt>(self, ctxt: &mut ParamsCtxt<Ctrl, Ctxt>, value: &str) -> GenApiResult<()>,
        /// Retruns the maximum length of the string.
        pub fn max_length<Ctrl, Ctxt>(self, ctxt: &mut ParamsCtxt<Ctrl, Ctxt>) -> GenApiResult<i64>,
        /// Returns `true` if the node is readable.
        pub fn is_readable<Ctrl, Ctxt>(self, ctxt: &mut ParamsCtxt<Ctrl, Ctxt>) -> GenApiResult<bool>,
        /// Returns `true` if the node is writable.
        pub fn is_writable<Ctrl, Ctxt>(self, ctxt: &mut ParamsCtxt<Ctrl, Ctxt>) -> GenApiResult<bool>,
    }
}

impl EnumerationNode {
    delegate! {
    expect_ienumeration_kind,
        /// Sets entry to the enumeration node by the entry name.
        pub fn set_entry_by_name<Ctrl, Ctxt>(self, ctxt: &mut ParamsCtxt<Ctrl, Ctxt>, name: &str) -> GenApiResult<()>,
        /// Sets entry to the enumeration node by the entry value.
        pub fn set_entry_by_value<Ctrl, Ctxt>(self, ctxt: &mut ParamsCtxt<Ctrl, Ctxt>, value: i64) -> GenApiResult<()>,
        /// Returns `true` if the node is readable.
        pub fn is_readable<Ctrl, Ctxt>(self, ctxt: &mut ParamsCtxt<Ctrl, Ctxt>) -> GenApiResult<bool>,
        /// Returns `true` if the node is writable.
        pub fn is_writable<Ctrl, Ctxt>(self, ctxt: &mut ParamsCtxt<Ctrl, Ctxt>) -> GenApiResult<bool>,
    }

    /// Allows the access to entries of the node.
    pub fn with_entries<Ctrl, Ctxt, F, R>(self, ctxt: &mut ParamsCtxt<Ctrl, Ctxt>, mut f: F) -> R
    where
        Ctrl: Device,
        Ctxt: GenApiCtxt,
        F: FnMut(&[EnumEntryNode]) -> R,
    {
        with_ctxt(ctxt, |_, ns, _| {
            f(self.0.expect_ienumeration_kind(ns).unwrap().entries(ns))
        })
    }

    /// Returns entries of the node.
    ///
    /// This method isn't cheap, consider using [`Self::with_entries`] instead of calling this
    /// method.
    pub fn entries<Ctrl, Ctxt>(self, ctxt: &mut ParamsCtxt<Ctrl, Ctxt>) -> Vec<EnumEntryNode>
    where
        Ctrl: Device,
        Ctxt: GenApiCtxt,
    {
        with_ctxt(ctxt, |_, ns, _| {
            self.0
                .expect_ienumeration_kind(ns)
                .unwrap()
                .entries(ns)
                .iter()
                .map(|entry| entry.clone())
                .collect()
        })
    }
}

impl CommandNode {
    delegate! {
        expect_icommand_kind,
        /// Execute the command.
        pub fn execute<Ctrl, Ctxt>(self, ctxt: &mut ParamsCtxt<Ctrl, Ctxt>) -> GenApiResult<()>,
        /// Returns `true` if the previous command is executed on the device.
        pub fn is_done<Ctrl, Ctxt>(self, ctxt: &mut ParamsCtxt<Ctrl, Ctxt>) -> GenApiResult<bool>,
        /// Returns `true` if the node is writable (executable).
        pub fn is_writable<Ctrl, Ctxt>(self, ctxt: &mut ParamsCtxt<Ctrl, Ctxt>) -> GenApiResult<bool>,
    }
}

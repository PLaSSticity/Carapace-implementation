#![allow(non_camel_case_types)]
//use serde::ser::{Serialize, Serializer, SerializeStruct};
/******************************************************************************************************
* Lattice and struct definition 
******************************************************************************************************/
// Three independent integrity negative labels at same level: NotX, NotY, NotZ
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Label_NotX { }
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Label_NotY { }
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Label_NotZ { }

// Three mid-level integrity labels for each combination of NotX, NotY, and NotZ
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Label_NotXY { }
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Label_NotYZ { }
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Label_NotXZ { }

unsafe impl crate::ternary_lattice::Label for Label_NotX {}
unsafe impl crate::ternary_lattice::Label for Label_NotY {}
unsafe impl crate::ternary_lattice::Label for Label_NotZ {}
unsafe impl crate::ternary_lattice::Label for Label_NotXY {}
unsafe impl crate::ternary_lattice::Label for Label_NotXZ {}
unsafe impl crate::ternary_lattice::Label for Label_NotYZ {}
unsafe impl crate::ternary_lattice::Label for Label_NotXYZ {}
unsafe impl crate::ternary_lattice::Label for Label_All {}

// Top (All) and bottom (NotXYZ) level integrity labels
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Label_All { }
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Label_NotXYZ { }

// Define the integrity level lattice using this trait
pub trait LowerIntegrityThan<T> { }

// encode lattice relationships
impl<T> LowerIntegrityThan<T> for T { }    // reflexive property

impl LowerIntegrityThan<Label_All> for Label_NotX { } 
impl LowerIntegrityThan<Label_All> for Label_NotY { } 
impl LowerIntegrityThan<Label_All> for Label_NotZ { }
impl LowerIntegrityThan<Label_All> for Label_NotXY { }
impl LowerIntegrityThan<Label_All> for Label_NotYZ { } 
impl LowerIntegrityThan<Label_All> for Label_NotXZ { } 
impl LowerIntegrityThan<Label_All> for Label_NotXYZ { }

impl LowerIntegrityThan<Label_NotX> for Label_NotXY { }
impl LowerIntegrityThan<Label_NotX> for Label_NotXZ { }
impl LowerIntegrityThan<Label_NotX> for Label_NotXYZ { }

impl LowerIntegrityThan<Label_NotY> for Label_NotXY { }
impl LowerIntegrityThan<Label_NotY> for Label_NotYZ { }
impl LowerIntegrityThan<Label_NotY> for Label_NotXYZ { }

impl LowerIntegrityThan<Label_NotZ> for Label_NotYZ { }
impl LowerIntegrityThan<Label_NotZ> for Label_NotXZ { }
impl LowerIntegrityThan<Label_NotZ> for Label_NotXYZ { }

impl LowerIntegrityThan<Label_NotXY> for Label_NotXYZ { }
impl LowerIntegrityThan<Label_NotYZ> for Label_NotXYZ { }
impl LowerIntegrityThan<Label_NotXZ> for Label_NotXYZ { }
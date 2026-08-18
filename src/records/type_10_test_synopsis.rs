use crate::stdf_codec::*;
use rust_stdf_derive::StdfRecordCodec;
use smart_default::SmartDefault;

#[cfg(feature = "serialize")]
use serde::Serialize;
#[cfg(feature = "serialize")]
use struct_field_names_as_array::FieldNamesAsArray;

#[cfg_attr(
    feature = "serialize",
    derive(Serialize, FieldNamesAsArray),
    serde(rename_all = "UPPERCASE"),
    field_names_as_array(rename_all = "UPPERCASE")
)]
#[derive(SmartDefault, Debug, Clone, PartialEq, StdfRecordCodec)]
pub struct TSR {
    pub head_num: U1, // Test head number
    pub site_num: U1, // Test site number
    #[default = ' ']
    pub test_typ: C1, // Test type
    pub test_num: U4, // Test number
    #[default = 4_294_967_295]
    pub exec_cnt: U4, // Number of test executions
    #[default = 4_294_967_295]
    pub fail_cnt: U4, // Number of test failures
    #[default = 4_294_967_295]
    pub alrm_cnt: U4, // Number of alarmed tests
    pub test_nam: Cn, // Test name
    pub seq_name: Cn, // Sequencer (program segment/flow) name
    pub test_lbl: Cn, // Test label or text
    pub opt_flag: B1, // Optional data flag
    pub test_tim: R4, // Average test execution time in seconds
    pub test_min: R4, // Lowest test result value
    pub test_max: R4, // Highest test result value
    pub tst_sums: R4, // Sum of test result values
    pub tst_sqrs: R4, // Sum of squares of test result values
}

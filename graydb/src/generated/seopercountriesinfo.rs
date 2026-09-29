//! 由 `codegen` 从 `sql/jzdb_secu_schema.sql` + `tables.toml` 生成 —— 请勿手改。
//! 重新生成：`cargo codegen`。业务不变量写在 `crate::domain` 的 `impl RowValidator`，不被本文件覆盖。
#![allow(dead_code)]

use crate::tables::RowValidator;
use crate::tables::{ColVal, ColumnName, DataTable};

/// `tb_seoper_countries_info`（codegen：字段与列名源自 DDL，`SeoperCountriesInfoColumn::as_str` 与本结构体字段同源，不可能写错）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SeoperCountriesInfo {
    pub row_id: i64,
    pub create_date: i32,
    pub create_time: i32,
    pub update_date: i32,
    pub update_time: i32,
    pub update_times: i32,
    pub country_name: String,
    pub country_name_en: String,
    pub country_fullname_en: String,
    pub country_code_two: String,
    pub country_code_three: String,
    pub country_code_no: i32,
    pub remark_info: String,
}

/// `tb_seoper_countries_info` 列枚举（codegen）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeoperCountriesInfoColumn {
    RowId,
    CreateDate,
    CreateTime,
    UpdateDate,
    UpdateTime,
    UpdateTimes,
    CountryName,
    CountryNameEn,
    CountryFullnameEn,
    CountryCodeTwo,
    CountryCodeThree,
    CountryCodeNo,
    RemarkInfo,
}

impl ColumnName for SeoperCountriesInfoColumn {
    const ALL: &'static [Self] = &[Self::RowId, Self::CreateDate, Self::CreateTime, Self::UpdateDate, Self::UpdateTime, Self::UpdateTimes, Self::CountryName, Self::CountryNameEn, Self::CountryFullnameEn, Self::CountryCodeTwo, Self::CountryCodeThree, Self::CountryCodeNo, Self::RemarkInfo];
    fn as_str(self) -> &'static str {
        match self {
            Self::RowId => "row_id",
            Self::CreateDate => "create_date",
            Self::CreateTime => "create_time",
            Self::UpdateDate => "update_date",
            Self::UpdateTime => "update_time",
            Self::UpdateTimes => "update_times",
            Self::CountryName => "country_name",
            Self::CountryNameEn => "country_name_en",
            Self::CountryFullnameEn => "country_fullname_en",
            Self::CountryCodeTwo => "country_code_two",
            Self::CountryCodeThree => "country_code_three",
            Self::CountryCodeNo => "country_code_no",
            Self::RemarkInfo => "remark_info",
        }
    }
}

impl DataTable for SeoperCountriesInfo {
    const ID: &'static str = "tb_seoper_countries_info";
    type Column = SeoperCountriesInfoColumn;

    fn column(&self, col: Self::Column) -> Option<ColVal<'_>> {
        match col {
            SeoperCountriesInfoColumn::RowId => Some(ColVal::Int(self.row_id)),
            _ => None,
        }
    }
}

impl RowValidator for SeoperCountriesInfo {}

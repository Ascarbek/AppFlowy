use crate::entities::{ChecklistFilterPB, FieldType};
use crate::impl_type_option;
use crate::services::cell::{CellDataChangeset, FromCellString, TypeCellData};
use crate::services::field::{
  BoxTypeOptionBuilder, SelectOptionCellChangeset, SelectOptionCellDataPB, SelectOptionIds,
  SelectOptionPB, SelectTypeOptionSharedAction, SelectedSelectOptions, TypeOption,
  TypeOptionBuilder, TypeOptionCellData, TypeOptionCellDataCompare, TypeOptionCellDataFilter,
};
use bytes::Bytes;
use database_model::{FieldRevision, TypeOptionDataDeserializer, TypeOptionDataSerializer};
use flowy_derive::ProtoBuf;
use flowy_error::FlowyResult;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

// Multiple select
#[derive(Clone, Debug, Default, Serialize, Deserialize, ProtoBuf)]
pub struct ChecklistTypeOptionPB {
  #[pb(index = 1)]
  pub options: Vec<SelectOptionPB>,

  #[pb(index = 2)]
  pub disable_color: bool,
}
impl_type_option!(ChecklistTypeOptionPB, FieldType::Checklist);

impl TypeOption for ChecklistTypeOptionPB {
  type CellData = SelectOptionIds;
  type CellChangeset = SelectOptionCellChangeset;
  type CellProtobufType = SelectOptionCellDataPB;
  type CellFilter = ChecklistFilterPB;
}

impl TypeOptionCellData for ChecklistTypeOptionPB {
  fn convert_to_protobuf(
    &self,
    cell_data: <Self as TypeOption>::CellData,
  ) -> <Self as TypeOption>::CellProtobufType {
    self.get_selected_options(cell_data)
  }

  fn decode_type_option_cell_str(
    &self,
    cell_str: String,
  ) -> FlowyResult<<Self as TypeOption>::CellData> {
    SelectOptionIds::from_cell_str(&cell_str)
  }
}

impl SelectTypeOptionSharedAction for ChecklistTypeOptionPB {
  fn number_of_max_options(&self) -> Option<usize> {
    None
  }

  fn options(&self) -> &Vec<SelectOptionPB> {
    &self.options
  }

  fn mut_options(&mut self) -> &mut Vec<SelectOptionPB> {
    &mut self.options
  }
}

impl CellDataChangeset for ChecklistTypeOptionPB {
  fn apply_changeset(
    &self,
    changeset: <Self as TypeOption>::CellChangeset,
    type_cell_data: Option<TypeCellData>,
  ) -> FlowyResult<(String, <Self as TypeOption>::CellData)> {
    let insert_option_ids = changeset
      .insert_option_ids
      .into_iter()
      .filter(|insert_option_id| {
        self
          .options
          .iter()
          .any(|option| &option.id == insert_option_id)
      })
      .collect::<Vec<String>>();

    let select_option_ids = match type_cell_data {
      None => SelectOptionIds::from(insert_option_ids),
      Some(type_cell_data) => {
        let mut select_ids: SelectOptionIds = type_cell_data.cell_str.into();
        for insert_option_id in insert_option_ids {
          if !select_ids.contains(&insert_option_id) {
            select_ids.push(insert_option_id);
          }
        }

        for delete_option_id in changeset.delete_option_ids {
          select_ids.retain(|id| id != &delete_option_id);
        }

        select_ids
      },
    };
    Ok((select_option_ids.to_string(), select_option_ids))
  }
}
impl TypeOptionCellDataFilter for ChecklistTypeOptionPB {
  fn apply_filter(
    &self,
    filter: &<Self as TypeOption>::CellFilter,
    field_type: &FieldType,
    cell_data: &<Self as TypeOption>::CellData,
  ) -> bool {
    if !field_type.is_check_list() {
      return true;
    }
    let selected_options =
      SelectedSelectOptions::from(self.get_selected_options(cell_data.clone()));
    filter.is_visible(&self.options, &selected_options)
  }
}

impl TypeOptionCellDataCompare for ChecklistTypeOptionPB {
  fn apply_cmp(
    &self,
    cell_data: &<Self as TypeOption>::CellData,
    other_cell_data: &<Self as TypeOption>::CellData,
  ) -> Ordering {
    cell_data.len().cmp(&other_cell_data.len())
  }
}

#[derive(Default)]
pub struct ChecklistTypeOptionBuilder(ChecklistTypeOptionPB);
impl_into_box_type_option_builder!(ChecklistTypeOptionBuilder);
impl_builder_from_json_str_and_from_bytes!(ChecklistTypeOptionBuilder, ChecklistTypeOptionPB);
impl ChecklistTypeOptionBuilder {
  pub fn add_option(mut self, opt: SelectOptionPB) -> Self {
    self.0.options.push(opt);
    self
  }
}

impl TypeOptionBuilder for ChecklistTypeOptionBuilder {
  fn field_type(&self) -> FieldType {
    FieldType::Checklist
  }

  fn serializer(&self) -> &dyn TypeOptionDataSerializer {
    &self.0
  }
}

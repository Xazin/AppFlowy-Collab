use crate::views::{
  filters_from_map_ref, group_setting_from_map_ref, layout_setting_from_map_ref,
  sorts_from_map_ref, view_from_map_ref, view_from_value, view_id_from_map_ref, DatabaseLayout,
  DatabaseView, FieldOrder, FieldOrderArray, FilterMap, GroupSettingMap, LayoutSetting, OrderArray,
  RowOrder, RowOrderArray, SortMap, ViewBuilder, ViewUpdate, FIELD_ORDERS, ROW_ORDERS, VIEW_LAYOUT,
};
use collab::preclude::{Map, MapRef, MapRefExtension, MapRefWrapper, ReadTxn, TransactionMut};

pub struct ViewMap {
  container: MapRefWrapper,
}

impl ViewMap {
  pub fn new(container: MapRefWrapper) -> Self {
    // let field_order = FieldOrderArray::new(field_order);
    Self { container }
  }

  pub fn insert_view(&self, view: DatabaseView) {
    self
      .container
      .with_transact_mut(|txn| self.insert_view_with_txn(txn, view))
  }

  pub fn insert_view_with_txn(&self, txn: &mut TransactionMut, view: DatabaseView) {
    let map_ref = self.container.insert_map_with_txn(txn, &view.id);
    ViewBuilder::new(&view.id, txn, map_ref).update(|update| {
      update
        .set_name(view.name)
        .set_database_id(view.database_id)
        .set_layout_settings(view.layout_settings)
        .set_layout_type(view.layout)
        .set_filters(view.filters)
        .set_groups(view.group_settings)
        .set_sorts(view.sorts)
        .set_field_orders(view.field_orders)
        .set_row_orders(view.row_orders);
    });
  }

  pub fn get_view_group_setting(&self, view_id: &str) -> Vec<GroupSettingMap> {
    let txn = self.container.transact();
    self.get_view_group_setting_with_txn(&txn, view_id)
  }

  pub fn get_view_group_setting_with_txn<T: ReadTxn>(
    &self,
    txn: &T,
    view_id: &str,
  ) -> Vec<GroupSettingMap> {
    if let Some(map_ref) = self.container.get_map_with_txn(txn, view_id) {
      group_setting_from_map_ref(txn, &map_ref)
    } else {
      vec![]
    }
  }

  pub fn get_view_sorts(&self, view_id: &str) -> Vec<SortMap> {
    let txn = self.container.transact();
    self.get_view_sorts_with_txn(&txn, view_id)
  }

  pub fn get_view_sorts_with_txn<T: ReadTxn>(&self, txn: &T, view_id: &str) -> Vec<SortMap> {
    if let Some(map_ref) = self.container.get_map_with_txn(txn, view_id) {
      sorts_from_map_ref(txn, &map_ref)
    } else {
      vec![]
    }
  }
  pub fn get_view_filters(&self, view_id: &str) -> Vec<FilterMap> {
    let txn = self.container.transact();
    self.get_view_filters_with_txn(&txn, view_id)
  }

  pub fn get_view_filters_with_txn<T: ReadTxn>(&self, txn: &T, view_id: &str) -> Vec<FilterMap> {
    if let Some(map_ref) = self.container.get_map_with_txn(txn, view_id) {
      filters_from_map_ref(txn, &map_ref)
    } else {
      vec![]
    }
  }

  pub fn get_layout_setting<T: From<LayoutSetting>>(
    &self,
    view_id: &str,
    layout_ty: &DatabaseLayout,
  ) -> Option<T> {
    let txn = self.container.transact();
    if let Some(map_ref) = self.container.get_map_with_txn(&txn, view_id) {
      layout_setting_from_map_ref(&txn, &map_ref)
        .get(layout_ty)
        .map(|value| T::from(value.clone()))
    } else {
      None
    }
  }

  pub fn get_view(&self, view_id: &str) -> Option<DatabaseView> {
    let txn = self.container.transact();
    self.get_view_with_txn(&txn, view_id)
  }

  pub fn get_view_with_txn<T: ReadTxn>(&self, txn: &T, view_id: &str) -> Option<DatabaseView> {
    let map_ref = self.container.get_map_with_txn(txn, view_id)?;
    view_from_map_ref(&map_ref, txn)
  }

  pub fn get_all_views(&self) -> Vec<DatabaseView> {
    let txn = self.container.transact();
    self.get_all_views_with_txn(&txn)
  }

  pub fn get_all_views_with_txn<T: ReadTxn>(&self, txn: &T) -> Vec<DatabaseView> {
    self
      .container
      .iter(txn)
      .flat_map(|(_k, v)| view_from_value(v, txn))
      .collect::<Vec<_>>()
  }

  pub fn get_view_layout(&self, view_id: &str) -> Option<DatabaseLayout> {
    let txn = self.container.transact();
    self
      .container
      .get_map_with_txn(&txn, view_id)
      .map(|map_ref| {
        map_ref
          .get_i64_with_txn(&txn, VIEW_LAYOUT)
          .map(|value| DatabaseLayout::try_from(value).ok())?
      })?
  }

  pub fn get_view_row_orders_with_txn<T: ReadTxn>(&self, txn: &T, view_id: &str) -> Vec<RowOrder> {
    self
      .container
      .get_map_with_txn(txn, view_id)
      .map(|map_ref| {
        map_ref
          .get_array_ref_with_txn(txn, ROW_ORDERS)
          .map(|array_ref| RowOrderArray::new(array_ref.into_inner()).get_orders_with_txn(txn))
          .unwrap_or_default()
      })
      .unwrap_or_default()
  }

  pub fn get_view_field_orders_txn<T: ReadTxn>(&self, txn: &T, view_id: &str) -> Vec<FieldOrder> {
    self
      .container
      .get_map_with_txn(txn, view_id)
      .map(|map_ref| {
        map_ref
          .get_array_ref_with_txn(txn, FIELD_ORDERS)
          .map(|array_ref| FieldOrderArray::new(array_ref.into_inner()).get_orders_with_txn(txn))
          .unwrap_or_default()
      })
      .unwrap_or_default()
  }

  pub fn update_view<F>(&self, view_id: &str, f: F)
  where
    F: FnOnce(ViewUpdate),
  {
    self
      .container
      .with_transact_mut(|txn| self.update_view_with_txn(txn, view_id, f))
  }

  pub fn update_view_with_txn<F>(&self, txn: &mut TransactionMut, view_id: &str, f: F)
  where
    F: FnOnce(ViewUpdate),
  {
    if let Some(map_ref) = self.container.get_map_with_txn(txn, view_id) {
      let update = ViewUpdate::new(view_id, txn, &map_ref);
      f(update)
    } else {
      tracing::warn!("Can't update the view. The view is not found")
    }
  }

  pub fn update_all_views_with_txn<F>(&self, txn: &mut TransactionMut, f: F)
  where
    F: Fn(ViewUpdate),
  {
    let map_refs = self
      .container
      .iter(txn)
      .flat_map(|(_k, v)| v.to_ymap())
      .collect::<Vec<MapRef>>();

    for map_ref in map_refs {
      if let Some(view_id) = view_id_from_map_ref(&map_ref, txn) {
        let update = ViewUpdate::new(&view_id, txn, &map_ref);
        f(update)
      }
    }
  }

  pub fn delete_view(&self, view_id: &str) {
    self.container.with_transact_mut(|txn| {
      self.container.remove(txn, view_id);
    })
  }

  pub fn clear_with_txn(&self, txn: &mut TransactionMut) {
    self.container.clear(txn);
  }

  pub fn delete_view_with_txn(&self, txn: &mut TransactionMut, view_id: &str) {
    self.container.remove(txn, view_id);
  }
}

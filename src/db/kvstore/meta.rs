use crate::db::data_structure::adlist::adlist::Node;
use crate::db::kvstore::kvstore::KvStoreDictMetadata;
use std::ptr::NonNull;

pub struct KvStoreDictMetaBase<T> {
    pub rehashing_node: Option<NonNull<Node<T>>>,
}

pub struct KvStoreDictMetaEx<T> {
    pub base: KvStoreDictMetaBase<T>,
    pub mata: KvStoreDictMetadata,
}

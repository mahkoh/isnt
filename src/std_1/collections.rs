// This file was generated

mod b_tree_map_private { pub trait Sealed<K, V> { } }

/// Extension for [`BTreeMap`](std::collections::BTreeMap)
pub trait IsntBTreeMapExt<K, V>: b_tree_map_private::Sealed<K, V> {
    /// The negation of [`is_empty`](std::collections::BTreeMap::is_empty)
    #[must_use]
    fn is_not_empty(&self) -> bool;
}

impl<K, V> b_tree_map_private::Sealed<K, V> for std::collections::BTreeMap<K, V> { }

impl<K, V> IsntBTreeMapExt<K, V> for std::collections::BTreeMap<K, V> {
    #[inline]
    fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }
}

mod b_tree_map2_private { pub trait Sealed<K, V> { } }

/// Extension for [`BTreeMap`](std::collections::BTreeMap)
pub trait IsntBTreeMap2Ext<K, V>: b_tree_map2_private::Sealed<K, V> {
    /// The negation of [`contains_key`](std::collections::BTreeMap::contains_key)
    #[must_use]
    fn not_contains_key<Q>(&self, key: &Q) -> bool where K: std::borrow::Borrow<Q>, Q: std::cmp::Ord + ?Sized,;
}

impl<K, V> b_tree_map2_private::Sealed<K, V> for std::collections::BTreeMap<K, V> where K: std::cmp::Ord { }

impl<K, V> IsntBTreeMap2Ext<K, V> for std::collections::BTreeMap<K, V> where K: std::cmp::Ord {
    #[inline]
    fn not_contains_key<Q>(&self, key: &Q) -> bool where K: std::borrow::Borrow<Q>, Q: std::cmp::Ord + ?Sized, {
        !self.contains_key::<Q>(key)
    }
}

mod b_tree_set_private { pub trait Sealed<T> { } }

/// Extension for [`BTreeSet`](std::collections::BTreeSet)
pub trait IsntBTreeSetExt<T>: b_tree_set_private::Sealed<T> {
    /// The negation of [`is_empty`](std::collections::BTreeSet::is_empty)
    #[must_use]
    fn is_not_empty(&self) -> bool;
}

impl<T> b_tree_set_private::Sealed<T> for std::collections::BTreeSet<T> { }

impl<T> IsntBTreeSetExt<T> for std::collections::BTreeSet<T> {
    #[inline]
    fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }
}

mod b_tree_set2_private { pub trait Sealed<T: std::cmp::Ord> { } }

/// Extension for [`BTreeSet`](std::collections::BTreeSet)
pub trait IsntBTreeSet2Ext<T: std::cmp::Ord>: b_tree_set2_private::Sealed<T> {
    /// The negation of [`is_disjoint`](std::collections::BTreeSet::is_disjoint)
    #[must_use]
    fn is_not_disjoint(&self, other: &std::collections::BTreeSet<T>) -> bool;
    /// The negation of [`is_subset`](std::collections::BTreeSet::is_subset)
    #[must_use]
    fn is_not_subset(&self, other: &std::collections::BTreeSet<T>) -> bool;
    /// The negation of [`is_superset`](std::collections::BTreeSet::is_superset)
    #[must_use]
    fn is_not_superset(&self, other: &std::collections::BTreeSet<T>) -> bool;
    /// The negation of [`contains`](std::collections::BTreeSet::contains)
    #[must_use]
    fn not_contains<Q>(&self, value: &Q) -> bool where Q: std::cmp::Ord + ?Sized, T: std::borrow::Borrow<Q>,;
    /// The negation of [`insert`](std::collections::BTreeSet::insert)
    #[must_use]
    fn not_insert(&mut self, value: T) -> bool;
    /// The negation of [`remove`](std::collections::BTreeSet::remove)
    #[must_use]
    fn not_remove<Q>(&mut self, value: &Q) -> bool where Q: std::cmp::Ord + ?Sized, T: std::borrow::Borrow<Q>,;
}

impl<T: std::cmp::Ord> b_tree_set2_private::Sealed<T> for std::collections::BTreeSet<T> { }

impl<T: std::cmp::Ord> IsntBTreeSet2Ext<T> for std::collections::BTreeSet<T> {
    #[inline]
    fn is_not_disjoint(&self, other: &std::collections::BTreeSet<T>) -> bool {
        !self.is_disjoint(other)
    }

    #[inline]
    fn is_not_subset(&self, other: &std::collections::BTreeSet<T>) -> bool {
        !self.is_subset(other)
    }

    #[inline]
    fn is_not_superset(&self, other: &std::collections::BTreeSet<T>) -> bool {
        !self.is_superset(other)
    }

    #[inline]
    fn not_contains<Q>(&self, value: &Q) -> bool where Q: std::cmp::Ord + ?Sized, T: std::borrow::Borrow<Q>, {
        !self.contains::<Q>(value)
    }

    #[inline]
    fn not_insert(&mut self, value: T) -> bool {
        !self.insert(value)
    }

    #[inline]
    fn not_remove<Q>(&mut self, value: &Q) -> bool where Q: std::cmp::Ord + ?Sized, T: std::borrow::Borrow<Q>, {
        !self.remove::<Q>(value)
    }
}

mod binary_heap_private { pub trait Sealed<T> { } }

/// Extension for [`BinaryHeap`](std::collections::BinaryHeap)
pub trait IsntBinaryHeapExt<T>: binary_heap_private::Sealed<T> {
    /// The negation of [`is_empty`](std::collections::BinaryHeap::is_empty)
    #[must_use]
    fn is_not_empty(&self) -> bool;
}

impl<T> binary_heap_private::Sealed<T> for std::collections::BinaryHeap<T> { }

impl<T> IsntBinaryHeapExt<T> for std::collections::BinaryHeap<T> {
    #[inline]
    fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }
}

mod hash_map_private { pub trait Sealed<K, V, S> { } }

/// Extension for [`HashMap`](std::collections::HashMap)
pub trait IsntHashMapExt<K, V, S>: hash_map_private::Sealed<K, V, S> {
    /// The negation of [`is_empty`](std::collections::HashMap::is_empty)
    #[must_use]
    fn is_not_empty(&self) -> bool;
}

impl<K, V, S> hash_map_private::Sealed<K, V, S> for std::collections::HashMap<K, V, S> { }

impl<K, V, S> IsntHashMapExt<K, V, S> for std::collections::HashMap<K, V, S> {
    #[inline]
    fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }
}

mod hash_map2_private { pub trait Sealed<K, V, S> { } }

/// Extension for [`HashMap`](std::collections::HashMap)
pub trait IsntHashMap2Ext<K, V, S>: hash_map2_private::Sealed<K, V, S> {
    /// The negation of [`contains_key`](std::collections::HashMap::contains_key)
    #[must_use]
    fn not_contains_key<Q: ?Sized>(&self, k: &Q) -> bool where K: std::borrow::Borrow<Q>, Q: std::hash::Hash + std::cmp::Eq,;
}

impl<K, V, S> hash_map2_private::Sealed<K, V, S> for std::collections::HashMap<K, V, S> where K: std::cmp::Eq + std::hash::Hash, S: std::hash::BuildHasher, { }

impl<K, V, S> IsntHashMap2Ext<K, V, S> for std::collections::HashMap<K, V, S> where K: std::cmp::Eq + std::hash::Hash, S: std::hash::BuildHasher, {
    #[inline]
    fn not_contains_key<Q: ?Sized>(&self, k: &Q) -> bool where K: std::borrow::Borrow<Q>, Q: std::hash::Hash + std::cmp::Eq, {
        !self.contains_key::<Q>(k)
    }
}

mod hash_set_private { pub trait Sealed<T, S> { } }

/// Extension for [`HashSet`](std::collections::HashSet)
pub trait IsntHashSetExt<T, S>: hash_set_private::Sealed<T, S> {
    /// The negation of [`is_empty`](std::collections::HashSet::is_empty)
    #[must_use]
    fn is_not_empty(&self) -> bool;
}

impl<T, S> hash_set_private::Sealed<T, S> for std::collections::HashSet<T, S> { }

impl<T, S> IsntHashSetExt<T, S> for std::collections::HashSet<T, S> {
    #[inline]
    fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }
}

mod hash_set2_private { pub trait Sealed<T, S> { } }

/// Extension for [`HashSet`](std::collections::HashSet)
pub trait IsntHashSet2Ext<T, S>: hash_set2_private::Sealed<T, S> {
    /// The negation of [`is_disjoint`](std::collections::HashSet::is_disjoint)
    #[must_use]
    fn is_not_disjoint(&self, other: &std::collections::HashSet<T, S>) -> bool;
    /// The negation of [`is_subset`](std::collections::HashSet::is_subset)
    #[must_use]
    fn is_not_subset(&self, other: &std::collections::HashSet<T, S>) -> bool;
    /// The negation of [`is_superset`](std::collections::HashSet::is_superset)
    #[must_use]
    fn is_not_superset(&self, other: &std::collections::HashSet<T, S>) -> bool;
    /// The negation of [`contains`](std::collections::HashSet::contains)
    #[must_use]
    fn not_contains<Q: ?Sized>(&self, value: &Q) -> bool where T: std::borrow::Borrow<Q>, Q: std::hash::Hash + std::cmp::Eq,;
    /// The negation of [`insert`](std::collections::HashSet::insert)
    #[must_use]
    fn not_insert(&mut self, value: T) -> bool;
    /// The negation of [`remove`](std::collections::HashSet::remove)
    #[must_use]
    fn not_remove<Q: ?Sized>(&mut self, value: &Q) -> bool where T: std::borrow::Borrow<Q>, Q: std::hash::Hash + std::cmp::Eq,;
}

impl<T, S> hash_set2_private::Sealed<T, S> for std::collections::HashSet<T, S> where T: std::cmp::Eq + std::hash::Hash, S: std::hash::BuildHasher { }

impl<T, S> IsntHashSet2Ext<T, S> for std::collections::HashSet<T, S> where T: std::cmp::Eq + std::hash::Hash, S: std::hash::BuildHasher {
    #[inline]
    fn is_not_disjoint(&self, other: &std::collections::HashSet<T, S>) -> bool {
        !self.is_disjoint(other)
    }

    #[inline]
    fn is_not_subset(&self, other: &std::collections::HashSet<T, S>) -> bool {
        !self.is_subset(other)
    }

    #[inline]
    fn is_not_superset(&self, other: &std::collections::HashSet<T, S>) -> bool {
        !self.is_superset(other)
    }

    #[inline]
    fn not_contains<Q: ?Sized>(&self, value: &Q) -> bool where T: std::borrow::Borrow<Q>, Q: std::hash::Hash + std::cmp::Eq, {
        !self.contains::<Q>(value)
    }

    #[inline]
    fn not_insert(&mut self, value: T) -> bool {
        !self.insert(value)
    }

    #[inline]
    fn not_remove<Q: ?Sized>(&mut self, value: &Q) -> bool where T: std::borrow::Borrow<Q>, Q: std::hash::Hash + std::cmp::Eq, {
        !self.remove::<Q>(value)
    }
}

mod linked_list_private { pub trait Sealed<T> { } }

/// Extension for [`LinkedList`](std::collections::LinkedList)
pub trait IsntLinkedListExt<T>: linked_list_private::Sealed<T> {
    /// The negation of [`is_empty`](std::collections::LinkedList::is_empty)
    #[must_use]
    fn is_not_empty(&self) -> bool;
    /// The negation of [`contains`](std::collections::LinkedList::contains)
    #[must_use]
    fn not_contains(&self, x: &T) -> bool where T: std::cmp::PartialEq<T>,;
}

impl<T> linked_list_private::Sealed<T> for std::collections::LinkedList<T> { }

impl<T> IsntLinkedListExt<T> for std::collections::LinkedList<T> {
    #[inline]
    fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }

    #[inline]
    fn not_contains(&self, x: &T) -> bool where T: std::cmp::PartialEq<T>, {
        !self.contains(x)
    }
}

mod vec_deque_private { pub trait Sealed<T> { } }

/// Extension for [`VecDeque`](std::collections::VecDeque)
pub trait IsntVecDequeExt<T>: vec_deque_private::Sealed<T> {
    /// The negation of [`is_empty`](std::collections::VecDeque::is_empty)
    #[must_use]
    fn is_not_empty(&self) -> bool;
    /// The negation of [`contains`](std::collections::VecDeque::contains)
    #[must_use]
    fn not_contains(&self, x: &T) -> bool where T: std::cmp::PartialEq<T>,;
}

impl<T> vec_deque_private::Sealed<T> for std::collections::VecDeque<T> { }

impl<T> IsntVecDequeExt<T> for std::collections::VecDeque<T> {
    #[inline]
    fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }

    #[inline]
    fn not_contains(&self, x: &T) -> bool where T: std::cmp::PartialEq<T>, {
        !self.contains(x)
    }
}

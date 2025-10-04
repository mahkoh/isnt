// This file was generated

mod b_tree_map_private { pub trait Sealed<K, V> { } }

/// Extension for [`BTreeMap`](std::collections::BTreeMap)
pub trait IsntBTreeMapExt<K, V>: b_tree_map_private::Sealed<K, V> {
    /// The negation of [`is_empty`](std::collections::BTreeMap::is_empty)
    #[must_use]
    fn is_not_empty(&self) -> bool;
    /// The negation of [`contains_key`](std::collections::BTreeMap::contains_key)
    #[must_use]
    fn not_contains_key<Q>(&self, key: &Q) -> bool where K: std::borrow::Borrow<Q> + Ord, Q: Ord + ?Sized,;
}

impl<K, V> b_tree_map_private::Sealed<K, V> for std::collections::BTreeMap<K, V> { }

impl<K, V> IsntBTreeMapExt<K, V> for std::collections::BTreeMap<K, V> {
    #[inline]
    fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }

    #[inline]
    fn not_contains_key<Q>(&self, key: &Q) -> bool where K: std::borrow::Borrow<Q> + Ord, Q: Ord + ?Sized, {
        !self.contains_key::<Q>(key)
    }
}

mod b_tree_set_private { pub trait Sealed<T> { } }

/// Extension for [`BTreeSet`](std::collections::BTreeSet)
pub trait IsntBTreeSetExt<T>: b_tree_set_private::Sealed<T> {
    /// The negation of [`contains`](std::collections::BTreeSet::contains)
    #[must_use]
    fn not_contains<Q>(&self, value: &Q) -> bool where T: std::borrow::Borrow<Q> + Ord, Q: Ord + ?Sized;
    /// The negation of [`is_disjoint`](std::collections::BTreeSet::is_disjoint)
    #[must_use]
    fn is_not_disjoint(&self, other: &std::collections::BTreeSet<T>) -> bool where T: Ord;
    /// The negation of [`is_subset`](std::collections::BTreeSet::is_subset)
    #[must_use]
    fn is_not_subset(&self, other: &std::collections::BTreeSet<T>) -> bool where T: Ord;
    /// The negation of [`is_superset`](std::collections::BTreeSet::is_superset)
    #[must_use]
    fn is_not_superset(&self, other: &std::collections::BTreeSet<T>) -> bool where T: Ord;
    /// The negation of [`insert`](std::collections::BTreeSet::insert)
    #[must_use]
    fn not_insert(&mut self, value: T) -> bool where T: Ord;
    /// The negation of [`remove`](std::collections::BTreeSet::remove)
    #[must_use]
    fn not_remove<Q>(&mut self, value: &Q) -> bool where T: std::borrow::Borrow<Q> + Ord, Q: Ord + ?Sized;
    /// The negation of [`is_empty`](std::collections::BTreeSet::is_empty)
    #[must_use]
    fn is_not_empty(&self) -> bool;
}

impl<T> b_tree_set_private::Sealed<T> for std::collections::BTreeSet<T> { }

impl<T> IsntBTreeSetExt<T> for std::collections::BTreeSet<T> {
    #[inline]
    fn not_contains<Q>(&self, value: &Q) -> bool where T: std::borrow::Borrow<Q> + Ord, Q: Ord + ?Sized {
        !self.contains::<Q>(value)
    }

    #[inline]
    fn is_not_disjoint(&self, other: &std::collections::BTreeSet<T>) -> bool where T: Ord {
        !self.is_disjoint(other)
    }

    #[inline]
    fn is_not_subset(&self, other: &std::collections::BTreeSet<T>) -> bool where T: Ord {
        !self.is_subset(other)
    }

    #[inline]
    fn is_not_superset(&self, other: &std::collections::BTreeSet<T>) -> bool where T: Ord {
        !self.is_superset(other)
    }

    #[inline]
    fn not_insert(&mut self, value: T) -> bool where T: Ord {
        !self.insert(value)
    }

    #[inline]
    fn not_remove<Q>(&mut self, value: &Q) -> bool where T: std::borrow::Borrow<Q> + Ord, Q: Ord + ?Sized {
        !self.remove::<Q>(value)
    }

    #[inline]
    fn is_not_empty(&self) -> bool {
        !self.is_empty()
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
    /// The negation of [`contains_key`](std::collections::HashMap::contains_key)
    #[must_use]
    fn not_contains_key<Q>(&self, k: &Q) -> bool where K: std::borrow::Borrow<Q> + std::hash::Hash + Eq, Q: std::hash::Hash + Eq + ?Sized, S: std::hash::BuildHasher;
}

impl<K, V, S> hash_map_private::Sealed<K, V, S> for std::collections::HashMap<K, V, S> { }

impl<K, V, S> IsntHashMapExt<K, V, S> for std::collections::HashMap<K, V, S> {
    #[inline]
    fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }

    #[inline]
    fn not_contains_key<Q>(&self, k: &Q) -> bool where K: std::borrow::Borrow<Q> + std::hash::Hash + Eq, Q: std::hash::Hash + Eq + ?Sized, S: std::hash::BuildHasher {
        !self.contains_key::<Q>(k)
    }
}

mod hash_set_private { pub trait Sealed<T, S> { } }

/// Extension for [`HashSet`](std::collections::HashSet)
pub trait IsntHashSetExt<T, S>: hash_set_private::Sealed<T, S> {
    /// The negation of [`is_empty`](std::collections::HashSet::is_empty)
    #[must_use]
    fn is_not_empty(&self) -> bool;
    /// The negation of [`contains`](std::collections::HashSet::contains)
    #[must_use]
    fn not_contains<Q>(&self, value: &Q) -> bool where T: std::borrow::Borrow<Q> + std::hash::Hash + Eq, Q: std::hash::Hash + Eq + ?Sized, S: std::hash::BuildHasher;
    /// The negation of [`is_disjoint`](std::collections::HashSet::is_disjoint)
    #[must_use]
    fn is_not_disjoint(&self, other: &std::collections::HashSet<T, S>) -> bool where T: std::hash::Hash + Eq, S: std::hash::BuildHasher;
    /// The negation of [`is_subset`](std::collections::HashSet::is_subset)
    #[must_use]
    fn is_not_subset(&self, other: &std::collections::HashSet<T, S>) -> bool where T: std::hash::Hash + Eq, S: std::hash::BuildHasher;
    /// The negation of [`is_superset`](std::collections::HashSet::is_superset)
    #[must_use]
    fn is_not_superset(&self, other: &std::collections::HashSet<T, S>) -> bool where T: std::hash::Hash + Eq, S: std::hash::BuildHasher;
    /// The negation of [`insert`](std::collections::HashSet::insert)
    #[must_use]
    fn not_insert(&mut self, value: T) -> bool where T: std::hash::Hash + Eq, S: std::hash::BuildHasher;
    /// The negation of [`remove`](std::collections::HashSet::remove)
    #[must_use]
    fn not_remove<Q>(&mut self, value: &Q) -> bool where T: std::borrow::Borrow<Q> + std::hash::Hash + Eq, Q: std::hash::Hash + Eq + ?Sized, S: std::hash::BuildHasher;
}

impl<T, S> hash_set_private::Sealed<T, S> for std::collections::HashSet<T, S> { }

impl<T, S> IsntHashSetExt<T, S> for std::collections::HashSet<T, S> {
    #[inline]
    fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }

    #[inline]
    fn not_contains<Q>(&self, value: &Q) -> bool where T: std::borrow::Borrow<Q> + std::hash::Hash + Eq, Q: std::hash::Hash + Eq + ?Sized, S: std::hash::BuildHasher {
        !self.contains::<Q>(value)
    }

    #[inline]
    fn is_not_disjoint(&self, other: &std::collections::HashSet<T, S>) -> bool where T: std::hash::Hash + Eq, S: std::hash::BuildHasher {
        !self.is_disjoint(other)
    }

    #[inline]
    fn is_not_subset(&self, other: &std::collections::HashSet<T, S>) -> bool where T: std::hash::Hash + Eq, S: std::hash::BuildHasher {
        !self.is_subset(other)
    }

    #[inline]
    fn is_not_superset(&self, other: &std::collections::HashSet<T, S>) -> bool where T: std::hash::Hash + Eq, S: std::hash::BuildHasher {
        !self.is_superset(other)
    }

    #[inline]
    fn not_insert(&mut self, value: T) -> bool where T: std::hash::Hash + Eq, S: std::hash::BuildHasher {
        !self.insert(value)
    }

    #[inline]
    fn not_remove<Q>(&mut self, value: &Q) -> bool where T: std::borrow::Borrow<Q> + std::hash::Hash + Eq, Q: std::hash::Hash + Eq + ?Sized, S: std::hash::BuildHasher {
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
    fn not_contains(&self, x: &T) -> bool where T: PartialEq;
}

impl<T> linked_list_private::Sealed<T> for std::collections::LinkedList<T> { }

impl<T> IsntLinkedListExt<T> for std::collections::LinkedList<T> {
    #[inline]
    fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }

    #[inline]
    fn not_contains(&self, x: &T) -> bool where T: PartialEq {
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
    fn not_contains(&self, x: &T) -> bool where T: PartialEq<T>,;
}

impl<T> vec_deque_private::Sealed<T> for std::collections::VecDeque<T> { }

impl<T> IsntVecDequeExt<T> for std::collections::VecDeque<T> {
    #[inline]
    fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }

    #[inline]
    fn not_contains(&self, x: &T) -> bool where T: PartialEq<T>, {
        !self.contains(x)
    }
}

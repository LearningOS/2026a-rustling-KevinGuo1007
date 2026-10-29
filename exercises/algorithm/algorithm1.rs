/*
    single linked list merge
    This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node { val: t, next: None }
    }
}
#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
    pub fn merge(mut list_a: LinkedList<T>, mut list_b: LinkedList<T>) -> Self
    where
        T: Ord,
    {
        let mut merged = Self::new();
        while list_a.start.is_some() || list_b.start.is_some() {
            let source = match (list_a.start, list_b.start) {
                // SAFETY: Both lists own their live nodes, and no mutable
                // references to those nodes exist during this comparison.
                (Some(a), Some(b)) => unsafe {
                    if a.as_ref().val <= b.as_ref().val {
                        &mut list_a
                    } else {
                        &mut list_b
                    }
                },
                (Some(_), None) => &mut list_a,
                (None, Some(_)) => &mut list_b,
                (None, None) => unreachable!(),
            };
            let mut node = source.start.take().unwrap();
            // SAFETY: Taking the head transfers this node from source to
            // merged. Both lists are exclusively borrowed; each allocation
            // remains owned by exactly one list.
            unsafe {
                source.start = node.as_ref().next;
                node.as_mut().next = None;
                if let Some(mut tail) = merged.end {
                    tail.as_mut().next = Some(node);
                } else {
                    merged.start = Some(node);
                }
            }
            source.length -= 1;
            if source.start.is_none() {
                source.end = None;
            }
            merged.end = Some(node);
            merged.length += 1;
        }
        merged
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while let Some(node) = self.start {
            // SAFETY: Nodes originate from Box::into_raw and are uniquely
            // owned by this list. Advance before freeing each node once.
            unsafe {
                let node = Box::from_raw(node.as_ptr());
                self.start = node.next;
            }
        }
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![1, 3, 5, 7];
        let vec_b = vec![2, 4, 6, 8];
        let target_vec = vec![1, 2, 3, 4, 5, 6, 7, 8];

        for i in 0..vec_a.len() {
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len() {
            list_b.add(vec_b[i]);
        }
        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }
    #[test]
    fn test_merge_linked_list_2() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![11, 33, 44, 88, 89, 90, 100];
        let vec_b = vec![1, 22, 30, 45];
        let target_vec = vec![1, 11, 22, 30, 33, 44, 45, 88, 89, 90, 100];

        for i in 0..vec_a.len() {
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len() {
            list_b.add(vec_b[i]);
        }
        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }
}
#[cfg(test)]
mod edge_tests {
    use super::*;

    #[test]
    fn merge_empty_duplicates_and_release_nodes() {
        use std::cell::Cell;
        use std::rc::Rc;
        #[derive(Eq, PartialEq, Ord, PartialOrd)]
        struct Tracked(i32, Rc<Cell<usize>>);
        impl Drop for Tracked {
            fn drop(&mut self) {
                self.1.set(self.1.get() + 1);
            }
        }
        let drops = Rc::new(Cell::new(0));
        let mut a = LinkedList::new();
        let mut b = LinkedList::new();
        for value in [1, 2, 2] {
            a.add(Tracked(value, Rc::clone(&drops)));
        }
        for value in [2, 3] {
            b.add(Tracked(value, Rc::clone(&drops)));
        }
        let mut merged = LinkedList::merge(a, b);
        assert_eq!(merged.length, 5);
        for (i, value) in [1, 2, 2, 2, 3].iter().enumerate() {
            assert_eq!(merged.get(i as i32).unwrap().0, *value);
        }
        assert_eq!(drops.get(), 0);
        drop(merged);
        assert_eq!(drops.get(), 5);
        let empty = LinkedList::<i32>::merge(LinkedList::new(), LinkedList::new());
        assert_eq!(empty.length, 0);
        assert!(empty.start.is_none() && empty.end.is_none());
        let mut one = LinkedList::new();
        one.add(String::from("owned"));
        let mut merged = LinkedList::merge(LinkedList::new(), one);
        assert_eq!(merged.get(0).map(String::as_str), Some("owned"));
    }
}

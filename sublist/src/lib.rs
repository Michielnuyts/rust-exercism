use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    match first_list.len().cmp(&second_list.len()) {
        Ordering::Greater if is_superlist(first_list, second_list) => Comparison::Superlist,
        Ordering::Equal if first_list == second_list => Comparison::Equal,
        Ordering::Less if is_superlist(second_list, first_list) => Comparison::Sublist,
        _ => Comparison::Unequal,
    }
}

fn is_superlist<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    b.is_empty() || a.windows(b.len()).any(|x| x == b)
}

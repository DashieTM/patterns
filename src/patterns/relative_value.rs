// the difference to value object is that this does ordering, unlike value_object which just does
// equality. In jafuck you would need to implement the comparable interface -> Year implements
// Comparable<Year> -> then public int compareTo(Year other) ....
//

pub struct Value2 {
    pub val: i32,
}

impl Eq for Value2 {}

impl PartialEq for Value2 {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl PartialOrd for Value2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Value2 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.val > other.val {
            return std::cmp::Ordering::Greater;
        }
        if self.val < other.val {
            return std::cmp::Ordering::Less;
        }
        std::cmp::Ordering::Equal
    }
}

// jafuck
// public final class Year implements Comparable<Year> {
// @Override
// public boolean equals(Object o) {
// // Bridge Method, override generic equals()
// if (o == null || getClass() != o.getClass()) return false;
// return equals((Year)o);
// // forward to typed method
// }
// public boolean equals(Year o) {
// // Override-Overload Method Pair
// if (this == o) return true;
// if (o == null) return false;
// return value == o.value;
// }
// @Override
// public int compareTo(Year o) {
// // Override-Overload Method Pair, Type Specific Overload
// if (o.value == value) return 0;
// return (value < o.value) ? -1 : 1;
// }
// }

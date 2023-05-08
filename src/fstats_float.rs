use num_traits::{Float, AsPrimitive};
use std::ops::{Bound, RangeBounds};
use std::default::Default;
use std::ops::{AddAssign, SubAssign};

#[derive(Default, Debug)]
pub struct Stats<T: Float + Default + AddAssign + SubAssign + AsPrimitive<T>> {
    pub data: Vec<T>,
    length: usize,
    sum: T,
    sum_of_squares: T,
    max: Option<T>,
    min: Option<T>,
}

impl<T: Float + Default + AddAssign + SubAssign + AsPrimitive<T>> Stats<T> 
where usize: AsPrimitive<T>
{
    pub fn new() -> Self {
        Default::default()
    }

    fn add_cache(&mut self, x: T) {
        self.sum += x;
        self.sum_of_squares += x * x;
        self.length += 1;

        if self.max.is_none() || self.max < Some(x) {
            self.max = Some(x)
        };
        if self.min.is_none() || self.min > Some(x) {
            self.min = Some(x)
        };
    }

    fn del_cache(&mut self, x: T) {
        self.sum -= x;
        self.sum_of_squares -= x * x;
        self.length -= 1;
        
        if self.length == 0 {
            self.max = None;
            self.min = None;
        } else if self.length == 1 {
            self.max = Some(self.data[0]);
            self.min = Some(self.data[0]);
        } else if self.max == Some(x) || self.min == Some(x) {
            let i = self.length - 1;
            self.max = Some(self.data[i]);
            self.min = Some(self.data[i]);
            for j in (0..i).rev() {
                if self.max < Some(self.data[j]) {
                    self.max = Some(self.data[j])
                };
                if self.min > Some(self.data[j]) {
                    self.min = Some(self.data[j])
                };
            }
        }
    }

    pub fn reset(&mut self) {
        self.data = vec![];
        self.length = 0;
        self.sum = 0.as_();
        self.sum_of_squares = 0.as_();
        self.max = None;
        self.min = None;
    }

    pub fn mean(&mut self) -> Option<T> {
        if self.length > 0 {
            Some(self.sum / self.length.as_())
        } else {
            None
        }
    }

    pub fn stddev(&mut self) -> Option<T> {
        if self.length > 0 {
            Some(T::sqrt(T::max(0.as_(), 
                self.length.as_() * self.sum_of_squares - self.sum * self.sum) / (self.length.as_() * self.length.as_())))
        } else {
            None
        }
    }

    pub fn min(&self) -> Option<T> {
        if self.length > 0 {
            self.min
        } else {
            None
        }
    }

    pub fn max(&self) -> Option<T> {
        if self.length > 0 {
            self.max
        } else {
            None
        }
    }

    pub fn append(&mut self, other: &mut [T]) {
        other.iter().for_each(|x| self.push(*x));
    }

    pub fn count_in_range<R>(&mut self, range: &R) -> [usize; 2] 
    where 
        R: RangeBounds<usize>,
    {
        use Bound::*;
        let start = range.start_bound();
        let end = range.end_bound();
        let start = match start {
            Unbounded => 0,
            Included(s) => *s,
            Excluded(s) => s.saturating_add(1),
        };
        let end = match end {
            Unbounded => self.data.len() - 1,
            Included(&e) => e,
            Excluded(e) => e.saturating_sub(1),
        };
        [start, end - start + 1]    
    }

    pub fn data(&mut self) -> Vec<T> {
        self.data.to_vec()
    }

    pub fn drain<R>(&mut self, range: R) -> Vec<T>
    where
        R: RangeBounds<usize>,
    {
        let start = self.count_in_range(&range)[0];
        let count = self.count_in_range(&range)[1];
        let mut del = vec![];
        for _ in 0..count {
            let x = self.remove(start);
            del.push(x);
        }
        del
    }

    pub fn insert(&mut self, index: usize, element: T) {
        self.data.insert(index, element);
        self.add_cache(element);
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn pop(&mut self) -> Option<T> {
        let option = self.data.pop();
        if let Some(x) = option {
            self.del_cache(x);
        }
        option
    }

    pub fn push(&mut self, x: T) {
        self.data.push(x);
        self.add_cache(x);
    }

    pub fn push_vec(&mut self, v: Vec<T>) {
        v.iter().for_each(|x| self.push(*x));
    }

    pub fn remove(&mut self, index: usize) -> T {
        let x = self.data.remove(index);
        self.del_cache(x);
        x
    }

    pub fn resize(&mut self, new_len: usize, value: T) {
        let old_length = self.length;
        if new_len > old_length {
            for _ in 0..(new_len - old_length) {
                self.push(value);
            } 
        } else if new_len < old_length {
            for _ in new_len..old_length {
                self.pop();
            }
        }
    }

    pub fn splice<R>(&mut self, range: R, replace_with: Vec<T>) -> Vec<T> 
    where
        R: RangeBounds<usize>,
    {
        let mut start = self.count_in_range(&range)[0];
        let count = self.count_in_range(&range)[1];
        let mut del = vec![];
        for _ in 0..count {
            let x = self.remove(start);
            del.push(x);
        }
        for x in replace_with.iter() {
            self.insert(start, *x);
            start += 1;
        }
        del
    }

    pub fn split_off(&mut self, at: usize) -> Vec<T> {
        let length = self.length;
        let mut del = vec![];
        for _ in at..length {
            let x = self.remove(at);
            del.push(x);
        }
        del
    }

    pub fn swap_remove(&mut self, index: usize) -> T {
        let x = self.remove(index);
        let last_val = self.data[self.length-1];
        self.insert(index, last_val);
        self.pop();
        x
    }

    pub fn trim(&mut self, len: usize) {
        let length = self.length;
        if len < length {
            for _ in 0..len {
                self.remove(0);
            }
        }
    }

    pub fn truncate(&mut self, len: usize) {
        let length = self.length;
        if len < length {
            for _ in len..length {
                self.remove(len);
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use float_cmp::approx_eq;
    use super::Stats;
    use crate::stats::{mean, stddev, min, max};

    #[test]
    fn reset_test() {
        let mut vfs = Stats::new();
        vfs.push_vec(vec![1.0, 2.0, 3.0]);
        vfs.reset();
        
        assert_eq!(vfs.data(), vec![])
    }

    #[test]
    fn mean_test() {
        let vs = vec![1.0, 2.0];
        let mean_s = mean(&vs);
        
        let mut vfs = Stats::new();
        vfs.push(1.0);
        vfs.push(2.0);
        let mean_fs  = vfs.mean();
    
        assert_eq!(mean_s, mean_fs);
    }

    #[test]
    fn stddev_test() {
        let vs = vec![1.0, 2.0];
        let stddev_s = stddev(&vs);
        
        let mut vfs = Stats::new();
        vfs.push(1.0);
        vfs.push(2.0);
        let stddev_fs  = vfs.stddev();
    
        assert_eq!(stddev_s, stddev_fs);
    }

    #[test]
    fn min_max_test() {
        let vs = vec![1.0, 2.0, 3.0];
        let min_s = min(&vs);
        let max_s = max(&vs);
        
        let mut vfs = Stats::new();
        vfs.push_vec(vec![1.0, 2.0, 3.0]);
        let min_fs  = vfs.min();
        let max_fs = vfs.max();
    
        assert_eq!(min_s, min_fs);
        assert_eq!(max_s, max_fs);
    }

    #[test]
    fn append_test() {
        let mut vs = vec![1.0, 2.0, 3.0];
        let new_s = &mut vec![4.0, 3.0, 2.0];
        vs.append(new_s);
        
        let mut vfs = Stats::new();
        vfs.push_vec(vec![1.0, 2.0, 3.0]);
        let new_fs = &mut vec![4.0, 3.0, 2.0];
        vfs.append(new_fs);
    
        assert_eq!(vs, vfs.data());
        assert_eq!(mean(&vs), vfs.mean());
        assert_eq!(stddev(&vs), vfs.stddev());
        assert_eq!(min(&vs), vfs.min());
        assert_eq!(max(&vs), vfs.max());
    }

    #[test]
    fn drain_test() {
        let mut vs = vec![1.0, 2.0, 3.0, 4.0, 1.0, 4.0];
        vs.drain(3..4);
        
        let mut vfs = Stats::new();
        let vf = vec![1.0, 2.0, 3.0, 4.0, 1.0, 4.0];
        vfs.push_vec(vf);
        vfs.drain(3..4);

        assert_eq!(vs, vfs.data);
        assert_eq!(mean(&vs), vfs.mean());
        assert!( approx_eq!(f64, stddev(&vs).unwrap(), vfs.stddev().unwrap(), epsilon = 1e-15) );
        assert_eq!(min(&vs), vfs.min());
        assert_eq!(max(&vs), vfs.max());
    }

    #[test]
    fn insert_test() {
        let mut vs = vec![1.0, 2.0, 3.0];
        vs.insert(1, 4.0);
        
        let mut vfs = Stats::new();
        vfs.push_vec(vec![1.0, 2.0, 3.0]);
        vfs.insert(1, 4.0);

        assert_eq!(vs, vfs.data());
        assert_eq!(mean(&vs), vfs.mean());
        assert_eq!(stddev(&vs), vfs.stddev());
        assert_eq!(min(&vs), vfs.min());
        assert_eq!(max(&vs), vfs.max());
    }

    #[test]
    fn is_empty_test() {
        let vs: Vec<f64> = vec![];
        let bs = vs.is_empty();
        let vfs: Stats<f64> = Stats::new();
        let bfs = vfs.is_empty();

        assert_eq!(bs, bfs);
    }

    #[test]
    fn len_test() {
        let vs = vec![1.0, 2.0, 3.0];
        let len_s = vs.len();
        
        let mut vfs = Stats::new();
        vfs.push_vec(vec![1.0, 2.0, 3.0]);
        let len_fs = vfs.len();

        assert_eq!(len_s, len_fs)
    }

    #[test]
    fn remove_test() {
        let mut vs = vec![1.0, 2.0, 3.0];
        vs.remove(1);
        
        let mut vfs = Stats::new();
        vfs.push_vec(vec![1.0, 2.0, 3.0]);
        vfs.remove(1);

        assert_eq!(vs, vfs.data());
        assert_eq!(mean(&vs), vfs.mean());
        assert_eq!(stddev(&vs), vfs.stddev());
        assert_eq!(min(&vs), vfs.min());
        assert_eq!(max(&vs), vfs.max());
    }

    #[test]
    pub fn resize_test() {
        let mut vs = vec![1.0, 2.0, 3.0];
        vs.resize(8, 0.0);
        
        let mut vfs = Stats::new();
        vfs.push_vec(vec![1.0, 2.0, 3.0]);
        vfs.resize(8, 0.0);

        assert_eq!(vs, vfs.data); 

        vs.resize(2, 0.0);
        vfs.resize(2, 0.0);

        assert_eq!(vs, vfs.data());
        assert_eq!(mean(&vs), vfs.mean());
        assert_eq!(stddev(&vs), vfs.stddev());
        assert_eq!(min(&vs), vfs.min());
        assert_eq!(max(&vs), vfs.max());       
    }

    #[test]
    fn splice_test() {
        let mut vs = vec![1.0, 2.0, 3.0];
        let new_s = vec![4.0, 5.0, 6.0];
        vs.splice(1..3, new_s);
        
        let mut vfs = Stats::new();
        vfs.push_vec(vec![1.0, 2.0, 3.0]);
        let new_fs = vec![4.0, 5.0, 6.0];
        vfs.splice(1..3, new_fs);
    
        assert_eq!(vs, vfs.data());
        assert_eq!(mean(&vs), vfs.mean());
        assert_eq!(stddev(&vs), vfs.stddev());
        assert_eq!(min(&vs), vfs.min());
        assert_eq!(max(&vs), vfs.max());   
    }

    #[test]
    fn split_off_test() {
        let mut vs = vec![1.0, 2.0, 3.0];
        let del_s = vs.split_off(0);
        
        let mut vfs = Stats::new();
        vfs.push_vec(vec![1.0, 2.0, 3.0]);
        let del_fs = vfs.split_off(0);

        assert_eq!(del_s, del_fs);
        assert_eq!(mean(&vs), vfs.mean());
        assert_eq!(stddev(&vs), vfs.stddev());
        assert_eq!(min(&vs), vfs.min());
        assert_eq!(max(&vs), vfs.max());           
    }

    #[test]
    fn swap_remove_test() {
        let mut vs = vec![1.0, 2.0, 3.0];
        vs.swap_remove(0);
        
        let mut vfs = Stats::new();
        vfs.push_vec(vec![1.0, 2.0, 3.0]);
        vfs.swap_remove(0);

        assert_eq!(vs, vfs.data());
        assert_eq!(mean(&vs), vfs.mean());
        assert_eq!(stddev(&vs), vfs.stddev());
        assert_eq!(min(&vs), vfs.min());
        assert_eq!(max(&vs), vfs.max());  
    }

    #[test]
    fn truncate_test() {
        let mut vs = vec![1.0, 2.0, 3.0];
        vs.truncate(2);
        
        let mut vfs = Stats::new();
        let vf = vec![1.0, 2.0, 3.0];
        vfs.push_vec(vf);
        vfs.truncate(2);

        assert_eq!(vs, vfs.data());
        assert_eq!(mean(&vs), vfs.mean());
        assert_eq!(stddev(&vs), vfs.stddev());
        assert_eq!(min(&vs), vfs.min());
        assert_eq!(max(&vs), vfs.max());  
    }

}
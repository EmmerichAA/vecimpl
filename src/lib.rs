mod vec;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use vec::Vec;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn create_vec() {
        let vec: Vec<i32> = Vec::new();
        assert_eq!(vec.len(), 0);
        assert_eq!(vec.cap(), 0);
    }

    #[test]
    fn vec_push() {
        let mut vec: Vec<i32> = Vec::new();
        vec.push(32);

        assert_eq!(vec.len(), 1);
        assert_eq!(vec[0], 32);
    }

    #[test]
    fn vec_iter() {
        let mut vec: Vec<i32> = Vec::new();
        vec.push(32);
        vec.push(13);
        vec.push(15);

        assert_eq!(vec.len(), 3);
    }
}

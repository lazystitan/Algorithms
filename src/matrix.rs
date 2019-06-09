use std::fmt;

pub struct Matrix {
    length: usize,
    width: usize,
    matrix: Vec<Vec<i32>>
}

impl Matrix {
    pub fn new(length: usize, width: usize) -> Matrix {
        let mut mat = Matrix {
            length,
            width,
            matrix: Vec::new()
        };

        for i in 0..width {
            let mut temp = Vec::new();
            for j in 0..length{
                temp.push(0);
            }
            mat.matrix.push(temp);
        }

        mat
    }

    pub fn get(&self, i:usize, j: usize) -> Option<i32>
    {
        if let  Some(vec) = self.matrix.get(i) {
            if let Some(t) = vec.get(j)
            {
                return Some(t.clone());
            }
        }

        None
    }

    pub fn set(&mut self, i : usize, j :usize, value : i32) -> Result<i32,&'static str>
    {
        let remove;

        if let Some(vec) = self.matrix.get_mut(i) {
            if j >= vec.len() {
//                println!("set here! {}-{}",j,vec.len());
                return Err("j is out of index");
            }

            remove = vec.remove(j);

            vec.insert(j, value);
//            println!("vec:{:?}",vec);

        } else {
            return Err("i is out of index");
        }

        Ok(remove)
    }
}


impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in &self.matrix {
            for value in line {
                write!(f,"{:>2},",value);
            }
            println!();
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn grammar() {
        let mut vec = vec![1,2,3];
        let num = vec.remove(1);
        assert_eq!(vec, [1,3]);
        vec.insert(1, 3);
        assert_eq!(vec, [1,3,3]);
        assert_eq!(vec.len(),2);
    }

    #[test]
    fn mat_test()
    {
        let mut mat = Matrix::new(10,10);
        mat.set(8,9, 8);
        println!("{}", mat);
    }
}
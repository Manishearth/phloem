//! The Blob
//!
//! Stores `data`, `diff` and the dimensions of the Blob.
use Numeric;

#[derive(Debug)]
/// The Container for the `data` Vec, its `diff` and its `shape` dimensions
pub struct Blob<T: Numeric> {
    data: Vec<T>,
    diff: Vec<T>,
    shape: Vec<isize>,
}

impl <T> Blob<T> where T: Numeric {

    /// Creates a new Blob
    pub fn new() -> Blob<T> {
        Blob::of_shape(vec![0])
    }

    /// Creates a new Blob with specified `shape`
    pub fn of_shape(new_shape: Vec<isize>) -> Blob<T> {
        let mut blob = Blob {
            data: vec![],
            diff: vec![],
            shape: vec![0],
        };
        blob.reshape(new_shape);
        blob
    }

    /// Reshapes the Blob to the new `shape` given
    pub fn reshape(&mut self, new_shape: Vec<isize>) {
        let mut new_capacity = 1;

        for dimension in &new_shape {
            new_capacity *= *dimension;
        }
        self.shape = new_shape;
        if new_capacity > self.data.capacity() as isize {
            self.data = Vec::with_capacity(new_capacity as usize);
            self.diff = Vec::with_capacity(new_capacity as usize);
        }
    }

    /// Returns the shape of the Blob
    pub fn shape(&self) -> Vec<isize> {
        self.shape.clone()
    }

    /// Returns a String representation of the Blobs' `shape`
    ///
    /// The first numbers represent the size of the dimension.
    /// The last number in brackets defines the dimensionality of the Blob.
    pub fn shape_string(&self) -> String {
        let mut string: String = "".to_owned();
        for dim in self.shape.clone() {
            string.push_str(&format!("{} ", &dim.to_string()));
        }
        string.push_str(&format!("({})", self.shape.len().to_string()));
        string
    }

    /// Returns the length of the Blob data
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns a boolean value whether the Blobs' data is empty or not
    pub fn is_empty(&self) -> bool {
        self.data.len() == 0
    }

    /// Returns the allocated capacity of the Blob data.
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    /// Returns a pointer to the data of the Blob
    pub fn cpu_data(&self) -> &Vec<T> {
        &self.data
    }

    /// Returns a mutable pointer to the data of the Blob
    pub fn mutable_cpu_data(&mut self) -> &mut Vec<T> {
        &mut self.data
    }

    /// Returns a pointer to the diff of the Blob
    pub fn cpu_diff(&self) -> &Vec<T> {
        &self.diff
    }

    /// Returns a mutable pointer to the diff of the Blob
    pub fn mutable_cpu_diff(&mut self) -> &mut Vec<T> {
        &mut self.diff
    }
}

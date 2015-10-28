//! The Blob
//!
//! Stores `data`, `diff` and the dimensions of the Blob.

#[derive(Debug)]
/// The Container for the `data` Vec, its `diff` and its `shape` dimensions
pub struct Blob {
    data: Vec<isize>,
    diff: Vec<isize>,
    shape: Vec<isize>
}

impl Blob {

    /// Creates a new Blob
    pub fn new() -> Blob {
        Blob::of_shape(vec![0])
    }

    /// Creates a new Blob with specified `shape`
    pub fn of_shape(new_shape: Vec<isize>) -> Blob {
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

    /// Returns a pointer to the data of the Blob
    pub fn cpu_data(&self) -> &Vec<isize> {
        &self.data
    }

    /// Returns a mutable pointer to the data of the Blob
    pub fn mutable_cpu_data(&mut self) -> &mut Vec<isize> {
        &mut self.data
    }

    /// Returns a pointer to the diff of the Blob
    pub fn cpu_diff(&self) -> &Vec<isize> {
        &self.diff
    }

    /// Returns a mutable pointer to the diff of the Blob
    pub fn mutable_cpu_diff(&mut self) -> &mut Vec<isize> {
        &mut self.diff
    }
}

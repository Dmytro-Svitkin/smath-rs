#[derive(Debug,Clone,Copy)]
pub struct Mat<const ROW:usize,const COL:usize>{pub data:[[f32;ROW];COL]}

pub type Mat3=Mat<3,3>;
pub type Mat4=Mat<4,4>;

impl<const ROW:usize,const COL:usize>Mat<ROW,COL>{
    /// Creates matrix from array.
    #[inline]
    pub fn new(data:[[f32;ROW];COL])->Self{
        Self{data}
    }

    ///Creates matrix of zeros.
    #[inline]
    pub fn zero()->Self{
        Self{data:[[0.0;ROW];COL]}
    }

    /// Retrieves element at (row, col).
    #[inline]
    pub fn get(&self,row:usize,col:usize)->f32{
        debug_assert!(row<ROW&&col<COL);
        self.data[col][row]
    }

    /// Sets element at (row, col).
    #[inline]
    pub fn set(&mut self,row:usize,col:usize,x:f32){
        debug_assert!(row<ROW&&col<COL);
        self.data[col][row]=x
    }
}

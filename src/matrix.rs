use core::ops::{Add,AddAssign,Div,DivAssign,Mul,MulAssign,Neg,Sub,SubAssign};

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

    /// Sets element at the given index (row, col) to the given value.
    #[inline]
    pub fn set(&mut self,row:usize,col:usize,x:f32){
        debug_assert!(row<ROW&&col<COL);
        self.data[col][row]=x
    }

    /// Sets a column at the given index (row, col) to the given array.
    #[inline]
    pub fn set_column(&mut self,col:usize,arr:[f32;ROW]){
        debug_assert!(col<COL);
        self.data[col]=arr;
    }

    /// Sets a column at the given index (row, col) to the given array.
    #[inline]
    pub fn set_row(&mut self,row:usize,arr:[f32;COL]){
        debug_assert!(row<ROW);
        for x in 0..COL{
            self.data[x][row]=arr[x];
        }
    }

    /// Shifts a single element at the given index (row, col) by a given scalar value.
    #[inline]
    pub fn shift_cell(&mut self,row:usize,col:usize,val:f32){
        debug_assert!(row<ROW&&col<COL);
        self.data[col][row]+=val
    }

    /// Shifts all elements in the matrix by a given scalar value.
    #[inline]
    pub fn shift_all(&mut self,x:f32){
        for col in 0..COL{
            for row in 0..ROW{
                self.data[col][row]+=x;
            }
        }
    }

    /// Shifts a column at the given index (row, col) the given array.
    #[inline]
    pub fn shift_column(&mut self,col:usize,arr:[f32;ROW]){
        debug_assert!(col<COL);
        for row in 0..ROW{
            self.data[col][row]+=arr[row];
        }
    }

    /// Shifts a row at the given index (row, col) by the given array.
    #[inline]
    pub fn shift_row(&mut self,row:usize,arr:[f32;COL]){
        debug_assert!(row<ROW);
        for col in 0..COL{
            self.data[col][row]+=arr[col]
        }
    }

    /// Returns the transpose of the matrix.
    #[inline]
    pub fn transpose(&self)->Mat<COL,ROW>{
        let mut rcl:Mat<COL,ROW>=Mat::<COL,ROW>::zero();
        for col in 0..COL{for row in 0..ROW{rcl.data[row][col]=self.data[col][row]}}rcl
    }

}

impl<const N:usize>Mat<N,N>{
    /// Creates an identity matrix (1.0 on the diagonal, 0.0 everywhere else).
    #[inline]
    pub fn identity()->Self{
        let mut rcl:Mat<N,N>=Self::zero();
        for i in 0..N{rcl.data[i][i]=1.0;}rcl
    }

    /// Transposes the square matrix in place.
    #[inline]
    pub fn transposed(&mut self){
        for col in 0..N{
            for row in (col+1)..N{
                let rcl:f32=self.data[col][row];
                self.data[col][row]=self.data[row][col];
                self.data[row][col]=rcl;
            }
        }
    }
}

impl<const ROW:usize,const COL:usize>Add<&Mat<ROW,COL>>for &Mat<ROW,COL>{
    type Output=Mat<ROW,COL>;

    #[inline]
    fn add(self,rhs:&Mat<ROW,COL>)->Self::Output{
        let mut res:Mat<ROW,COL>=Mat::<ROW,COL>::zero();
        for col in 0..COL{for row in 0..ROW{res.data[col][row]=self.data[col][row]+rhs.data[col][row]}}res
    }
}

impl<const ROW:usize,const COL:usize>Sub<&Mat<ROW,COL>>for &Mat<ROW,COL>{
    type Output=Mat<ROW,COL>;

    #[inline]
    fn sub(self,rhs:&Mat<ROW,COL>)->Self::Output{
        let mut res:Mat<ROW,COL>=Mat::<ROW,COL>::zero();
        for col in 0..COL{for row in 0..ROW{res.data[col][row]=self.data[col][row]-rhs.data[col][row]}}res
    }
}

impl<const ROW:usize,const COL:usize>Mul<f32>for &Mat<ROW,COL>{
    type Output=Mat<ROW,COL>;

    #[inline]
    fn mul(self,rhs:f32)->Self::Output{
        let mut res:Mat<ROW,COL>=Mat::<ROW,COL>::zero();
        for col in 0..COL{
            for row in 0..ROW{res.data[col][row]=self.data[col][row]*rhs}}res
    }
}

impl<const ROW1:usize,const COL1:usize,const COL2:usize>Mul<&Mat<COL1,COL2>>for &Mat<ROW1,COL1>{
    type Output=Mat<ROW1,COL2>;

    #[inline]
    fn mul(self,rhs:&Mat<COL1,COL2>)->Self::Output{
        let mut res:Mat<ROW1,COL2>=Mat::<ROW1,COL2>::zero();
        for row in 0..ROW1{
            for col in 0..COL2{
                let mut sum:f32=0.0;
                for i in 0..COL1{sum+=self.data[i][row]*rhs.data[col][i];}res.data[col][row]=sum
            }
        }
        res
    }
}

impl<const ROW:usize,const COL:usize>Div<f32>for &Mat<ROW,COL>{
    type Output=Mat<ROW,COL>;

    #[inline]
    fn div(self,rhs:f32)->Self::Output{
        let mut res:Mat<ROW,COL>=Mat::<ROW,COL>::zero();
        for col in 0..COL{for row in 0..ROW{res.data[col][row]=self.data[col][row]/rhs}}res
    }
}

impl<const ROW:usize,const COL:usize>Neg for &Mat<ROW,COL>{
    type Output=Mat<ROW,COL>;

    #[inline]
    fn neg(self)->Self::Output{
        let mut res:Mat<ROW,COL>=Mat::<ROW,COL>::zero();
        for col in 0..COL{for row in 0..ROW{res.data[col][row]=-self.data[col][row]}}res
    }
}

impl<const ROW:usize,const COL:usize>AddAssign<&Mat<ROW,COL>>for Mat<ROW,COL>{
    #[inline]
    fn add_assign(&mut self,rhs:&Mat<ROW,COL>){
        for col in 0..COL{for row in 0..ROW{self.data[col][row]+=rhs.data[col][row]}}
    }
}

impl<const ROW:usize,const COL:usize>SubAssign<&Mat<ROW,COL>>for Mat<ROW,COL>{
    #[inline]
    fn sub_assign(&mut self,rhs:&Mat<ROW,COL>){
        for col in 0..COL{for row in 0..ROW{self.data[col][row]-=rhs.data[col][row]}}
    }
}

impl<const ROW:usize,const COL:usize>MulAssign<f32>for Mat<ROW,COL>{
    #[inline]
    fn mul_assign(&mut self,rhs:f32){
        for col in 0..COL{for row in 0..ROW{self.data[col][row]*=rhs}}
    }
}

impl<const ROW:usize,const COL:usize>DivAssign<f32>for Mat<ROW,COL>{
    #[inline]
    fn div_assign(&mut self,rhs:f32){
        for col in 0..COL{for row in 0..ROW{self.data[col][row]/=rhs}}
    }
}
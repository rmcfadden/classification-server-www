#[cfg(test)]
mod tests {
    use classification_server_www::math::matrix::Matrix;

    #[test]
    fn matrix_index_test(){
        let mut m1 = Matrix::<f64>::new(3,3);
        m1[0][0] = 1.1;
        m1[1][1] = 2.1;
        m1[2][2] = 235.1234;
        assert_eq!(1.1, m1[0][0]);
        assert_eq!(2.1, m1[1][1]);
        assert_eq!(235.1234, m1[2][2]); 
    } 


    #[test]
    fn matrix_add_number_test(){ 
        let mut  m1 = Matrix::<f64>::new(3,3);
        let mut m2 = Matrix::<f64>::new(3,3);
        m1[0][0] = 1.1;
        m1[1][1] = 1.1;
        m1[2][2] = 1.1;

        m2[0][0] = 1.1;
        m2[1][1] = 1.1;
        m2[2][2] = 1.1;

        let m3 = m1 + m2;

        assert_eq!(2.2, m3[0][0]);
        assert_eq!(2.2, m3[1][1]);
        assert_eq!(2.2, m3[2][2]); 
    }
}
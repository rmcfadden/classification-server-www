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
    fn matrix_add_matrixes_test(){ 
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

    #[test]
    fn matrix_mult_matrixes_simple_test(){ 
        let mut  m1 = Matrix::<i32>::new(2,2);
        let mut m2 = Matrix::<i32>::new(2,1);
        m1[0][0] = 1;
        m1[0][1] = 2;
        m1[1][0] = 3;
        m1[1][1] = 4;

        m2[0][0] = 2;
        m2[1][0] = 2;

        let m3 = m1 * m2;

        assert_eq!(6, m3[0][0]);
        assert_eq!(14, m3[1][0]);
    }

    #[test]
    fn matrix_mult_matrixes_simple2_test(){ 
        let mut  m1 = Matrix::<i32>::new(1,4);
        let mut m2 = Matrix::<i32>::new(4,1);
        m1[0][0] = 1;
        m1[0][1] = 2;
        m1[0][2] = 3;
        m1[0][3] = 4;

        m2[0][0] = 1;
        m2[1][0] = 2;
        m2[2][0] = 3;
        m2[3][0] = 4;

        let m3 = m1 * m2;

        assert_eq!(30, m3[0][0]);

    }


    #[test]
    fn matrix_mult_matrixes_medium_test(){ 
        let mut  m1 = Matrix::<i32>::new(3,3);
        let mut m2 = Matrix::<i32>::new(3,2);
        m1[0][0] = 1;
        m1[0][1] = 2;
        m1[0][2] = 3;
        m1[1][0] = 4;
        m1[1][1] = 5;
        m1[1][2] = 6;
        m1[2][0] = 7;
        m1[2][1] = 8;
        m1[2][2] = 9;
        
        m2[0][0] = 1;
        m2[0][1] = 2;
        m2[1][0] = 3;
        m2[1][1] = 4;
        m2[2][0] = 5;
        m2[2][1] = 6;


        let m3 = m1 * m2;

        assert_eq!(22, m3[0][0]);
        assert_eq!(28, m3[0][1]);
    }



}
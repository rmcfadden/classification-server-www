#[cfg(test)]
mod tests {
    use classification_server_www::math::matrix::Matrix;

    #[test]
    fn matrix_index_test(){
        let mut m1 = Matrix::<f64>::new(3,3);
        m1[0][0] = 1.1;
        m1[1][1] = 1.1;
        m1[2][2] = 1.1;
        println!("{}", m1);
    } 


    #[test]
    fn matrix_add_number_test(){ 
        let mut  m1 = Matrix::<f64>::new(3,3);
        let  m2 = Matrix::<f64>::new(3,3);
        m1[0][0] = 1.1;
        m1[1][1] = 1.1;
        m1[2][2] = 1.1;

        let m3 = m1 + m2;
        println!("{}", m3);

    }
}
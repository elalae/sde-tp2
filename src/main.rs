#[cfg(test)]
mod test{
    use crate::Ds;

    #[test]
    fn test_push(){
        let mut test= Ds::new();
    
        test.push(5);
        assert_eq! (test.vect, vec![5]);
        test.push(6);
        assert_eq! (test.vect, vec![5,6]);
        test.push(3);
        assert_eq! (test.vect, vec![3,5,6]);
        test.push(1);
        assert_eq! (test.vect, vec![1,3,5,6]);

        //assert_eq! (test.vect, vec![]);
    }

    #[test]
    fn test_remove_and_isprime(){


        let mut test= Ds::new();

        test.push(5);
        assert_eq! (test.vect, vec![5]);
        test.push(6);
        assert_eq! (test.vect, vec![5,6]);
        test.push(3);
        assert_eq! (test.vect, vec![3,5,6]);
        test.push(1);
        assert_eq! (test.vect, vec![1,3,5,6]);
         
        test.remove(6);
        assert_eq! (test.vect, vec![1,3,5]);

        test.remove(1);
        assert_eq! (test.vect, vec![3,5]);

        assert_eq!(test.is_prime(), vec![3,5]);


    }
   /*  #[test]
    fn test_is_prime(){

        let test= Ds::new();
        assert_eq!(test.is_prime(), vec![3,5]);
    } */
}
struct Ds{
   
    vect: Vec<u64>
    


}

impl Ds{
    fn new()->Ds{
        Ds{
            vect: Vec::new() // vec![]
        }
    }

    fn push(&mut self, a:u64){
       self.vect.push(a);
       self.vect.sort()
    }

    fn remove(&mut self, a:u64){
       
        // let index = self.vect.iter().position(|&x| x == a).unwrap();
        let mut index:i64=-1;
        for i in 0.. self.vect.len(){
            if self.vect[i]==a{
               index=i as i64;
            }
        }
        if index>-1{
            self.vect.remove(index as usize);
        }
        


    }

    fn display(&self){
        // for i in 0.. self.vect.len(){
        //     println!("{}",self.vect[i]);
        // }

        println!("{:?}", self.vect);
    }


    fn is_prime(&self)-> Vec<u64>{
        let mut result1 = Vec::new();

        for n in self.vect.iter(){
            if is_prime_num(*n)== true{
               result1.push(*n)
            }
        }

        return result1;
}
    fn get_slice(&self, a:u64, b:u64)->Vec<u64> {
        let mut result2 = Vec::new();
        let c= self.vect.iter().position(|&x| x == a).unwrap();
        let d= self.vect.iter().position(|&x| x == b).unwrap();
        
        for n in c..d+1{
            
               result2.push(self.vect[n])
            
        }

        
        return result2;
    }

}

fn main() {

    let mut test= Ds::new();
    
    test.push(5);
    test.push(6);
    test.push(3);
    test.push(1);
    test.push(2);
     
    test.display();
 
    test.remove(6);
    test.display();

    println!("The prime numbers are {:?}", test.is_prime());
 
    
   
    println!("The prime numbers are {:?}", test.get_slice(1, 3));

}

fn is_prime_num(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    for a in 2..n {
        if n % a == 0 {
            return false; // if it is not the last statement you need to use `return`
        }
    }
    true // last value to return
}
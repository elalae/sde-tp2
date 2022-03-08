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
    fn test_remove(){
        let mut test= Ds::new();
         
        test.remove(6);
        assert_eq! (test.vect, vec![1,3,5]);

        test.remove(1);
        assert_eq! (test.vect, vec![3,5]);


    }
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


}

fn main() {

    let mut test= Ds::new();
    
    test.push(5);
    test.push(6);
    test.push(3);
    test.push(1);
     
    test.display();

    test.remove(6);
    test.display();


}

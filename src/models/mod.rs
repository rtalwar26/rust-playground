pub struct Animal{
    species:String
}

pub fn dog()->Animal{

  let d  =  Animal{species: String::from("dog")};
  println!("{}", d.species );
  d

}
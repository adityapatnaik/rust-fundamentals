pub fn run(){
    let name = "Brad";
    let age = 23;
    //age = 24; not allowed
    let mut mut_age = 23;
    println!("{} {} {}",name,age,mut_age);
    mut_age = 24;
    println!("{} {} {}",name,age,mut_age);
    const ID: i32 = 001;
    println!("{}",ID);

    let (mname,mage) = ("adi",23);
    println!("{} {}",mname,mage);
    let (mut hname,mut hage) = ("adii",23);
    hname = "adityapatnaik";
    println!("{} {}",hname,hage);

}
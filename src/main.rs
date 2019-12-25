use std::io;

fn main() {
    println!("The Main Function Call");

    let mut student_name=String::new(); //Decalare a string variable
    println!("Please Enter your Name");
    io::stdin().read_line(&mut student_name).ok();
    let name= student_name.trim().parse().unwrap();

    let mut subject_one=String::new(); //Decalare a string variable
    println!("Please Enter your Subjects One Marks");
    io::stdin().read_line(&mut subject_one).ok();
    let subject_one:f32=subject_one.trim().parse().unwrap();
    
     let mut subject_two=String::new();
    println!("Please Enter your Subjects Two Marks");
    io::stdin().read_line(&mut subject_two).ok();
    let subject_two:f32=subject_two.trim().parse().unwrap();

    let result = student_result(name,subject_one,subject_two);
    if result>=70.0 {
        println!("{} is Pass ",student_name);
    }
    if result<70.0 {
        println!("{} is Fail ",student_name);
    }
     

     
}

fn student_result(name:String,subject_one:f32,subject_two:f32)->(f32){

    let total_marks:f32=200.0;
    let obtain_marks:f32 = subject_one + subject_two ;
    let percenetage:f32 = (obtain_marks/total_marks)*100 as f32;
    println!("{} have score {} percentage in two subjects",name,percenetage);
    percenetage
}
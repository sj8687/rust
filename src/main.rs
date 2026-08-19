// datatpes, mutable, string  and nums

// fn main() {
//     let mut  s:String = String::from("str");
//     s.push_str(" orld!");
//     println!("{}",s);

// }

// fn main() {
//     let mut  s:u8 = 12;
//     s=13;
//     println!("{}",s);

// }




//float type

// fn main() {
//     let float1:f32 = 3.14;
//     let float2 = 6.3; //type interface

//     println!("{}",float1);
//     print!("{}",float2);
// }




//charcter type like emoji or ASCII it took 4 bits

// let emoji  = '👽';



//type inferenec
// menas it will by default infer the types of value like
// let  x  = 5; it will be i32




//tuple

// fn main() {
//   let employee : (&str,u8) = ("sj", 22);

//   let (emp_name, emp_age) = employee;

//   println!("wtf {}, r u serious right now {}",emp_name,emp_age);

// }



//arrays   we can give it a fix length

// fn main() {
//     let arr : [u64;8] = [1,2,3,4,5,6,7,8];
//     println!("{}",arr.len())
// }


// or..

// let mut arr;
// arr = [1,2,3,4,5];
// println!("{}",arr[0]);


// ex its expensive

// fn main() {
//     let mut arr: [&str; 3] = ["hello","hii","sj"];
//     write_arr(arr);
//     println!("{:?}",arr); //as its is
// }

// fn write_arr(mut arr1:[&str; 3]){
//     arr1[0] = "fellow";
//     println!("{:?}",arr1) //only this print fellow because new copy memory created
// }


//or passing by ref

// fn main() {
//     let mut arr: [&str; 3] = ["hello","hii","sj"];
//     write_arr(&mut arr);
//     println!("{:?}",arr); //as its is
// }

// fn write_arr( arr2:&mut [&str; 3]){
//     arr2[0] = "fellow";
//     println!("{:?}",arr2) //now both print same beacuse of ref
// }




//vector it will be like dynamic array we also do mut and references


// fn main() {
//     let mut v:Vec<i32> = Vec::new();
//     // let mut v = Vec::<i32>::new();

//     v.push(1);
//         v.push(2);
//           v.pop();

//     println!("{:?}",v);
// }

// or

// fn main() {
//     let mut xs:Vec<i32> = vec![1,2,3];

//     print!("{}",xs.len());

//     xs.push(4);

//       print!("{}",xs.len());
// }



//if else we dont use bracts

// pub fn main(){
//   let a:u8 = 13;

//   if a==12 {
//       print!("wohhh");
//   }

//   else {
//       print!("ohh")
//   }
// }




//for loop 

// pub fn main() {
//     let str = String::from("shreyash");
//     println!("first name {}", get_first(str));

// }

// pub fn get_first(str:String) -> String {
//     let mut first_name = String::from("");
//     for c in str.chars() {
//         if c == ' ' {
//             break ;
//         }
//         first_name.push(c);
//     }

//     return  first_name;
// }





//match its similar to switch

// fn main(){
//     let number = 2;
    
//     match number {
//         1=>println!("no 1"),
//          2=>println!("no 2"),
//           3=>println!("no 3"),

//           _=>println!("oh")  //by default if not then not works
//     }
// }

//with  function

// fn main(){

//     fn even(num:i8) -> bool {
//         if(num%2==0){
//             return true;
//         }
//         return false;
//     }

//     let number = 2;
    
//     match number {
//          x if even(x)=>println!("even"),
//           _=>println!("oh")  //by default if not then not works
//     }
// }




//functions and return values

// fn main() {
//     let num:u8 = 10;
//     let num2:u8 = 12;
//     let result:u8 = add(num,num2);
//     println!("{}", result);
// }

// fn add(item:u8,item2:u8)->u8{
//    return  item + item2;
// }




// scope

//in rust its little bit weird when we declare global varible like we use static for itt now

// const Global:u64 = 100;
// fn main(){
//     let outside:u8 = 123;

//     {
//         // block scope
//         let inside:u8 = 34;
//         println!("{}",inside)
//     }

// //function scope 
//     println!("{}", outside);
//     global();
// }


// fn global(){
//     println!("{}",Global)
// }










//imp



// ownership or memory management who is the owner and and when it will be clear

//this is works because its static not dynamic like strings it will be store in stack (copy trait)
// fn main() {
//     let a: i32 = -5;
//     let b:i32 = a;
//     println!("{}",a);
//      println!("{}",a);
// }


//heap memmory beacuse of dynamic and ownership rules 
// fn main() {
//   let str1:String = String::from("awwchgh");
//   let str2 = str1; //new  owner now str1 get garbage 

//   println!("{}", str1);
//   println!("{}", str2);
// }



//ownership with functions

// this works fine 
// fn main() {
//     let x:u8 = 4;
//     proccess(x);
//     println!("{}",x)
// }

// fn proccess(x:u8){
//     println!("{}",x)
// }



//heap ownership works here it doesnt works

// fn main() {
//     let x:string = String::from("hii");
//     proccess(x);  //ownership  transfer
//     println!("{}",x)
// }

// fn proccess(x:String){
//     println!("{}",x)
// }



//how to deal with ownership we dont want (clone) we can also use tuple it only for read not for modify

// fn main(){
//   let s1:u8 = 12;
//   let s2:u8 = s1.clone(); //creating independent another clone its expensive task

//   println!("{}",s2)
// }


// fn main() {
//     let s1:String = String::from("heello");
//     let len = calculate_length(s1.clone());
//     println!("the lenght {}", s1.len());
// }

// fn calculate_length(s:String) -> usize {
//     let length:usize = s.len();
//     return  length;
// }

//or

// fn main() {
//     let s1:String = String::from("shreyash");
//     let ( len,s1) = get_length(s1);

//     print!("{}", len);

//     print!("{} ",s1)
// }

// fn get_length(s2:String) -> (usize,String) {
//     return(s2.len(),s2);
// }



//browwing (reference) more reliable thann this  it only for read not for modify

// fn main() {
//     let s1:String = String::from("heello");
//     let len = calculate_length(&s1); //borrow operation
//     println!("the lenght {}", s1.len());
// }

// fn calculate_length(s:&String) -> usize {
//     let length:usize = s.len();
//     return  length;
// }


// or

// fn main() {
//     let mut s1:String = String::from("sj");
//     let r1 = &s1;
//     let r2 = &s1;

//     println!("{}",r2)
// }


//refrence means we only give a address of value not the ownership like it have a metadata about that value


// fn main() {
//     let x = 5;
//     let y = &x;
//     println!("{}",y);  //auto dereferencing
// }



//mutablity with reference

// fn main() {
//     let mut x = 5;
//     x=x+1;
//     let y = &mut x;
//     *y = *y+1;
//     println!("{}",y)
// }

// or

// fn main() {
//     let mut s1:String = String::from("shreyash");

//     let s2: &mut String = &mut s1;   //mutable reference
//     s2.push_str("foo");

//     let s3: &String =  &s1; //immutable
//     println!("{}",s2);
//     let s4:&String = &s1;  //immutable

//     println!("{},{}",s3,s4)  //err

// }
















//shadowing we can do this things in shadowing only work with when we reasign not declare itt

// fn main() {
//     let x = 5;
//     let x = "sj";
//     let x = x.len();

//     println!("{}", x)
// }




//input output program

// use std::io;

// fn main() {
//     let mut input = String::new();
//     io::stdin().read_line(&mut input);
//     println!("{}",input)
// }




//structs

// struct Rect {
//     height: f64,
//     width:f64,
// }

// fn main() {
//     let r = Rect {
//         width: 10.0,
//         height: 10.0
//     };

//     println!("{},{}",r.height,r.width);
// }



// implementation in member function

// struct Rect {
//     height: f64,
//     width:f64,
// }

// impl Rect {                       //membership function
//     fn area(&self) -> f64 {
//         return self.width * self.height;
//     }

//     fn prints(a:u32) {
//         println!("static function")
//     }
// }

// fn main() {
//     let r = Rect {
//         width: 10.0,
//         height: 10.0
//     };

//     println!("{},{}",r.height,r.width);
//     print!("{}", r.area());
//     Rect::prints(10);
// }





// enum
// #[derive(PartialEq)]
// enum Direaction  {
//     North,
//     South,
//     East
// }


// fn main() {
//     let direction:Direaction = Direaction::North;

//     steer(direction);
// }

// fn steer(dir:Direaction){
//     if dir ==Direaction::North{
//         print!("wow")
//     }
// }


//we can also store an values in enum and member fun

// enum Shape {
//     Square(f32),
//     Circle(f32),
//     Rectangle(f32, f32)
// }

// fn main(){
//     let shape = Shape::Square(10.0);
//     let shape_circle = Shape::Circle(10.0);
//     let shape_rect = Shape::Rectangle(10.0,45.3);

//     print!(steer(shape))

// }

// fn steer(){

// }


//error handling

// use std::fs;

// enum Result {
//     Ok(String),
//     Err(String),
// }

// fn main() {
//     let contents = fs::read_to_string(path);

//     match contents {
//         Ok(contents)=> println!("{}", contents),
//         Err( e)=> println!("oww")
//     }
// }


// option num we have 2 option of res or return type in enum





//how to import pkg we use chrono library date and time lib  (cargo add chrono)

// use chrono::Utc;

// fn main() {
//     let utc = Utc::now();
//     println!("{}",utc);
// }



//how to read .env files

// use dotenv::dotenv;
// use std::env;

// fn main() {
//     dotenv().ok();
//     let var = env::var(key:"REDIS_ADDRESS").unwrap();
//     println!("{}", var)

//     match var{
//         Ok(str) => println!("{}", str),
//         Err(_e) => println!("wtf")
//     }
// }




//generics and traits we set one type of all kind of values

// use std::ops::Add;

// fn main() {
//     let res = sum(1, 2);
//     print!("{}",res)
// }

// fn sum<T : Add<Output=T>> (a:T, b:T) -> T {
//     return a+b;
// }

//for big numbers traaits

// fn bigg<T:Ord>(a:T,b:T) -> T {
//     if a>b{
//         return a;
//     }

//     return b;
// }

//for display traits

// use std::fmt::Display;

// fn bigg<T:Display>(a:T,b:T) {
//    println!("{},{}",a,b);
// }




//generics over structs
 
// #[derive(Clone, Copy)]

// struct Rect<T> {
//  width: T,
//  height: T,
// }


// impl<T: std::ops::Mul<Output = T> + Copy>  Rect<T> {
//     fn area(&self) -> T {
//         return  self.width * self.height;
//     }
// }

// fn main() {
//     let r = Rect {
//         width:12,
//         height:1
//     };

//      let r1 = Rect {
//         width:1.2,
//         height:1.1
//     };

//     println!("{}",r1.area());
//       println!("{}",r.area());
// }



// enum with generics

// enum Option<T> {
//     Some(T),
//     None
// }




// imp traits bounds implemetation (its just a shape of a final output or thing)

// trait Shape {  //just a signature of area fun like whoever imp this have a area fun
//     fn area(&self) -> f32;
// }

// struct Rect {
//     width:f32,
//     height:f32
// }


// impl Shape for Rect {
//     fn area(&self) -> f32 {
//         return self.height * self.width;
//     }
// }


// fn print<T:Shape>(s:T) {
//     println!("{}",s.area())
// }

// fn main(){
//     let r:Rect = Rect { 
//         width: 10.0,
//          height: 10.0
//     };

//     print(r);
// }






//macro is a code behind the rust like where we can write a another language for creting language like js 
//if you want to see it have to insatll (cargo install cargo-expand) do (cargo expand)

//declarative macros

// function take input of fun name
// macro_rules! generate_functions {
//     ($($func_name:ident),*) => {
//         $(
//             fn $func_name(){
//                 println!("hello {}", stringify!($func_name));
//             }
//         )*
//     };
// }

// generate_functions!(foo,bar,baz);

// fn main() {
//     foo();
//     bar();
//     baz();
// }








// procedral macro

// #[derive(serialize, Deserialize)]

// struct  Use{
//     nm: String,
//     age:f32
// }


//attribute macro

// #[route("GET")]
// fn Home() {
//     println!("hii");
// }

// #[route("POST")]
// fn create() {
//     println!("hii");
// }

//function like macro

// #[derive(Sql("users"))]  //do something like sql query

// struct  User{
//     nm: String,
//     age:f32
// }

// impl User {
//     fn insert(){}
//     fn delete(){}
// }

// u.insert()




//debug trait display trait

// #[derive(Debug)]
// struct User {
//     username: String,
//     password: String,
//     age: u32
// }

// fn main() {
//     let u = User {
//         username: String::from("sj"),
//         password:String::from("sj"),
//         age:22,
//     };

    // print!("{:?}",u);  //debug
 //   print!("{}",u)    //display
// } 
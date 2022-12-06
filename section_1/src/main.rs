#![deny(clippy::all)]

fn main() {
    // let x = 5;
    // println!("The value of x is {}", x);

    // let mut y = 6; //make mutable so that y can change "mut"
    // println!("The value of y is {}", y);
    // y = 7;
    // println!("The value of y is {}", y);

    // const SECONDS: i8 = 60; // constant names always in capital
    // println!("Constants always capital and not mutable SECONDS is {}", SECONDS);

    // // Scalar types: Integers, integers, literals, floating points, boolean and characters

    // // Integers: 8, 16, 32, 64 and 128 bit
    // let za: i8 = 10;
    // let zb: i8 = -10;
    // let zc: u8 = 10;
    // println!("i is for pos and neg values {}, {}", za, zb);
    // println!("u is for pos only values {}", zc);
    // // Lirerals: Hex, Dec, Oct....
    // let decimal = 2_55;
    // let hex = 0xff;
    // let octal = 0o377;
    // let binary = 0b1111_1111;
    // println!("decimal is {}, hex is {}, octal is {}, binary is {}", decimal, hex, octal, binary);
    // // print the ASCI value b(byte)
    // let bytea = b'A';
    // println!("The ASCI value of A is {}", bytea);
    // let bytex = b'X';
    // println!("The ASCI value of X is {}", bytex);

    // //floating points
    // let floata = 2.2; // 64 bit for better accuracy
    // println!("floata is {}", floata);

    // //boolean
    // let boola = true;
    // let boolb: bool = false;
    // println!( "The boolean is {}, {}", boola, boolb);

    // //Characters
    // let c='c'; //Rust knows by default what type the vlue will be
    // println!("Print c {}", c);

    //Arihthmatic: + - % /....
    // let a1 = 10;
    // let a2 = 4;
    // let remainder = a1 % a2;
    // println!("Remainder is {}", remainder);

    // Tuple: Grouping together a number of values, compound tuples have fixed lenght, cannot grow or shrink
    let tup = (500, "Hi Marius", true);
    println!("tuple is {}", tup.0);
    println!("tuple is {}", tup.1);
    println!("tuple is {}", tup.2);
    let (tup0, tup1, tup2)=tup;
    println!("tup0 is {}, tup1 is {}, tup2 is {}", tup0, tup1, tup2);

    // Arrays
    let array1 = [4, 5, 6];
    println!("array1_0 is {}", array1[0]);
    println!("array1_1 is {}", array1[1]);
    println!("array1_2 is {}", array1[2]);

    let mut array2: [i32; 3] = [1, 2, 3]; // array can be mutable to change values
    println!("array2_0 is {}", array2[0]);
    println!("array2_1 is {}", array2[1]);
    println!("array2_2 is {}", array2[2]);
    array2[0] = 10; // to change the vallue in mutable array 2
    println!("array2_0 is {}", array2[0]);

    // Vectors, are on the heap
    let mut vec1 = vec![1, 2, 3];
    println!("vec1 is {:?}", vec1);
    vec1.push(4); // Pushes 4 on the heap
    println!("vec1 push is {:?}", vec1);
    vec1.pop(); // Pop the last number on the heap
    println!("vec1 pop is {:?}", vec1);

    // Below is the struct that is called by the makro "vec!"
    let mut vec2 = Vec::new(); //Same as "vec!"
    vec2.push("Test");
    vec2.push("String");
    vec2.push("100");
    println!("vec2 method push is {:?}", vec2);
    vec2.pop();
    println!("vec2 pop 1 is {:?}", vec2);
    vec2.pop();
    println!("vec2 pop 2 is {:?}", vec2);
    vec2.push("100");
    println!("vec2 is {:?}", vec2);
    vec2.reverse();
    println!("vec reverse is {:?}", vec2);

    let mut vec3 = Vec::<i32>::with_capacity(2);
    vec3.push(100);
    vec3.push(200);
    vec3.push(300);
    vec3.push(400);
    println!("vec3 capacity is {}", vec3.capacity());
    println!("vec3 is {:?}", vec3);

    let vec4: Vec<i32> = (0..6).collect();
    println!("vec4 is {:?}", vec4);

    // Slices
    

}


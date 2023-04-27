use::std::thread;
use::std::time::Duration;



struct User<'a> {
    name: &'a str,
    email: &'a str,
    active: bool,
    sign_in_count: u64
}

fn create_user(email: &'static str, name: &'static str) -> User<'static> {
    User {
        name,
        email,
        active: false,
        sign_in_count: 0
    }
}

impl User<'static> {

    fn get_email(&self) -> &str{
        return self.email
    }

}
enum IpAddressKind {
    V4,
    V6
}
fn main() {


    let f = IpAddressKind::V6;







    let mut s = String::from("Hello world!");
    let lemg = first_word(&s);
    // s.clear();

    let user1 = User {
        name: "Nhy",
        email: "asdsad",
        active: true,
        sign_in_count: 3
    };

    let user2 = create_user("nhi", "asdasd");

    println!("{:?}", user1.name);


    println!("bytes {:?} {:?}", lemg, s); // bytes [72, 69, 76, 76, 79]

    println!("bytes {:?}", lemg);



    let vector = vec![1, 2, 3];


    let handle = thread::spawn(move || {
        println!("Here is the vector: {:?}",vector );
    });
    handle.join().unwrap();

    let some = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread,", i);
            thread::sleep(Duration::from_millis(1000));
        }
    });
    let result= some.join();
    match result {
        Ok(v) => println!("What df is v: {v:?}" ),
        Err(e) => println!("What df is e: {e:?}")
    }

    let builder = thread::Builder::new();

    let join_handle: thread::JoinHandle<_> = builder.spawn(|| {
        for i in 1..10 {
            println!("{i}  {i} {i} {i}");
            thread::sleep(Duration::from_secs(1));
        }
    }).unwrap();
    join_handle.join().expect("Couldn't join on the associated thread");

    for i in 1..10 {
        println!("hi number {} from the main thread", i);
        thread::sleep(Duration::from_millis(1000));
    }

    let x = 22;
    let y = x;


}

fn dangle() ->String {
    let s = String::from("asdasd");
    return s
}

fn get_length(s: &String) -> (usize) {
    s.len()
}

struct Person {
    name: String,
    address: String
}

impl Person {

    // fn get_name(&self) -> String {
    //     return self.name
    //     // what error ???
    // }

}

// fn mutate_string(s: &mut String)  {
//     return s.push_str(" World")
// }

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b'w' {
            return &s[..i];
        }
    }

    &s[..]
}

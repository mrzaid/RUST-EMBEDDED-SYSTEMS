fn main() {
    let mut name=String::from("women");
    let closure_fnOnce=|| name+" power";
    info(closure_fnOnce);
}
fn info<F:FnOnce()->String>(func:F){
    print!("{}",func());
}

/********************************************************QUESTION 2 */
fn main() {
    let mut name=String::from("Pakistan");
    let closure_fnOnce=|| name.push_str(" is a good country.");
    info(closure_fnOnce);
    println!("{}",name);
}
fn info<F:FnMut()>(mut func:F){
    func();
    func();
}
/****************************************Question 3 */
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("I , hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    let handle_1 = thread::spawn(|| {
        for j in 1..20 {
            println!("J ,hi number {} from the spawned thread!", j);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for k in 1..5 {
        println!("K, hi number {} from the main thread!", k);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
    handle_1.join().unwrap();
}
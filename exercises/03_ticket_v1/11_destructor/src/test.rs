fn main() {
    let y = "Hello".to_string();
    let x = "World".to_string();
    let h = "!".to_string();
    // Variables are dropped in reverse order of declaration
    drop(h);
    drop(x);
    drop(y);
    
 }
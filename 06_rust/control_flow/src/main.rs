fn main(){

    let mut i = 0;
    println!("Loop");
    
    'outer: loop {
        if i > 100 {
            break 'outer;
        }
        
        if i % 2 == 1 {
            i+=1;
            continue 'outer;    
        }
        
        println!("{}" , i);
        i+=1;
    }
    println!("");
    println!("");
    println!("for");
    
    for i in 0..=100 {
        if i % 2 == 1 {
            continue;    
        }
        println!("{}" , i);
    }
    println!("");
    println!("");
    println!("while");
    i = 0;
    while i <= 100 {
        if i % 2 == 1 {
            i+=1;
            continue;
        }
        
        println!("{}" , i);
        i+=1;
    }
}
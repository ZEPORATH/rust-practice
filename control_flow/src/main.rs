use ::function_name::named;

fn if_control_flow() {
    let condition = true;
    let number = if condition {5} else {6};

    println!("the value of number is: {number}");
}

#[named]
fn ret_loop(){
    let mut counter = 1;
    let res = loop {
        counter += 1;
        if counter == 10 {
            break counter+2;
        }
    };
    println!("[{}] the res is: {res}", function_name!());
}

#[named]
fn labeled_loops(){
    let mut count = 0;
    'counting_parent_loop: loop{
        println!("count = {count}");
        let mut remaining = 10;

        loop{
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_parent_loop;
            }
            remaining -= 1;
        }
        count += 1;

    }
    println!("[{}] end of the loop, count= {count}", function_name!());
}

#[named]
fn while_loop(){
    println!("[{}]", function_name!());
    let arr = [1,23,3,5,6];
    let mut idx = 0;
    while idx < arr.len() {
        println!("the current elem: {}", arr[idx]);
        idx+=1;
    }
}

#[named]
fn for_loop(arr:&[i32]){
    println!("[{}]", function_name!());
    for elem in arr{
        println!("the current elem: {elem}");
    }
}

fn main() {
    println!("Hello, world!");
    if_control_flow();
    ret_loop();
    labeled_loops();
    while_loop();
    let arr = [1,23,3,5,6];
    for_loop(&arr);
}

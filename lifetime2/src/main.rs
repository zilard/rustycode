fn main() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

// how does the borrowchecker know that result is not a dangling reference
// we just told the borrowchecker that
// whatever gets returned from 'longest' will have a lifetime equal to the smallest lifetime being
// passed in

    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);
}

// generic lifetime annotations dont actually change the lifetim 
// they just create a relationship between lifetimes of multiple references

// the lifetime of the returned reference will be the same as the smallest lifetime of the
// arguments

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

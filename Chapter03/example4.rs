struct Parent<'p> {
    age: u8,
    child: Option<&'p Child>,
}

struct Child {
    age: u8,
}

fn main() {
    let child = Child { age: 10 };
    let parent = Parent {
        age: 35,
        child: Some(&child),
    };

    println!("Child's age: {} years.", parent.child.unwrap().age);
}

fn oldest_child<'f>(child1: &'f Child, child2: &'f Child) -> &'f Child {
    if child1.age > child2.age {
        child1
    } else {
        child2
    }
}

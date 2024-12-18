pub mod basicdemo {
    pub fn main() {
        let x = 5;
        //x=6; //error: cannot assign twice to immutable variable `x`
        println!("The value of x is: {}", x);

        let mut y = 10;
        y = 15;
        println!("The value of y is: {}", y);

        let name = String::from("John");
        println!("My name is {}", name);
        func(&name);
        println!("{}", name);
        let mut name = String::from("Tom");
        func_mut(&mut name);
        println!("{}", name);

        if name.len() > 4 {
            println!("{} is longer than 4 characters", name);
        } else {
            println!("{} is less than 4 characters", name);
        }

        let tuple = (1, 2.5, true);
        let (a, b, c) = tuple;

        for i in 0..10 {
            println!("{}", i);
        }

        let list = vec![1, 2, 3];
        for i in &list {
            println!("{}", i);
        }
        println!("{:?}", list);

        let list2 = vec![1, 2, 3];
        for i in list2.iter() {
            println!("{}", i);
        }
        println!("xxxx{:?}", list2);

    }

    fn func(s: &String) {
        println!("{}", s);
    }

    fn func_mut(s: &mut String) {
        s.push_str(" is a name");
    }
}

pub mod traitdemo {
    pub trait Animal {
        fn say(&self) -> ();
    }
    
    pub struct Dog {
        name: String,
    }
    
    impl Animal for Dog {
        fn say(&self) -> () {
            println!("{} says: Woof!", self.name);
        }
    }
    
    pub struct Cat {
        name: String,
    }
    
    impl Animal for Cat {
        fn say(&self) -> () {
            println!("{} says: Mia!", self.name);
        }
    }

    pub fn main() {
        let animals: Vec<Box<dyn Animal>> = vec![
            Box::new(Dog{name: String::from("Tom")}),
            Box::new(Cat{name: String::from("Jerry")})
        ];
        for animal in animals {
            animal.say();
        }  
    }
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}


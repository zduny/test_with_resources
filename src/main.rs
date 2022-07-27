fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    struct Resources {
        pub some_resource: u32,
    }

    impl Drop for Resources {
        fn drop(&mut self) {
            // Put your cleanup code here
            println!("Cleaning up resources!")
        }
    }

    // This test is failing on purpose
    #[test]
    fn test_something() {
        let resources = Resources { some_resource: 42 };
        println!("I'm using a resource: {}!", resources.some_resource);

        panic!("Now I'm panicking!!!");

        // Resources drop should be called despite of panic.
    }
}

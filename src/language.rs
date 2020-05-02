use uuid::Uuid;

pub struct Command {
    compile: Option<String>,
    execute: String,
}

pub struct Mfunction {
    pub func: String,
}

/// Simple parser for modification function
/// # Example 
/// 2X* => X*2
/// 1000X+3* => 3*(X+1000)
impl Mfunction {
    fn parse(&self, base: i32) -> Result<i32, ()> {
        let mut stack: Vec<i32> = Vec::new();
        let mut is_number = false;
        let mut tmp: String = String::new();
        for c in self.func.chars() {
            match c {
                '*' => {
                    if is_number {
                        stack.push(tmp.parse::<i32>().unwrap());
                        tmp = String::new();
                        is_number = false;
                    }
                    if stack.len() < 2 {
                        return Err(());
                    } else {
                        let (a, b) = (stack.pop().unwrap(), stack.pop().unwrap());
                        stack.push(a*b);
                    }
                },
                '/' => {
                    if is_number {
                        stack.push(tmp.parse::<i32>().unwrap());
                        tmp = String::new();
                        is_number = false;
                    }
                    if stack.len() < 2 {
                        return Err(());
                    } else {
                        let (a, b) = (stack.pop().unwrap(), stack.pop().unwrap());
                        stack.push(a/b);
                    }
                },
                '+' => {
                    if is_number {
                        stack.push(tmp.parse::<i32>().unwrap());
                        tmp = String::new();
                        is_number = false;
                    }
                    if stack.len() < 2 {
                        return Err(());
                    } else {
                        let (a, b) = (stack.pop().unwrap(), stack.pop().unwrap());
                        stack.push(a+b);
                    }
                },
                '-' => {
                    if is_number {
                        stack.push(tmp.parse::<i32>().unwrap());
                        tmp = String::new();
                        is_number = false;
                    }
                    if stack.len() < 2 {
                        return Err(());
                    } else {
                        let (a, b) = (stack.pop().unwrap(), stack.pop().unwrap());
                        stack.push(a-b);
                    }
                },
                'X' => {
                    stack.push(base);
                },
                _ => {
                    is_number = true;
                    tmp.push(c);
                },
            }
        }
        if stack.is_empty() {
            return Err(());
        } else {
            return Ok(*stack.first().unwrap());
        }
    }
}

pub trait Limitation {
    fn new(func: &str) -> Self where Self: Sized;
    fn evaluate(&self, base: i32) -> Result<i32, ()> where Self: Sized;
}

impl Limitation for Mfunction {
    fn new(func: &str) -> Self {
        Self { func: func.to_string() }
    }

    fn evaluate(&self, base: i32) -> Result<i32, ()> {
        self.parse(base)
    }
}

pub struct Runtime {
    display_name: String,
    command: Command,
    version: String,
    // argument of time modification function must be (ms) (milliseconds)
    modification_time: Option<Box<dyn Limitation>>,
    // argument of memory modification function must be (KB) (killobytes)
    modification_memory: Option<Box<dyn Limitation>>,
}

pub struct Language {
    id: Uuid,
    name: String,
    version: String,
    runtime: Vec<Runtime>,
}
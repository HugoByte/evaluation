use derive_macro::Calculator;

trait Calc{
    fn execute(&self) -> i32 ;
}

#[derive(Debug)]
#[derive(Calculator)]
#[Operation = "addition"]
struct Task1 {
    op1 : i32,
    op2 : i32,
}

impl Calc for Task1{
    fn execute( &self) -> i32{
        println!("Addition of two numbers {},{} is {}", self.op1, self.op2, self.run());
        return self.run()
    }
}

#[derive(Calculator)]
#[Operation = "subtraction"]
struct Task2{
    op1 : i32,
    op2 : i32,
}

impl Calc for Task2{
    fn execute(&self) -> i32{
         println!("Subtracion of two numbers {},{} is {} ", self.op1, self.op2, self.run());
        return self.run()
    }
}

#[derive(Calculator)]
#[Operation = "multiplication"]
struct Task3{
    op1 : i32,
    op2 : i32,
}

impl Calc for Task3{
    fn execute(&self) -> i32{
        println!("Multiplication of two numbers {},{} is {}", self.op1, self.op2, self.run());
        return self.run()
    }
    }

#[derive(Calculator)]
#[Operation = "modulus"]
struct Task4{
    op1 : i32,
    op2 : i32,
}

impl Calc for Task4{
    fn execute(&self) -> i32{
        println!("Modulus of two numbers {},{} is {}", self.op1, self.op2, self.run());
        return self.run()
    }
    }

fn main() {

    let mut array: Vec<Box<dyn Calc>> = Vec::new();

    let add = Task1 {op1:2, op2:2};
    let sub = Task2 {op1:2, op2:2};
    let mul = Task3 {op1:3, op2:3};
    let modul = Task4 {op1:2, op2:2};

    array.push(Box::new(add));
    array.push(Box::new(sub));
    array.push(Box::new(mul));
    array.push(Box::new(modul));

    for i in array{
        i.execute();
    }

   
}

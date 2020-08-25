fn main() {
    // Statements here are executed when the compiled binary is called

    // Print text to the console
	for i in 1..20
	{
	if(i % 3 ==0 && i % 5==0)
	{
	println!("fizzbuzz");
	}
	else if (i % 5==0)
	{
	println!("buzz");
	}
	else if ( i % 3==0)
	{
	println!("fizz");
	}
	else{
	println!("{}", i);
}
	}
}
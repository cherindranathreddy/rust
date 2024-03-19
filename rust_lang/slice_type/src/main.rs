fn main() {
    println!("The Slice type");

    let mut sentence = String::new(); 
    std::io::stdin().read_line(&mut sentence);
    println!("The enetered sentence is: {}", sentence);

    let first_word = first_word(&mut sentence);
    println!("The first in this sentence: {sentence} is {first_word}");
}

fn first_word(s:&mut String) -> &str
{
    let words: Vec<&str> = s.split(" ");
    words[0]
}

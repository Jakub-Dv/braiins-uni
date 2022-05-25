struct TokenIterator<'a> {
    text: &'a str,
    delimiter: &'a str,
    end: bool,
}

impl<'a> TokenIterator<'a> {
    fn new(input: &'a str, delimiter: &'a str) -> TokenIterator<'a> {
        TokenIterator { text: input, delimiter, end: false }
    }
}

impl<'a> Iterator for TokenIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.end {
            None
        } else {
            match self.text.find(self.delimiter) {
                Some(index) => {
                    let ret_str = &self.text[0..index];
                    self.text = &self.text[index + 1..];
                    Some(ret_str)
                }
                None => {
                    let ret_str = self.text;
                    self.end = true;
                    Some(ret_str)
                }
            }
        }
    }
}

#[test]
fn it_works() {
    let test_input = "1 2 3 4 5";
    let tokens: Vec<_> = TokenIterator::new(test_input, " ").collect();

    assert_eq!(tokens, vec!["1", "2", "3", "4", "5"]);
}

#[test]
fn tail() {
    let test_input = "1 2 3 4 ";
    let tokens: Vec<_> = TokenIterator::new(test_input, " ").collect();

    assert_eq!(tokens, vec!["1", "2", "3", "4", ""]);
}

fn main() {}

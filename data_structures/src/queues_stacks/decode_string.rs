
pub fn decode(s: String) -> String{
    let mut d = String::from("");
    let mut main_stack: Vec<char> = s.chars().rev().collect();
    main_stack.reverse();
    let mut sub_stack: Vec<char> = vec![];
    const OPEN: &str = "[";
    const CLOSE: &str = "]";
    const RADIX: u32 = 10;


    while !main_stack.is_empty() {
        // || !sub_stack.is_empty()
        let mut do_loop = true;

        while do_loop && !main_stack.is_empty() {
            let curr = main_stack.pop().unwrap();
            sub_stack.push(curr);


            if main_stack.is_empty() || (curr.is_digit(RADIX) && !main_stack[main_stack.len()-1].is_digit(RADIX)) {
                do_loop = false;
            }
        }



        if !sub_stack.is_empty() && sub_stack.last().unwrap().is_digit(RADIX)  {
            let mut times: usize = 0;
            // let mut text = String::from("");
            let mut text_vec = vec![];
            let mut form_text = true;
    
            while form_text && !sub_stack.is_empty() {
                let t = sub_stack.pop().unwrap();

                
                text_vec.push(t);

                // let st = String::from(t);
                let mut st = String::from("");
                st.push(t);
                let sst = st.as_str();
                if sst == CLOSE {
                    form_text = false;
                }
            }
            
            // let d: usize = text_vec[0].to_string().parse::<usize>().unwrap();
            // times = d;


            let mut text_figure: String = String::from("");

            for index in 0..text_vec.len() {

                let d: i32 = text_vec[index].to_string().parse::<i32>().unwrap_or(-1);

                if d >= 0 {
                    let ds = d.to_string();
                    text_figure.push_str(ds.as_str());
                } else {
                    break;
                }
            }

            let dd: usize = text_figure.parse::<usize>().unwrap();
            times = dd;

            // start from two in text_vec to overlook integer and "[" values at index 0 and 1 respectively
            // use text_vec.len() - 2 because the full characters is text_vec.len() -1 but we do not neeed the last character i.e. "]"
            let the_tex_vec = &text_vec[text_figure.len() +1..text_vec.len()-1].to_owned();
            let text_all: Vec<&Vec<char>> = vec![the_tex_vec; times];
              

          let mut multiplied_text = text_all.into_iter().flatten().collect::<Vec<&char>>();

          while !multiplied_text.is_empty() {
              sub_stack.push(*multiplied_text.pop().unwrap());
          }
        }


    }

    while !sub_stack.is_empty() {
        let curr = sub_stack.pop().unwrap().to_string();

        d.push_str(curr.as_str());
    }

    return d;
}

use std::usize;

use super::test_wrapper::TestHandler;

#[cfg(test)]
mod test_decode_string_cont {
    use super::*;
    #[test]
    fn test_decode_string() {
        let all_tests = vec![
            TestHandler::new(String::from("abc3[cd]xyz"), String::from("abccdcdcdxyz")), 
            TestHandler::new(String::from("2[abc]3[cd]ef"), String::from("abcabccdcdcdef")), 
            TestHandler::new(String::from("3[a2[c]]"), String::from("accaccacc")),
            TestHandler::new(String::from("10[a]"), String::from("aaaaaaaaaa")),
            TestHandler::new(String::from("12[ab]"), String::from("abababababababababababab")),
            TestHandler::new(String::from("3[a]2[bc]"), String::from("aaabcbc"))
            ];
        let s = String::from("abc3[cd]xyz");

        for test in all_tests {
            assert_eq!(decode(test.input), test.expected);
        }

    }
}
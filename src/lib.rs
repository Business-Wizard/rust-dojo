#![allow(unused_variables, dead_code, unused_mut, unused_imports)]

use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;
    // println!("{r1}");
}

pub fn brackets_are_balanced(string: &str) -> bool {
    if string.is_empty() {
        return true;
    }

    let opening_brackets: HashSet<char> = HashSet::from(['{', '[', '(']);
    let closing_brackets: HashSet<char> = HashSet::from(['}', ']', ')']);
    let matching_bracket: HashMap<char, char> = HashMap::from([('}', '{'), (']', '['), (')', '(')]);

    let mut stack: Vec<char> = Vec::new();

    for character in string.chars() {
        let is_opening_bracket: bool = opening_brackets.contains(&character);
        let is_closing_bracket: bool = closing_brackets.contains(&character);

        if !is_opening_bracket && !is_closing_bracket {
            continue;
        }

        if is_opening_bracket {
            stack.push(character);
        } else if let Some(last_element) = stack.pop() {
            if last_element != matching_bracket[&character] {
                return false;
            }
        } else {
            return false;
        }
    }
    stack.is_empty()
}

#[test]
fn paired_square_brackets() {
    assert!(brackets_are_balanced("[]"));
}

#[test]
fn empty_string() {
    assert!(brackets_are_balanced(""));
}

#[test]
fn unpaired_brackets() {
    assert!(!brackets_are_balanced("[["));
}

#[test]
fn wrong_ordered_brackets() {
    assert!(!brackets_are_balanced("}{"));
}

#[test]
fn wrong_closing_bracket() {
    assert!(!brackets_are_balanced("{]"));
}

#[test]
fn paired_with_whitespace() {
    assert!(brackets_are_balanced("{ }"));
}

#[test]
fn partially_paired_brackets() {
    assert!(!brackets_are_balanced("{[])"));
}

#[test]
fn simple_nested_brackets() {
    assert!(brackets_are_balanced("{[]}"));
}

#[test]
fn several_paired_brackets() {
    assert!(brackets_are_balanced("{}[]"));
}

#[test]
fn paired_and_nested_brackets() {
    assert!(brackets_are_balanced("([{}({}[])])"));
}

#[test]
fn unopened_closing_brackets() {
    assert!(!brackets_are_balanced("{[)][]}"));
}

#[test]
fn unpaired_and_nested_brackets() {
    assert!(!brackets_are_balanced("([{])"));
}

#[test]
fn paired_and_wrong_nested_brackets() {
    assert!(!brackets_are_balanced("[({]})"));
}

#[test]
fn paired_and_incomplete_brackets() {
    assert!(!brackets_are_balanced("{}["));
}

#[test]
fn too_many_closing_brackets() {
    assert!(!brackets_are_balanced("[]]"));
}

#[test]
fn early_incomplete_brackets() {
    assert!(!brackets_are_balanced(")()"));
}

#[test]
fn early_mismatched_brackets() {
    assert!(!brackets_are_balanced("{)()"));
}

#[test]
fn math_expression() {
    assert!(brackets_are_balanced("(((185 + 223.85) * 15) - 543)/2"));
}

#[test]
fn complex_latex_expression() {
    let input = "\\left(\\begin{array}{cc} \\frac{1}{3} & x\\\\ \\mathrm{e}^{x} &... x^2 \
                 \\end{array}\\right)";

    assert!(brackets_are_balanced(input));
}

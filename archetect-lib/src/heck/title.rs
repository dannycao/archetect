/// This trait defines a title case conversion.
///
/// In Title Case, word boundaries are indicated by spaces, and every word is
/// capitalized.
///
/// ## Example:
///
/// ```rust
/// fn main() {
///     
///     use archetect::heck::TitleCase;
///
///     let sentence = "We have always lived in slums and holes in the wall.";
///     assert_eq!(sentence.to_title_case(), "We Have Always Lived In Slums And Holes In The Wall");
/// }
/// ```
pub trait TitleCase: ToOwned {
    /// Convert this type to title case.
    fn to_title_case(&self) -> Self::Owned;
}

impl TitleCase for str {
    fn to_title_case(&self) -> String {
        crate::heck::transform(self, crate::heck::capitalize, |s| s.push(' '))
    }
}

#[cfg(test)]
mod tests {
    use super::TitleCase;

    macro_rules! t {
        ($t:ident : $s1:expr => $s2:expr) => {
            #[test]
            fn $t() {
                assert_eq!($s1.to_title_case(), $s2)
            }
        };
    }

    t!(test1: "PascalCase" => "Pascal Case");
    t!(test2: "This is Human case." => "This Is Human Case");
    t!(test3: "MixedUP PascalCase, with some Spaces" => "Mixed Up Pascal Case With Some Spaces");
    t!(test4: "mixed_up_ snake_case, with some _spaces" => "Mixed Up Snake Case With Some Spaces");
    t!(test5: "train-case" => "Train Case");
    t!(test6: "CONSTANT_CASE" => "Constant Case");
    t!(test7: "snake_case" => "Snake Case");
    t!(test8: "this-contains_ ALLKinds OfWord_Boundaries" => "This Contains All Kinds Of Word Boundaries");
    t!(test9: "XΣXΣ baﬄe" => "Xσxς Baﬄe");
    t!(test10: "XMLHttpRequest" => "Xml Http Request");
    t!(test11: "package.case" => "Package Case");
    t!(test12: "directory/case" => "Directory Case");
}
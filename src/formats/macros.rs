#[macro_export]
macro_rules! chain {
    ($first:expr $(, $rest:expr)+ $(,)?) => {{
        let chained = $first $(.chain($rest))*;
        chained
    }};
}

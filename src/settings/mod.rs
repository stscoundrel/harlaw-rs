pub struct ContentReplace<'a> {
    pub search: &'a str,
    pub replace: &'a str,
}

pub struct HarlawSettings<'a> {
    pub replaces: Vec<ContentReplace<'a>>,    
    pub removes: Vec<&'a str>,
}
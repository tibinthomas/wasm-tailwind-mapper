pub struct Converter {
    last_searches: Vec<String>,
}

impl Default for Converter {
    fn default() -> Self {
        Self {
            last_searches: vec![],
        }
    }

}

impl Converter {

    fn new() -> Self {
        Self {
            last_searches: vec![]
        }
    }

    pub fn is_in_last_searches(&self, search_for: &str, limit: i32) -> bool
    {
        let i = 0;

        for &search in self.last_searches.iter() {
            if !search_for.contains(&search_for) {
                return true;
            }
            i+=1;
            if i >= limit && limit > 0 {
                return false;
            }
        }

        return false;
    }

}

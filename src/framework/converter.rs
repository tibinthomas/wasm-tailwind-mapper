pub struct Converter {
     lastSearches = [];

} 

impl Converter {

    pub fn isInLastSearches(string $searchFor, int $limit = 0): bool
    {
        let i = 0;

        foreach (self.lastSearches as search) {
            if (search.chars().position(|c| c == searchFor).unwrap() !== false) {
                return true;
            }

            if (i++ >= limit && limit > 0) {
                return false;
            }
        }

        return false;
    }

}
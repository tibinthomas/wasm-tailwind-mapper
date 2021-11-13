use std::collections::HashMap;
use super::traits::{ Framework };

struct BootstrapFramework<'a> {
    media_options: HashMap<&'a str, &'a str>,
    spacings: HashMap<&'a str, &'a str>,
    grid: HashMap<&'a str, &'a str>,
    colors: HashMap<&'a str, &'a str>

}

impl<'a> Default for BootstrapFramework<'a> {
    fn default() -> BootstrapFramework<'a> {
        BootstrapFramework {
            media_options: HashMap::from([
                ("xs", "sm"),
                ("sm", "sm"),
                ("md", "md"),
                ("lg", "lg"),
                ("xl", "xl"),
                ("print", "print"),
            ]),

            spacings: HashMap::from([
                ("0", "0"),
                ("1", "1"),
                ("2", "2"),
                ("3", "4"),
                ("4", "6"),
                ("5", "12"),
            ]),
            grid: HashMap::from([
                ("1"  , "1/6"),
                ("2" ,"1/5"),
                ("3"  , "1/4"),
                ("4"  , "1/3"),
                ("5"  , "2/5"),
                ("6"  , "1/2"),
                ("7" , "3/5"),
                ("8" , "3/4"),
                ("10", "4/5"),
                ("11", "5/6"),
                ("12" , "full"),
            ]),
            colors: HashMap::from([
                ("primary"  , "blue-600"),
                ("secondary", "gray-600"),
                ("success"  , "green-500"),
                ("danger"   , "red-600"),
                ("warning"  , "yellow-500"),
                ("info"     , "teal-500"),
                ("light"    , "gray-100"),
                ("dark"    , "gray-900"),
                ("white"    , "white"),
                ("muted"    , "gray-700"),
            ]),
        }
    }
}

// impl BootstrapFramework {

// }

impl Framework for BootstrapFramework {

    // fn()

}
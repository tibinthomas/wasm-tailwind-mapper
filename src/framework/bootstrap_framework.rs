use std::{collections::HashMap, hash::Hash};
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

impl<'a> BootstrapFramework<'a> {
    fn forms() -> HashMap<&'a str, &'a str>{
        HashMap::from([
            ("form-group"         , "mb-4"),
            ("form-control"       , "block appearance-none w-full py-1 px-2 mb-1 text-base leading-normal bg-white text-gray-800 border border-gray-200 rounded"),
            ("form-control-lg"    , "py-2 px-4 text-lg leading-normal rounded"),
            ("form-control-sm"    , "py-1 px-2 text-sm leading-normal rounded"),
            ("form-control-file"  , "block appearance-none"),
            ("form-control-range" , "block appearance-none"),
            ("form-inline" , "flex items-center"),

            ("col-form-label"    , "pt-2 pb-2 mb-0 leading-normal"),
            ("col-form-label-lg" , "pt-3 pb-3 mb-0 leading-normal"),
            ("col-form-label-sm" , "pt-1 pb-1 mb-0 leading-normal"),

            ("col-form-legend"    , "pt-2 pb-2 mb-0 text-base"),
            ("col-form-plaintext" , "pt-2 pb-2 mb-0 leading-normal bg-transparent border-transparent border-r-0 border-l-0 border-t border-b"),

            ("form-text"        , "block mt-1"),
            ("form-row"         , "flex flex-wrap -mr-1 -ml-1"),
            ("form-check"       , "relative block mb-2"),
            ("form-check-label" , "text-gray-700 pl-6 mb-0"),
            ("form-check-input" , "absolute mt-1 -ml-6"),

            ("form-check-inline" , "inline-block mr-2"),
            ("valid-feedback"    , "hidden mt-1 text-sm text-green"),
            ("valid-tooltip"     , "absolute z-10 hidden w-4 font-normal leading-normal text-white rounded p-2 bg-green-700"),
            ("is-valid"          , "bg-green-700"),
            ("invalid-feedback"  , "hidden mt-1 text-sm text-red"),
            ("invalid-tooltip"   , "absolute z-10 hidden w-4 font-normal leading-normal text-white rounded p-2 bg-red-700"),
            ("is-invalid"        , "bg-red-700"),
        ])
    }
    
    fn input_groups() -> HashMap<&'a str, &'a str> {
        HashMap::from([
            ("input-group"         , "relative flex items-stretch w-full"),
            ("input-group-addon"   , "py-1 px-2 mb-1 text-base font-normal leading-normal text-gray-900 text-center bg-gray-300 border border-4 border-gray-100 rounded"),
            ("input-group-addon-lg", "py-2 px-3 mb-0 text-lg"),
            ("input-group-addon-sm", "py-3 px-4 mb-0 text-lg"),
        ])
    }

    fn pagination() -> HashMap<&'a str, &'a str> {
        HashMap::from([
            ("pagination"   , "flex list-reset pl-0 rounded"),
            ("pagination-lg", "text-xl"),
            ("pagination-sm", "text-sm"),
            ("page-link"    , "relative block py-2 px-3 -ml-px leading-normal text-blue bg-white border border-gray-200 no-underline hover:text-blue-800 hover:bg-gray-200"),
            // 'page-link' => 'relative block py-2 px-3 -ml-px leading-normal text-blue bg-white border border-gray-',
        ])
    }

}

impl<'a> Framework<'a> for BootstrapFramework<'a> {

    fn default_css() -> HashMap<&'a str, &'a str>{
        HashMap::from([
            ("h1", ""),
            ("fieldset", ""),
            ("del", ""),
            ("a" ,""),
            ("p" ,""),
        ])
    }

    fn framework_name() -> &'static str {
        "Bootstrap"
    }

    fn supported_version() -> Vec<&'static str> {
        vec![
            "4.4.1", // bootstrap
            "1.4.0", //tailwind
        ]
    }

    // fn get() {
    //     // Option()
    // }

    fn get() -> Option<std::string::String> { todo!() }
}
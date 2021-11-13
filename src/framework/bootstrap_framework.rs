use std::{collections::HashMap, hash::Hash};
use super::traits::{ Framework };
use super::converter::Converter;

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
            // "page-link" , "relative block py-2 px-3 -ml-px leading-normal text-blue bg-white border border-gray-",
        ])
    }

    fn general(&self) -> HashMap<&'a str, &'a str> {
        let main_classes = HashMap::from([
            ("container-fluid", "container max-w-full mx-auto sm:px-4"),
            ("container",  {
                if Converter::isInLastSearches("jumbotron", 1) {
                   "container mx-auto max-w-2xl sm:px-4"
                } else {
                 "container mx-auto sm:px-4"
            }}),

            //http://getbootstrap.com/docs/4.0/utilities/embed/
            ("embed-responsive"       , ""),
            ("embed-responsive-item"  , ""),
            ("embed-responsive-21by9" , ""),
            ("embed-responsive-16by9" , ""),
            ("embed-responsive-4by3"  , ""),
            ("embed-responsive-1by1"  , ""),

            // http://getbootstrap.com/docs/4.0/utilities/image-replacement/
            ("text-hide" , ""),

            // http://getbootstrap.com/docs/4.0/utilities/screenreaders/
            ("sr-only"           , "sr-only"),
            ("sr-only-focusable" , "focus:not-sr-only"),

            // http://getbootstrap.com/docs/4.0/content/images/
            ("img-fluid"     , "max-w-full h-auto"),
            ("img-thumbnail" , "max-w-full h-auto border-1 border-gray-200 rounded p-1"),

            //http://getbootstrap.com/docs/4.0/content/tables/
            ("table"    , "w-full max-w-full mb-4 bg-transparent"),
            ("table-sm", "p-1"),
            // 'table-bordered' , '',
            // 'table-striped' , "",
            ("table-responsive"                , "block w-full overflow-auto scrolling-touch"),
            ("table-responsive-{regex_string}" , "block w-full overflow-auto scrolling-touch"),

            //http://getbootstrap.com/docs/4.0/content/figures/
            ("figure"         , "inline-block mb-4"),
            ("figure-img"     , "mb-2 leading-none"),
            ("figure-caption" , "text-gray-"),

            ("fade"     , "opacity-0"),
            ("show"     , "opacity-100 block"), //need to be checked
            ("disabled" , "opacity-75"),

            //http://getbootstrap.com/docs/4.0/components/collapse/
            // "collapse" , "hidden",
            ("collapsing" , "relative h-0 overflow-hidden "), //there should be a h-0

            //http://getbootstrap.com/docs/4.0/utilities/close-icon/
            ("close" , "absolute top-0 bottom-0 right-0 px-4 py-3"),

            //http://getbootstrap.com/docs/4.0/components/jumbotron/
            ("jumbotron"       , "py-8 px-4 md:py-16 md:px-8 mb-8 bg-gray-200 rounded"),
            ("jumbotron-fluid" , "pr-0 pl-0 rounded-none"),
        ]);

        let main_classes_each_screen = HashMap::from([
            ("container-{screen}"       , "container min-w-{screen} mx-auto sm:px-4")
        ]);

        let items = vec![];
        foreach (main_classes as bt_class ,tw_class) {
            items[bt_class] = tw_class;
        }

        foreach (main_classes_each_screen as bt_class , tw_class) {
            foreach (self.media_options as bt_media , tw_media) {
                items[str_replace("{screen}", bt_media, bt_class)] = str_replace("{screen}", tw_media, tw_class);
            }
        }

        return items;
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
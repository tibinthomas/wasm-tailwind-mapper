use std::{collections::HashMap, hash::Hash};
use super::traits::{ Framework };
use super::converter::Converter;

extern crate regex;
use regex::Regex;

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
        let converter = Converter {
            last_searches: vec![],
        };
        let main_classes = HashMap::from([
            ("container-fluid", "container max-w-full mx-auto sm:px-4"),
            ("container",  {
                if converter.is_in_last_searches("jumbotron", 1) {
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
            // "table-bordered" , "",
            // "table-striped" , "",
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

        let items: HashMap<&str, &str> = HashMap::new();
        for (bt_class, tw_class) in main_classes {
            items.insert(bt_class, tw_class);
        }

        
        for (bt_class, tw_class) in main_classes_each_screen {
            for (bt_media, tw_media) in self.media_options {
                items.insert(
                    &(str::replace("{screen}", bt_media, bt_class)), 
                    &(str::replace("{screen}", tw_media, tw_class))
                );
            }
        }
        return items;
    }

    fn grid(&self)
    {
        let items = HashMap::from([
            ("row" , "flex flex-wrap "),
            ("col" , "relative flex-grow max-w-full flex-1 px-4"),
        ]);

        //col-(xs|sm|md|lg|xl) = (sm|md|lg|xl):flex-grow
        //ml-(xs|sm|md|lg|xl)-auto = (sm|md|lg|xl):mx-auto:ml-auto
        //mr-(xs|sm|md|lg|xl)-auto = (sm|md|lg|xl):mx-auto:mr-auto
        for (btMedia, twMedia) in self.media_options {
            items.insert("col-"+btMedia, "relative "+twMedia+":flex-grow "+twMedia+":flex-1");
            items.insert("ml-"+btMedia+"-auto", twMedia+":ml-auto");
            items.insert("mr-"+btMedia+"-auto", twMedia+":mr-auto");

            //col-btElem
            //col-(xs|sm|md|lg|xl)-btElem = (sm|md|lg|xl):w-twElem
            //offset-(xs|sm|md|lg|xl)-btElem = (sm|md|lg|xl):mx-auto
            for (btElem, twElem) in self.grid {
                if (btMedia == "xs") {
                    items.insert("col-"+btElem, "w-"+twElem);
                }

                items.insert("col-"+btMedia+"-".btElem, wMedia+":w-"+twElem+" pr-4 pl-4");

                //might work :)
                items.insert("offset-"+btMedia+"-"+btElem, twMedia+":mx-"+twElem);
            }
        }

        return items;
    }

     fn media_object() -> HashMap<&str, &str>
    {
        //http://getbootstrap.com/docs/4.0/layout/media-object/
        HashMap::from([
            ("media"      , "flex items-start"),
            ("media-body" , "flex-1"),
        ])
    }

     fn borders(&self)
    {
        let items = HashMap::new();

        for (btSide , twSide) in HashMap::from([
            ("top" , "t"),
            ("right" , "r"),
            ("bottom" , "b"),
            ("left" , "l"),
        ]) {
            items.insert("border-"+btSide, "border-"+twSide);
            items.insert("border-"+btSide+"-0", "border-"+twSide+"-0");
        }

        for (btColor , twColor) in self.colors {
            items["border-"+btColor] = "border-"+twColor;
        }

        for (btStyle , twStyle) in HashMap::from([
            ("top" , "t"),
            ("right" , "r"),
            ("bottom" , "b"),
            ("left" , "l"),
            ("circle" , "full"),
            ("pill" , "full py-2 px-4"),
            ("0" , "none"),
        ]) {
            items["rounded-"+btStyle] = "rounded-"+twStyle;
        }

        return items;
    }

     fn colors(&self)
    {
        items = [];

        for (btColor , twColor) in self.colors {
            items["text-"+btColor] = "text-"+twColor;
            items["bg-"+btColor] = "bg-"+twColor;
            items["table-"+btColor] = "bg-"+twColor;
            // items["bg-gradient-"+btColor] = "bg-"+twColor;
        }

        return items;
    }

     fn display(&self)
    {
        //.d-none
        //.d-{sm,md,lg,xl}-none
        items = [];

        for (btElem , twElem) in HashMap::from([
            ("none" , "hidden"),
            ("inline" , "inline"),
            ("inline-block" , "inline-block"),
            ("block" , "block"),
            ("table" , "table"),
            ("table-cell" , "table-cell"),
            ("table-row" , "table-row"),
            ("flex" , "flex"),
            ("inline-flex" , "inline-flex"),
        ]) {
            items["d-"+btElem] = twElem;

            for (btMedia , twMedia) in self.media_options {
                items["d-"+btMedia+"-"+btElem] = twMedia+":"+twElem;
            }
        }

        return items;
    }

     fn flexElements(&self)
    {
        let items = HashMap::new();
        let media_options = self.media_options.clone();
        media_object().insert("", "");

        for (bt_media , tw_media) in media_options {
            for (key, _) in HashMap::from([("row", "row-reverse"), ("column", "column-reverse")]) {
                items.insert((("flex" + if bt_media.is_empty()  { "" } else { "-" }) + bt_media + "-" + key),
                (if tw_media.is_empty() { "" } else { tw_media + ":" } + "flex-" + str::replace(key, "column", "col")))
            }

            for key in ["grow-0", "grow-1", "shrink-0", "shrink-1"] {
                items.insert(
                    "flex" + if bt_media.is_empty() { "" } else { "-" } + bt_media + "-" + key,
                    if tw_media.is_empty() { "" } else { ":" } + "flex" + str::replace(key, "-1", "")
                    )
            }

            for key in ["start", "end", "center", "between", "around"].iter() {
                items.insert(
                    "justify-content" + if bt_media.is_empty() { "" } else { "-" } + bt_media + "-" + key,
                    if tw_media.is_empty() { "" } else {tw_media + ":" } + "justify-" + key
                    )
            }

            for key in ["start", "end", "center", "stretch", "baseline"].iter() {
                items.insert(
                    "align-items" + if bt_media.is_empty() { "" } else { "-" } + bt_media + "-" + key,
                    if tw_media.is_empty() { "" } else {tw_media + ":" } + "items-" + key
                    );
            }

            for key in ["start", "end", "center", "stretch", "baseline"].iter() {
                items.insert(
                    "align-content" + if bt_media.is_empty() { "" } else { "-" } + bt_media + "-" + key,
                    if tw_media.is_empty() { "" } else {tw_media + ":" } + "content-" + key
                    );
            }

            for key in ["start", "end", "center", "stretch", "baseline"].iter() {
                items.insert(
                    "align-self" + if bt_media.is_empty() { "" } else { "-" } + bt_media + "-" + key,
                    if tw_media.is_empty() { "" } else {tw_media + ":" } + "self-" + key
                    );
            }

            items.insert(
                "flex" + if bt_media.is_empty() { "" } else { "-" } + bt_media + "-wrap",
                if tw_media.is_empty() { "" } else {tw_media + ":" } + "flex-wrap"
                );

            items.insert(
                "flex" + if bt_media.is_empty() { "" } else { "-" } + bt_media + "-wrap-reverse",
                if tw_media.is_empty() { "" } else {tw_media + ":" } + "flex-wrap-reverse"
                );

            items.insert(
                "flex" + if bt_media.is_empty() { "" } else { "-" } + bt_media + "-nowrap",
                if tw_media.is_empty() { "" } else {tw_media + ":" } + "flex-no-wrap"
                );


            // items["flex".(empty(btMedia) ? "" : "-").btMedia."-wrap"] = (empty(twMedia) ? "" : twMedia.":")."flex-wrap";
            // items["flex".(empty(btMedia) ? "" : "-").btMedia."-wrap-reverse"] = (empty(twMedia) ? "" : twMedia.":")."flex-wrap-reverse";
            // items["flex".(empty(btMedia) ? "" : "-").btMedia."-nowrap"] = (empty(twMedia) ? "" : twMedia.":")."flex-no-wrap";

            // items["flex".(empty(btMedia) ? "" : "-").btMedia."-nowrap"] = (empty(twMedia) ? "" : twMedia.":")."flex-no-wrap";

            if (btMedia != "") {
                items.insert(
                    "order-" + bt_media+ "-{regex_number}",
                    tw_media + ":order-{regex_number}"
                    )
                // items["order-".btMedia."-{regex_number}"] = twMedia.":order-{regex_number}";
            }
        }

        return items;
    }

     fn sizing()
    {
        let items: [&str; _] = ["sdr"];

        foreach (HashMap::from[
            ("25" , "1/4"),
            ("50" , "1/2"),
            ("75" , "3/4"),
            ("100" , "full"),
        ] as btClass , twClass) {
            items["w-".btClass] = "w-".twClass;

            //no percentages in TW for heights except for full
            if (btClass == 100) {
                items["h-".btClass] = "h-".twClass;
            }
        }

        items["mw-100"] = "max-w-full";
        items["mh-100"] = "max-h-full";

        return items;
    }

     fn spacing(&self)
    {
        let items = HashMap::new();
        let spacing_properties = ["p", "m"];

        for property in spacing_properties.iter() {
            for (bt_spacing, tw_spacing) in self.spacings {
                items.insert(property + "-" + bt_spacing, property + "-" + twSpacing);
            }
        }

        for property in spacing_properties.iter() {
            for (bt_media , tw_media) in  self.media_options {
                for (bt_spacing , tw_spacing) in self.spacings {
                    items.insert(property + "-" +bt_media+ "-" +bt_spacing, tw_media + ":" + property + "-" + tw_spacing);
                    items.insert(property + "{regex_string}-" + bt_media + "-" + bt_spacing, tw_media + ":" + property+ "{regex_string}-" + tw_spacing);
                }

                items.insert(property + "{regex_string}-" + bt_media + "-auto", tw_media + ":" + property + "{regex_string}-auto");
            }
        }
         items
    }

     fn text()
    {
        items = HashMap::from([
            ("text-nowrap"   , "whitespace-nowrap"),
            ("text-truncate" , "truncate"),

            ("text-lowercase"  , "lowercase"),
            ("text-uppercase"  , "uppercase"),
            ("text-capitalize" , "capitalize"),

            ("initialism" , ""),
            ("lead"       , "text-xl font-light"),
            ("small"      , "text-xs"),
            ("mark"       , ""),
            ("display-1"  , "text-xl"),
            ("display-2"  , "text-2xl"),
            ("display-3"  , "text-3xl"),
            ("display-4"  , "text-4xl"),

            ("h-1" , "mb-2 font-medium leading-tight text-4xl"),
            ("h-2" , "mb-2 font-medium leading-tight text-3xl"),
            ("h-3" , "mb-2 font-medium leading-tight text-2xl"),
            ("h-4" , "mb-2 font-medium leading-tight text-xl"),
            ("h-5" , "mb-2 font-medium leading-tight text-lg"),
            ("h-6" , "mb-2 font-medium leading-tight text-base"),

            ("blockquote"        , "mb-6 text-lg"),
            ("blockquote-footer" , "block text-gray-"),

            ("font-weight-bold"   , "font-bold"),
            ("font-weight-normal" , "font-normal"),
            ("font-weight-300"    , "font-light"),
            ("font-italic"        , "italic"),
        ]);

        foreach (["left", "right", "center", "justify"] as alignment) {
            foreach (array_merge(this->mediaOptions, ["",""]) as btMedia , twMedia) {
                items["text".(empty(btMedia) ? "" : "-".btMedia)."-".alignment] = (empty(twMedia) ? "" : twMedia.":")."text-".alignment;
            }
        }

        return items;
    }

     fn floats()
    {
        items = [];

        foreach (this->mediaOptions as btMedia , twMedia) {
            foreach (["left", "right", "none"] as alignment) {
                items["float-".btMedia."-".alignment] = twMedia.":float-".alignment;
            }
        }

        return items;
    }

     fn positioning()
    {
        items = [];

        foreach (HashMap::from[
            ("position-static" , "static"),
            ("position-relative" , "relative"),
            ("position-absolute" , "absolute"),
            ("position-fixed" , "fixed"),
            ("position-sticky" , ""),
            ("fixed-top" , "top-0"),
            ("fixed-bottom" , "bottom-0"),
        ] as btPosition , twPosition) {
            items[btPosition] = twPosition;
        }

        return items;
    }

     fn verticalAlignment()
    {
        //same
        items = [];
        // foreach ([
        //     "baseline", "top", "middle", "bottom", "text-top", "text-bottom"
        // ] as btAlign, twAlign) {
        //     items["align-".btAlign] = 'align-".twAlign;
        // }
        return items;
    }

     fn visibility()
    {
        //same
        return [];
    }

     fn alerts()
    {
        items = HashMap::from[
            ("alert"             , "relative px-3 py-3 mb-4 border rounded"),
            ("alert-heading"     , ""), //color: inherit
            ("alert-link"        , "font-bold no-underline text-current"),
            ("alert-dismissible" , ""),
        ];

        colors = HashMap::from([
            ("primary"   , "bg-blue-200 border-blue-300 text-blue-800"),
            ("secondary" , "bg-gray-300 border-gray-400 text-gray-800"),
            ("success"   , "bg-green-200 border-green-300 text-green-800"),
            ("danger"    , "bg-red-200 border-red-300 text-red-800"),
            ("warning"   , "bg-orange-200 border-orange-300 text-orange-800"),
            ("info"      , "bg-teal-200 border-teal-300 text-teal-800"),
            ("light"     , "bg-white text-gray-600"),
            ("dark"      , "bg-gray-400 border-gray-500 text-gray-900"),
        ]);

        foreach (colors as btColor , twColor) {
            items["alert-".btColor] = twColor;
        }

        return items;
    }

     fn badges()
    {
        items = HashMap::from([
            ("badge"      , "inline-block p-1 text-center font-semi-bold text-sm align-baseline leading-none rounded"),
            ("badge-pill" , "rounded-full py-1 px-3"),
        ]);

        colors = HashMap::from([
            ("primary"   , "bg-blue-500 text-white hover:bg-blue-600"),
            ("secondary" , "bg-gray-600 text-white hover:bg-gray-700"),
            ("success"   , "bg-green-500 text-white hover:green-600"),
            ("danger"    , "bg-red-600 text-white hover:bg-red-700"),
            ("warning"   , "bg-orange-400 text-black hover:bg-orange-500"),
            ("info"      , "bg-teal-500 text-white hover:bg-teal-600"),
            ("light"     , "bg-gray-100 text-gray-800 hover:bg-gray-200"),
            ("dark"      , "bg-gray-900 text-white"),
        ]);

        foreach (colors as btColor , twColor) {
            items["badge-".btColor] = twColor;
        }

        return items;
    }

     fn breadcrumb()
    {
        return HashMap::from([
            ("breadcrumb"     , "flex flex-wrap list-reset pt-3 pb-3 py-4 px-4 mb-4 bg-gray-200 rounded"),
            ("breadcrumb-item", "inline-block px-2 py-2 text-gray-700"),
        ]);
    }

     fn buttons()
    {
        let items = HashMap::from([
            ("btn"                , "inline-block align-middle text-center select-none border font-normal whitespace-no-wrap rounded {tailwindo|py-1 px-3 leading-normal} no-underline"),
            ("btn-group"          , "relative inline-flex align-middle"),
            ("btn-group-vertical" , "relative inline-flex align-middle flex-col items-start justify-center"),
            ("btn-toolbar"        , "flex flex-wrap justify-start"),
            ("btn-link"           , "font-normal text-blue-700 bg-transparent"),
            ("btn-block"          , "block w-full"),
        ]);

        for (bt_media, ts_classes) in [
            ("sm" , "{tailwindo|py-1 px-2 leading-tight} text-xs "),
            ("lg" , "{tailwindo|py-3 px-4 leading-tight} text-xl"),
        ].iter() {
            items.insert("btn-" + bt_media, tw_classes);
            items.insert("btn-group-" + bt_media, tw_classes);
        }

        colors = HashMap::from([
            ("primary"   , "bg-blue-600 text-white hover:bg-blue-600"),
            ("secondary" , "bg-gray-600 text-white hover:bg-gray-700"),
            ("success"   , "bg-green-500 text-white hover:bg-green-600"),
            ("danger"    , "bg-red-600 text-white hover:bg-red-700"),
            ("warning"   , "bg-orange-400 text-black hover:bg-orange-500"),
            ("info"      , "bg-teal-500 text-white hover:bg-teal-600"),
            ("light"     , "bg-gray-100 text-gray-800 hover:bg-gray-200"),
            ("dark"      , "bg-gray-900 text-white hover:bg-gray-900"),
        ]);

        for (bt_color , tw_color) in colors {
            items.insert("btn-" + bt_color, tw_color);
            items.insert("btn-outline-" + bt_color, preg_replace_callback("/(?<!hover:)(text-[^\s]+|bg-[^\s]+)/i", {
                if (strpos(m[1], "bg-") !== false) {
                    color = str_replace("bg-", "", m[1]);

                    return "text-".color." border-".color." hover:bg-".color." hover:text-white";
                } else {
                    return "bg-white";
                }
            }, twColor);
        }

        return items;
    }

     fn cards()
    {
        return HashMap::from([
            ("card-deck"  , "flex flex-row flex-wrap md:flex-no-wrap -mx-1"),
            ("card-group" , "flex flex-col"),
            ("card"       , fn () {
                if (this->isInLastSearches("card-deck")) {
                    return "relative block md:flex w-full md:min-w-0 md:mx-4 flex-col flex-no-shrink flex-grow rounded break-words border bg-white border-1 border-gray-300";
                } else {
                    return "relative flex flex-col min-w-0 rounded break-words border bg-white border-1 border-gray-300";
                }
            }),
            ("card-body"         , "flex-auto p-6"),
            ("card-title"        , "mb-3"),
            ("card-text"         , "mb-0"),
            ("card-subtitle"     , "-mt-2 mb-0"),
            ("card-link"         , "ml-6"),
            ("card-header"       , "py-3 px-6 mb-0 bg-gray-200 border-b-1 border-gray-300 text-gray-900"),
            ("card-footer"       , "py-3 px-6 bg-gray-200 border-t-1 border-gray-300"),
            ("card-header-tabs"  , "border-b-0 -ml-2 -mb-3"),
            ("card-header-pills" , "-ml-3 -mr-3"),
            ("card-img-overlay"  , "absolute inset-y-0 inset-x-0 p-6"),
            ("card-img"          , "w-full rounded"),
            ("card-img-top"      , "w-full rounded rounded-t"),
            ("card-img-bottom"   , "w-full rounded rounded-b"),
        ]);
    }

     fn dropdowns()
    {
        return HashMap::from([
            ("dropdown"         , "relative"),
            ("dropup"           , "relative"),
            ("dropdown-toggle"  , " inline-block w-0 h-0 ml-1 align border-b-0 border-t-1 border-r-1 border-l-1"),
            ("dropdown-menu"    , " absolute left-0 z-50 float-left hidden list-reset	 py-2 mt-1 text-base bg-white border border-gray-300 rounded"),
            ("dropdown-divider" , "h-0 my-2 overflow-hidden border-t-1 border-gray-300"),
            ("dropdown-item"    , "block w-full py-1 px-6 font-normal text-gray-900 whitespace-no-wrap border-0"),
            ("dropdown-header"  , "block py-2 px-6 mb-0 text-sm text-gray-800 whitespace-no-wrap"),
        ]);
    }

     fn forms()
    {
        return HashMap::from([
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
        ]);
    }

     fn inputGroups()
    {
        return HashMap::from([
            ("input-group"          , "relative flex items-stretch w-full"),
            ("input-group-addon"    , "py-1 px-2 mb-1 text-base font-normal leading-normal text-gray-900 text-center bg-gray-300 border border-4 border-gray-100 rounded"),
            ("input-group-addon-lg" , "py-2 px-3 mb-0 text-lg"),
            ("input-group-addon-sm" , "py-3 px-4 mb-0 text-lg"),
        ]);
    }

     fn listGroups()
    {
        items = HashMap::from([
            ("list-group"             , "flex flex-col pl-0 mb-0 border rounded border-gray-300"),
            ("list-group-item-action" , "w-full"),
            ("list-group-item"        , "relative block py-3 px-6 -mb-px border border-r-0 border-l-0 border-gray-300 no-underline"),
            ("list-group-flush"       , ""),
        ]);

        //TODO
        foreach (this->colors as btColor , twColor) {
            if (btColor === "dark") {
                items["list-group-item-".btColor] = "text-white bg-gray-700";
            } elseif (btColor == "light") {
                items["list-group-item-".btColor] = "text-black bg-gray-200";
            } else {
                items["list-group-item-".btColor] = "bg-".twColor."-200 text-".twColor."-900";
            }
        }

        return items;
    }

     fn modals()
    {
        //TODO
        return [];
    }

     fn navs()
    {
        items = HashMap::from([
            ("nav"           , "flex flex-wrap list-none pl-0 mb-0"),
            ("nav-tabs"      , "border border-t-0 border-r-0 border-l-0 border-b-1 border-gray-200"),
            ("nav-pills"     , ""),
            ("nav-fill"      , ""),
            ("nav-justified" , ""),
        ]);

        items["nav-link"] = fn () {
            navLinkClasses = "inline-block py-2 px-4 no-underline";
            if (this->isInLastSearches("nav-tabs", 5)) {
                navLinkClasses .= " border border-b-0 mx-1 rounded rounded-t";
            } elseif (this->isInLastSearches("nav-pills", 5)) {
                navLinkClasses .= " border border-blue bg-blue rounded text-white mx-1";
            }

            return navLinkClasses;
        };

        items["nav-item"] = fn () {
            navItemClasses = "";

            if (this->isInLastSearches("nav-tabs", 5)) {
                navItemClasses .= "-mb-px";
            } elseif (this->isInLastSearches("nav-fill", 5)) {
                navItemClasses .= " flex-auto text-center";
            } elseif (this->isInLastSearches("nav-justified", 5)) {
                navItemClasses .= " flex-grow text-center";
            }

            return navItemClasses;
        };

        items["navbar"] = "relative flex flex-wrap items-center content-between py-3 px-4";
        items["navbar-brand"] = "inline-block pt-1 pb-1 mr-4 text-lg whitespace-no-wrap";
        items["navbar-nav"] = "flex flex-wrap list-reset pl-0 mb-0";
        items["navbar-text"] = "inline-block pt-2 pb-2";
        items["navbar-dark"] = "text-white";
        items["navbar-light"] = "text-black";
        items["navbar-collapse"] = "flex-grow items-center";
        items["navbar-expand"] = "flex-no-wrap content-start";
        items["navbar-expand-{regex_string}"] = "";
        items["navbar-toggler"] = "py-1 px-2 text-md leading-normal bg-transparent border border-transparent rounded";

        //for now
        items["collapse"] = "hidden";
        items["navbar-toggler-icon"] = "px-5 py-1 border border-gray-600 rounded";

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

    fn framework_name() -> &"static str {
        "Bootstrap"
    }

    fn supported_version() -> Vec<&"static str> {
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
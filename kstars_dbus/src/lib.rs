pub mod kstars {
    include!(concat!(env!("OUT_DIR"), "/kstars.rs"));
}

pub mod ekos {
    include!(concat!(env!("OUT_DIR"), "/ekos.rs"));

    pub mod capture {
        include!(concat!(env!("OUT_DIR"), "/Capture.rs"));
    }

    pub mod focus {
        include!(concat!(env!("OUT_DIR"), "/Focus.rs"));
    }

    pub mod guide {
        include!(concat!(env!("OUT_DIR"), "/Guide.rs"));
    }

    pub mod mount {
        include!(concat!(env!("OUT_DIR"), "/Mount.rs"));
    }

    pub mod align {
        include!(concat!(env!("OUT_DIR"), "/Align.rs"));
    }
}

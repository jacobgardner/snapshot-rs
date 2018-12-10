use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "yaml")] {
        mod yaml;
        pub use self::yaml::*;
    } else if #[cfg(feature = "json")] {
        pub use serde_json::*;
    } else if #[cfg(feature = "snap-ron")] {
        mod ron;

        pub use self::ron::*;
    }

}

macro_rules! cfg_resource {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "resource")]
            #[cfg_attr(docsrs, doc(cfg(feature = "resource")))]
            $item
        )*
    };
}

macro_rules! cfg_derive {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "derive")]
            #[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
            $item
        )*
    };
}

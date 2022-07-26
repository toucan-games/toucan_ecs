macro_rules! cfg_resource {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "resource")]
            #[cfg_attr(docsrs, doc(cfg(feature = "resource")))]
            $item
        )*
    };
}

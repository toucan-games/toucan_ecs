macro_rules! cfg_resource {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "resource")]
            #[cfg_attr(any(docsrs, all(doc, CHANNEL_NIGHTLY)), doc(cfg(feature = "resource")))]
            $item
        )*
    };
}

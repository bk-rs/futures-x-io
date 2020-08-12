#[cfg(all(
    feature = "futures_util_io",
    not(feature = "futures_lite_io"),
    not(feature = "tokio_io_util"),
    feature = "futures_io",
    not(feature = "tokio_io")
))]
#[cfg(test)]
#[path = "./inner_tests/io_util.rs"]
mod only_futures_util_io_tests;

#[cfg(all(
    not(feature = "futures_util_io"),
    feature = "futures_lite_io",
    not(feature = "tokio_io_util"),
    feature = "futures_io",
    not(feature = "tokio_io")
))]
#[cfg(test)]
#[path = "./inner_tests/io_util.rs"]
mod only_futures_lite_io_tests;

#[cfg(all(
    not(feature = "futures_util_io"),
    not(feature = "futures_lite_io"),
    feature = "tokio_io_util",
    not(feature = "futures_io"),
    feature = "tokio_io"
))]
#[cfg(test)]
#[path = "./inner_tests/io_util.rs"]
mod only_tokio_io_util_tests;

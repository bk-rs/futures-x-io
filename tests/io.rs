#[cfg(all(feature = "futures_io", not(feature = "tokio_io")))]
#[cfg(test)]
#[path = "./inner_tests/io.rs"]
mod only_futures_io_tests;

#[cfg(all(not(feature = "futures_io"), feature = "tokio_io"))]
#[cfg(test)]
#[path = "./inner_tests/io.rs"]
mod only_tokio_io_tests;

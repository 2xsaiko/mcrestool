#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[cxx::bridge(namespace = "mcrestool::lib")]
mod ffi {

}

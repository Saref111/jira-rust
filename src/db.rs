trait Database {
    fn read_db() -> String;
    fn write_db() -> String;
}

struct JSONFileDatabase {}

impl Database for JSONFileDatabase {
    fn read_db() -> String {
        String::from("read")
    }

    fn write_db() -> String {
        String::from("write")
    }
}

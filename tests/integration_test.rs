extern crate rustybin;

mod common;

mod all {
    use rustybin::models::bin::Bin;
    use common;

    #[test]
    fn it_should_get_all_keys_when_empty() {
        common::reset_db();

        let all = Bin::all().unwrap();
        assert_eq!(0, all.len());
    }

    #[test]
    fn it_should_get_all_keys_when_keys_exist() {
        common::reset_db();
        Bin::create().unwrap();

        let all = Bin::all().unwrap();
        assert_eq!(1, all.len());
    }
}

mod create {
    use rustybin::models::bin::Bin;
    use common;

    #[test]
    fn it_should_get_all_keys_when_empty() {
        common::reset_db();

        let new = Bin::create().unwrap();

        assert_eq!(0, new.dumps.len());
        assert_eq!(36, new.id.len());
    }
}

mod delete {
    use rustybin::models::bin::Bin;
    use common;

    #[test]
    fn it_should_delete() {
        common::reset_db();

        let new1 = Bin::create().unwrap();
        let new2 = Bin::create().unwrap();

        Bin::delete(&new1.id).unwrap();
        assert_eq!(1, Bin::all().unwrap().len());

        Bin::delete(&new2.id).unwrap();
        assert_eq!(0, Bin::all().unwrap().len());
    }
}

mod find {
    use std::collections::HashMap;
    use rustybin::models::bin::Bin;
    use rustybin::models::dump::Dump;

    use common;

    #[test]
    fn it_should_find_by_id() {
        common::reset_db();

        let bin = Bin::create().unwrap();

        let res = Bin::find(&bin.id).unwrap();

        assert_eq!(bin.id, res.id);
        assert_eq!(0, res.dumps.len());
    }

    #[test]
    fn it_should_find_with_dumps() {
        common::reset_db();

        let bin = Bin::create().unwrap();
        let uri = String::from("GET");
        let dump = Dump {
            method: String::from("GET"),
            uri: uri.clone(),
            headers: HashMap::new(),
            body: Some(String::from("GET")),
            time: 0,
        };

        // make sure it was 0 before first capture
        assert_eq!(0, Bin::find(&bin.id).unwrap().dumps.len());

        Bin::capture(bin.id.clone(), dump).unwrap();

        let res = Bin::find(&bin.id).unwrap();

        assert_eq!(bin.id, res.id);
        assert_eq!(1, res.dumps.len());
        assert_eq!(uri, res.dumps[0].uri);
    }

    #[test]
    fn it_should_not_find_by_id() {
        common::reset_db();

        let res = Bin::find("expired id");

        assert!(res.is_err());
    }
}

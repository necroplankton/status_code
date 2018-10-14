#[cfg(test)]

mod tests {

    extern crate status_code;
    use self::status_code::statuses::*;

    #[test]
    fn check_valid() {
        assert_eq!(200, status(200));
        assert_eq!(301, status(301));
        assert_eq!(418, status(418));
    }

    #[test]
    #[should_panic]
    fn check_status_panic() {
        status(444);
    }

    #[test]
    fn check_code() {
        assert_eq!("OK", code(200));
        assert_eq!("Moved Permanently", code(301));
        assert_eq!("Not Found", code(404));
        assert_eq!("I'm a teapot", code(418));
    }

    #[test]
    fn check_message() {
        assert_eq!(200, message("OK"));
        assert_eq!(301, message("moved permanently"));
        assert_eq!(418, message("I'm a teapot"));
    }

    #[test]
    #[should_panic]
    fn check_message_panic() {
        message("err input");
    }

    #[test]
    fn check_redirect() {
        assert_eq!(true, redirect(301));
        assert_eq!(false, redirect(200));
    }

    #[test]
    fn check_empty() {
        assert_eq!(true, empty(204));
        assert_eq!(false, empty(200));
    }

    #[test]
    fn check_retry() {
        assert_eq!(true, retry(502));
        assert_eq!(false, retry(200));
    }

    #[test]
    #[should_panic]
    fn check_other_panic() {
       empty(444);
    }

}

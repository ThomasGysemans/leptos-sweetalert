#[cfg(test)]
mod tests {
    use crate::SwalOptions;
    use crate::SwalIconLike;
    use crate::SwalIcon;

    #[test]
    fn test_default_swal() {
        let opts = SwalOptions::<&str>::default();
        assert_eq!(opts.title, "");
        assert_eq!(opts.text, "");
        assert_eq!(opts.icon, SwalIcon::NONE);
        assert_eq!(opts.show_confirm_button, true);
    }

    #[test]
    fn test_basic() {
        let opts = SwalOptions::<&str>::basic("Hello");
        assert_eq!(opts.title, "Hello");
        assert_eq!(opts.text, "");
    }

    #[test]
    fn test_basic_icon() {
        let opts = SwalOptions::basic_icon("Hello", SwalIcon::ERROR);
        assert_eq!(opts.icon, SwalIcon::ERROR);
    }

    #[test]
    fn test_common() {
        let opts = SwalOptions::common("Hello", "World", SwalIcon::ERROR);
        assert_eq!(opts.title, "Hello");
        assert_eq!(opts.text, "World");
        assert_eq!(opts.icon, SwalIcon::ERROR);
    }

    #[test]
    fn test_has_icon() {
        let opts = SwalOptions::basic_icon("Hello", SwalIcon::SUCCESS);
        assert!(opts.icon.is_defined());
        let opts = SwalOptions::basic_icon("Hello", SwalIcon::NONE);
        assert!(!opts.icon.is_defined());
    }

    #[test]
    fn test_has_text() {
        let opts = SwalOptions::common("Hello", "Some text", SwalIcon::INFO);
        assert!(opts.has_text());
        let opts = SwalOptions::<&str>::basic("Hello");
        assert!(!opts.has_text());
    }

    #[test]
    fn test_has_title() {
        let opts = SwalOptions::<&str>::basic("Hello");
        assert!(opts.has_title());
        let opts = SwalOptions::<&str> {
            title: "",
            text: "Hello",
            ..SwalOptions::default()
        };
        assert!(!opts.has_title());
    }

    // We make sure that this test works by panicking voluntarily.
    // It's the best way to know if the assert!(false) was called or not,
    // within the `pre_confirm` callback.
    #[test]
    #[should_panic]
    fn test_pre_confirm() {
        let opts = SwalOptions::<&str> {
            title: "Confirm this!!",
            pre_confirm: || {
                assert!(false);
            },
            ..SwalOptions::default()
        };
        (opts.pre_confirm)();
    }

    #[test]
    #[should_panic]
    fn test_pre_deny() {
        let opts = SwalOptions::<&str> {
            title: "Deny this!!",
            pre_deny: || {
                assert!(false);
            },
            ..SwalOptions::default()
        };
        (opts.pre_deny)();
    }
}

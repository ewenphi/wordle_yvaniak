mod integration {

    #[test]
    fn tui() {
        let mut cmd = assert_cmd::Command::cargo_bin("wordle").unwrap();
        let assert = cmd.args(["tui", "quitting_test"]).assert();
        assert
            .failure()
            .stderr("Problem parsing arguments: cli testing\n");
    }

    #[test]
    fn gui() {
        let mut cmd = assert_cmd::Command::cargo_bin("wordle").unwrap();
        let assert = cmd.arg("gui").assert();
        assert
            .failure()
            .stderr("Application error: tui and gui not implemented yet\n");
    }
}

pub mod session {

    use tmux_interface::TmuxCommand;

    pub fn create_or_connect(session_name: &str) {
        let tmux = TmuxCommand::new();

        let has_session = tmux
            .has_session()
            .target_session(session_name)
            .output()
            .unwrap()
            .success();

        if !has_session {
            tmux.new_session()
                .detached()
                .session_name(session_name)
                .output()
                .unwrap();
        }

        tmux.switch_client()
            .target_session(session_name)
            .output()
            .unwrap();
    }
}

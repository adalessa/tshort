pub mod session {

    use tmux_interface::{NewSession, TmuxCommand};
    use crate::project::selector::Project;

    pub fn create_or_connect(item: Project) -> bool {
        create(item.to_owned());
        connect(&item.session_name())
    }

    pub fn connect(session_name: &str) -> bool {
        let tmux = TmuxCommand::new();

        tmux.switch_client()
            .target_session(session_name)
            .output()
            .unwrap()
            .success()
    }


    pub fn create(item: Project) -> bool {
        let tmux = TmuxCommand::new();

        let has_session = tmux
            .has_session()
            .target_session(item.session_name())
            .output()
            .unwrap()
            .success();

        if !has_session {
            return NewSession::new()
                .session_name(item.session_name())
                .detached()
                .start_directory(item.path().to_str().unwrap())
                .shell_command("nvim")
                .output()
                .unwrap()
                .success()
            ;
        }

        return has_session;
    }
}

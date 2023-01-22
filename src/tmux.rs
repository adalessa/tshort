pub mod session {
    use crate::project::Project;
    use tmux_interface::{NewSession, TmuxCommand};

    pub fn exists(session_name: &str) -> bool {
        TmuxCommand::new()
            .has_session()
            .target_session(session_name)
            .output()
            .unwrap()
            .success()
    }

    pub fn connect(session_name: &str) -> bool {
        TmuxCommand::new()
            .switch_client()
            .target_session(session_name)
            .output()
            .unwrap()
            .success()
    }

    pub fn create(item: Project) -> bool {
        if !exists(&item.session_name()) {
            return NewSession::new()
                .session_name(item.tmux_display())
                .detached()
                .start_directory(item.path().to_str().unwrap())
                .shell_command(std::env::var("EDITOR").unwrap_or("nvim".to_string()))
                .output()
                .unwrap()
                .success();
        }

        return true;
    }

    pub fn connect_or_create(item: Project) -> bool {
        create(item.to_owned());
        connect(&item.tmux_display())
    }
}
